
use game_state::GameState;
use minion_card::UID;
use std::fmt;

pub trait Rune : Send {
    fn execute_rune(&self, game_state: &mut GameState);
    fn can_see(&self, controller: UID, game_state: &GameState) -> bool;
    fn to_json(&self) -> String;
    fn into_box(&self) -> Box<Rune>;
}
