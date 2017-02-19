#![allow(dead_code)]

use rune_vm::Rune;
use std::any::{Any, TypeId};
use std::net::TcpStream;
use card::{Card, ECardType};
use controller::{Controller, EControllerState};
use minion_card::{Minion, UID, EMinionState};
use game_thread::GameThread;
use client_option::{ClientOption, OptionType, OptionsPackage};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::Decodable;
use client_message::OptionsMessage;
use tags_list::AURA;

use rand::thread_rng;
use entity::Entity;
use rhai::{Engine, FnRegister, Scope};

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{VecDeque, HashMap};
use std::slice::Iter;


use runes::deal_card::DealCard;
use runes::start_game::StartGame;
use runes::rotate_turn::RotateTurn;
use runes::shuffle_card::ShuffleCard;
use runes::new_controller::NewController;
use runes::mulligan::Mulligan;
use runes::play_card::PlayCard;
use runes::kill_minion::KillMinion;
use runes::modify_health::ModifyHealth;
use runes::set_health::SetHealth;
use runes::set_attack::SetAttack;



#[derive(Clone)]
pub struct GameStateData {
    controllers: Vec<Controller>,
    minions: HashMap<UID, Minion>,
    controller_uid_to_client_id: HashMap<UID, u32>,
    client_id_to_controller_uid: HashMap<UID, u32>,
    attacked_this_turn: Vec<UID>,
    entity_count: u32,
    on_turn_player: i8,
}

impl GameStateData {
    pub fn new() -> GameStateData {
        GameStateData {
            controllers: vec![],
            entity_count: 1, //this is really emportant that we start this at 1, beause new UIDS are the current value of this, but we need a value for not a UID, which is 0
            minions: HashMap::new(),
            controller_uid_to_client_id: HashMap::new(),
            client_id_to_controller_uid: HashMap::new(),
            on_turn_player: 0,
            attacked_this_turn: vec![],
        }
    }

    pub fn add_player_controller(&mut self, controller: Controller) {

        self.controller_uid_to_client_id
            .insert(controller.uid.clone(), controller.client_id.clone());

        self.client_id_to_controller_uid
            .insert(controller.client_id.clone(), controller.uid.clone());

        self.controllers.push(controller);

    }

    pub fn get_on_turn_player(&self) -> i8 {
        self.on_turn_player.clone()
    }

    pub fn add_has_attack(&mut self, uid: UID) {
        self.attacked_this_turn.push(uid);
    }

    pub fn set_on_turn_player(&mut self, on_turn_player: i8) {
        self.on_turn_player = on_turn_player;
    }

    pub fn get_controllers(&self) -> &Vec<Controller> {
        &self.controllers
    }

    pub fn get_mut_controllers(&mut self) -> &mut Vec<Controller> {
        &mut self.controllers
    }

    pub fn get_mut_minion(&mut self, minion_uid: UID) -> Option<&mut Minion> {
        self.minions.get_mut(&minion_uid)
    }

    pub fn get_minion(&self, minion_uid: UID) -> Option<&Minion> {
        self.minions.get(&minion_uid)
    }

    pub fn get_uid(&mut self) -> UID {
        self.entity_count = self.entity_count + 1;
        self.entity_count
    }

    pub fn get_number_of_controllers(&self) -> usize {
        self.controllers.len().clone()
    }

    pub fn get_client_id_from_controller_uid(&self, controller_uid: UID) -> &u32 {
        self.controller_uid_to_client_id.get(&controller_uid).unwrap()
    }

    pub fn get_controler_uid_from_client_id(&self, client_id: u32) -> UID {
        self.client_id_to_controller_uid.get(&client_id).unwrap().clone()
    }

    pub fn add_minion_to_minions(&mut self, minion: Minion) {
        self.minions.insert(minion.get_uid(), minion);
    }

    pub fn get_client_ids(&self) -> Vec<u32> {
        let mut ids = Vec::new();
        for controller in self.controllers.iter() {
            ids.push(controller.client_id.clone());
        }
        ids
    }

    pub fn get_controller_uids(&self) -> Vec<UID> {
        let mut uids = Vec::new();
        for controller in self.controllers.iter() {
            uids.push(controller.uid.clone());
        }
        uids
    }

