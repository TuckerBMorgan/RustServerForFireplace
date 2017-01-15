use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use minion_card::UID;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Mulligan {

}

impl Mulligan {
    pub fn new() -> Mulligan {
        Mulligan {}
    }
}

impl Rune for Mulligan {
    fn execute_rune(&self, _game_state: &mut GameState) {}

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        "{\"runeType\":\"Mulligan\"}".to_string()
    }
}
