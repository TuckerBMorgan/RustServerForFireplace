use crate::rune_vm::Rune;
use crate::minion_card::UID;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::runes::*;
use hlua;

#[derive(Serialize, Deserialize, Clone)]
pub struct DamageRune {
    target_uid: UID,
    source_uid: UID,
    amount: u32,
}

implement_for_lua!(DamageRune, |mut metatable| {});

impl DamageRune {
    pub fn new(target_uid: UID, source_uid: UID, amount: u32) -> DamageRune {
        DamageRune {
            target_uid: target_uid,
            source_uid: source_uid,
            amount: amount,
        }
    }
}

impl Rune for DamageRune {
    fn execute_rune(&self, game_state: &mut GameState) {
        let m_h = ModifyHealth::new(self.target_uid, (self.amount as i32) * -1);
        game_state.execute_rune(Box::new(m_h));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"DamageRune\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
