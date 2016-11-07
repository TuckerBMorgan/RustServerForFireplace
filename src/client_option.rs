
#[derive(Copy, Clone)]
pub enum EOptionType {
    play_card,
    mulligan,
}

pub struct ClientOption {
    option_type: EOptionType,
    source_guid: String,
    target_guid: String,
}

impl ClientOption {
    pub fn new(source_guid: String, target_guid: String, option_type: EOptionType) -> ClientOption {
        ClientOption {
            option_type: option_type,
            source_guid: source_guid,
            target_guid: target_guid,
        }
    }
}
