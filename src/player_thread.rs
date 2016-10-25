use std::str;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Sender, Receiver};
use ::game_thread::ThreadMessage;

pub struct PlayerThread
{
    pub client_id : u32,
    pub stream : TcpStream,
    pub join_handle : Option<JoinHandle<()>>,
}

impl PlayerThread
{
    pub fn new(client_id : u32, mut stream : TcpStream) -> PlayerThread {
        let mut p_thread = PlayerThread{client_id : client_id, stream : stream, join_handle : None};
        return p_thread;
    }

    pub fn start_thread(self, to_server : Sender<ThreadMessage>, from_server : Receiver<ThreadMessage>) -> JoinHandle<()> {
        Some(thread::spawn(move || {
            player_thread_function(self, to_server, from_server);
        })).unwrap()
    }
}

fn player_thread_function(mut player_thread : PlayerThread, to_server : Sender<ThreadMessage>, from_server : Receiver<ThreadMessage>){
        let mut buffer = [0;128];

        let ready = ThreadMessage{client_id : player_thread.client_id, payload : String::from("{\"message_type\":\"connection\"}")};
        to_server.send(ready);

        while true
        {
            let read_bytes = player_thread.stream.read(&mut buffer).unwrap();
            let mut message = str::from_utf8(&buffer[0..read_bytes]);


            match message{
                Ok(message_string) =>{
            
                    let t_messsage = ThreadMessage{client_id : player_thread.client_id, payload : String::from(message_string.trim())};

                    to_server.send(t_messsage);
                },
                Err(_) => {
                    println!("Bad message");
                }
            }


        }
}
