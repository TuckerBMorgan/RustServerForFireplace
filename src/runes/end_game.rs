use rune_vm::Rune;
use game_state::GameState;
use minion_card::UID;
use hlua;
use bson;
use bson::Document;
use database_utils::{write_history, to_doc};

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct EndGame {
    #[serde(with = "bson::compat::u2f")]
    pub controller_1_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    pub controller_2_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    pub controller_1_score: u8,
    #[serde(with = "bson::compat::u2f")]
    pub controller_2_score: u8,
}

implement_for_lua!(EndGame, |mut _metatable| {});

impl EndGame {
    pub fn new(uid_1: UID, uid_2: UID, score_1: u8, score_2: u8) -> EndGame {
        EndGame {
            controller_1_uid: uid_1,
            controller_2_uid: uid_2,
            controller_1_score: score_1,
            controller_2_score: score_2,
        }
    }
}

impl Rune for EndGame {
    fn execute_rune(&self, _game_state: &mut GameState) {
        if !_game_state.get_wrote(){
            println!("WRITING GAME: {}", _game_state.get_name());
            write_history(_game_state.get_history());
            _game_state.write();
            _game_state.end_game();
        }
        
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return false;
    }

    fn to_json(&self) -> String {
        "{\"runeType\":\"EndGame\"}".to_string()
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }

    fn to_bson_doc(&self, game_name: String, count: usize) -> Document{
        return to_doc(bson::to_bson(&self).unwrap(), game_name, count, "EndGame".to_string());
    }
}
