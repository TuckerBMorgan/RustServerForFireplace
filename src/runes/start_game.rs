use rune_vm::Rune;
use game_state::GameState;
use minion_card::UID;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct StartGame {}

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

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
