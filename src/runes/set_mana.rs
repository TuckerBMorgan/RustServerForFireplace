use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use hlua;
use bson;
use bson::Document;

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct SetMana {
    #[serde(with = "bson::compat::u2f")]
    controller_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    mana: u8,
}

implement_for_lua!(SetMana, |mut _metatable| {});

impl SetMana {
    pub fn new(controller_uid: UID, mana: u8) -> SetMana {
        SetMana {
            controller_uid: controller_uid,
            mana: mana,
        }
    }
 
    pub fn to_rune(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}

impl Rune for SetMana {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_controller_by_uid(self.controller_uid).unwrap().set_mana(self.mana);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"SetMana\",")
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
                        d.insert("RuneType", "SetMana");
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
