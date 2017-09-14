use ai::ai_utils::{perspective_score,  attack_score};
use game_state::GameStateData;
use client_option::{ClientOption};
use std::collections::{BinaryHeap};
use minion_card::UID;
use std::cmp::Ordering;



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
	fn new(gsd : &GameStateData,ops : Vec<ClientOption>, my_uid: UID)->PlayRuneSquare{
		PlayRuneSquare{
			ops_sel: ops.clone(),
			score: perspective_score(ops.clone(), gsd, my_uid),
		}
	}
	fn new_attack_sq(gsd : &GameStateData,ops : Vec<ClientOption>, my_uid: UID)->PlayRuneSquare{
		PlayRuneSquare{
			ops_sel: ops.clone(),
			score: attack_score(&gsd, ops[0].source_uid, ops[0].target_uid, my_uid),
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
	pub uid: UID,
}
impl CardPlayMatrix {
	pub fn new(ops_sel: Vec<ClientOption>, gsd : GameStateData, my_uid: UID)->CardPlayMatrix { 

		if gsd.get_controllers().len() > 0{
			let mut cpm = CardPlayMatrix{
				mana: gsd.get_controllers()[1].get_base_mana(),
				start_gsd: gsd.clone(),
				seen_gsds: Vec::new(),
				matrix_tiles: Vec::new(),
				ops: ops_sel,
				selected_ops: Vec::new(),
				uid: my_uid,
			};
			cpm.run_matrix();
			return cpm;
		}
		else{
			let mut cpm = CardPlayMatrix{
				mana: 0,
				start_gsd: gsd.clone(),
				seen_gsds: Vec::new(),
				matrix_tiles: Vec::new(),
				ops: ops_sel,
				selected_ops: Vec::new(),
				uid: my_uid,
			};
			cpm.run_matrix();
			return cpm;
		}
		
	}

	fn run_matrix(&mut self){
		//first step generate the row length of the matrix, this is done by adding copies of the GSD and their scores to a vec
			//this is done for every mana given+1 for a 0 mana available
		let mut init_row : Vec<PlayRuneSquare> =  Vec::new();
		let empt_ops : Vec<ClientOption>= Vec::new();
		let initsqr = PlayRuneSquare::new(&self.start_gsd, empt_ops, self.uid);
		for _ in 0..self.mana+1{
			init_row.push(initsqr.clone());
		}
		//println!("mana to manaSpots {0} : {1}", self.mana ,init_row.len());
		
		self.matrix_tiles.push(init_row);
		//second step generate the column length of the matrix, this is done by adding a copy of 0,0 to 0,x where x<#of ops
		for _ in 1..self.ops.len()+1{
			let col_start = self.matrix_tiles[0][0].clone();
			self.matrix_tiles.push(vec![col_start]);
		}
		//println!("ops to cols {0} : {1}", self.ops.len(), self.matrix_tiles.len());
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
				//println!("Cost analysis {0}:{1}", min.unwrap().get_cost(), (j as u32));

				if min.unwrap().get_cost() <= (j as u32){
					//get the mana lvl - cost as an index
					let cost_select = (j-(min.unwrap().get_cost() as u8)) as usize;
					//get the index for the row directly above the current row
					let above = (i-1) as usize;
					//get the square that is the (one row above, mana lvl - cost)
					let i_j = self.matrix_tiles[above][cost_select].clone();
					//get the options list for that square and append the current option to it
					let mut get_ops = i_j.ops_sel.clone();
					get_ops.push(self.ops[i-1]);
					//create our new square & compare the scores
					let square = PlayRuneSquare::new(&self.start_gsd, get_ops, self.uid);
					//if the score is bigger then push the new square to i,j
					//otherwise copy the square at i,j-1 and use that again
					//println!("Score analysis {0}:{1}", square.score, i_j_min1.score);
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
		//println!("{0} : {1} ", (self.mana as usize), self.ops.len());
		//println!("Matrix size {0}:{1}", self.matrix_tiles.len(), self.matrix_tiles[0].len());
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
	pub fn new(current_gsd: GameStateData, ops: Vec<ClientOption>, my_uid: UID)->AttackHeap{
		let mut a_heap = BinaryHeap::new();
		for i in ops{
			a_heap.push(PlayRuneSquare::new_attack_sq(&current_gsd, vec![i], my_uid));
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
