use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use hlua;


#[derive(Serialize, Deserialize, Clone)]
pub struct AddEnchantment {
    target_uid: UID,
    source_uid: UID,
}

impl AddEnchantment {
    pub fn new(source_uid: UID, target_uid: UID) -> AddEnchantment {
        AddEnchantment {
            source_uid: source_uid,
            target_uid: target_uid,
        }
    }
}

implement_for_lua!(AddEnchantment, |mut metatable| {});

impl Rune for AddEnchantment {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        game_state.get_mut_minion(self.target_uid).unwrap().add_enchantment(self.source_uid);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"AddEnchantment\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
