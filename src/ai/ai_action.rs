use ai::ai_utils::{AI_Player, AI_Update_Request, OpsClassify};
use client_option::{OptionsPackage, ClientOption, OptionType};
use std::sync::mpsc::{Sender};
use game_thread::ThreadMessage;
use player_thread::PlayerThread;
use rune_match::get_rune;
use game_state::GameStateData;
use rustc_serialize::json::Json;
use rustc_serialize::json;
use runes::new_controller::NewController;
use minion_card::UID;

pub fn message_to_action(message_type: String , mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
    match message_type.as_ref(){
        "Mulligan"=> {
            let mulligan_message = format!("{{ \"{k}\":\"{v}\", \"{h}\" : [] }}",
                k = "message_type",
                v = "mulligan",
                h = "index");
            let to_server_message = ThreadMessage {
                client_id: player_thread.client_id.clone(),
                payload: mulligan_message,
            };

            to_server.send(to_server_message);
        }, 
        //if we have just recieved an options package
        "optionRune"=> {
            option_runner(&mut ai_current_state, message, &to_server, &player_thread);
        }, 
        //this here updates the player_thread ai track
        "AI_Update"=>{
            run_ai_update(&mut ai_current_state, message, &to_server, &player_thread);
        },
        //ANY THAT ARE EMPTY RUNES ARE IGNORE CONDITIONS
        "ReportMinionToClient"=>{},
        "AddTag"=>{},
        "SummonMinion"=>{},
        "RotateTurn"=>{},
        "PlayCard"=>{},
        "Attack"=>{},
        "NewController"=>{
            new_controller(&mut ai_current_state, message, &to_server, &player_thread)
        },
        //any of the runes which do not require special rules are executed below
        _=> {
            recieve_non_special(&mut ai_current_state, message, &to_server, &player_thread)
        }
    }
}

fn option_setup(mut ai_current_state: &mut AI_Player, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
    if ai_current_state.update_count == ai_current_state.public_runes.len() as u32 
        && ai_current_state.ops_recieved.options.len() > 0
    {
        ai_current_state.option_engine();
        run_option(&player_thread, &to_server, &mut ai_current_state);
    }
}


fn option_runner(mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
//dump the options
    println!("AI {} OPTIONS : {}", ai_current_state.uid, message);
    //decode into an options package
    let ops_msg = message.replace("{\"runeType\":\"optionRune\",", "{"); 
    let ops : OptionsPackage = json::decode(&ops_msg).unwrap();
    //if we have any options we can run, otherwise we just end it all
    ai_current_state.ops_recieved = ops;
    option_setup(ai_current_state, to_server, player_thread);
    
}


fn run_ai_update(mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
    //copy the response, get the GSD give that to the AI 
    ai_current_state.update(message.clone());
    //we just updated the AI, announce what number update this is
    println!("AI {} UPDATED {}", ai_current_state.uid, ai_current_state.update_count);
    //if the update count is less than the number of rune updates, continue updating
    if ai_current_state.update_count < ai_current_state.public_runes.len() as u32 {
        let rne = ai_current_state.public_runes[ai_current_state.update_count as usize].clone();
        queue_ai_update(&player_thread, &to_server, rne, ai_current_state.game_state_data.clone());
    }
    else{
        option_setup(ai_current_state, to_server, player_thread);
    }
    
}

fn new_controller(mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
    //get a new controller object so we can have the boolean
    //theres a better way to do this
    //probably
    let ns = message.clone().replace("{\"runeType\":\"NewController\",","{");
    let run : NewController = json::decode(ns.trim()).unwrap();
    //the second controller always has uid 3 and controller 1 always has uid 2
    ai_current_state.uid = run.uid;
    if run.uid == 3 {
        ai_current_state.queue_update(message.clone());
    }
    else{
        ai_current_state.public_runes.insert(0, message.clone());
        let rne = message.clone();
        queue_ai_update(&player_thread, &to_server, rne, ai_current_state.game_state_data.clone());
    }
}

fn recieve_non_special(mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
    println!("ai uid:{} msg: {}", ai_current_state.uid, message.clone());
    ai_current_state.queue_update(message.clone());
    if ai_current_state.public_runes.len() as u32 -  ai_current_state.update_count  == 1{
        let rne = ai_current_state.public_runes[ai_current_state.update_count as usize].clone();
        queue_ai_update(&player_thread, &to_server, rne, ai_current_state.game_state_data.clone());
    }
}



fn queue_ai_update(player_thread : &PlayerThread, to_server: &Sender<ThreadMessage>, message : String, gsd : GameStateData){
    let ai_request = AI_Update_Request::new(
        gsd, 
        message.clone()
    );
    println!("AI ATTEMPTING TO SEND {}", message.clone());
    let t_messsage = ThreadMessage {
        client_id: player_thread.client_id,
        payload: String::from(ai_request.toJson()),
    };
    to_server.send(t_messsage);

}

fn run_option(player_thread : &PlayerThread, to_server: &Sender<ThreadMessage>, ai_current_state : &mut AI_Player){
    let iter = ai_current_state.iterative.clone();
    println!("AI {} opsPack {}", ai_current_state.uid, ai_current_state.ops_recieved.to_json());
    let current_op : ClientOption  = ai_current_state.options_order[iter].clone();
    let ind =  ai_current_state.ops_recieved.options.iter().position(|&r| r==current_op).unwrap();
    let option_message = format!("{{ \"{k}\":\"{v}\", \"{h}\" : {i}, \"{l}\" : 0,  \"{j}\" : 0}}",
        k = "message_type",
        v = "option",
        h = "index",
        i=ind,
        l = "board_index",
        j = "timeStamp");
    println!("SENDING OPTION {0}", option_message.clone());
    let to_server_message = ThreadMessage {
        client_id: player_thread.client_id.clone(),
        payload: option_message
    };
    let _ = &to_server.send(to_server_message);

    ai_current_state.iter_up();
    println!("AI {} iter {} : option lens{}", ai_current_state.uid, ai_current_state.iterative, ai_current_state.options_order.len());


    match current_op.option_type {
        OptionType::EEndTurn=>{
            ai_current_state.options_test_recieved = false;
            ai_current_state.iterative = 0;
            ai_current_state.ops_recieved.options = vec![];
        },
        OptionType::EAttack=>{},
        OptionType::EPlayCard=>{},
    }
    println!("AI ITER : {} ", ai_current_state.iterative);
}

