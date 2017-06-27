/*
*Title: AI_Utils.rs
*Author: John B. Casey <github:caseyj><email: caseyjohnb@gmail.com>
*Language: Rust
*Description: Tools needed to run the AI system and various functions which
	assist in those purposes.
*
*/
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::iter::Iterator;

use minion_card::{UID, Minion};
use card::Card;
use std::collections::{HashSet, HashMap, BinaryHeap};
use rand::{thread_rng, Rng};
use controller::Controller;
use game_state::GameStateData;
use rune_vm::Rune;
use runes::play_minion::PlayMinion;
use runes::play_card::PlayCard;
use rustc_serialize::json;
use client_option::{ClientOption, OptionType, OptionsPackage};
use client_message::OptionsMessage;


/**
*Formats a known gamestate and a rune that should run into a json friendly
*	string that can be sent to the game thread to be updated
*/
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct AI_Update_Request{
	pub game_state_data : GameStateData,
	pub rune : String,

}
impl AI_Update_Request{
	pub fn new(gsd : GameStateData, rne: String)->AI_Update_Request{
		AI_Update_Request{
			game_state_data : gsd,
			rune: rne,
		}
	}

	pub fn toJson(&self)->String{
		let mut msg : String = json::encode(self).unwrap();
		let mut front= "{\"message_type\":\"AIPlay\",";
		let sendMsg = format!("{}{}", front, &msg.clone()[1..msg.len()]);
		return sendMsg;

	}
}


/**
*Formats a known gamestate and an option set that should run into a json friendly
*	string that can be sent to the game thread to be updated
*/
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct AI_Option_Set_Request{
	pub game_state_data : GameStateData,
	pub theo_options : Vec<ClientOption>,
}
impl AI_Option_Set_Request{
	pub fn new(gsd : GameStateData, ops : Vec<ClientOption>)->AI_Option_Set_Request{
		AI_Option_Set_Request{
			game_state_data: gsd,
			theo_options : ops,
		}
	}
	pub fn toJson(&self)->String{
		let msg : String = json::encode(self).unwrap();
		let front= "{\"message_type\":\"OptionsSimulation\",";
		let sendMsg = format!("{}{}", front, &msg.clone()[1..msg.len()]);
		return sendMsg;
	}
	pub fn from_json(json_message: String)->AI_Option_Set_Request{
		let msg = json_message.replace("{\"message_type\":\"OptionsSimulation\",", "{");
		let ops_set : AI_Option_Set_Request = json::decode(msg.trim()).unwrap();
		return ops_set;
	}
}

/*
*Takes in a given&known set of options and splits them by type
*	This allows other processes to operate on different option
*	sets in parallel
*/
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct OpsClassify {
	pub end : ClientOption,
	pub plays : Vec<ClientOption>,
	pub attacks  : Vec<ClientOption>,
}

impl OpsClassify {
	pub fn new(ops_list : OptionsPackage)->OpsClassify{

		let mut end_op : ClientOption = ops_list.options[0].clone();

		let mut play_ops : Vec<ClientOption> = Vec::new();
		let mut attack_ops : Vec<ClientOption> = Vec::new();
		for i in 0..(ops_list.options.len()-1){
			match ops_list.options[i].option_type{
				OptionType::EPlayCard=> play_ops.push(ops_list.options[i].clone()),
				OptionType::EAttack=> attack_ops.push(ops_list.options[i].clone()),
				OptionType::EEndTurn=> end_op = ops_list.options[i].clone(),
			}		
		}

		OpsClassify{
			end : end_op,
			plays : play_ops,
			attacks : attack_ops,
		}
	}
}


pub struct AI_Player{
	pub game_state_data : GameStateData,
	pub score : f32,
	pub public_runes : Vec<String>,
	pub update_count : u32,
	pub options_order : Vec<ClientOption>,
	pub options_test_recieved : bool,
	pub ops_recieved : OptionsPackage,
	pub iterative : usize,
}
impl AI_Player{
	pub fn new()->AI_Player{
		let mut gsd = GameStateData::new(true);
		let mut scre = 0.0 as f32;
		let mut pr = Vec::new();
		let mut uc = 0;
		let options_test_recieved_false = false;
		gsd.get_uid();
		gsd.get_uid();
		AI_Player{
			game_state_data : gsd.clone() ,
			score : scre,
			public_runes : pr ,
			update_count : uc,
			options_order : Vec::new(),
			options_test_recieved : options_test_recieved_false,
			ops_recieved : OptionsPackage{options : Vec::new()},
			iterative: 0,
		}
	}

