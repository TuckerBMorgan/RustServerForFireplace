use crate::rune_vm::Rune;
use crate::minion_card::{UID, EMinionState};
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use hlua;

#[derive(Serialize, Deserialize, Clone)]
pub struct Destroy {
    target_uid: UID,
}

implement_for_lua!(Destroy, |mut metatable| {});

impl Destroy {
    pub fn new(target_uid: UID) -> Destroy {
        Destroy { target_uid: target_uid }
    }
}

impl Rune for Destroy {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_minion(self.target_uid)
            .unwrap()
            .set_minion_state(EMinionState::MarkForDestroy);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"Destroy\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
