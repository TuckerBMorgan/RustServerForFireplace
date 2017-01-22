use game_state::GameState;
use controller::Controller;
use minion_card::UID;

#[derive(Copy, Clone, Debug)]
pub enum OptionType {
    EAttack,
    EPlayCard
}

#[derive(Copy, Clone, Debug)]
pub struct ClientOption {
    option_type: OptionType
    source_uid: UID,
    target_uid: UID,
}

impl ClientOption {
    #[allow(dead_code)]
    pub fn new(source_uid: UID, target_uid: UID, option_type: OptionType) -> ClientOption {
        ClientOption {
            option_type: OptionType,
            source_uid: source_uid,
            target_uid: target_uid,
        }
    }
}

pub trait OptionGenerator {
    fn generate_options(&self, game_state : &mut GameState, controller: &Controller) -> Vec<ClientOption>;
}
