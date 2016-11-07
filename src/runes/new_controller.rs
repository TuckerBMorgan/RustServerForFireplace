
use ::card::Card;
use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use ::controller::{eControllerType, eControllerState, Controller};


#[derive(RustcDecodable, RustcEncodable)]
pub struct NewController {
    pub guid: String,
    pub controller_type: eControllerType,
    pub hero: String,
    pub client_id: u32,
}

impl NewController {
    pub fn new(guid: String,
               controller_type: eControllerType,
               hero: String,
               client_id: u32)
               -> NewController {
        NewController {
            guid: guid,
            controller_type: controller_type,
            hero: hero,
            client_id: client_id,
        }
    }
}

impl Rune for NewController {
    fn execute_rune(&self, game_state: &mut GameState) {

        let mut new_controller = Controller {
            name: "controller".to_string(),
            hero: self.hero.clone(),
            controller_type: self.controller_type,
            guid: game_state.get_guid(),
            baseMana: 0,
            mana: 0,
            team: game_state.get_team(),
            controller_state: eControllerState::waiting_for_start,

            deck: vec![],
            hand: vec![],
            unplayed_minions: vec![],
            //   in_play: vec![],
            //      graveyard: vec![],
            //    seen_cards: HashMap::new(),
            client_id: self.client_id.clone(),
        };

        game_state.add_player_controller(new_controller);
    }

    fn can_see(&self, controller: &Controller, game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap()
    }
}
