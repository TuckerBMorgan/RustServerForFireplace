use crate::game_state::GameState;
use crate::controller::Controller;
use crate::minion_card::UID;
use serde::{Deserialize, Serialize};
use hlua;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum OptionType {
    EAttack,
    EPlayCard,
    EEndTurn,
}

#[derive(Serialize, Deserialize)]
#[derive(Copy, Clone, Debug)]
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

implement_for_lua!(ClientOption, |mut metatable| {});

#[derive(Serialize, Deserialize)]
pub struct OptionsPackage {
    pub options: Vec<ClientOption>,
}

impl OptionsPackage {
    pub fn to_json(&self) -> String {
        let mut _str = serde_json::to_string(self).unwrap();
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
