use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use hlua;


#[derive(Serialize, Deserialize, Clone)]
pub struct AddTag {
    pub minion_uid: UID,
    pub tag: String,
}

impl AddTag {
    pub fn new(minion_uid: UID, tag: String) -> AddTag {
        AddTag {
            minion_uid: minion_uid,
            tag: tag,
        }
    }
}

implement_for_lua!(AddTag, |mut metatable| {});

impl Rune for AddTag {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        let minion = game_state.get_mut_minion(self.minion_uid);
        match minion {
            Some(minion) => {
                minion.add_tag_to(self.tag.to_string().clone());
            }
            None => {
                println!("We could not find the minion with the UID {}",
                         self.minion_uid)
            }
        }
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"AddTag\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
