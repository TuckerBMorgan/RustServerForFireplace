use std::str;
use std::io::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::net::TcpStream;
use rustc_serialize::json::Json;

use std::sync::mpsc::{Sender, Receiver};
use game_thread::ThreadMessage;

use AI_Utils::{AI_Player, AI_Update_Request};
use std::mem;
use rune_match::get_rune;
use game_state::GameStateData;
use client_option::{OptionsPackage, ClientOption, OptionType};
use runes::new_controller::NewController;
use rustc_serialize::json;

pub struct PlayerThread {
    pub client_id: u32,
    pub stream: Option<TcpStream>,
    pub join_handle: Option<JoinHandle<()>>,
}

impl PlayerThread {
    pub fn new(client_id: u32, stream: Option<TcpStream>) -> PlayerThread {
        
        let p_thread = PlayerThread {
            client_id: client_id,
            stream: stream,
            join_handle: None,
        };
        return p_thread;
    }

    pub fn start_thread(self,
                        to_server: Sender<ThreadMessage>,
                        from_server: Receiver<ThreadMessage>)
                        -> JoinHandle<()> {
        Some(thread::spawn(move || { player_thread_function(self, to_server, from_server); }))
            .unwrap()
    }

    
    // pub fn from_stream(stream : TcpStream, client_id: u32) -> Result<PlayerThread> {
    // let p_thread = PlayerThread {
    // client_id: client_id,
    // stream: Some(stream),
    // join_handle: None,
    // buf: vec![0..128],
    // pending: 0..0
    // };
    //
    // Ok(p_thread)
    // }
    //
    // pub fn set_channels(&mut self, to_server: Sender<ThreadMessage>, from_server: Receiver<ThreadMessage>) {
    // self.to_server = Some(to_server);
    // self.from_server = Some(from_server);
    // }
    //
    // pub fn create_ai(client_id : u32) -> Result<PlayerThread> {
    // let p_thread = PlayerThread {
    // client_id: client_id,
    // stream: None,
    // join_handle: None,
    // buf: vec![0..128],
    // pending: 0..0
    // };
    //
    // Ok(p_thread)
    // }
    //
}

