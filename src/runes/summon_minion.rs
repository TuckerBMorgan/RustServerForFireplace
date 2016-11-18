
use ::card::Card;
use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use minion_card::{UID};
use std::collections::HashMap;
use rustc_serialize::json::Json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct SummonMinion {
    pub card_uid: UID,
    pub controller_uid: UID,
    pub field_index : u8,
}

impl SummonMinion {
    pub fn new(card_uid: UID,
               controller_uid: UID,
               field_index: u8)
               -> SummonMinion {
        SummonMinion {
            card_uid: card_uid,
            controller_uid: controller_uid,
            field_index: field_index
        }
    }
}

impl Rune for SummonMinion {

    fn execute_rune(&self, game_state: &mut GameState) {
        let mut controller = game_state.get_controller_by_uid(self.controller_uid);    
        match controller {
            Some(controller) => {
                controller.move_minion_from_unplayed_into_play(self.card_uid);
            },
            None => {
                println!("Was unable to find controller in SummonMinion with uuid of {}", self.controller_uid);
            }
        }
    }

    fn can_see(&self, controller: u32, game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap()
    }
}
