/*
*Title: AI_Utils.rs
*Author: John B. Casey <github:caseyj><email: caseyjohnb@gmail.com>
*Language: Rust
*Description: Tools needed to run the AI system and various functions which
	assist in those purposes.
*
*/


use minion_card::UID;
use card::Card;
use std::collections::HashSet;
use rand::{thread_rng, Rng};
use controller::Controller;
use game_state::GameStateData;
use minion_card::Minion;
use rune_vm::Rune;
use runes::play_minion::PlayMinion;
use runes::play_card::PlayCard;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct AI_Request{
	pub game_state_data : GameStateData,
	pub rune : String,

}
impl AI_Request{
	pub fn new(gsd : GameStateData, rne: String)->AI_Request{
		AI_Request{
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
pub struct AI_Player{
	pub game_state_data : GameStateData,
	pub score : f32,
	pub public_runes : Vec<String>,
	pub  update_count : u32,
}
impl AI_Player{
	pub fn new()->AI_Player{
		let mut gsd = GameStateData::new(true);
		let mut scre = 0.0 as f32;
		let mut pr = Vec::new();
		let mut uc = 0;
		gsd.get_uid();
		gsd.get_uid();
		AI_Player{
			game_state_data : gsd ,
			score : scre,
			public_runes : pr ,
			update_count : uc,
		}
	}

	/*
	*Updates the AI_Player using a json string of GameStateData
	*/
	pub fn update(&mut self, updateData: String){
		let response = updateData.clone().replace("{\"message_type\": \"AI_Update\",", "{" );
        self.game_state_data= json::decode(response.trim()).unwrap();
		self.update_count = self.update_count + 1; 
	}

	pub fn queue_update(&mut self, rune : String){
		self.public_runes.push(rune)
	}
}

/*
*gets how much active AP is on the field for a given 
*
*
fn getAP_field(controller: Controller, game: GameStateData) -> u8{

	let mut this_AP = 0;
	//get the vectors of uuids of the minions on this part of the field.
	let field = controller.get_copy_of_in_play();
	//iterate through the list of uids detected.
	for i in &field{
		this_AP += game.get_minion().get_current_attack();
	}
	return this_AP;
}

fn getHP_field(controller: Controller, game: GameStateData) -> u8{

	let mut this_AP = 0;
	//get the vectors of uuids of the minions on this part of the field.
	let field = controller.get_copy_of_in_play();
	//iterate through the list of uids detected.
	for i in &field{
		this_AP += game.get_minion().get_current_health();
	}
	return this_AP;
}
//basic for now add taunts later
fn score_controllers(p1Ind: usize, p2Ind: usize, game:GameStateData)->f32{
	let controller1 = game.get_controller_by_index(p1Ind);
	let controller2 = game.get_controller_by_index(p2Ind);
	let mut score = 0;
	let Con1AP = getAP_field(controller_1, game);
	let Con2AP = getAP_field(Controller_2, game);
	let Con1HP = getHP_field(controller_1, game);
	let Con2HP = getHP_field(controller_2, game);
	return ((Con1HP)/(Con2AP+1))-((Con2HP)/(Con1AP+1));
}

#[derive(Clone)]
pub struct proto_play{
	Score: f32,
	p1Index: usize,
	p2Index: usize,
	Game: GameStateData,
	Runes_Used: Vec<Rune>
}
impl proto_play{
	pub fn new(p1Ind: usize, p2Ind: usize,game: GameStateData)->proto_play{
		let mut sc = score_controllers(p1Ind, p2Ind, game);
		let mut ga = game;
		let mut run : Vec<Rune> = Vec::new();

		proto_play{
			Score = sc,
			p1Index:p1Ind,
			p2Index:p2Ind,
			Game: ga,
			Runes_Used: run
		}
	}

}

pub struct play_hand{
	Matrix: Vec<Vec<proto_play>>
}

impl play_hand {
	pub fn new(size: usize) -> play_hand{
		let mut N_V : Vec::new();
		for i in 0 .. size+1{
			let mut m : Vec<proto_play> = Vec::new();
			N_V.push(m)
		}
		play_hand{Matrix: N_V};
	}

	pub fn Summoning_Matrix(controller_1: Controller, Controller_2: Controller, game:GameStateData)->Vec<Vec<proto_play>>{
		let mut hand = controller_1.get_mut_hand();
		let controller_1_uid = controller_1.get_uid();

		let mut Play_Matrix : play_hand = play_hand::new(hand.len());

		//sort hand by cost here
		hand = hand.sort_by(|a,b| a.get_cost().cmp(b.get_cost()));
		let mana = controller_1.get_mana();

		//get starting position here; 
		let controllerVector = game.get_controllers();
		let locC1 = controllerVector.iter().position(|&b| b==controller_1).unwrap();
		let locC2 = controllerVector.iter().position(|&b| b==controller_2).unwrap();

		let &mut initialGame : proto_play = proto_play::new(locC1, locC2, game);
		
		//iterate over the hand elements(columns)
		for i in 0..hand.len()+1 {
			let &mut initClone = initialGame.clone();
			//if we are at the 0th for hand-level
			if Play_Matrix.Matrix[i].is_empty(){
				Play_Matrix[i].push(initClone);
			}
			else{
				Play_Matrix[i][0] = initClone;
			}
			//otherwise we need to checkout the cards 
			else{
				//iterate over the mana size
				for j in 1..mana+1{
					//if we are at the 0th index of mana 
					if j==0{
						Play_Matrix[i][j] = initClone;
					}
					//otherwise...
					else{
						if j >= hand[i].get_cost(){
							//create playState where minion is played on PlayMatrix[i-1][j-hand[i].cost] 
							let mut new_play = Play_Matrix[i-1][j-hand[i].get_cost()].clone();
							let play_minion : PlayMinion =  PlayMinion::new(hand[i].get_uid(), game.get_controller_by_index(p1Ind), hand.len(), 0);
							new_play.game.execute_rune(play_minion);
							new_play.Score = score_controllers(new_play.p1Index, new_play.p2Index, new_play.game);
							
							//check if the current score > the score right above
							if new_play.Score >= Play_Matrix[i-1][j]{
								Play_Matrix[i][j] = new_play;
							}
						}
						else{
							
							let mut repeat = Play_Matrix[i-1][j].clone();
							Play_Matrix[i][j] = repeat; is now equal to that clone
							
						}
					}
				}
			}
		}
		return Play_Matrix;	
	}
}

*/
