extern crate rustc_serialize;


use ::rune_vm;
use ::game_state::GameState;
use rustc_serialize::json::Json;
use ::controller::EControllerType;
use ::runes::new_controller::NewController;

#[allow(dead_code)]
pub fn process_client_message(message: String, client_id: u32, game_state: &mut GameState) {
    let j_message: Json = Json::from_str(message.trim()).unwrap();
    let obj = j_message.as_object().unwrap();

    let message_type = match obj.get("message_type") {
        Some(message_type) => message_type.to_string(),
        _ => {
            // key does not exist
            return;
        }
    };

    match message_type.as_ref() {
        "connection" => {
           // let ready_message : ConnectionMessage = json::decode(message.trim()).unwrap();
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
            // unknown type
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
        deck : "test".to_string()
    };
    
    //????
    rune_vm::execute_rune(Box::new(new_controller_rune), &mut game_state);
}
