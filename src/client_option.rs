#[derive(Copy, Clone)]
pub enum EOptionType {
    play_card,
    mulligan,
}

pub struct ClientOption {
    option_type: EOptionType,
    source_uid: u32,
    target_uid: u32,
}

impl ClientOption {
    pub fn new(source_uid: u32, target_uid: u32, option_type: EOptionType) -> ClientOption {
        ClientOption {
            option_type: option_type,
            source_uid: source_uid,
            target_uid: target_uid,
        }
    }
}
