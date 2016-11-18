use rune_vm::Rune;
use std::any::Any;
use std::net::TcpStream;
use card::{Card, ECardType};
use controller::Controller;
use minion_card::{Minion, UID};
use game_thread::GameThread;
use runes::new_controller::NewController;
use runes::start_game::StartGame;
use entity::Entity;
use std::collections::{VecDeque, HashMap};
use rhai::{Engine, FnRegister, Scope};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone)]
pub struct GameStateData {
    controllers: Vec<Controller>,
    minions : HashMap<UID, Minion>, 
    entity_count: u32,
}

impl GameStateData {
    pub fn new() -> GameStateData {
        GameStateData {
            controllers: vec![],
            entity_count: 0,
            minions : HashMap::new()
        }
    }

    pub fn add_player_controller(&mut self, controller: Controller) {
        self.controllers.push(controller);
    }

    pub fn get_controllers(&self) -> &Vec<Controller> {
        &self.controllers
    }

    pub fn get_mut_controllers(&mut self) -> &mut Vec<Controller> {
        &mut self.controllers
    }

    pub fn get_minion(&mut self, minion_uid : UID) -> Option<&mut Minion> {
       self.minions.get_mut(&minion_uid)
    }

    pub fn get_uid(&mut self) -> UID {
        self.entity_count = self.entity_count + 1;
        self.entity_count
    }

    pub fn get_number_of_controllers(&self ) -> usize {
        self.controllers.len().clone()
    }

    pub fn add_minion_to_minions(&mut self, minion : Minion) {
       self.minions.insert(minion.get_uid(), minion);
    }

    pub fn get_client_ids(&self) -> Vec<u32> {
        let mut ids = Vec::new();
        for controller in self.controllers.iter() {
            ids.push(controller.client_id.clone());
        }
        ids
    }
}

pub struct GameState<'a> {
    // the number of players who have done all parts of handshake
    players_ready: u8,
    game_scope: Scope,

    team_count: u8,
    connection_number: u8,

    game_thread: Option<&'a GameThread>,
    game_state_data: GameStateData,
    script_engine: Engine,

    // the current runes waiting to be fired
    rune_queue: VecDeque<Box<Rune>>,
    // all entities in the game, spells, minions, and controllers
    entities: HashMap<String, Box<Entity>>,
    // all the streams of the current people connected so we can talk to them again
    connections: Vec<Box<TcpStream>>,
    // those entities that have attacked this turn
    attacked_this_turn: Vec<Box<Entity>>,

    first_to_connect : Option<NewController>
}

impl<'a> GameState<'a> {
    pub fn new(game_thread: &GameThread) -> GameState {
        GameState {
            game_thread: Some(game_thread),
            game_state_data: GameStateData::new(),
            players_ready: 0,
            team_count: 0,
            connection_number: 0,
            connections: vec![],
            rune_queue: VecDeque::new(),
            attacked_this_turn: vec![],
            entities: HashMap::new(),
            script_engine: Engine::new(),
            game_scope: vec![],
            first_to_connect : None
        }
    }

    // rhai requires that we tell it about what it is going to be interacting with
    // this function does that
    pub fn filled_up_scripting_engine(&mut self) {
        
        self.script_engine.register_type::<GameStateData>();
        self.script_engine.register_type::<Minion>();
        self.script_engine.register_fn("new_minion", Minion::new_other);
        self.script_engine.register_fn("minion_basic_info", Minion::set_basic_info);
        self.script_engine.register_fn("minion_attack_health_basic",
                                       Minion::set_attack_and_health_basics);
        self.script_engine.register_fn("minion_add_tag", Minion::add_tag_to);
    } 
  
    pub fn run_rhai_statement<T: Any + Clone>(&mut self, rhai_statement: &String) -> T {

        self.game_scope.push(("game_state".to_string(), Box::new(self.game_state_data.clone())));

        let result = self.script_engine
            .eval_with_scope::<T>(&mut self.game_scope, &rhai_statement[..]);

        for &mut (_, ref mut val) in &mut self.game_scope.iter_mut().rev() {
            match val.downcast_mut::<GameStateData>() {
                Some(as_down_cast_struct) => {
                    self.game_state_data = as_down_cast_struct.clone();
                }
                None => {
                    println!("problem getting game state back");
                }
            }
        }

        result.unwrap() 
    }

