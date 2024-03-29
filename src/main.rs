#![feature(available_parallelism)]

extern crate rand;
extern crate rhai;
extern crate regex;
#[macro_use]
extern crate hlua;
extern crate quinn;
extern crate rustls;

#[macro_use]
mod macros;

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
mod tags_list;

use futures::stream::{self, StreamExt};
use std::process;
use std::thread;
use std::io;
use game_thread::{GameThread, ThreadMessage};
use std::sync::mpsc::channel;
use player_thread::PlayerThread;
use std::net::TcpStream;
use std::net::TcpListener;



use quinn::{Endpoint, Incoming, ServerConfig};
use std::{error::Error, net::SocketAddr, sync::Arc};


fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = ServerConfig::with_single_cert(cert_chain, priv_key)?;
    Arc::get_mut(&mut server_config.transport)
        .unwrap()
        .max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}

pub fn make_server_endpoint(bind_addr: SocketAddr) -> Result<(Incoming, Vec<u8>), Box<dyn Error>> {
    let (server_config, server_cert) = configure_server().unwrap();
    let (_endpoint, incoming) = Endpoint::server(server_config, bind_addr)?;
    Ok((incoming, server_cert))
}

fn run_server(addr: SocketAddr) -> Result<Vec<u8>, Box<dyn Error>> {
    let (mut incoming, server_cert) = make_server_endpoint(addr)?;
    // accept a single connection
    tokio::spawn(async move {
        let quinn::NewConnection { connection, .. } = incoming.next().await.unwrap().await.unwrap();
        println!(
            "[server] incoming connection: addr={}",
            connection.remote_address()
        );
    });

    Ok(server_cert)
}

fn spawn_new_player(client_id: u32, stream: TcpStream) -> PlayerThread {
    PlayerThread::new(client_id, Some(stream))
}
fn spawn_new_ai(client_id: u32) -> PlayerThread {
    PlayerThread::new(client_id, None)
}

fn terminal_commands() {
    terminal_help();
    let mut buffer = String::new();

    loop {
        let _ = io::stdin().read_line(&mut buffer);
        let st = buffer.to_string();

        if st.contains("clear") {
            println!("{}[2J", 27 as char);
        } else if st.contains("exit") {
            process::exit(0);
        } else if st.contains("help") {
            terminal_help();
        }

        buffer.clear()
    }
}

fn terminal_help() {
    println!("Terminal Commands");
    println!("clear -- clear the screen, will not reset cusor on windows");
    println!("exit -- will exit the program(Best to use this instead of crlt-c, that may crash \
              terminal)");
    println!("help -- print this again");
}

fn main() {

    thread::spawn(move || (terminal_commands()));

    let mut connected_clients: u32 = 0;
    let mut players: Vec<PlayerThread> = vec![];
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    let mut games = vec![];

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                let p_thread = spawn_new_player(connected_clients, stream);
                connected_clients += 1;

                players.push(p_thread);
                if players.len() >= 1 {
                    let (tx_client, rx_server) = channel();
                    let (tx_server_to_client_1, rx_client_1) = channel();
                    let (tx_server_to_client_2, rx_client_2) = channel();

                    let new_client_thread_1 = players.remove(0);
                    let new_client_thread_2 = spawn_new_ai(connected_clients);

                    let client_id_1 = new_client_thread_1.client_id.clone();
                    let client_id_2 = new_client_thread_2.client_id.clone();

                    let new_game_thread = GameThread::new(tx_server_to_client_1,
                                                          tx_server_to_client_2,
                                                          rx_server,
                                                          client_id_1,
                                                          client_id_2);

                    let payload_message =
                        format!("{{ \"{k}\":\"{v}\"}}", k = "message_type", v = "connection");

                    let ready_1 = ThreadMessage {
                        client_id: new_client_thread_1.client_id.clone(),
                        payload: payload_message.clone(),
                    };

                    let ready_2 = ThreadMessage {
                        client_id: new_client_thread_2.client_id.clone(),
                        payload: payload_message.clone(),
                    };

                    let _ = tx_client.send(ready_1);
                    let _ = tx_client.send(ready_2);

                    new_client_thread_1.start_thread(tx_client.clone(), rx_client_1);
                    new_client_thread_2.start_thread(tx_client.clone(), rx_client_2);
                    let jh = new_game_thread.start_thread();
                    games.push(jh);
                }
            }
            Err(_) => {
                println!("Bad Connection");
            }
        }
    }

}

// fn test_main() {
//
// let addr = env::args().nth(1).unwrap_or("127.0.0.1:1337".to_string());
// let addr = addr.parse::<SocketAddr>().unwrap();
// let mut core = Core::new().unwrap();
// let handle = core.handle();
//
// let socket = TcpListener::bind(&addr, &handle).unwrap();
//
//
// let done = socket.incoming().for_each(move | (socket, addr)|{
// let (reader, writer) = socket.split();
//
//
// handle.spawn(msg);
// Ok(())
// });
//
// core.run(done).unwrap();
//
// }
//