	pub fn gsd_from_json(json_message : String)->GameStateData{
		let response = json_message.clone().replace("{\"message_type\": \"AI_Update\",", "{" );
        return json::decode(response.trim()).unwrap();
	}

	/*
	*Updates the AI_Player using a json string of GameStateData
	*/
	pub fn update(&mut self, updateData: String){
        self.game_state_data = AI_Player::gsd_from_json(updateData); 
		self.update_count = self.update_count + 1;
		if self.update_count > 1{
			self.score = score_controllers(&self.game_state_data);
			println!("UPDATED SCORE: {}", self.score.clone());
		}
	}
	/**
	*Enqueues a rune to the update list
	*/
	pub fn queue_update(&mut self, rune : String){
		self.public_runes.push(rune)
	}

	/*
	*Takes an options package given by the server and generates responses
	*/
	pub fn option_engine(&mut self){
		self.options_order = vec![];
		println!("AI options selections");
		let ops_classi = OpsClassify::new(self.ops_recieved.clone());
		println!("Options classified");
		self.options_test_recieved = true;
		let mut matr = CardPlayMatrix::new(ops_classi.plays.clone(), self.game_state_data.clone());
		println!("Running matrix");
		if ops_classi.plays.len() > 0{
			matr.run_matrix();
			for i in matr.selected_ops{
				self.options_order.push(i)
			}
		}
		println!("Running attack Heap");
		if ops_classi.attacks.len() > 0{
			let mut att_heap = AttackHeap::new(self.game_state_data.clone(), ops_classi.attacks);
			self.options_order.push(att_heap.pop_attack());
		}
		self.options_order.push(ops_classi.end);
		let n_pack = OptionsPackage{options: self.options_order.clone()};
		println!("OPS SELECTED {}", n_pack.to_json());
		self.iterative = 0;
	}
	pub fn prep_option(){
		
	}

	pub fn iter_up(&mut self){
		self.iterative = self.iterative + 1;
	}
}

/*
*gets how much active AP is on the field for a given board. 
*
*
*/
fn getAP_field(ref controller: &Controller, game: &GameStateData) -> u32{

	let mut this_AP = 0;
	//get the vectors of uuids of the minions on this part of the field.
	let field = controller.get_copy_of_in_play();
	//iterate through the list of uids detected.
	for i in field{
		this_AP += game.get_minion(i).unwrap().get_current_attack();
	}
	return this_AP;
}

fn getHP_field(ref controller: &Controller, game: &GameStateData) -> u32{

	let mut this_AP = 0;
	//get the vectors of uuids of the minions on this part of the field.
	let field = controller.get_copy_of_in_play();
	//iterate through the list of uids detected.
	for i in field{
		this_AP += game.get_minion(i).unwrap().get_current_health();
	}
	return this_AP;
}
//basic for now add taunts later
fn score_controllers(game: &GameStateData)->f32{
	let ctrlrs = game.get_controllers();
	let controller_1 = &ctrlrs[0];
	let controller_2 = &ctrlrs[1];
	let mut score = 0;
	let con1_ap = getAP_field(controller_1, game) as f32;
	let con2_ap = getAP_field(controller_2, game) as f32;
	let con1_hp = controller_1.get_life().clone() as f32;
	let con2_hp = controller_2.get_life().clone() as f32;
	return ((con2_hp)/(con1_ap+1.0))-((con1_hp)/(con2_ap+1.0));
}