    pub fn get_all_minions(&self) -> Vec<Minion> {
        let mut mins = vec![];
        for (_, v) in self.minions.clone() {
            mins.push(v.clone());
        }
        mins
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

    first_to_connect: Option<NewController>,

    mulligan_played_out: u8,
}

impl<'a> GameState<'a> {
    pub fn new(game_thread: &GameThread) -> GameState {
        let mut gs = GameState {
            game_thread: Some(game_thread),
            game_state_data: GameStateData::new(),
            players_ready: 0,
            team_count: 0,
            connection_number: 0,
            connections: vec![],
            rune_queue: VecDeque::new(),
            entities: HashMap::new(),
            script_engine: Engine::new(),
            game_scope: vec![],
            first_to_connect: None,
            mulligan_played_out: 0,
        };
        gs.filled_up_scripting_engine();
        return gs;
    }

    // rhai requires that we tell it about what it is going to be interacting with
    // this function does that
    pub fn filled_up_scripting_engine(&mut self) {

        //        self.script_engine.register_std_lin();
        self.script_engine.register_type::<GameStateData>();
        self.script_engine.register_type::<UID>();
        self.script_engine.register_type::<Minion>();
        self.script_engine.register_type::<ClientOption>();
        self.script_engine.register_type::<Vec<ClientOption>>();
        self.script_engine.register_type::<Vec<Minion>>();
        self.script_engine.register_type::<ModifyHealth>();
        self.script_engine.register_type::<SetHealth>();
        self.script_engine.register_type::<SetAttack>();
        self.script_engine.register_type::<&Rune>();
        self.script_engine.register_type::<Vec<&Rune>>();

        self.script_engine.register_fn("new_minion", Minion::new_other);
        self.script_engine.register_fn("minion_basic_info", Minion::set_basic_info);
        self.script_engine.register_fn("minion_attack_health_basic",
                                       Minion::set_attack_and_health_basics);
        self.script_engine.register_fn("minion_vec_push", );
        self.script_engine.register_fn("minion_set_spell_damage", Minion::set_spell_damage);
        self.script_engine.register_fn("set_uid", Minion::set_uid);
        self.script_engine.register_fn("minion_get_uid", Minion::get_uid);
        self.script_engine.register_fn("minion_get_total_health", Minion::get_total_health);
        self.script_engine.register_fn("minion_add_tag", Minion::add_tag_to);
        self.script_engine.register_fn("get_uid", GameStateData::get_uid);
        self.script_engine.register_fn("print", GameState::print);
        self.script_engine.register_fn("new_set_health", SetHealth::new);
        self.script_engine.register_fn("new_modify_health", ModifyHealth::new);
        self.script_engine.register_fn("new_set_attack", SetAttack::new);
    }

    pub fn print(string: String) {
        println!("{}", string);
    }

    //PLEASE READ THIS BEFORE USING
    //this will add the varible with the supplied name into the game Scope/
    //calling run_rhai_statement will execute a rhai statement with those in its scope, but also remove them in the same cal
    //also the reason we only support i64s is that that is what all numbers in rhai are
    pub fn add_i64_to_game_scope(&mut self, name: String, varible: i64) {
        self.game_scope.push((name.clone(), Box::new(varible.clone())));
    }

    //this only lasts in game scope until you call run_rhai_statement
    pub fn add_string_to_game_scope(&mut self, name: String, varible: String) {
        self.game_scope.push((name.clone(), Box::new(varible.clone())));
    }

    pub fn add_minion_to_game_scope(&mut self, name: String, varible: Minion) {
        self.game_scope.push((name.clone(), Box::new(varible.clone())));
    }

    pub fn add_type_vec_to_game_scope<T: 'static + Clone + fmt::Debug>(&mut self,
                                                                       name: String,
                                                                       varible: Vec<T>) {
        self.game_scope.push((name.clone(), Box::new(varible.clone())));
    }

