use crate::rune_vm::Rune;
use crate::game_state::GameState;
use crate::minion_card::UID;
use serde::{Serialize, Deserialize};
use hlua;

#[derive(Serialize, Deserialize, Clone)]
pub struct Mulligan {}

impl Mulligan {
    pub fn new() -> Mulligan {
        Mulligan {}
    }
}

implement_for_lua!(Mulligan, |mut metatable| {});

impl Rune for Mulligan {
    fn execute_rune(&self, _game_state: &mut GameState) {}

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        "{\"runeType\":\"Mulligan\"}".to_string()
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
