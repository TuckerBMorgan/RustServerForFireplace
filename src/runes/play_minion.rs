use rune_vm::{Rune, ERuneType};
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use runes::add_tag::AddTag;
use runes::summon_minion::SummonMinion;
use tags_list::{CHARGE, SUMMONING_SICKNESS, TARGET};
use hlua;
use bson;
use bson::Document;

// the play_minion rune is called when you play a minion
// out of your hand. It will call battle_cry if it has one
// and it will remove the card from your hand
// it however wont directaly place the card into play
// it kicks off a summon minion rune after it calls battle_cry
//


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct PlayMinion {
    #[serde(with = "bson::compat::u2f")]
    pub minion_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    pub controller_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    pub field_index: u8,
    #[serde(with = "bson::compat::u2f")]
    pub target_uid: UID,
}

implement_for_lua!(PlayMinion, |mut _metatable| {});

impl PlayMinion {
    pub fn new(minion_uid: UID,
               controller_uid: UID,
               field_index: usize,
               target_uid: UID)
               -> PlayMinion {
        PlayMinion {
            minion_uid: minion_uid,
            controller_uid: controller_uid,
            field_index: field_index as u8,
            target_uid: target_uid,
        }
    }
}

impl Rune for PlayMinion {
    fn execute_rune(&self, mut game_state: &mut GameState) {

        {
            let min = game_state.get_minion(self.minion_uid).unwrap().clone();

            if min.has_tag(TARGET.to_string()) {
                //there is no reason for this statment to return anything
                game_state.run_lua_statement::<i8>(&min.get_function("target_function"
                                                           .to_string())
                                                       .unwrap(),
                                                   true);
            }

            if !min.has_tag(CHARGE.to_string()) {
                let at = AddTag::new(self.minion_uid.clone(), SUMMONING_SICKNESS.to_string());
                game_state.execute_rune(Box::new(at));
            }

            match min.get_function("battle_cry_function".to_string()) {
                Some(function) => {
                    let rune_vec = {
                        game_state.add_number_to_lua("controller_uid".to_string(),
                                                     self.controller_uid as u32);
                        game_state.add_number_to_lua("index".to_string(), 0);
                        let mut resutlt =
                            game_state.run_lua_statement::<hlua::LuaTable<_>>(&function, true)
                                .unwrap();
                        let ret = resutlt.iter::<i32, ERuneType>()
                            .filter_map(|e| e)
                            .map(|(_, v)| v)
                            .collect::<Vec<ERuneType>>()
                            .clone();
                        ret
                    };
                    for rune in rune_vec {
                        game_state.execute_rune(rune.unfold());
                    }
                }
                _ => {}
            }
        }

        let s_r = SummonMinion::new(self.minion_uid, self.controller_uid, self.field_index as u8);
        game_state.stage_rune(Box::new(s_r));
    //    game_state.process_rune(Box::new(s_r));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"PlayMinion\",")
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
                        d.insert("RuneType", "PlayMinion");
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
