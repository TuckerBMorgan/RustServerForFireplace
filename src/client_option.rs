use game_state::GameState;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum EOptionType {
    PlayCard,
    Mulligan,
}

#[allow(dead_code)]
pub struct ClientOption {
    option_type: EOptionType,
    source_uid: u32,
    target_uid: u32,
}

impl ClientOption {
    #[allow(dead_code)]
    pub fn new(source_uid: u32, target_uid: u32, option_type: EOptionType) -> ClientOption {
        ClientOption {
            option_type: option_type,
            source_uid: source_uid,
            target_uid: target_uid,
        }
    }
}

pub trait OptionGenerator {
    fn generate_options(&self, game_state : &GameState) -> Vec<ClientOption>;
}
