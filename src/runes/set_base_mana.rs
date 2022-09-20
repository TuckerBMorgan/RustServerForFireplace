use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use hlua;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetBaseMana {
    controller_uid: UID,
    base_mana: u8,
}

implement_for_lua!(SetBaseMana, |mut metatable| {});

impl SetBaseMana {
    pub fn new(controller_uid: UID, base_mana: u8) -> SetBaseMana {
        SetBaseMana {
            controller_uid: controller_uid,
            base_mana: base_mana,
        }
    }
}

impl Rune for SetBaseMana {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        game_state.get_mut_controller_by_uid(self.controller_uid)
            .unwrap()
            .set_base_mana(self.base_mana);
    }


    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"SetBaseMana\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
