
use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::minion_card::UID;
use crate::game_state::GameState;
use std::collections::HashSet;
use crate::controller::{EControllerState, Controller};
use hlua;

#[derive(Serialize, Deserialize, Clone)]
pub struct NewController {
    pub uid: UID,
    pub hero: String,
    pub client_id: u32,
    pub deck: String,
    pub is_me: bool,
}

impl NewController {
    #[allow(dead_code)]
    pub fn new(uid: UID, hero: String, client_id: u32, deck: String, is_me: bool) -> NewController {
        NewController {
            uid: uid,
            hero: hero,
            client_id: client_id,
            deck: deck,
            is_me: is_me,
        }
    }
}

implement_for_lua!(NewController, |mut metatable| {});

impl Rune for NewController {
    fn execute_rune(&self, game_state: &mut GameState) {
        println!("New Controller with client_id {}", self.client_id.clone());
            let new_controller = Controller {
            name: "controller".to_string(),
            hero: self.hero.clone(),
            uid: self.uid,
            base_mana: 0,
            mana: 0,
            life: 30,
            total_life: 30,
            team: game_state.get_team() as u32,
            controller_state: EControllerState::WaitingForStart,
            current_options: vec![],
            deck: vec![],
            hand: vec![],
            unplayed_minions: vec![],
            in_play_minions: vec![],
            client_id: self.client_id.clone(),
            seen_cards: HashSet::new(),
            played_cards: vec![],
            graveyard: vec![],
        };

        game_state.add_player_controller(new_controller, self.deck.clone());
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"NewController\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