    #[allow(dead_code)]
    pub fn populate_deck(&mut self, controller: &mut Controller, card_ids: Vec<String>) {
        for card_id in card_ids {   
            let mut f = File::open(card_id.clone()).unwrap();

            let mut contents = String::new();
            let result = f.read_to_string(&mut contents);

            match result {
                Ok(_) => {

                },
                Err(_) => {
                    println!("error reading file {} for deck creations", card_id.clone());
                }
            }

            let spl : Vec<&str> = contents.split("@@").collect();

            if spl[0] == "minion" {
                let proto_minion = Minion::parse_minion_file(contents.clone());
                match proto_minion {
                    Ok(proto_minion_good) => {

                        let mut minion =
                            self.run_rhai_statement::<Minion>(
                                &proto_minion_good.create_minion_function);

                        minion.set_battle_cry(proto_minion_good.battle_cry_function);
                        minion.set_take_damage(proto_minion_good.take_damage_function);
                        let play_card = Card::new(minion.get_cost(),
                                                ECardType::Minion,
                                                minion.get_id(),
                                                self.get_uid(),
                                                minion.get_name(),
                                                minion.get_uid().to_string());

                        // we do this because, a minion is not a card,
                        // but something placed into the field BY a card
                        // on the client we tell them about the minion
                        // right before we tell them they can play it
                        // and so the client can tell what to display based on the uid of the
                        controller.add_minion_to_unplayed(minion.get_uid());
                        self.game_state_data.add_minion_to_minions(minion);
                        controller.add_card_to_deck(play_card);
                    }
                    Err(_) => {
                        println!("Problem loadingcard file");
                    }
                }
            }
        }
    }

    pub fn parse_deck(deck_file_name : String ) -> Vec<String> {
        let f = File::open("decks/".to_string() + &deck_file_name).unwrap();
        let reader = BufReader::new(f);
        let mut cards : Vec<String> = Vec::new();

        for line in reader.lines() {
            cards.push(line.unwrap().clone());
        }
        cards
    }

    pub fn get_minion(&mut self, minion_uid : UID) -> Option<&mut Minion> {
        self.game_state_data.get_minion(minion_uid)
    }

    // adds a rune to the rune queue, this is down when a executing rune creates a rune
    pub fn add_rune_to_queue(&mut self, rune: Box<Rune>) {
        self.rune_queue.push_back(rune);
    }

    pub fn report_rune_to_client(&self, client_id: u32, rune_string: String) {
        self.game_thread.unwrap().report_message(client_id, rune_string);
    }

    // do we have any runes wating to be executed
    pub fn is_rune_queue_empty(&self) -> bool {
        self.rune_queue.is_empty()
    }

    pub fn add_player_connection(&mut self, stream: Box<TcpStream>) {
        self.connections.push(stream);
    }

    // a player has finished the handshake for game start
    pub fn a_player_is_ready(&mut self) {
        self.players_ready = self.players_ready + 1;
    }

    // get the number of players who are ready
    pub fn get_players_ready(&self) {
        self.players_ready;
    }

    pub fn get_uid(&mut self) -> UID {
        self.game_state_data.entity_count = self.game_state_data.entity_count + 1;
        self.game_state_data.entity_count
    }

    pub fn get_team(&mut self) -> u8 {
        let ret_team = self.team_count;
        self.team_count = self.team_count + 1;
        return ret_team;
    }

    pub fn get_connection_number(&self) -> u8 {
        self.connection_number
    }

    pub fn new_connection(&mut self, new_controller : NewController) {
        let use_first = self.first_to_connect.clone();
        let good_first = self.first_to_connect.clone().unwrap();

        match use_first {
            Some(first_to_connect) => {
                first_to_connect.execute_rune(self);
                new_controller.execute_rune(self);
            },
            None => {
                self.first_to_connect = Some(new_controller);
                return;
            }
        }
        
        self.report_rune_to_client(good_first.client_id.clone(), good_first.to_json());
        self.report_rune_to_client(new_controller.client_id.clone(), new_controller.to_json());
    }

    pub fn add_player_controller(&mut self, controller: Controller) {

        self.game_state_data.add_player_controller(controller);
        if self.game_state_data.get_number_of_controllers() == 2 {
            println!("Start game!!\n");
            //they need to be told the game has started, and dealt their first hands
            let sg = StartGame::new();
            self.game_thread.unwrap().report_message_to_all(sg.to_json().clone());

        }
    }

    pub fn get_controller_by_uid(&mut self, controller_uid : UID) -> Option<&mut Controller> {        
        let index = self.game_state_data.controllers.iter().position(|x| x.uid == controller_uid).unwrap();
        self.game_state_data.controllers.get_mut(index)
    }

    pub fn get_controller_client_id(&self) -> Vec<u32> {
        self.game_state_data.get_client_ids()
    }
}