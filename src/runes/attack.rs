use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use crate::runes::*;
use hlua;

#[derive(Serialize, Deserialize, Clone)]
pub struct Attack {
    pub source_uid: UID,
    pub target_uid: UID,
}

impl Attack {
    pub fn new(source_uid: UID, target_uid: UID) -> Attack {
        Attack {
            source_uid: source_uid,
            target_uid: target_uid,
        }
    }
}

implement_lua_read!(Attack);
implement_lua_push!(Attack, |mut metatable| {});

impl Rune for Attack {
    fn execute_rune(&self, mut game_state: &mut GameState) {

        let attacker = game_state.get_mut_minion(self.source_uid).unwrap().clone();
        let defender = game_state.get_mut_minion(self.target_uid).unwrap().clone();

        let dr_1 = DamageRune::new(self.source_uid, self.target_uid, attacker.get_base_attack());
        let dr_2 = DamageRune::new(self.source_uid, self.source_uid, defender.get_base_attack());

        game_state.add_to_attacked_this_turn(self.source_uid);

        game_state.execute_rune(Box::new(dr_1));
        game_state.execute_rune(Box::new(dr_2));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"Attack\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
