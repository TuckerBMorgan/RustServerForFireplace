use std::str;
use std::io::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::net::TcpStream;
use rustc_serialize::json::Json;

use std::sync::mpsc::{Sender, Receiver};
use game_thread::ThreadMessage;

use AI_Utils::{AI_Player, AI_Request};
use std::mem;
use rune_match::get_rune;

pub struct PlayerThread {
    pub client_id: u32,
    pub stream: Option<TcpStream>,
    pub join_handle: Option<JoinHandle<()>>,
    pub ai_current_state : Option<AI_Player>,
}

impl PlayerThread {
    pub fn new(client_id: u32, stream: Option<TcpStream>, ai_state: bool) -> PlayerThread {
        let mut ai_player = None;
        if ai_state {
            ai_player = Some(AI_Player::new());
        }
        let p_thread = PlayerThread {
            client_id: client_id,
            stream: stream,
            join_handle: None,
            ai_current_state : ai_player,
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

    pub fn swap_ai(&mut self, ai_state : AI_Player){
        mem::swap(&mut self.ai_current_state, &mut Some(ai_state))
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

fn player_thread_function(mut player_thread: PlayerThread,
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
            loop {
                let to_client_message = from_server.try_recv();


                match to_client_message {
                    Ok(to_client_message) => {
                        let message = to_client_message.payload.clone();
                        println!("AI JUST GOT {0}", message.clone());
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
                        println!("mt {0}", message_type);
                        if message_type.contains("Mulligan") {
                            let mulligan_message = format!("{{ \"{k}\":\"{v}\", \"{h}\" : [] }}",
                                                           k = "message_type",
                                                           v = "mulligan",
                                                           h = "index");
                            let to_server_message = ThreadMessage {
                                client_id: player_thread.client_id,
                                payload: mulligan_message,
                            };

                            let _ = to_server.send(to_server_message);
                        } else if message_type.contains("option_rune") {
              
                        let option_message = format!("{{ \"{k}\":\"{v}\", \"{h}\" : 0, \"{l}\" : 0,  \"{j}\" : 0}}",
                                                           k = "message_type",
                                                           v = "option",
                                                           h = "index",
                                                           l = "board_index",
                                                           j = "timeStamp");
                            let to_server_message = ThreadMessage {
                                client_id: player_thread.client_id,
                                payload: option_message
                            };
                            let _ = to_server.send(to_server_message);
                        } 
                        //this here updates the player_thread ai track
                        else if message_type.contains("AI_Update"){
                            
                            //copy the response, get the GSD give that to the AI 
                            let mut tmp = player_thread.ai_current_state.clone().unwrap(); 
                            tmp.update(message.clone());
                            player_thread.swap_ai(tmp);
                            println!("AI UPDATED TO {0}", (player_thread.ai_current_state.clone().unwrap().toJson()));
                        }
                        else {
                            println!("AI GONNA TRY AND UPDATE WITH {0}", message.clone());
                            let ai_request = AI_Request::new(
                                player_thread.ai_current_state.clone().unwrap().game_state_data, 
                                message.clone()
                            );
                            let t_messsage = ThreadMessage {
                                    client_id: player_thread.client_id,
                                    payload: String::from(ai_request.toJson()),
                                };
                                println!("AI REQ {0}", t_messsage.payload.clone());
                                let res = to_server.send(t_messsage);
                        }
                    }
                    Err(_) => {}

                }
            }
        }
    }
    
}
