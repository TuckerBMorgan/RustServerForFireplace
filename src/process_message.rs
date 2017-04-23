extern crate rustc_serialize;

use game_state::{GameState, GameStateData};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use runes::new_controller::NewController;
use client_message::{MulliganMessage, OptionsMessage};
use rune_vm::Rune;
use rune_match::get_rune;
use AI_Utils::AI_Request;




#[allow(dead_code)]
pub fn process_client_message(message: String, client_id: u32, game_state: &mut GameState) {

    if !message.contains("AIPlay"){
        println!("processing message {}", message);
    }
    let j_message: Json = Json::from_str(message.trim()).unwrap();
    let obj = j_message.as_object().unwrap();

    let message_type = match obj.get("message_type") {
        Some(message_type) => {
            match *message_type {
                Json::String(ref v) => format!("{}", v),
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
            let ops_message: OptionsMessage = json::decode(message.trim()).unwrap();
            game_state.execute_option(ops_message);
        }

        "mulligan" => {
            let mull_message: MulliganMessage = json::decode(message.trim()).unwrap();
            game_state.mulligan(client_id, mull_message.index.clone());
        }
        "AIPlay"=>{
            let mut ai_play : AI_Request = json::decode(message.trim()).unwrap();
            let mut ai_gsd : GameStateData = ai_play.game_state_data;
            let mut rune_request : Box<Rune> = get_rune(ai_play.rune.as_ref());

            game_state.swap_gsd(&mut ai_gsd);
            game_state.execute_rune(rune_request);
            game_state.swap_gsd(&mut ai_gsd);

            let mut json_response = json::encode(&ai_gsd).unwrap();
            let mut front= "{\"runeType\":\"AI_Update\",";
		    let sendMsg = format!("{}{}", front, 
                    &json_response.clone()[1..json_response.len()]);
            
            game_state.send_msg(client_id, sendMsg); 

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
