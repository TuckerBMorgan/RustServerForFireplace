use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use hlua;
use bson;
use bson::Document;
use database_utils::{to_doc};

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct ModifyAttack {
    #[serde(with = "bson::compat::u2f")]
    target_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    amount: u32,
}

implement_for_lua!(ModifyAttack, |mut _metatable| {});

impl ModifyAttack {
    pub fn new(target_uid: UID, amount: u32) -> ModifyAttack {
        ModifyAttack {
            target_uid: target_uid,
            amount: amount,
        }
    }
}

impl Rune for ModifyAttack {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_minion(self.target_uid).unwrap().set_current_attack(self.amount);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        !_game_state.is_ai_copy_running()
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ModifyAttack\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }

    fn to_bson_doc(&self, game_name: String, count: usize) -> Document{
        return to_doc(bson::to_bson(&self).unwrap(), game_name, count, "ModifyAttack".to_string());
    }
}
