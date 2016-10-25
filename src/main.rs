
extern crate rand;
extern crate rustc_serialize;
extern crate rhai;

mod card;
mod runes;
mod entity;
mod rune_vm;
mod game_state;
mod controller;
mod minion_card;
mod game_thread;
mod player_thread;
mod client_message;
mod process_message;

use std::str;
use std::mem;
use std::thread;
use std::io::prelude::*;
use runes::new_controller;
use game_thread::GameThread;
use std::sync::mpsc::channel;
use player_thread::PlayerThread;
use std::net::{TcpListener, TcpStream};
use rhai::{Engine, FnRegister, Scope};
use game_state::GameState;

fn spawn_new_player(client_id : u32, mut stream : TcpStream) -> PlayerThread {
    PlayerThread::new(client_id, stream)
}

#[derive(Clone)]
struct TestStruct{
    x : i64,
    y : Vec<i64>
}

impl TestStruct {
    
    fn update(&mut self){
        self.x += 1000;
    }

    fn new() -> TestStruct{
        TestStruct{x : 1, y : vec![]}
    }

}


fn main()
{

    /*
    let mut engine = Engine::new();
    let mut scope : Scope = Vec::new();

    let mut ts : TestStruct = TestStruct::new();

    let mut gs : GameState = GameState::no_game_thread_new();

    scope.push(("game_state".to_string(), Box::new(ts)));
    scope.push(("game_state_check".to_string(), Box::new(gs)));

    engine.register_type::<TestStruct>();

    engine.register_fn("update", TestStruct::update);
    engine.register_fn("new_ts", TestStruct::new);

    println!("{:?}", scope);

    gs = engine.eval_with_scope::<GameState>(&mut scope, "game_state_check").unwrap();
    
    for &mut (ref name, ref mut val) in &mut scope.iter_mut().rev(){
        match val.downcast_mut::<GameState>(){
            Some(mut as_Test_Struct) => {
                println!("good cast");    
            },
            None =>{
                println!("___");
            }
        }
    }

    //let i_result = engine.eval_with_scope::<TestStruct>(&mut scope, "game_state");
    */
   // println!("{:?}", i_result.unwrap().x);

    let mut connected_clients : u32 = 0;
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    let mut players : Vec<PlayerThread> = Vec::new();

    for stream in listener.incoming()
    {
        match stream {
            Ok(stream) =>{
                
                let mut p_thread = spawn_new_player(connected_clients, stream);
                connected_clients+=1;
                
                players.push(p_thread);
                if players.len() >= 2
                {
                    let (tx_client, rx_server) = channel();
                    let (tx_server_to_client_1, rx_client_1) = channel();
                    let (tx_server_to_client_2, rx_client_2) = channel();
                    
                    let new_client_thread_1 = players.remove(0);
                    let new_client_thread_2 = players.remove(0);

                    let client_id_1 = new_client_thread_1.client_id.clone();
                    let client_id_2 = new_client_thread_2.client_id.clone();

                    let mut new_game_thread = GameThread::new(tx_server_to_client_1, tx_server_to_client_2, rx_server, client_id_1, client_id_2);
                    
                    new_client_thread_1.start_thread(tx_client.clone(), rx_client_1);
                    new_client_thread_2.start_thread(tx_client.clone(), rx_client_2);
                }

            },
            Err(_) =>{println!("Bad Connection");}
        }
    }
}