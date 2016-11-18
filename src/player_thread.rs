use std::str;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
use ::game_thread::ThreadMessage;

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
        Some(thread::spawn(move || {
                player_thread_function(self, to_server, from_server);
            }))
            .unwrap()
    }
}

fn player_thread_function(player_thread: PlayerThread,
                          to_server: Sender<ThreadMessage>,
                          from_server: Receiver<ThreadMessage>) {
    
    let mut payload_message = format!("{{ \"{k}\":\"{v}\"}}", k = "message_type", v = "connection");

    match player_thread.stream {
        Some(mut stream) => {

        loop {             
            if !payload_message.clone().is_empty() {
                    let ready = ThreadMessage {
                        client_id: player_thread.client_id,
                        payload: payload_message.clone()
                    };

                    let _ = to_server.send(ready);
                    
                    let to_client_message = from_server.recv().unwrap();
                    let with_flag = to_client_message.payload.clone() + "@@";
                    let _ = stream.write(&with_flag.into_bytes()[..]);

                    let mut buffer = [0; 128];

                    let read_bytes = stream.read(&mut buffer).unwrap();
            
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
                                    //println!("No Error");
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

            payload_message = "".to_string();
            }
        },
        //is AI
        None => {
         let ready = ThreadMessage {
                client_id: player_thread.client_id,
                payload: String::from("{\"message_type\":\"connection\"}"),            
            };
            let _ = to_server.send(ready);
            loop {

            }
        }
    }
}
