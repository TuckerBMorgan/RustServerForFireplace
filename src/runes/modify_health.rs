use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use hlua;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct ModifyHealth {
    target_uid: UID,
    amount: i32,
}

implement_for_lua!(ModifyHealth, |mut _metatable| {});

impl ModifyHealth {
    pub fn new(target_uid: UID, amount: i32) -> ModifyHealth {
        ModifyHealth {
            target_uid: target_uid,
            amount: amount,
        }
    }
}

impl Rune for ModifyHealth {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_minion(self.target_uid).unwrap().shift_current_health(self.amount);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ModifyHealth\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
