use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use hlua;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct SetAttack {
    card_uid: UID,
    amount: u32,
}

implement_for_lua!(SetAttack, |mut _metatable| {});

impl SetAttack {
    pub fn new(card_uid: UID, amount: u32) -> SetAttack {
        SetAttack {
            card_uid: card_uid,
            amount: amount,
        }
    }
}

impl Rune for SetAttack {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        game_state.get_mut_minion(self.card_uid).unwrap().set_total_attack(self.amount);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"SetAttack\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
