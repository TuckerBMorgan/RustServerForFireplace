use std::str;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
use ::game_thread::ThreadMessage;

pub struct PlayerThread {
    pub client_id: u32,
    pub stream: TcpStream,
    pub join_handle: Option<JoinHandle<()>>,
}

impl PlayerThread {
    pub fn new(client_id: u32, stream: TcpStream) -> PlayerThread {
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

fn player_thread_function(mut player_thread: PlayerThread,
                          to_server: Sender<ThreadMessage>,
                          _from_server: Receiver<ThreadMessage>) {
    let mut buffer = [0; 128];

    let ready = ThreadMessage {
        client_id: player_thread.client_id,
        payload: String::from("{\"message_type\":\"connection\"}"),
    };
    let first_result = to_server.send(ready);

    match first_result {
        Ok(_) => {
            println!("No Error");
        }
        Err(_) => {
            println!("error ");
        }
    }
    loop {
        let read_bytes = player_thread.stream.read(&mut buffer).unwrap();
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
                        println!("No Error");
                    }
                    Err(_) => {
                        println!("error ");
                    }
                }

            }
            Err(_) => {
                println!("Bad message");
            }
        }


    }
}
