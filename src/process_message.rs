extern crate rustc_serialize;


use ::rune_vm;
use rustc_serialize::json;
use ::game_state::GameState;
use rustc_serialize::json::Json;
use ::controller::eControllerType;
use ::runes::new_controller::NewController;
use ::client_message::{ConnectionMessage};//, MulliganMessage, OptionsMessage};

pub fn process_client_message(message : String, client_id : u32, game_state  :  &mut GameState)
{
    let j_message : Json = Json::from_str(message.trim()).unwrap();
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

fn new_connection(client_id : u32, mut game_state : &mut GameState) {
    
    let new_controller_rune = NewController{guid : game_state.get_guid().to_string(), controller_type : eControllerType::player, hero : "hunter".to_string(), client_id : client_id};
    rune_vm::execute_rune(Box::new(new_controller_rune), &mut game_state);
    
}
/*
fn execute_option(ops_mess : OptionsMessage, client_id : u32, game_state : &mut GameState){

}

fn execute_mulligan(mulligan_message : MulliganMessage, client_id : u32, game_state : &mut GameState){

}
*/