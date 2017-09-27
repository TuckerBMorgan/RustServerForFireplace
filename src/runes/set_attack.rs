use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use hlua;
use bson;
use bson::Document;
use database_utils::{to_doc};

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct SetAttack {
    #[serde(with = "bson::compat::u2f")]
    card_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    amount: u32,
}

implement_for_lua!(SetAttack, |mut _metatable| {});

impl SetAttack {
    pub fn new(card_uid: UID, amount: u32) -> SetAttack {
        SetAttack {
            card_uid: card_uid,
            amount: amount,
        }
    }
}

impl Rune for SetAttack {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        game_state.get_mut_minion(self.card_uid).unwrap().set_total_attack(self.amount);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"SetAttack\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
    fn to_bson_doc(&self, game_name: String, count: usize) -> Document{
        return to_doc(bson::to_bson(&self).unwrap(), game_name, count, "SetAttack".to_string());
    }
}
