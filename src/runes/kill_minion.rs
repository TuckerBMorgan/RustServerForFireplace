use rune_vm::Rune;
use minion_card::{UID, EMinionState};
use tags_list::DEATH_RATTLE;
use rustc_serialize::json;
use game_state::GameState;
use hlua;
use bson;
use bson::Document;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct KillMinion {
    #[serde(with = "bson::compat::u2f")]
    controller_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    minion_uid: UID,
}

implement_for_lua!(KillMinion, |mut _metatable| {});

impl KillMinion {
    pub fn new(controller_uid: UID, minion_uid: UID) -> KillMinion {
        KillMinion {
            controller_uid: controller_uid,
            minion_uid: minion_uid,
        }
    }
}

impl Rune for KillMinion {
    fn execute_rune(&self, game_state: &mut GameState) {

        if game_state.get_minion(self.minion_uid).unwrap().has_tag(DEATH_RATTLE.to_string()) {
            //preform deathratte
        }

        game_state.get_mut_controller_by_uid(self.controller_uid)
            .unwrap()
            .move_minion_from_play_to_graveyard(self.minion_uid);
        game_state.get_mut_minion(self.minion_uid).unwrap().set_minion_state(EMinionState::Dead);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        !_game_state.is_ai_copy_running()
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"KillMinion\",")
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
                        d.insert("RuneType", "KillMinion");
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
