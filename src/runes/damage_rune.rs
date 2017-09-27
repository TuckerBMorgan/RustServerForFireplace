use rune_vm::Rune;
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use runes::modify_health::ModifyHealth;
use hlua;
use bson;
use bson::Document;
use database_utils::{to_doc};

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct DamageRune {
    #[serde(with = "bson::compat::u2f")]
    target_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    source_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    amount: u32,
}

implement_for_lua!(DamageRune, |mut _metatable| {});

impl DamageRune {
    pub fn new(target_uid: UID, source_uid: UID, amount: u32) -> DamageRune {
        DamageRune {
            target_uid: target_uid,
            source_uid: source_uid,
            amount: amount,
        }
    }
}

impl Rune for DamageRune {
    fn execute_rune(&self, game_state: &mut GameState) {
        let m_h = ModifyHealth::new(self.target_uid, (self.amount as i32) * -1);
        
        game_state.execute_rune(Box::new(m_h));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        !_game_state.is_ai_copy_running()
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"DamageRune\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
    fn to_bson_doc(&self, game_name: String, count: usize) -> Document{
        return to_doc(bson::to_bson(&self).unwrap(), game_name, count, "DamageRune".to_string());
    }
}
