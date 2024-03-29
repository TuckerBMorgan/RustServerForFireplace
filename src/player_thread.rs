use std::str;
use std::io::prelude::*;
use std::thread;
use std::thread::JoinHandle;
use std::net::TcpStream;
use serde_json::{Value};

use std::sync::mpsc::{Sender, Receiver};
use crate::game_thread::ThreadMessage;

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
            loop {
                let to_client_message = from_server.try_recv();


                match to_client_message {
                    Ok(to_client_message) => {
                        let message = to_client_message.payload.clone();

                        let j_message: Value = serde_json::from_str(message.trim()).unwrap();
                        let obj = j_message.as_object().unwrap();

                        let message_type = match obj.get("runeType") {
                            Some(message_type) => {
                                match *message_type {
                                    Value::String(ref v) => format!("{}", v),
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
                    }
                    Err(_) => {}

                }
            }
        }
    }
}
