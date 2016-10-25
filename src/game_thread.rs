

use std::str;
use std::thread;
use ::process_message;
use std::io::prelude::*;
use ::game_state::GameState;
use std::thread::JoinHandle;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender, Receiver};


pub struct ThreadMessage
{
    pub client_id : u32,
    pub payload : String
}

pub struct GameThread
{

    pub client_1 : Sender<ThreadMessage>,
    pub client_2 : Sender<ThreadMessage>,
    pub server : Receiver<ThreadMessage>,
    pub client_1_id : u32,
    pub client_2_id : u32
}


impl GameThread {

    pub fn new(client_1 : Sender<ThreadMessage>, client_2 : Sender<ThreadMessage>, server : Receiver<ThreadMessage>, client_1_id : u32, client_2_id : u32) -> GameThread {
        
        GameThread{client_1 : client_1, client_2 : client_2, server : server, client_1_id : client_1_id, client_2_id : client_2_id}
       
    }

    pub fn start_thread(self) -> JoinHandle<()> {
        Some(thread::spawn(move || {
            game_thread_main(self);
        })).unwrap()
    }

    pub fn report_message(&self, client_id : u32, message : String){
        let thread_message = ThreadMessage{client_id: client_id, payload : message};
        
        if client_id == self.client_1_id
        {
            self.client_1.send(thread_message);    
        }
        else 
        {
            self.client_2.send(thread_message);
        }
    }
}

pub fn game_thread_main(mut game_thread : GameThread){

    let mut game_state = GameState::new(& game_thread);

    
    while true
    {
        let t_message = game_thread.server.recv().unwrap();
        process_message::process_client_message(t_message.payload, t_message.client_id, &mut game_state);
    }
}

