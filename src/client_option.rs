use game_state::GameState;
use controller::Controller;
use minion_card::UID;
use rustc_serialize::json;
use hlua;

#[derive(Copy, Clone, Debug, RustcDecodable, RustcEncodable, PartialEq)]
pub enum OptionType {
    EAttack,
    EPlayCard,
    EEndTurn,
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClientOption {
    pub option_type: OptionType,
    pub source_uid: UID,
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
}

implement_for_lua!(ClientOption, |mut _metatable| {});

#[derive(RustcDecodable, RustcEncodable, Clone)]
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
}

pub trait OptionGenerator {
    fn generate_options(&self,
                        game_state: &mut GameState,
                        controller: &Controller)
                        -> Vec<ClientOption>;
}
