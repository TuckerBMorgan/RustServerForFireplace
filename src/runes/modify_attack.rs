use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use hlua;

#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct ModifyAttack {
    target_uid: UID,
    amount: u32,
}

implement_for_lua!(ModifyAttack, |mut _metatable| {});

impl ModifyAttack {
    pub fn new(target_uid: UID, amount: u32) -> ModifyAttack {
        ModifyAttack {
            target_uid: target_uid,
            amount: amount,
        }
    }
}

impl Rune for ModifyAttack {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_minion(self.target_uid).unwrap().set_current_attack(self.amount);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ModifyAttack\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
