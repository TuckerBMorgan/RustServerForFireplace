use serde::{Deserialize, Serialize};
pub trait ClientMessage {}

#[derive(Serialize, Deserialize)]
pub struct MulliganMessage {
    pub message_type: String,
    pub index: Vec<u8>,
}


#[derive(Serialize, Deserialize)]
pub struct OptionsMessage {
    pub message_type: String,
    pub index: u8,
    pub board_index: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ReadyMessage {
    pub message_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionMessage {
    pub message_type: String,
}
