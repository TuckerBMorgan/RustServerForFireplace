use rune_vm::Rune;
use minion_card::{UID, EMinionState};
use rustc_serialize::json;
use game_state::GameState;
use hlua;
use bson;
use bson::Document;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct Destroy {
    target_uid: UID,
}

implement_for_lua!(Destroy, |mut _metatable| {});

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
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"Destroy\",")
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
                        d.insert("RuneType", "Destroy");
                        return d
                    },
                    _=>{}
                }
            },
            Err(e)=>{
                return Document::new();
            }
        }
        return Document::new();
    }
}
