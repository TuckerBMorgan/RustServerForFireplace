
use ::card::Card;
use ::rune_vm::Rune;
use rustc_serialize::json;
use minion_card::UID;
use ::game_state::GameState;
use std::collections::{HashMap, HashSet};
use rustc_serialize::json::Json;
use ::controller::{EControllerType, EControllerState, Controller};


#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct NewController {
    pub uid: UID,
    pub controller_type: EControllerType,
    pub hero: String,
    pub client_id: u32,
    pub deck: String
}

impl NewController {
    pub fn new(uid: UID,
               controller_type: EControllerType,
               hero: String,
               client_id: u32,
               deck : String)
               -> NewController {
        NewController {
            uid: uid,
            controller_type: controller_type,
            hero: hero,
            client_id: client_id,
            deck : deck
        }
    }
}

impl Rune for NewController {
    fn execute_rune(&self, game_state: &mut GameState) {

        let mut new_controller = Controller {
            name: "controller".to_string(),
            hero: self.hero.clone(),
            controller_type: self.controller_type,
            uid: game_state.get_uid(),
            base_mana: 0,
            mana: 0,
            team: game_state.get_team(),
            controller_state: EControllerState::WaitingForStart,

            deck: vec![],
            hand: vec![],
            unplayed_minions: vec![],
            in_play_minions : vec![],
            client_id: self.client_id.clone(),
            seen_cards : HashSet::new(),
        };
        let mut card_names = GameState::parse_deck(self.deck.clone());

        game_state.populate_deck(&mut new_controller, card_names);

        game_state.add_player_controller(new_controller);

        println!("New Controller with client_id {}\n", self.client_id.clone());
    }

    fn can_see(&self, controller: u32, game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"NewController\",\n")
    }
}
