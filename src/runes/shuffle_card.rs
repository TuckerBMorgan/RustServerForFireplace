
use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use hlua;

#[derive(Serialize, Deserialize, Clone)]
pub struct ShuffleCard {
    pub card_uid: UID,
    pub controller_uid: UID,
}

implement_for_lua!(ShuffleCard, |mut metatable| {});

impl ShuffleCard {
    pub fn new(card_uid: UID, controller_uid: UID) -> ShuffleCard {
        ShuffleCard {
            card_uid: card_uid,
            controller_uid: controller_uid,
        }
    }
}

impl Rune for ShuffleCard {
    fn execute_rune(&self, game_state: &mut GameState) {

        game_state.get_mut_controller_by_uid(self.controller_uid)
            .unwrap()
            .move_card_from_hand_to_deck(self.card_uid.clone());

    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"ShuffleCard\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
