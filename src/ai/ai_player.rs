use game_state::GameStateData;
use rustc_serialize::json;
use client_option::{ClientOption, OptionsPackage};
use ai::ai_play_card::{CardPlayMatrix, AttackHeap};
use ai::ai_utils::{OpsClassify, score_controllers};
use minion_card::UID;




pub struct AiPlayer{
	pub game_state_data : GameStateData,
	pub score : f32,
	pub public_runes : Vec<String>,
	pub update_count : u32,
	pub options_order : Vec<ClientOption>,
	pub options_test_recieved : bool,
	pub ops_recieved : OptionsPackage,
	pub iterative : usize,
	pub uid : UID,
}
impl AiPlayer{
	pub fn new()->AiPlayer{
		let mut gsd = GameStateData::new(true);
		let scre = 0.0 as f32;
		let pr = Vec::new();
		let uc = 0;
		let options_test_recieved_false = false;
		gsd.get_uid();
		gsd.get_uid();
		AiPlayer{
			game_state_data : gsd.clone() ,
			score : scre,
			public_runes : pr ,
			update_count : uc,
			options_order : Vec::new(),
			options_test_recieved : options_test_recieved_false,
			ops_recieved : OptionsPackage{options : Vec::new()},
			iterative: 0,
			uid: 0,
		}
	}

	pub fn gsd_from_json(json_message : String)->GameStateData{
		let response = json_message.clone().replace("{\"message_type\": \"AI_Update\",", "{" );
        return json::decode(response.trim()).unwrap();
	}

	/*
	*Updates the AiPlayer using a json string of GameStateData
	*/
	pub fn update(&mut self, update_data: String){
        self.game_state_data = AiPlayer::gsd_from_json(update_data); 
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
		if self.options_order.len() != self.iterative && (self.iterative!=0 && self.options_order.len()!=0) {
			return;
		}
		self.options_order = vec![];
		//println!("AI options selections");
		let ops_classi = OpsClassify::new(self.ops_recieved.clone());
		//println!("Options classified");
		self.options_test_recieved = true;
		if ops_classi.plays.len() > 0{
			//println!("Running matrix");
			self.options_order = CardPlayMatrix::new(ops_classi.plays.clone(), self.game_state_data.clone()).selected_ops;
		}
		else{
			//println!("Running attack Heap");
			if ops_classi.attacks.len() > 0{
				self.options_order.push(AttackHeap::new(self.game_state_data.clone(), ops_classi.attacks).pop_attack());
			}
		}
		if self.options_order.len() == 0{
			self.options_order.push(ops_classi.end);
		}
		//let n_pack = OptionsPackage{options: self.options_order.clone()};
		//println!("OPS SELECTED {}", n_pack.to_json());
		self.iterative = 0;
	}

	pub fn iter_up(&mut self){
		self.iterative = self.iterative + 1;
	}
}