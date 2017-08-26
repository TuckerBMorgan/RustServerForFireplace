use ai::ai_utils::{AI_Player, AI_Update_Request, OpsClassify};
use client_option::{OptionsPackage, ClientOption, OptionType};
use std::sync::mpsc::{Sender, Receiver};
use game_thread::ThreadMessage;
use player_thread::PlayerThread;
use rune_match::get_rune;
use game_state::GameStateData;
use rustc_serialize::json::Json;
use rustc_serialize::json;
use runes::new_controller::NewController;


pub fn option_runner(mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
//dump the options
    println!("OPTIONS : {}", message.clone());
    //decode into an options package
    let ops_msg = message.clone().replace("{\"runeType\":\"optionRune\",", "{"); 
    let ops : OptionsPackage = json::decode(&ops_msg).unwrap();
    //if we have any options we can run, otherwise we just end it all
    if ops.options.len() as u32 > 2{
        //get those options and clone them into the ai to track
        ai_current_state.ops_recieved = ops.clone();
        let t_classify = OpsClassify::new(ops.clone());
        //have we already built an options strategy?
        //println!("TEST REC {}",ai_current_state.options_test_recieved );
        //if we havent we need to test if we even can, if we can then we will
        //we can only under the condition that we are up to date with our game_state
        if !ai_current_state.options_test_recieved {
            println!("Checking if the update count is equal to the rune count");
            if ai_current_state.update_count == ai_current_state.public_runes.len() as u32{
                //there is no options plan and so we build an options plan and then run the first one we can
                ai_current_state.option_engine();
                run_option(&player_thread, &to_server, &mut ai_current_state);
            }
            else{

            }
        }
        //the ai has an options plan we run the next one we can
        else{
            if (t_classify.plays.len()==0) && (t_classify.attacks.len() > 0){
                ai_current_state.option_engine();
            }
            run_option(&player_thread, &to_server, &mut ai_current_state);
        }
    }
}


pub fn run_ai_update(mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
    //copy the response, get the GSD give that to the AI 
    ai_current_state.update(message.clone());
    //we just updated the AI, announce what number update this is
    println!("AI UPDATED {0}", ai_current_state.update_count);
    //if the update count is less than the number of rune updates, continue updating
    if ai_current_state.update_count < ai_current_state.public_runes.len() as u32 {
        let rne = ai_current_state.public_runes[ai_current_state.update_count as usize].clone();
        queue_ai_update(&player_thread, &to_server, rne, ai_current_state.game_state_data.clone());
    }
    //otherwise we know they are equal and we can run option requests
    else{
        println!("AI Checking if recieved options exist and runs if they are");
        //first we check and see if there are even options to run
        if ai_current_state.ops_recieved.options.len() > 0 {
        //if there is yet to be a decision on how to run these things
            let t_classify = OpsClassify::new(ai_current_state.ops_recieved.clone());
            if !ai_current_state.options_test_recieved{
                &ai_current_state.option_engine();
            }
            else{
                if ((t_classify.plays.len()==0) && (t_classify.attacks.len() > 0)){
                    &ai_current_state.option_engine();
                }
            }
            if ai_current_state.iterative < ai_current_state.options_order.len(){
                run_option(&player_thread, &to_server, &mut ai_current_state);
            }
            else{
                ai_current_state.options_test_recieved = false;
            }
        }
    }
}

pub fn new_controller(mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
    //get a new controller object so we can have the boolean
    //theres a better way to do this
    //probably
    let ns = message.clone().replace("{\"runeType\":\"NewController\",","{");
    let run : NewController = json::decode(ns.trim()).unwrap();
    if run.is_me {
        ai_current_state.queue_update(message.clone());
    }
    else{
        ai_current_state.public_runes.insert(0, message.clone());
        let rne = message.clone();
        queue_ai_update(&player_thread, &to_server, rne, ai_current_state.game_state_data.clone());
    }
}

pub fn recieve_non_special(mut ai_current_state: &mut AI_Player, message: String, to_server: &Sender<ThreadMessage>, player_thread: &PlayerThread){
    println!("msg: {}", message.clone());
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
    println!("opsPack {}", ai_current_state.ops_recieved.to_json());
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

    match current_op.option_type {
        OptionType::EEndTurn=>{
            println!("hit endTurn");
            ai_current_state.options_test_recieved = false;
            ai_current_state.iterative = 0;
            ai_current_state.ops_recieved.options = vec![];
        },
        OptionType::EAttack=>{},
        OptionType::EPlayCard=>{},
    }
    println!("AI ITER : {} ", ai_current_state.iterative);
}

