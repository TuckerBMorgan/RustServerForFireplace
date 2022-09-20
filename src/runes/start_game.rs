use crate::rune_vm::Rune;
use crate::game_state::GameState;
use crate::minion_card::UID;
use serde::{Serialize, Deserialize};
use hlua;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartGame {}

implement_for_lua!(StartGame, |mut metatable| {});

impl StartGame {
    pub fn new() -> StartGame {
        StartGame {}
    }
}

impl Rune for StartGame {
    fn execute_rune(&self, _game_state: &mut GameState) {}

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        "{\"runeType\":\"StartGame\"}".to_string()
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
