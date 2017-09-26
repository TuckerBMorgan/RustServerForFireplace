use rune_vm::Rune;
use game_state::GameState;
use minion_card::UID;
use hlua;
use bson;
use bson::Document;

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct StartGame {}

implement_for_lua!(StartGame, |mut _metatable| {});

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

    fn to_bson_doc(&self, game_name: String, count: usize) -> Document{
        let mut doc = bson::to_bson(&self);
        match doc{
            Ok(document)=>{
                match document{
                    bson::Bson::Document(mut d)=>{
                        d.insert("game", game_name);
                        d.insert("RuneCount", count as u64);
                        d.insert("RuneType", "StartGame");
                        return d
                    },
                    _=>{
                        println!("STARTGAME NO DOC {}", document);
                    },
                }
            },
            Err(e)=>{
                println!("STARTGAME NO DOC {}", e);
                return Document::new();
            },
        }
        println!("STARTGAME");
        return Document::new();
    }
}
