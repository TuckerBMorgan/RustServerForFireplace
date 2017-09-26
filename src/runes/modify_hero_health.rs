use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use hlua;
use bson;
use bson::Document;
use std::process;

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyHeroHealth {
    #[serde(with = "bson::compat::u2f")]
    target_uid: UID,
    amount: i32,
}

implement_for_lua!(ModifyHeroHealth, |mut _metatable| {});

impl ModifyHeroHealth {
    pub fn new(target_uid: UID, amount: i32) -> ModifyHeroHealth {
        ModifyHeroHealth {
            target_uid: target_uid,
            amount: amount,
        }
    }
}

impl Rune for ModifyHeroHealth {
    fn execute_rune(&self, game_state: &mut GameState) {
        
        game_state.get_mut_controller_by_uid(self.target_uid).unwrap().set_current_life(self.amount);

        if game_state.get_mut_controller_by_uid(self.target_uid).unwrap().get_life() == 0{
            game_state.write_history();
            process::exit(0);
        }
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        !_game_state.is_ai_copy_running()
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ModifyHeroHealth\",")
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
                        d.insert("RuneType", "ModifyHeroHealth");
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
