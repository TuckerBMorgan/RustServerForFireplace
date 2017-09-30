use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver};
use std::collections::HashMap;
use std::process;

#[derive(Debug)]
pub enum ManagementType {
    START,
    KILL,
}

pub struct Management{
    pub handle: Option<JoinHandle<()>>,
    pub game_name: String,
    pub message_type: ManagementType,
}

impl Management{
    pub fn new_start(handle: JoinHandle<()>, game_name: String)->Management{
        Management{
            handle: Some(handle),
            game_name: game_name,
            message_type: ManagementType::START,
        }
    }
    pub fn new_kill(game_name: String)->Management{
        Management{
            handle: None,
            game_name: game_name,
            message_type: ManagementType::KILL,
        }
    }
}

pub struct ThreadManager{
    games: HashMap<String, JoinHandle<()>>,
    manager: Receiver<Management>,
}

impl ThreadManager{
    fn new(manager: Receiver<Management>)->ThreadManager{
        ThreadManager{
            games: HashMap::new(),
            manager: manager,
        }
    }

    fn kill_action(&mut self, manage_msg: Management){
        let _ = self.games.remove(&manage_msg.game_name).unwrap().join();
        if self.games.is_empty(){
            process::exit(0);
        }
    }

    fn start_action(&mut self, manage_msg: Management){
        self.games.insert(manage_msg.game_name, manage_msg.handle.unwrap());
    }

    pub fn manage(&mut self){

        loop{
            let t_message = self.manager.recv().unwrap();
            match t_message.message_type{
                ManagementType::KILL=> self.kill_action(t_message),
                ManagementType::START=>self.start_action(t_message)
            }
        }
    }

    pub fn start(manager: Receiver<Management>){
        thread::spawn(move || { let mut t = ThreadManager::new(manager); t.manage(); });
    }
}