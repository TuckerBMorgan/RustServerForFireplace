use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use hlua;
use bson;
use bson::Document;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct AddTag {
    #[serde(with = "bson::compat::u2f")]
    pub minion_uid: UID,
    pub tag: String,
}

impl AddTag {
    pub fn new(minion_uid: UID, tag: String) -> AddTag {
        AddTag {
            minion_uid: minion_uid,
            tag: tag,
        }
    }
}

implement_for_lua!(AddTag, |mut _metatable| {});

impl Rune for AddTag {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        let minion = game_state.get_mut_minion(self.minion_uid);
        match minion {
            Some(minion) => {
                minion.add_tag_to(self.tag.to_string().clone());
            }
            None => {
                println!("We could not find the minion with the UID {}",
                         self.minion_uid)
            }
        }
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"AddTag\",")
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
                        d.insert("RuneType", "AddTag");
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