fn perspective_score(ops : Vec<ClientOption>, game : &GameStateData)->f32{
	let ctrlrs = game.get_controllers();
	let controller_1 = &ctrlrs[0];
	let controller_2 = &ctrlrs[1];
	let mut score = 0;
	let con1_ap = getAP_field(controller_1, game) as f32;
	let mut con2_ap = getAP_field(controller_2, game) as f32;
	let con1_hp = controller_1.get_life().clone() as f32;
	let mut con2_hp = controller_2.get_life().clone() as f32;
	//get the HP and AP for the AI minions that we will play and use those as scores
	for i in &ops{
		con2_ap = con2_ap + (game.get_minion(i.source_uid.clone()).unwrap().get_current_attack() as f32);
	}
	return ((con2_hp)/(con1_ap+1.0))-((con1_hp)/(con2_ap+1.0));
}

pub fn attack_score(game : &GameStateData, attacker: UID, defender: UID)->f32{
	let mut copy_game = game.clone();

	let mut attacker_min :Minion = copy_game.get_mut_minion(attacker).unwrap().clone();
	let mut defender_min :Minion = copy_game.get_mut_minion(defender).unwrap().clone();

	
	{
		let mut ctrls = copy_game.get_mut_controllers();
		let mut attacker_ctrl : usize = 0;
		let mut defender_ctrl : usize = 0;
		for i in 0..ctrls.len(){
			if ctrls[i].get_team() == attacker_min.get_team(){
				attacker_ctrl = i;
			}
			if ctrls[i].get_team() == defender_min.get_team(){
				defender_ctrl = i;
			}
		}
		

		attacker_min.shift_current_health((defender_min.get_current_attack()as i32)*-1);
		defender_min.shift_current_health((attacker_min.get_current_attack()as i32)*-1);
		//check if minions are dead, then remove that minion from play

		if attacker_min.get_current_health() <= 0{
			ctrls[attacker_ctrl].move_minion_from_play_to_graveyard(attacker);
		}
		if defender_min.get_current_health() <=0{
			ctrls[defender_ctrl].move_minion_from_play_to_graveyard(defender);
		}
	}
	return score_controllers(&copy_game);
}


/*
*Data structure to keep track of individual
*
*
*/
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct PlayRuneSquare{
	pub ops_sel : Vec<ClientOption>,
	pub score : f32,
}
impl PlayRuneSquare{
	fn new(gsd : &GameStateData,ops : Vec<ClientOption>)->PlayRuneSquare{
		PlayRuneSquare{
			ops_sel: ops.clone(),
			score: perspective_score(ops.clone(), gsd),
		}
	}
	fn new_attack_sq(gsd : &GameStateData,ops : Vec<ClientOption>)->PlayRuneSquare{
		PlayRuneSquare{
			ops_sel: ops.clone(),
			score: attack_score(&gsd, ops[0].source_uid, ops[0].target_uid),
		}
	}
}

impl Eq for PlayRuneSquare{}

impl Ord for PlayRuneSquare{
	fn cmp(&self, other: &PlayRuneSquare) -> Ordering {
		self.partial_cmp(&other).unwrap()
	}
}
impl PartialEq for PlayRuneSquare{
	fn eq(&self, other: &PlayRuneSquare) -> bool{
		return self.score==other.score
	}
}
impl PartialOrd for PlayRuneSquare{
	fn partial_cmp(&self, other: &PlayRuneSquare)->Option<Ordering>{
		self.score.partial_cmp(&other.score)
	}
}

pub struct CardPlayMatrix {
	pub mana : u8,
	pub start_gsd : GameStateData,
	pub seen_gsds : Vec<String>,
	pub matrix_tiles : Vec<Vec<PlayRuneSquare>>,
	pub ops : Vec<ClientOption>,
	pub selected_ops: Vec<ClientOption>,
}
impl CardPlayMatrix {
	fn new(ops_sel: Vec<ClientOption>, gsd : GameStateData)->CardPlayMatrix { 

		if gsd.get_controllers().len() > 0{
			return CardPlayMatrix{
				mana: gsd.get_controllers()[1].get_base_mana(),
				start_gsd: gsd.clone(),
				seen_gsds: Vec::new(),
				matrix_tiles: Vec::new(),
				ops: ops_sel,
				selected_ops: Vec::new(),
			}
		}
		else{
			return CardPlayMatrix{
				mana: 0,
				start_gsd: gsd.clone(),
				seen_gsds: Vec::new(),
				matrix_tiles: Vec::new(),
				ops: ops_sel,
				selected_ops: Vec::new(),
			}
		}
	}

