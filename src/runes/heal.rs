use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use runes::modify_health::ModifyHealth;
use hlua;
use bson;
use bson::Document;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct Heal {
    #[serde(with = "bson::compat::u2f")]
    target_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    source_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    amount: u32,
}

implement_for_lua!(Heal, |mut _metatable| {});

impl Heal {
    pub fn new(target_uid: UID, source_uid: UID, amount: u32) -> Heal {
        Heal {
            target_uid: target_uid,
            source_uid: source_uid,
            amount: amount,
        }
    }
}

impl Rune for Heal {
    fn execute_rune(&self, game_state: &mut GameState) {
        let m_h = ModifyHealth::new(self.target_uid, self.amount as i32);
        game_state.execute_rune(Box::new(m_h));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"Heal\",")
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
                        d.insert("RuneType", "Heal");
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
