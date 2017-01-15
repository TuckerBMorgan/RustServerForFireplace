
use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use minion_card::UID;

#[derive(RustcDecodable, RustcEncodable)]
pub struct ShuffleCard {
    pub card_uid: UID,
    pub controller_uid: UID,
}

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
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ShuffleCard\",")
    }
}
