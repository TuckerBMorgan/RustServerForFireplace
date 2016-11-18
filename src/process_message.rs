extern crate rustc_serialize;

use ::game_state::GameState;
use rustc_serialize::json::Json;
use ::controller::EControllerType;
use ::runes::new_controller::NewController;



#[allow(dead_code)]
pub fn process_client_message(message: String, client_id: u32, game_state: &mut GameState) {
    
    println!("processing message {}", message);

    let j_message: Json = Json::from_str(message.trim()).unwrap();
    
    let obj = j_message.as_object().unwrap();

    let message_type = match obj.get("message_type") {
        Some(message_type) => {   
            match *message_type {
                Json::String(ref v) => format!("{}" ,v),
                _ => {
                    return;} } },
        _ => {
            // key does not exist
            return;
        }
    };

    println!("message type is {:?}", message_type);

    match  message_type.as_ref() {

        "connection" => {

                new_connection( client_id, game_state);
        },
        /*
        "ready" => {

        },
        "option" => {
          //  let ops_message : OptionsMessage = json::decode(message.trim()).unwrap();
            //execute_option(ops_message);
        },
        "mulligan" =>{
            let mull_message : MulliganMessage = json::decode(message.trim()).unwrap();
        },
        */
        _ => {
            println!("{}", message_type);
        }
    }
}



#[allow(dead_code)]
fn new_connection(client_id: u32, mut game_state: &mut GameState) {

    let new_controller_rune = NewController {
        uid: game_state.get_uid(),
        controller_type: EControllerType::Player,
        hero: "hunter".to_string(),
        client_id: client_id,
        deck : "test.deck".to_string()
    };
    
    game_state.new_connection(new_controller_rune);
}