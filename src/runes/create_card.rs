use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::{UID, Minion, EMinionState};
use hlua;

use std::fs::File;
use std::io::prelude::*;
use bson;
use bson::Document;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct CreateCard {
    card_id: String,
    #[serde(with = "bson::compat::u2f")]
    uid: UID,
    #[serde(with = "bson::compat::u2f")]
    controller_uid: UID,
}

implement_for_lua!(CreateCard, |mut _metatable| {});

impl CreateCard {
    #[allow(dead_code)]
    pub fn new(card_id: String, uid: UID, controller_uid: UID) -> CreateCard {

        CreateCard {
            card_id: card_id,
            uid: uid,
            controller_uid: controller_uid,
        }
    }
}

impl Rune for CreateCard {
    fn execute_rune(&self, game_state: &mut GameState) {
        println!("{}", "content/cards/".to_string() + &self.card_id.clone() +
                               &".lua".to_string());
        let mut f = File::open("content/cards/".to_string() + &self.card_id.clone() +
                               &".lua".to_string())
            .unwrap();

        let mut contents = String::new();
        let _result = f.read_to_string(&mut contents);
        let spl: Vec<&str> = contents.split("@@").collect();
        if spl[0].contains("minion") {
            let proto_minion = Minion::parse_minion_file(contents.clone());
            game_state.add_number_to_lua("give_uid".to_string(), self.uid);
            let mut minion = game_state.run_lua_statement::<Minion>(&proto_minion.get(&"create_minion_function".to_string()).unwrap(), true).unwrap();
            let team =
                game_state.get_mut_controller_by_uid(self.controller_uid).unwrap().get_team();

            minion.set_team(team);
            minion.set_minion_state(EMinionState::NotInPlay);
            minion.set_functions(proto_minion);

            game_state.get_mut_controller_by_uid(self.controller_uid)
                .unwrap()
                .add_minion_to_unplayed(minion.get_uid());
            game_state.add_minion_to_minions(minion);
        }
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return false;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"CreateCard\",")
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
                        d.insert("RuneType", "CreateCard");
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
