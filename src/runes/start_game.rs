use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use minion_card::UID;

#[derive(RustcDecodable, RustcEncodable)]
pub struct StartGame {

}
 
impl StartGame {
    pub fn new( )
               -> StartGame {
            StartGame {
        }
    }
}

impl Rune for StartGame {

    fn execute_rune(&self, _game_state: &mut GameState) {
        
    }

    fn can_see(&self, _controller:UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"StartGame\"")
    }
}
