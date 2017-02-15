use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use minion_card::UID;



#[derive(RustcDecodable, RustcEncodable)]
pub struct SetHealth {
    card_uid: UID,
    amount: u8,
}

impl SetHealth {
    pub fn new(card_uid: UID, amount: u8) -> SetHealth {
        SetHealth {
            card_uid: card_uid,
            amount: amount,
        }
    }
}

impl Rune for SetHealth {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        game_state.get_mut_minion(self.card_uid).set_total_health(self.amount);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"SetHealth\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(*self.clone())
    }
}
