use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use runes::modify_health::ModifyHealth;
use hlua;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Heal {
    target_uid: UID,
    source_uid: UID,
    amount: i64,
}

implement_for_lua!(Heal, |mut _metatable| {});

impl Heal {
    pub fn new(target_uid: UID, source_uid: UID, amount: i64) -> Heal {
        Heal {
            target_uid: target_uid,
            source_uid: source_uid,
            amount: amount,
        }
    }
}

impl Rune for Heal {
    fn execute_rune(&self, game_state: &mut GameState) {
        let m_h = ModifyHealth::new(self.target_uid, self.amount);
        game_state.execute_rune(Box::new(m_h));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"Heal\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
