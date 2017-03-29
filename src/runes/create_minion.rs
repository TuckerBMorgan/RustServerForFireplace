use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use minion_card::Minion;
use hlua;

// this is a dummy rune for the client, IS NOT TO BE RUN THROUGH THE RUNE_VM
#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct CreateMinion {
    cost: i64,
    id: String,
    uid: UID,
    name: String,
    set: String,

    base_attack: i64,
    current_attack: i64,
    total_attack: i64,

    base_health: i64,
    current_health: i64,
    total_health: i64,

    controller_uid: UID,
}

implement_for_lua!(CreateMinion, |mut _metatable| {});

impl CreateMinion {
    #[allow(dead_code)]
    pub fn new(cost: i64,
               id: String,
               uid: UID,
               name: String,
               set: String,

               base_attack: i64,
               current_attack: i64,
               total_attack: i64,

               base_health: i64,
               current_health: i64,
               total_health: i64,
               controller_uid: UID)
               -> CreateMinion {

        CreateMinion {
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
        }
    }

    pub fn from_minion(minion: &Minion, controller_uid: UID) -> CreateMinion {
        CreateMinion {
            cost: minion.get_cost(),
            id: minion.get_id(),
            uid: minion.get_uid(),
            name: minion.get_name(),
            set: minion.get_set(),
            base_attack: minion.get_base_attack(),
            current_attack: minion.get_current_attack(),
            total_attack: minion.get_total_attack(),
            base_health: minion.get_base_health() as i64,
            current_health: minion.get_current_health() as i64,
            total_health: minion.get_total_health() as i64,
            controller_uid: controller_uid.clone(),
        }
    }
}

impl Rune for CreateMinion {
    fn execute_rune(&self, _game_state: &mut GameState) {}

    fn can_see(&self, controller: UID, _game_state: &GameState) -> bool {
        let result = controller == self.controller_uid;
        result
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"CreateMinion\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
