use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use minion_card::Minion;
use hlua;
use bson;
use bson::Document;

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct ReportMinionToClient {
    #[serde(with = "bson::compat::u2f")]
    cost: u32,
    id: String,
    #[serde(with = "bson::compat::u2f")]
    uid: UID,
    name: String,
    set: String,
    #[serde(with = "bson::compat::u2f")]
    base_attack: u32,
    #[serde(with = "bson::compat::u2f")]
    current_attack: u32,
    #[serde(with = "bson::compat::u2f")]
    total_attack: u32,
    #[serde(with = "bson::compat::u2f")]
    base_health: u32,
    #[serde(with = "bson::compat::u2f")]
    current_health: u32,
    #[serde(with = "bson::compat::u2f")]
    total_health: u32,
    #[serde(with = "bson::compat::u2f")]
    controller_uid: UID,
    is_deal: bool,
}

implement_for_lua!(ReportMinionToClient, |mut _metatable| {});

impl ReportMinionToClient {
    #[allow(dead_code)]
    pub fn new(cost: u32,
               id: String,
               uid: UID,
               name: String,
               set: String,

               base_attack: u32,
               current_attack: u32,
               total_attack: u32,

               base_health: u32,
               current_health: u32,
               total_health: u32,
               controller_uid: UID,
               is_deal: bool)
               -> ReportMinionToClient {

        ReportMinionToClient {
            cost: cost,
            id: id,
            uid: uid,
            name: name,
            set: set,
            base_attack: base_attack,
            current_attack: current_attack,
            total_attack: total_attack,
            base_health: base_health,
            current_health: current_health,
            total_health: total_health,
            controller_uid: controller_uid,
            is_deal: is_deal,
        }
    }

    pub fn from_minion(minion: &Minion,
                       controller_uid: UID,
                       is_deal: bool)
                       -> ReportMinionToClient {
        ReportMinionToClient {
            cost: minion.get_cost(),
            id: minion.get_id(),
            uid: minion.get_uid(),
            name: minion.get_name(),
            set: minion.get_set(),
            base_attack: minion.get_base_attack(),
            current_attack: minion.get_current_attack(),
            total_attack: minion.get_total_attack(),
            base_health: minion.get_base_health(),
            current_health: minion.get_current_health(),
            total_health: minion.get_total_health(),
            controller_uid: controller_uid.clone(),
            is_deal: is_deal,
        }
    }
}

impl Rune for ReportMinionToClient {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_controller_by_uid(self.controller_uid).unwrap().add_card_to_seen(self.uid);
    }

    fn can_see(&self, controller: UID, _game_state: &GameState) -> bool {
        if self.is_deal {
            let result = controller == self.controller_uid;
            return result;
        }
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ReportMinionToClient\",")
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
                        d.insert("RuneType", "ReportMinionToClient");
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
