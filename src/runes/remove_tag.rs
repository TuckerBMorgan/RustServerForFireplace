use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use hlua;


#[derive(Serialize, Deserialize, Clone)]
pub struct RemoveTag {
    pub minion_uid: UID,
    pub tag: String,
}

implement_for_lua!(RemoveTag, |mut metatable| {});

impl RemoveTag {
    pub fn new(minion_uid: UID, tag: String) -> RemoveTag {
        RemoveTag {
            minion_uid: minion_uid,
            tag: tag,
        }
    }
}

impl Rune for RemoveTag {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        let minion = game_state.get_mut_minion(self.minion_uid);
        match minion {
            Some(minion) => {
                minion.remove_tag(self.tag.clone());
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
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"RemoveTag\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
