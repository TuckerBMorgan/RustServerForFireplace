
use crate::game_state::GameState;
use serde_json::{Value};
use crate::runes::new_controller::NewController;
use crate::client_message::*;



#[allow(dead_code)]
pub fn process_client_message(message: String, client_id: u32, game_state: &mut GameState) {

    println!("processing message {}", message);
    let j_message: Value = serde_json::from_str(message.trim()).unwrap();
    let obj = j_message.as_object().unwrap();

    let message_type = match obj.get("message_type") {
        Some(message_type) => {
            match *message_type {
                Value::String(ref v) => format!("{}", v),
                _ => {
                    return;
                }
            }
        }
        _ => {
            // key does not exist
            return;
        }
    };

    match message_type.as_ref() {

        "connection" => {
            new_connection(client_id, game_state);
        }

        "ready" => {}
        "option" => {
            let ops_message: OptionsMessage = serde_json::from_str(message.trim()).unwrap();
            game_state.execute_option(ops_message);
        }

        "mulligan" => {
            let mull_message: MulliganMessage = serde_json::from_str(message.trim()).unwrap();
            game_state.mulligan(client_id, mull_message.index.clone());
        }

        _ => {
            println!("{}", message_type);
        }
    }
}

#[allow(dead_code)]
fn new_connection(client_id: u32, mut game_state: &mut GameState) {

    let new_controller_rune = NewController {
        uid: game_state.get_uid(),
        hero: "hunter".to_string(),
        client_id: client_id,
        deck: "test.deck".to_string(),
        is_me: false,
    };

    game_state.new_connection(new_controller_rune);
}
