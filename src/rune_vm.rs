
use game_state::GameState;

pub trait Rune: Send {
    fn execute_rune(&self, game_state: &mut GameState);
    fn can_see(&self, controller: u32, game_state: &GameState) -> bool;
    fn to_json(&self) -> String;
}