    pub fn run_rhai_statement<T: Any + Clone>(&mut self,
                                              rhai_statement: &String,
                                              with_write_back: bool)
                                              -> T {

        self.game_scope.push(("game_state".to_string(), Box::new(self.game_state_data.clone())));

        let result = self.script_engine
            .eval_with_scope::<T>(&mut self.game_scope, &rhai_statement[..]);
        if with_write_back == true {
            for &mut (_, ref mut val) in &mut self.game_scope.iter_mut().rev() {
                match val.downcast_mut::<GameStateData>() {
                    Some(as_down_cast_struct) => {
                        self.game_state_data = as_down_cast_struct.clone();
                    }
                    None => {
                        //                    println!("problem getting game state back");
                    }
                }
            }
            // since we have a scope we carry around, we have to do this, because we can have two varibles with the same name in the scope
            self.game_scope.clear();
        }
        //  I like keeping this print statement around so that it I can use it when the rhai system breaks
        //  println!("{:?}", &result);
        result.unwrap()
    }

    pub fn execute_rune(&mut self, rune: Box<Rune>) {

        if self.is_rune_queue_empty() == false {
            self.add_rune_to_queue(rune)
        } else {
            self.process_rune(rune);
        }
    }

    pub fn process_rune(&mut self, rune: Box<Rune>) {

        println!("executing rune {}", rune.to_json());
        rune.execute_rune(self);

        let controllers = self.game_state_data.get_controller_uids();

        for controller in controllers {
            if rune.can_see(controller, self) {
                self.report_rune_to_client(self.game_state_data
                                               .get_client_id_from_controller_uid(controller)
                                               .clone(),
                                           rune.to_json());
            }
        }

        if self.is_rune_queue_empty() == false {
            let next_rune = self.remove_rune_from_queue();
            self.process_rune(next_rune);
        }
    }

