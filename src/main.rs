#![allow(dead_code)]

extern crate rand;
extern crate rustc_serialize;
extern crate rhai;
extern crate regex;

mod card;
mod runes;
mod entity;
mod rune_vm;
mod game_state;
mod controller;
mod minion_card;
mod game_thread;
mod client_option;
mod player_thread;
mod client_message;
mod process_message;

use game_thread::GameThread;
use std::sync::mpsc::channel;
use player_thread::PlayerThread;
use std::net::{TcpListener, TcpStream};

fn spawn_new_player(client_id: u32, stream: TcpStream) -> PlayerThread {
    PlayerThread::new(client_id, stream)
}

fn main() {

    let mut connected_clients: u32 = 0;
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    let mut players: Vec<PlayerThread> = Vec::new();
    let mut games: Vec<GameThread> = Vec::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                let p_thread = spawn_new_player(connected_clients, stream);
                connected_clients += 1;

                players.push(p_thread);
                if players.len() >= 2 {
                    let (tx_client, rx_server) = channel();
                    let (tx_server_to_client_1, rx_client_1) = channel();
                    let (tx_server_to_client_2, rx_client_2) = channel();

                    let new_client_thread_1 = players.remove(0);
                    let new_client_thread_2 = players.remove(0);

                    let client_id_1 = new_client_thread_1.client_id.clone();
                    let client_id_2 = new_client_thread_2.client_id.clone();

                    let new_game_thread = GameThread::new(tx_server_to_client_1,
                                                          tx_server_to_client_2,
                                                          rx_server,
                                                          client_id_1,
                                                          client_id_2);

                    new_client_thread_1.start_thread(tx_client.clone(), rx_client_1);
                    new_client_thread_2.start_thread(tx_client.clone(), rx_client_2);
                    games.push(new_game_thread);
                }
            }
            Err(_) => {
                println!("Bad Connection");
            }
        }
    }
}
