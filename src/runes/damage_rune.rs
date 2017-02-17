use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use runes::modify_health::ModifyHealth;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct DamageRune {
    target_uid: UID,
    source_uid: UID,
    amount: i32,
}

impl DamageRune {
    pub fn new(target_uid: UID, source_uid: UID, amount: i32) -> DamageRune {
        DamageRune {
            target_uid: target_uid,
            source_uid: source_uid,
            amount: amount,
        }
    }
}

impl Rune for DamageRune {
    fn execute_rune(&self, game_state: &mut GameState) {
        let m_h = ModifyHealth::new(self.target_uid, self.amount * -1);
        game_state.execute_rune(Box::new(m_h));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"DamageRune\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
