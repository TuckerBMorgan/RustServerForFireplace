use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use hlua;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct ModifyHeroHealth {
    target_uid: UID,
    amount: i32,
}

implement_for_lua!(ModifyHeroHealth, |mut _metatable| {});

impl ModifyHeroHealth {
    pub fn new(target_uid: UID, amount: i32) -> ModifyHeroHealth {
        ModifyHeroHealth {
            target_uid: target_uid,
            amount: amount,
        }
    }
}

impl Rune for ModifyHeroHealth {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_controller_by_uid(self.target_uid).unwrap().set_current_life(self.amount);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ModifyHeroHealth\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