    #[allow(dead_code)]
    pub fn populate_deck(&mut self, controller: &mut Controller, card_ids: Vec<String>) {

        for card_id in card_ids {

            let mut f = File::open("content/cards/".to_string() + &card_id.clone() +
                                   &".arhai".to_string())
                .unwrap();

            let mut contents = String::new();
            let result = f.read_to_string(&mut contents);

            match result {
                Ok(_) => {}
                Err(_) => {
                    println!("error reading file {} for deck creations", card_id.clone());
                }
            }

            let spl: Vec<&str> = contents.split("@@").collect();
            if spl[0].contains("minion") {
                let proto_minion = Minion::parse_minion_file(contents.clone());

                let p_copy = proto_minion.clone();

                let ks = p_copy.keys().clone();
                let mut minion =
                    self.run_rhai_statement::<Minion>(&proto_minion.get(&"create_minion_function"
                                                              .to_string())
                                                          .unwrap(),
                                                      true);

                minion.set_minion_state(EMinionState::NotInPlay);

                minion.set_functions(proto_minion);
                let play_card = Card::new(minion.get_cost() as u8,
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
        }
    }

    pub fn parse_deck(deck_file_name: String) -> Vec<String> {
        // println!("{}", "content/decks/".to_string() + &deck_file_name);
        let f = File::open("content/decks/".to_string() + &deck_file_name).unwrap();
        let reader = BufReader::new(f);
        let mut cards: Vec<String> = Vec::new();

        for line in reader.lines() {
            cards.push(line.unwrap().clone());
        }
        cards
    }

    pub fn get_minion(&self, minion_uid: UID) -> Option<&Minion> {
        self.game_state_data.get_minion(minion_uid)
    }

    pub fn get_mut_minion(&mut self, minion_uid: UID) -> Option<&mut Minion> {
        self.game_state_data.get_mut_minion(minion_uid)
    }

    // adds a rune to the rune queue, this is down when a executing rune creates a rune
    pub fn add_rune_to_queue(&mut self, rune: Box<Rune>) {
        self.rune_queue.push_back(rune);
    }

    pub fn remove_rune_from_queue(&mut self) -> Box<Rune> {
        self.rune_queue.pop_front().unwrap()
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

    pub fn new_connection(&mut self, mut new_controller: NewController) {
        let use_first = self.first_to_connect.clone();

        match use_first {
            Some(first_to_connect) => {
                let mut good_first = self.first_to_connect.clone().unwrap();

                good_first.is_me = true;
                self.report_rune_to_client(good_first.client_id.clone(), good_first.to_json());
                good_first.is_me = false;

                self.report_rune_to_client(good_first.client_id.clone(), new_controller.to_json());

                new_controller.is_me = true;
                self.report_rune_to_client(new_controller.client_id.clone(),
                                           new_controller.to_json());
                new_controller.is_me = false;

                self.report_rune_to_client(new_controller.client_id.clone(), good_first.to_json());

                first_to_connect.execute_rune(self);
                new_controller.execute_rune(self);
            }
            None => {
                self.first_to_connect = Some(new_controller);
                return;
            }
        }
    }

    pub fn mulligan(&mut self, client_id: u32, indices: Vec<u8>) {

        let controller_uid = self.game_state_data.get_controler_uid_from_client_id(client_id);
        {
            match self.get_mut_controller_by_uid(controller_uid).unwrap().controller_state {
                EControllerState::Mulligan => {
                    let mut counter: usize = 0;
                    for i in indices.iter() {
                        let card_uid = self.get_mut_controller_by_uid(controller_uid)
                                .unwrap()
                                .get_mut_hand()
                                           [(*i as usize) - counter]
                            .get_uid();
                        counter += 1;
                        let sc = ShuffleCard::new(card_uid.clone(), controller_uid.clone());
                        self.execute_rune(Box::new(sc));
                    }
                }
                _ => {}
            }
        }

        let replacements = self.get_mut_controller_by_uid(controller_uid)
            .unwrap()
            .get_n_card_uids_from_deck(indices.len())
            .clone();

        for uid in replacements {
            let new_deal_card_rune = DealCard::new(uid.clone(), controller_uid.clone());
            self.execute_rune(Box::new(new_deal_card_rune));
        }

        if self.mulligan_played_out == 1 {

            let sg = StartGame::new();
            self.execute_rune(Box::new(sg));

            let rt = RotateTurn::new();
            self.execute_rune(Box::new(rt));

            let options = self.get_mut_controller_by_uid(controller_uid)
                .unwrap()
                .clone()
                .generate_options_from_every_source(self)
                .clone();
            self.get_mut_controller_by_uid(controller_uid)
                .unwrap()
                .set_client_options(options.clone());

            let op = OptionsPackage { options: options };

            self.report_rune_to_client(client_id.clone(), op.to_json());
        } else {
            self.mulligan_played_out += 1;
        }

        let mut controller = self.get_mut_controller_by_uid(controller_uid).unwrap();

        match controller.controller_state {
            EControllerState::Mulligan => {
                controller.controller_state = EControllerState::WaitingForStart;
            }
            _ => {}
        }
    }

    pub fn execute_option(&mut self, option_message: OptionsMessage) {
        let index = option_message.index.clone();
        let controller_index = self.get_on_turn_player();

        let controller_uid =
            self.game_state_data.get_controllers()[controller_index as usize].get_uid().clone();
        let option = self.game_state_data.get_controllers()[controller_index as usize]
            .get_client_option(index as usize)
            .clone();
        match option.option_type {
            OptionType::EAttack => {}
            OptionType::EPlayCard => {
                let card = self.game_state_data.get_controllers()[controller_index as usize]
                    .get_copy_of_card_from_hand(option.source_uid)
                    .unwrap();
                match card.get_card_type() {
                    ECardType::Minion => {
                        let pc = PlayCard::new(card.get_uid(),
                                               controller_uid,
                                               option_message.board_index as usize,
                                               option.target_uid);
                        self.execute_rune(Box::new(pc));
                    }  
                    ECardType::Spell => {}
                    ECardType::Weapon => {}
                }
            }
            OptionType::EEndTurn => {
                let rt = RotateTurn::new();
                self.execute_rune(Box::new(rt));
            }
        }

        self.resolve_state();
        let controller_index = self.get_on_turn_player();

        let new_op = self.game_state_data.get_controllers()[controller_index as usize]
            .clone()
            .generate_options_from_every_source(self);
        let mut_uid = self.game_state_data.get_controllers()[controller_index as usize].get_uid();
        let client_id = self.game_state_data.get_controllers()[controller_index as usize].client_id;

        self.get_mut_controller_by_uid(mut_uid).unwrap().set_client_options(new_op.clone());

        let op = OptionsPackage { options: new_op };
        self.report_rune_to_client(client_id, op.to_json());

        let mut resolve = self.resolve_state();

        while resolve {
            resolve = self.resolve_state();
        }
    }

    pub fn resolve_state(&mut self) -> bool {

        // if anything that could touch off a call of the function again, deaths, summons, etc etc, we set this to true
        let mut redo = false;

        let gsd = self.game_state_data.clone();

        let controllers = gsd.get_controllers().clone();


        for controller in controllers.iter() {

            if controller.get_life() <= 0 {
                //mark controller for death
            }

            let minions = controller.get_copy_of_in_play();
            for min in minions.iter() {

                let minion = gsd.get_minion(*min).unwrap().clone();

                let mut dead_minions = vec![];
                if minion.get_current_health() <= 0 {
                    let km = KillMinion::new(controller.get_uid(), minion.get_uid());
                    dead_minions.push(Box::new(km));
                    redo = true;
                }
                for rune in dead_minions {
                    self.execute_rune(rune);
                }
            }


            let mut previous_auras: HashMap<UID, Vec<UID>> = HashMap::new();

            for min in minions.iter() {

                let minion = self.get_minion(*min).unwrap().clone();

                if minion.get_auras().len() > 0 {
                    previous_auras.insert(minion.get_uid(), minion.get_auras());
                }
            }

            for min in minions.iter() {
                let mut minion = self.get_mut_minion(*min).unwrap().clone();
                minion.clear_auras();
            }

            for min in minions.iter() {
                let minion = self.get_minion(*min).unwrap().clone();
                
                if minion.has_tag(AURA.to_string()) {

                    let all_else = self.game_state_data.get_all_minions().clone();

                    self.add_minion_to_game_scope("minion".to_string(), minion.clone());
                    self.add_type_vec_to_game_scope::<Minion>("minions".to_string(),
                                                              all_else.clone());
                    let passed =
                        self.run_rhai_statement::<Vec<Minion>>(minion.get_function("filter_function"
                                                                       .to_string())
                                                                   .unwrap(),
                                                               false);

                    for get_auras in passed {
                        self.get_mut_minion(get_auras.get_uid())
                            .unwrap()
                            .add_aura(minion.get_uid());
                    }
                }
            }
            
            let mut current_auras: HashMap<UID, Vec<UID>> = HashMap::new();
            for min in minions.iter() {
                let minion = self.get_minion(*min).unwrap().clone();
                if minion.get_auras().len() > 0 {
                    current_auras.insert(minion.get_uid(), minion.get_auras());
                }
            }

            let old_keys: Vec<UID> = previous_auras.keys().map(|&k| k).collect();
            let new_keys: Vec<UID> = current_auras.keys().map(|&k| k).collect();

            for key in new_keys {

                let olds = previous_auras.get(&key.clone());

                match olds {
                    Some(olds) => {

                        let mut adds: Vec<UID> = vec![];
                        let mut removes = vec![];

                        let new_auras: Vec<UID> = current_auras.get(&key.clone()).unwrap().clone();

                        adds = new_auras.iter()
                            .filter(|x| match olds.iter().position(|y| *x == y) {
                                Some(_) => true,
                                _ => false,
                            })
                            .map(|&u| u)
                            .collect::<Vec<UID>>()
                            .clone();

                        removes = olds.iter()
                            .filter(|x| match new_auras.iter().position(|y| *x == y) {
                                Some(_) => true,
                                _ => false,
                            })
                            .map(|&u| u)
                            .collect::<Vec<UID>>()
                            .clone();

                        for remove in removes.iter() {
                            let enchanter = self.get_minion(*remove).unwrap().clone();
                            self.add_minion_to_game_scope("enchanter".to_string(),
                                                          enchanter.clone());
                            let loser = self.get_minion(key.clone()).unwrap().clone();
                            self.add_minion_to_game_scope("loser".to_string(), loser);
                            let rhai_statement = enchanter.get_function("remove_aura".to_string())
                                .clone();

                            //because of sized i ssues, this will call the remove runes itself
                            let runes =
                                self.run_rhai_statement::<Vec<&Rune>>(rhai_statement.unwrap(),
                                                                      false);

                            for rune in runes.iter() {
                                self.execute_rune(rune.into_box());
                                redo = true;
                            }
                        }

                        for add in adds.iter() {
                            let enchanter = self.get_minion(*add).unwrap().clone();
                            self.add_minion_to_game_scope("giver".to_string(), enchanter.clone());
                            let getter = self.get_minion(key.clone()).unwrap().clone();
                            self.add_minion_to_game_scope("getter".to_string(), getter);
                            let rhai_statment = enchanter.get_function("apply_aura".to_string())
                                .clone();

                            let runes =
                                self.run_rhai_statement::<Vec<&Rune>>(rhai_statment.unwrap(),
                                                                      false);

                            for rune in runes.iter() {
                                redo = true;
                                self.execute_rune(rune.into_box());
                            }
                        }

                    }
                    _ => {}
                }
            }

        }

        redo
    }

    pub fn get_controller_number(&self) -> usize {
        self.game_state_data.get_number_of_controllers()
    }

    pub fn add_player_controller(&mut self, controller: Controller) {

        self.game_state_data.add_player_controller(controller);

        if self.game_state_data.get_number_of_controllers() == 2 {

            let _rng = thread_rng();
            let first: u16 = 0; //in the release this has to be a

            let other = 1 - first;

            let first_hand = self.game_state_data.get_mut_controllers()[first as usize]
                .get_n_card_uids_from_deck(3);

            let second_hand = self.game_state_data.get_mut_controllers()[other as usize]
                .get_n_card_uids_from_deck(4);
            let first_uid = self.game_state_data.get_mut_controllers()[first as usize].uid.clone();
            let sec_uid = self.game_state_data.get_mut_controllers()[other as usize].uid.clone();

            for uid in first_hand {
                let new_deal_card_rune = DealCard::new(uid.clone(), first_uid);
                self.execute_rune(Box::new(new_deal_card_rune));
            }
            self.game_state_data.get_mut_controllers()[first as usize].controller_state =
                EControllerState::Mulligan;

            for uid in second_hand {
                let new_deal_card_rune = DealCard::new(uid.clone(), sec_uid);
                self.execute_rune(Box::new(new_deal_card_rune));
            }
            self.game_state_data.get_mut_controllers()[other as usize].controller_state =
                EControllerState::Mulligan;
            self.game_state_data.set_on_turn_player(other as i8);

            let mul = Mulligan::new();
            self.execute_rune(Box::new(mul));
        }
    }

    pub fn get_controller_by_index(&self, index: usize) -> &Controller {
        &self.game_state_data.get_controllers()[index]
    }

    pub fn get_mut_controller_by_index(&mut self, index: usize) -> &mut Controller {
        &mut self.game_state_data.get_mut_controllers()[index]
    }

    pub fn get_mut_controller_by_uid(&mut self, controller_uid: UID) -> Option<&mut Controller> {
        let index =
            self.game_state_data.controllers.iter().position(|x| x.uid == controller_uid).unwrap();
        self.game_state_data.controllers.get_mut(index)
    }

    pub fn get_controller_by_uid(&self, controller_uid: UID) -> Option<&Controller> {
        let index =
            self.game_state_data.controllers.iter().position(|x| x.uid == controller_uid).unwrap();
        self.game_state_data.controllers.get(index)
    }

    pub fn get_controller_client_id(&self) -> Vec<u32> {
        self.game_state_data.get_client_ids()
    }

    pub fn get_on_turn_player(&self) -> i8 {
        self.game_state_data.get_on_turn_player().clone()
    }

    pub fn set_on_turn_player(&mut self, on_turn_player: i8) {
        self.game_state_data.set_on_turn_player(on_turn_player);
    }

    pub fn get_other_controller(&self, not_this_controller_uid: UID) -> &Controller {
        let index = self.game_state_data
            .controllers
            .iter()
            .position(|x| x.uid != not_this_controller_uid)
            .unwrap();
        self.game_state_data.controllers.get(index).unwrap()
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
        let ret_team = self.team_count.clone();
        self.team_count = self.team_count + 1;
        return ret_team;
    }

    pub fn get_connection_number(&self) -> u8 {
        self.connection_number
    }

    pub fn add_to_attacked_this_turn(&mut self, uid: UID) {
        self.game_state_data.add_has_attack(uid);
    }
}