fn player_thread_function(player_thread: PlayerThread,
                          to_server: Sender<ThreadMessage>,
                          from_server: Receiver<ThreadMessage>) {

    match player_thread.stream {

        Some(mut stream) => {
            // stream.set_read_timeout(Some(Duration::from_millis(10)));
            let _ = stream.set_nonblocking(true);
            loop {
                /*
                if !payload_message.is_empty() {
                    let ready = ThreadMessage {
                        client_id: player_thread.client_id,
                        payload: payload_message,
                    };
                    let _ = to_server.send(ready);
                    payload_message = "".to_string();
                }
                */
                let to_client_message = from_server.try_recv();

                match to_client_message {
                    Ok(to_client_message) => {
                        let with_flag = to_client_message.payload.clone() + "@@";
                        let e = stream.write(&with_flag.into_bytes()[..]);
                        match e {
                            Ok(_) => {}
                            Err(_) => {
                                //  println!("{}", e);
                            }
                        }
                    }
                    _ => {}
                }

                let mut buffer = [0; 128];

                let read_bytes = stream.read(&mut buffer);

                match read_bytes {
                    Ok(read_bytes) => {

                        let message = str::from_utf8(&buffer[0..read_bytes]);

                        match message {
                            Ok(message_string) => {
                                let t_messsage = ThreadMessage {
                                    client_id: player_thread.client_id,
                                    payload: String::from(message_string.trim()),
                                };

                                let res = to_server.send(t_messsage);

                                match res {
                                    Ok(_) => {
                                        // println!("No Error");
                                    }
                                    Err(_) => {
                                        println!("Error in sending message to server");
                                        break;
                                    }
                                }
                            }
                            Err(_) => {
                                println!("Bad message");
                            }
                        }
                    }
                    Err(_) => {}
                }

            }
        }
        // is AI
        None => {
            let mut ai_current_state = AI_Player::new();
            loop {
                let to_client_message = from_server.try_recv();


                match to_client_message {
                    Ok(to_client_message) => {
                        let message = to_client_message.payload.clone();
                        //println!("AI JUST GOT {0}", message.clone());
                        let j_message: Json = Json::from_str(message.trim()).unwrap();
                        let obj = j_message.as_object().unwrap();
                        let message_type = match obj.get("runeType") {
                            Some(message_type) => {
                                match *message_type {
                                    Json::String(ref v) => format!("{}", v),
                                    _ => {
                                        println!("Happens here");
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                println!("No here");
                                // key does not exist
                                continue;
                            }
                        };
                        //println!("mt {0}", message_type);
                        if message_type.contains("Mulligan") {
                            let mulligan_message = format!("{{ \"{k}\":\"{v}\", \"{h}\" : [] }}",
                                                           k = "message_type",
                                                           v = "mulligan",
                                                           h = "index");
                            let to_server_message = ThreadMessage {
                                client_id: player_thread.client_id.clone(),
                                payload: mulligan_message,
                            };

                            let _ = &to_server.send(to_server_message);
                        } 
                        //if we have just recieved an options package
                        else if message_type.contains("optionRune") {
                            //dump the options
                            println!("OPTIONS : {}", message.clone());
                            //decode into an options package
                            let ops_msg = message.clone().replace("{\"runeType\":\"optionRune\",", "{"); 
                            let ops : OptionsPackage = json::decode(&ops_msg).unwrap();
                            //if we have any options we can run, otherwise we just end it all
                            if ops.options.len() as u32 > 2{
                                //get those options and clone them into the ai to track
                                ai_current_state.ops_recieved = ops.clone();
                                //have we already built an options strategy?
                                println!("TEST REC {}",ai_current_state.options_test_recieved );
                                //if we havent we need to test if we even can, if we can then we will
                                //we can only under the condition that we are up to date with our game_state
                                if !ai_current_state.options_test_recieved{
                                    println!("Checking if the update count is equal to the rune count");
                                    if ai_current_state.update_count == ai_current_state.public_runes.len() as u32{
                                        //there is no options plan and so we build an options plan and then run the first one we can
                                        ai_current_state.option_engine();
                                        run_option(&player_thread, &to_server, &mut ai_current_state);
                                    }
                                }
                                //the ai has an options plan we run the next one we can
                                else{
                                    run_option(&player_thread, &to_server, &mut ai_current_state);
                                }
                                
                            }
                            //only 2 options came accross, just send an end turn signal
                            else{

                                let option_message = format!("{{ \"{k}\":\"{v}\", \"{h}\" : 0, \"{l}\" : 0,  \"{j}\" : 0}}",
                                                           k = "message_type",
                                                           v = "option",
                                                           h = "index",
                                                           l = "board_index",
                                                           j = "timeStamp");
                                    let to_server_message = ThreadMessage {
                                        client_id: player_thread.client_id.clone(),
                                        payload: option_message
                                    };
                                    let _ = &to_server.send(to_server_message);
                            }
                        } 
                        //this here updates the player_thread ai track
                        else if message_type.contains("AI_Update"){
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
                                    if !ai_current_state.options_test_recieved {
                                        &ai_current_state.option_engine();
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
                        //LOGIC FOR RUNNING A TURN GOES HERE
                        //else if message_type.contains("optionRune"){

                        //}
                        //ANY THAT ARE EMPTY RUNES ARE IGNORE CONDITIONS
                        else if message_type.contains("ReportMinionToClient"){
                            //IGNORE
                        }
                        else if message_type.contains("AddTag"){
                            //IGNORE
                        }
                        else if message_type.contains("SummonMinion"){
                            //IGNORE
                        }
                        else if message_type.contains("RotateTurn"){
                            //IGNORE
                        }
                        else if message_type.contains("PlayCard"){
                            //IGNORE
                        }
                        else if message_type.contains("Attack"){
                            //IGNORE
                        }
                        else if message_type.contains("NewController"){
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
                        //any of the runes which do not require special rules are executed below
                        else {
                            //borrow the update count
                            let uDcount = ai_current_state.update_count;
                            //if the update count is the same as the length of the 
                            if (uDcount) == ai_current_state.public_runes.len() as u32 
                                    && ai_current_state.public_runes.len() as u32 > 1  
                            {         
                                println!("SENDING UPDATE {} {}", uDcount, ai_current_state.public_runes.len());
                                //let rne = ai_current_state.public_runes[ai_current_state.update_count as usize].clone();
                                queue_ai_update(&player_thread, &to_server, message.clone(), ai_current_state.game_state_data.clone());
                                ai_current_state.queue_update(message.clone());
                            }
                            else if (uDcount) == ai_current_state.public_runes.len() as u32 
                                    && ai_current_state.public_runes.len() as u32 == 0
                            {
                                //println!("SENDING UPDATE {} {}", uDcount, ai_current_state.public_runes.len());
                                let rne = message.clone();
                                queue_ai_update(&player_thread, &to_server, rne, ai_current_state.game_state_data.clone());
                                ai_current_state.queue_update(message.clone());
                            }
                            else{
                                //println!("QUEUEING UPDATE {} {}", uDcount, ai_current_state.public_runes.len());
                                ai_current_state.queue_update(message.clone());
                            }
                        }
                    }
                    Err(_) => {}

                }
            }
        }
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
            ai_current_state.options_test_recieved = false;
            ai_current_state.iterative = 0;
            ai_current_state.ops_recieved.options = vec![];
        },
        OptionType::EAttack=>{},
        OptionType::EPlayCard=>{},
    }
    println!("AI ITER : {}", ai_current_state.iterative);
}
