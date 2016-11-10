
use ::card::Card;
use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use ::controller::{eControllerType, eControllerState, Controller};

#[derive(RustcDecodable, RustcEncodable)]
pub struct SummonMinion {
    pub card_uid: u16,
    pub controller_uid: u16,
    pub field_index : u8,
}

impl SummonMinion {
    pub fn new(card_uid: u16,
               controller_uid: String,
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
        let controller = game_state.get_controller_by_uid(self.controller_uid);    
     //   controller.
    }

    fn can_see(&self, controller: &Controller, game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap()
    }
}
