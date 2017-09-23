/*
*Title: AI_Utils.rs
*Author: John B. Casey <github:caseyj><email: caseyjohnb@gmail.com>
*Language: Rust
*Description: Tools needed to run the AI system and various functions which
	assist in those purposes.
*
*/

use minion_card::{UID, Minion};
use controller::Controller;

use game_state::GameStateData;
use rustc_serialize::json;
use client_option::{ClientOption, OptionType, OptionsPackage};


/**
*Formats a known gamestate and a rune that should run into a json friendly
*	string that can be sent to the game thread to be updated
*/
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct AiUpdateRequest{
	pub game_state_data : GameStateData,
	pub rune : String,

}
impl AiUpdateRequest{
	pub fn new(gsd : GameStateData, rne: String)->AiUpdateRequest{
		AiUpdateRequest{
			game_state_data : gsd,
			rune: rne,
		}
	}

	pub fn to_json(&self)->String{
		let msg : String = json::encode(self).unwrap();
		let front= "{\"message_type\":\"AIPlay\",";
		let send_message = format!("{}{}", front, &msg.clone()[1..msg.len()]);
		return send_message;

	}
}


/**
*Formats a known gamestate and an option set that should run into a json friendly
*	string that can be sent to the game thread to be updated

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
	pub fn to_json(&self)->String{
		let msg : String = json::encode(self).unwrap();
		let front= "{\"message_type\":\"OptionsSimulation\",";
		let send_message = format!("{}{}", front, &msg.clone()[1..msg.len()]);
		return send_message;
	}
	pub fn from_json(json_message: String)->AI_Option_Set_Request{
		let msg = json_message.replace("{\"message_type\":\"OptionsSimulation\",", "{");
		let ops_set : AI_Option_Set_Request = json::decode(msg.trim()).unwrap();
		return ops_set;
	}
}

*/

/*
*Takes in a given&known set of options and splits them by type
*	This allows other processes to operate on different option
*	sets in parallel
*/
#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
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


/*
*gets how much active AP is on the field for a given board. 
*
*
*/
fn get_ap_field(ref controller: &Controller, game: &GameStateData) -> u32{

	let mut this_ap = 0;
	//get the vectors of uuids of the minions on this part of the field.
	let field = controller.get_copy_of_in_play();
	//iterate through the list of uids detected.
	for i in field{
		this_ap += game.get_minion(i).unwrap().get_current_attack();
	}
	return this_ap;
}

/*
fn get_hp_field(ref controller: &Controller, game: &GameStateData) -> u32{

	let mut this_ap = 0;
	//get the vectors of uuids of the minions on this part of the field.
	let field = controller.get_copy_of_in_play();
	//iterate through the list of uids detected.
	for i in field{
		this_ap += game.get_minion(i).unwrap().get_current_health();
	}
	return this_ap;
}
*/

//basic for now add taunts later
pub fn score_controllers(game: &GameStateData, my_uid: UID)->f32{
	let ctrlrs = game.get_controllers();
	let controller_1 = &ctrlrs[0];
	let controller_2 = &ctrlrs[1];

	let con1_ap = get_ap_field(controller_1, game) as f32;
	let con2_ap = get_ap_field(controller_2, game) as f32;
	let con1_hp = controller_1.get_life().clone() as f32;
	let con2_hp = controller_2.get_life().clone() as f32;
	
	if controller_1.uid == my_uid{
		return ((con2_hp)/(con1_ap+1.0))-((con1_hp)/(con2_ap+1.0));	
	}
	else{
		return ((con1_hp)/(con2_ap+1.0))-((con2_hp)/(con1_ap+1.0));
	}
	
}

pub fn perspective_score(ops : Vec<ClientOption>, game : &GameStateData, my_uid: UID)->f32{
	let ctrlrs = game.get_controllers();
	let controller_1 = &ctrlrs[0];
	let controller_2 = &ctrlrs[1];
	
	let mut con1_ap = get_ap_field(controller_1, game) as f32;
	let mut con2_ap = get_ap_field(controller_2, game) as f32;
	let con1_hp = controller_1.get_life().clone() as f32;
	let con2_hp = controller_2.get_life().clone() as f32;
	

	if controller_1.uid == my_uid{
		//get the HP and AP for the AI minions that we will play and use those as scores
		for i in &ops{
			con2_ap = con2_ap + (game.get_minion(i.source_uid.clone()).unwrap().get_current_attack() as f32);
		}
		return ((con2_hp)/(con1_ap+1.0))-((con1_hp)/(con2_ap+1.0));	
	}
	else{
		//get the HP and AP for the AI minions that we will play and use those as scores
		for i in &ops{
			con1_ap = con1_ap + (game.get_minion(i.source_uid.clone()).unwrap().get_current_attack() as f32);
		}
		return ((con1_hp)/(con2_ap+1.0))-((con2_hp)/(con1_ap+1.0));
	}
}

pub fn attack_score(game : &GameStateData, attacker: UID, defender: UID, my_uid: UID)->f32{
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
	return score_controllers(&copy_game, my_uid);
}