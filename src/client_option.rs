use game_state::GameState;
use controller::Controller;
use minion_card::UID;
use rustc_serialize::json;
use hlua;
use bson;
use bson::Document;
use database_utils::{to_doc};

#[derive(Copy, Clone, Debug, RustcDecodable, RustcEncodable, PartialEq, Serialize, Deserialize)]
pub enum OptionType {
    EAttack,
    EPlayCard,
    EEndTurn,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Copy, Clone, Debug, PartialEq,  Serialize, Deserialize)]
pub struct ClientOption {
    pub option_type: OptionType,
    #[serde(with = "bson::compat::u2f")]
    pub source_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    pub target_uid: UID,
}

impl ClientOption {
    #[allow(dead_code)]
    pub fn new(source_uid: UID, target_uid: UID, option_type: OptionType) -> ClientOption {
        ClientOption {
            option_type: option_type,
            source_uid: source_uid,
            target_uid: target_uid,
        }
    }

    pub fn to_bson_doc(&self, game: String, count: usize) -> Document{
        return to_doc(bson::to_bson(&self).unwrap(), game, count, "Option".to_string());
    }
}

implement_for_lua!(ClientOption, |mut _metatable| {});

#[derive(RustcDecodable, RustcEncodable, Clone, Serialize, Deserialize)]
pub struct OptionsPackage {
    pub options: Vec<ClientOption>,
}

impl OptionsPackage {
    pub fn to_json(&self) -> String {
        let mut _str = json::encode(self).unwrap();
        _str.remove(0);
        let mut added_front = "{\"runeType\":\"optionRune\",".to_string();
        added_front += &_str[..];
        added_front
    }

    pub fn to_bson_doc(&self, game: String, count: usize) -> Document{
        return to_doc(bson::to_bson(&self).unwrap(), game, count, "optionRune".to_string());
    }

}

pub trait OptionGenerator {
    fn generate_options(&self,
                        game_state: &mut GameState,
                        controller: &Controller)
                        -> Vec<ClientOption>;
}
