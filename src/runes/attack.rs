use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use runes::damage_rune::DamageRune;
use tags_list::{HERO};
use hlua;
use bson;
use bson::Document;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct Attack {
    #[serde(with = "bson::compat::u2f")]
    pub source_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    pub target_uid: UID,
}

impl Attack {
    pub fn new(source_uid: UID, target_uid: UID) -> Attack {
        Attack {
            source_uid: source_uid,
            target_uid: target_uid,
        }
    }
}

implement_lua_read!(Attack);
implement_lua_push!(Attack, |mut _metatable| {});

impl Rune for Attack {
    fn execute_rune(&self, mut game_state: &mut GameState) {

        let attacker = game_state.get_mut_minion(self.source_uid).unwrap().clone();
        let defender = game_state.get_mut_minion(self.target_uid).unwrap().clone();
        if !defender.has_tag(HERO.to_string()){
            let dr_1 = DamageRune::new(self.source_uid, self.target_uid, defender.get_base_attack());
            game_state.execute_rune(Box::new(dr_1));
        }
        

        let dr_2 = DamageRune::new(self.target_uid, self.source_uid, attacker.get_base_attack());
            
        

        game_state.add_to_attacked_this_turn(self.source_uid);
        
        

        game_state.execute_rune(Box::new(dr_2));
        
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"Attack\",")
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
                        d.insert("RuneType", "Attack");
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
