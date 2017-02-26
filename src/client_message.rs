
pub trait ClientMessage {}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MulliganMessage {
    pub message_type: String,
    pub index: Vec<u8>,
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct OptionsMessage {
    pub message_type: String,
    pub index: u8,
    pub board_index: u8,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ReadyMessage {
    pub message_type: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct ConnectionMessage {
    pub message_type: String,
}
