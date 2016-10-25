
use rune_vm::Rune;
use std::net::TcpStream;
use controller::Controller;
use game_thread::GameThread;
use entity::{Entity, eEntityType};
use std::collections::{VecDeque, HashMap};

#[derive(Clone)]
pub struct GameStateData {

    controllers : Vec<Controller>

}

impl GameStateData {

    pub fn new() -> GameStateData {
        GameStateData { controllers : vec![]}
    }

   pub fn add_player_controller(&mut self, controller: Controller){
        self.controllers.push(controller);
    }

    pub fn get_controllers(&self) -> & Vec<Controller>{
        &self.controllers
    }

    pub fn get_mut_controllers(&mut self) -> &mut Vec<Controller>{
        &mut self.controllers
    }

}

pub struct GameState<'a>
{
    players_ready : u8, // the number of players who have done the full handshake for the start of the game
    entity_count : u32,
    team_count : u8,
    connection_number : u8,

//  game_thread : &'a GameThread,
    game_thread : Option<&'a GameThread>,
    game_state_data : GameStateData,
 
    rune_queue: VecDeque<Box<Rune>>, // the current runes waiting to be fired
    entities: HashMap<String, Box<Entity>>, // all entities in the game, spells, minions, and controllers
    connections: Vec<Box<TcpStream>>, // all the streams of the current people connected so we can talk to them again
    attacked_this_turn : Vec<Box<Entity>>, // those entities that have attacked this turn
}

impl<'a> GameState<'a>
{
    pub fn new(game_thread : & GameThread) -> GameState {
        GameState {game_thread : Some(game_thread), game_state_data : GameStateData::new(),
                   players_ready : 0, entity_count : 0, team_count : 0, connection_number : 0,
                   connections : vec![], rune_queue : VecDeque::new(), attacked_this_turn : vec![],
                   entities : HashMap::new()}
    }

    /*
    pub fn no_game_thread_new() -> GameState<'a> {
        GameState { players_ready : 0, entity_count : 0, team_count : 0, connection_number : 0,
                    controllers : vec![], rune_queue : VecDeque::new(),
                    connections : vec![], game_thread : None}    
    }
    */

    //adds a rune to the rune queue, this is down when a executing rune creates a rune
    pub fn add_rune_to_queue(&mut self, rune: Box<Rune>){
        self.rune_queue.push_back(rune);
    }

    pub fn report_rune_to_client(&self, client_id : u32, rune_string : String){
        self.game_thread.unwrap().report_message(client_id, rune_string);
    }

    //do we have any runes wating to be executed
    pub fn is_rune_queue_empty(&self) -> bool{
        self.rune_queue.is_empty()
    }
    
    pub fn add_player_connection(&mut self, stream: Box<TcpStream>){
        self.connections.push(stream);
    }

    //a player has finished the handshake for game start
    pub fn a_player_is_ready(&mut self){
        self.players_ready = self.players_ready + 1;
    }

    //get the number of players who are ready 
    pub fn get_players_ready(&self){
        self.players_ready;
    }

    pub fn get_guid(&mut self) -> u32{
        self.entity_count = self.entity_count + 1;
        self.entity_count
    }

    pub fn get_team(&mut self) -> u8 {
        let ret_team = self.team_count;
        self.team_count = self.team_count + 1;
        return ret_team;
    }

    pub fn get_connection_number(&self) -> u8{
        self.connection_number
    }

    pub fn add_player_controller(&mut self, controller: Controller){
        self.game_state_data.add_player_controller(controller);
    }

}