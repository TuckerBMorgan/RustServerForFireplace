
use ::rune_vm::Rune;
use rustc_serialize::json;
use minion_card::UID;
use ::game_state::GameState;
use std::collections::HashSet;
use ::controller::{EControllerType, EControllerState, Controller};


#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct NewController {
    pub uid: UID,
    pub hero: String,
    pub client_id: u32,
    pub deck: String,
    pub isMe: bool,
}

impl NewController {
    pub fn new(uid: UID, hero: String, client_id: u32, deck: String, isMe: bool) -> NewController {
        NewController {
            uid: uid,
            hero: hero,
            client_id: client_id,
            deck: deck,
            isMe: isMe,
        }
    }
}

impl Rune for NewController {
    fn execute_rune(&self, game_state: &mut GameState) {
        println!("New Controller with client_id {}", self.client_id.clone());
        let mut new_controller = Controller {
            name: "controller".to_string(),
            hero: self.hero.clone(),
            uid: self.uid,
            base_mana: 0,
            mana: 0,
            team: game_state.get_team(),
            controller_state: EControllerState::WaitingForStart,
            current_options: vec![],
            deck: vec![],
            hand: vec![],
            unplayed_minions: vec![],
            in_play_minions: vec![],
            client_id: self.client_id.clone(),
            seen_cards: HashSet::new(),
            played_cards: vec![]
        };
        let card_names = GameState::parse_deck(self.deck.clone());

        game_state.populate_deck(&mut new_controller, card_names);

        game_state.add_player_controller(new_controller);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"NewController\",")
    }
}
