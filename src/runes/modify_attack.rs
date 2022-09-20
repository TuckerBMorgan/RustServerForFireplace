use crate::rune_vm::Rune;
use crate::minion_card::UID;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use hlua;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModifyAttack {
    target_uid: UID,
    amount: u32,
}

implement_for_lua!(ModifyAttack, |mut metatable| {});

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
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"ModifyAttack\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