	fn run_matrix(&mut self){
		//first step generate the row length of the matrix, this is done by adding copies of the GSD and their scores to a vec
			//this is done for every mana given+1 for a 0 mana available
		let mut initRow : Vec<PlayRuneSquare> =  Vec::new();
		let emptOps : Vec<ClientOption>= Vec::new();
		let initsqr = PlayRuneSquare::new(&self.start_gsd, emptOps);
		for i in 0..self.mana+1{
			initRow.push(initsqr.clone());
		}
		println!("mana to manaSpots {0} : {1}", self.mana ,initRow.len());
		
		self.matrix_tiles.push(initRow);
		//second step generate the column length of the matrix, this is done by adding a copy of 0,0 to 0,x where x<#of ops
		for i in 1..self.ops.len()+1{
			let mut colStart = self.matrix_tiles[0][0].clone();
			self.matrix_tiles.push(vec![colStart]);
		}
		println!("ops to cols {0} : {1}", self.ops.len(), self.matrix_tiles.len());
		//Third step is to initialize the optimization engine which is a hashmap between seen optionsets and gsd's 
		//fourth is to run the matrix using the knapsack solution
		for i in 1..self.ops.len()+1{
			//loop through mana level
			for j in 1..self.mana+1{
				//get the minion data
				let min = &self.start_gsd.get_minion(self.ops[i-1].source_uid);
				//get the index of one level left 
				let index_min1 = (j-1) as usize;
				//get the square that is immediately left of the current operating square
				let i_j_min1 = self.matrix_tiles[i][index_min1].clone();
				//if the cost is greaterthan or = to the current mana lvl
				println!("Cost analysis {0}:{1}", min.unwrap().get_cost(), (j as u32));

				if min.unwrap().get_cost() <= (j as u32){
					//get the mana lvl - cost as an index
					println!();
					let costSel = (j-(min.unwrap().get_cost() as u8)) as usize;
					//get the index for the row directly above the current row
					let above = (i-1) as usize;
					//get the square that is the (one row above, mana lvl - cost)
					let i_j = self.matrix_tiles[above][costSel].clone();
					//get the options list for that square and append the current option to it
					let mut getOps = i_j.ops_sel.clone();
					getOps.push(self.ops[i-1]);
					//create our new square & compare the scores
					let square = PlayRuneSquare::new(&self.start_gsd, getOps);
					//if the score is bigger then push the new square to i,j
					//otherwise copy the square at i,j-1 and use that again
					println!("Score analysis {0}:{1}", square.score, i_j_min1.score);
					if square.score > i_j_min1.score {
						self.matrix_tiles[i].push(square);
					}
					else{
						self.matrix_tiles[i].push(i_j_min1.clone());
					}
				}
				//if you cant play anything then add the i,j-1 solution
				else{
					self.matrix_tiles[i].push(i_j_min1.clone());
				}
				//get score from [i][j-1] the immediate left position
				
			}
		}
		//set the selected options to run to the max position in (mana,options#)
		println!("{0} : {1} ", (self.mana as usize), self.ops.len());
		println!("Matrix size {0}:{1}", self.matrix_tiles.len(), self.matrix_tiles[0].len());
		//println("cols {0} : rows : {}");
		self.selected_ops = self.matrix_tiles[self.ops.len()][(self.mana as usize)].ops_sel.clone();
	}
}

pub struct AttackHeap{
	pub ops : Vec<ClientOption>,
	pub current_gsd: GameStateData,
	pub attack_heap : BinaryHeap<PlayRuneSquare>,
}

impl AttackHeap{
	pub fn new(current_gsd: GameStateData, ops: Vec<ClientOption>)->AttackHeap{
		let mut a_heap = BinaryHeap::new();
		for i in ops{
			a_heap.push(PlayRuneSquare::new_attack_sq(&current_gsd, vec![i]));
		}

		AttackHeap{
			ops: vec![],
			current_gsd : current_gsd.clone(),
			attack_heap: a_heap,
		}
	}

	pub fn pop_attack(&mut self)->ClientOption{
		return self.attack_heap.pop().unwrap().ops_sel[0];
	}

}