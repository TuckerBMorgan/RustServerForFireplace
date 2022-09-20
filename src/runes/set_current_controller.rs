use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use hlua;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetCurrentController {
    controller_uid: UID
}

impl SetCurrentController {
    pub fn new(controller_uid: UID) -> SetCurrentController {
        SetCurrentController{
            controller_uid
        }
    }
}

implement_for_lua!(SetCurrentController, |mut metatable| {});

impl Rune for SetCurrentController {
    fn execute_rune(&self, mut game_state: &mut GameState) {}

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"SetCurrentController\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}