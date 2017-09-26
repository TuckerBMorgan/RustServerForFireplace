use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use hlua;
use bson;
use bson::Document;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct AddEnchantment {
    #[serde(with = "bson::compat::u2f")]
    target_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    source_uid: UID,
}

impl AddEnchantment {
    pub fn new(source_uid: UID, target_uid: UID) -> AddEnchantment {
        AddEnchantment {
            source_uid: source_uid,
            target_uid: target_uid,
        }
    }
}

implement_for_lua!(AddEnchantment, |mut _metatable| {});

impl Rune for AddEnchantment {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        game_state.get_mut_minion(self.target_uid).unwrap().add_enchantment(self.source_uid);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"AddEnchantment\",")
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
                        d.insert("RuneType", "AddEnchantment");
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
