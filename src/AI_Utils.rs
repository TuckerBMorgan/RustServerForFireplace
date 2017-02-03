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

/*
*gets how much active AP is on the field for a given 
*
*
*/
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
fn score_controllers(controller_1: Controller, Controller_2: Controller, game:GameStateData)->f32{
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
	Proto_controller_1: Controller,
	Proto_controller_2: Controller,
	p1Index: usize,
	p2Index: usize,
	Game: GameStateData
}
impl proto_play{
	pub fn new(controller_1:Controller, Controller_2: Controller, p1Ind: usize, p2Ind: usize,game: GameStateData)->proto_play{
		proto_play{Score = score_controllers(controller_1, Controller_2, game),
		Proto_controller_1 : controller_1,
		Proto_controller_2: controller_2,
		p1Index:p1Ind,
		p2Index:p2Ind,
		Game: game}
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

		let mut Play_Matrix : play_hand = play_hand::new(hand.len());

		//sort hand by cost here
		hand = hand.sort_by(|a,b| a.get_cost().cmp(b.get_cost()));
		let mana = controller_1.get_mana();

		//get starting position here; 
		let controllerVector = game.get_controllers();
		let locC1 = controllerVector.iter().position(|&b| b==controller_1).unwrap();
		let locC2 = controllerVector.iter().position(|&b| b==controller_2).unwrap();

		let &mut initialGame : proto_play = proto_play::new(controller_1, controller_2, locC1, locC2, game);
		
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
						//check if j >= hand[i]
							//create playState where minion is played on PlayMatrix[i-1][j-hand[i].cost] 
							//check if playState.Score >=  Play_Matrix[i-1][j]
								//playstate is new Play_Matrix[i][j]
							//otherwise
								//Play_Matrix[i-1][j] is cloned
								//Play_Matrix[i][j] is now equal to that clone
						//otherwise
							//Play_Matrix[i][j] = Play_Matrix[i][j-1];
				}
			}
		}
		return Play_Matrix;	
	}
}

