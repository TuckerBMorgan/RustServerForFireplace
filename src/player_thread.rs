use std::str;
use std::io::prelude::*;
use std::ops::Range;
use std::thread;
use std::thread::JoinHandle;
use std::net::TcpStream;

use std::sync::mpsc::{Sender, Receiver};
use ::game_thread::ThreadMessage;
use std::time::Duration;

pub struct PlayerThread {
    pub client_id: u32,
    pub stream: Option<TcpStream>,
    pub join_handle: Option<JoinHandle<()>>, /*   pub buf: Vec<u8>,
                                              *   pub pending: Range<usize>,
                                              *   pub to_server: Option<Sender<ThreadMessage>>,
                                              *    pub from_server: Option<Receiver<ThreadMessage>> */
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

    let mut payload_message = format!("{{ \"{k}\":\"{v}\"}}", k = "message_type", v = "connection");

    match player_thread.stream {

        Some(mut stream) => {
            // stream.set_read_timeout(Some(Duration::from_millis(10)));
            stream.set_nonblocking(true);
            loop {

                if !payload_message.is_empty() {
                    let ready = ThreadMessage {
                        client_id: player_thread.client_id,
                        payload: payload_message,
                    };
                    let _ = to_server.send(ready);
                    payload_message = "".to_string();
                }

                let to_client_message = from_server.try_recv();

                match to_client_message {
                    Ok(to_client_message) => {
                        let with_flag = to_client_message.payload.clone() + "@@";
                        let e = stream.write(&with_flag.into_bytes()[..]);
                        match e {
                            Ok(_) => {}
                            Err(e) => {
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
                    Err(e) => {
                       // println!("----------{:?}", e);
                    }
                }
                //       stream.set_nonblocking(false);
                //   println!("asdfasdfasdf");

            }
        }
        // is AI
        None => {
            let ready = ThreadMessage {
                client_id: player_thread.client_id,
                payload: payload_message,
            };
            let _ = to_server.send(ready);
            loop {
                let to_client_message = from_server.try_recv();
                match to_client_message {
                    Ok(message_string) => {

                    },
                    Err(_) => {
                        
                    }
                    
                }
            }
        }
    }
}
