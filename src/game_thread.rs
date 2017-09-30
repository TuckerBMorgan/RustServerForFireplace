

use std::str;
use std::thread;
use process_message;
use game_state::GameState;
use std::thread::JoinHandle;
use std::sync::mpsc::{Sender, Receiver};
use thread_management::Management;


pub struct ThreadMessage {
    pub client_id: u32,
    pub payload: String,
}

pub struct GameThread {
    pub client_1: Sender<ThreadMessage>,
    pub client_2: Sender<ThreadMessage>,
    pub server: Receiver<ThreadMessage>,
    pub client_1_id: u32,
    pub client_2_id: u32,
    pub self_sender: Sender<ThreadMessage>,
}

impl GameThread {
    pub fn new(client_1: Sender<ThreadMessage>,
               client_2: Sender<ThreadMessage>,
               server: Receiver<ThreadMessage>,
               client_1_id: u32,
               client_2_id: u32,
               self_send: Sender<ThreadMessage>)
               -> GameThread {

        GameThread {
            client_1: client_1,
            client_2: client_2,
            server: server,
            client_1_id: client_1_id,
            client_2_id: client_2_id,
            self_sender: self_send
        }

    }

    #[allow(dead_code)]
    pub fn start_thread(self, name: String, end_sender: Sender<Management>) -> JoinHandle<()> {
        Some(thread::Builder::new()
                .name("game_thread".to_string())
                .spawn(move || { game_thread_main(self, name, end_sender); }))
            .unwrap()
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn report_message(&self, client_id: u32, message: String) {
        let thread_message = ThreadMessage {
            client_id: client_id,
            payload: message,
        };

        if client_id == self.client_1_id {

            let result = self.client_1.send(thread_message);

            match result {
                Ok(_) => {
                    print!("");
                }
                err => {
                    println!("{:?}", err);
                }
            }
        } else {
            let result = self.client_2.send(thread_message);
            match result {
                _ => {
                    print!("");
                }
            }
        }
    }

    pub fn _report_message_to_all(&self, message: String) {
        println!("Sending message to all clcarients, {}", message);
        let thread_message_1 = ThreadMessage {
            client_id: self.client_1_id,
            payload: message.clone(),
        };

        let thread_message_2 = ThreadMessage {
            client_id: self.client_1_id,
            payload: message.clone(),
        };

        let _ = self.client_1.send(thread_message_1);
        let _ = self.client_2.send(thread_message_2);
    }

    pub fn send_self(&self, message: String){
        let _ = self.self_sender.send(ThreadMessage{
            client_id: 0,
            payload: message,
            }
        );
    }
}

#[allow(dead_code)]
pub fn game_thread_main(game_thread: GameThread, name: String, end_sender: Sender<Management>) {

    let mut game_state = GameState::new(&game_thread, name.clone());
    loop {
        let t_message = game_thread.server.recv().unwrap();
        if process_message::process_client_message(t_message.payload, t_message.client_id, &mut game_state){
            break;
        }

    }
    let _ = end_sender.send(Management::new_kill(name.clone()));
}
