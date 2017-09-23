#![allow(dead_code)]

use rune_vm::{Rune, ERuneType};
use std::net::TcpStream;
use std::fmt::Display;
use card::{Card, ECardType};
use controller::{Controller, EControllerState};
use minion_card::{Minion, UID, EMinionState};
use game_thread::GameThread;
use client_option::{OptionType, OptionsPackage, ClientOption};
use client_message::OptionsMessage;
use tags_list::AURA;
use hlua::{self, Lua};
use std::fs::OpenOptions;

use rand::thread_rng;
use entity::Entity;
use rhai::{Engine, Scope};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::{VecDeque, HashMap, HashSet};

use runes::deal_card::DealCard;
use runes::start_game::StartGame;
use runes::rotate_turn::RotateTurn;
use runes::shuffle_card::ShuffleCard;
use runes::new_controller::NewController;
use runes::mulligan::Mulligan;
use runes::play_card::PlayCard;
use runes::kill_minion::KillMinion;
use runes::set_mana::SetMana;
use runes::set_health::SetHealth;
use runes::set_attack::SetAttack;
use runes::modify_attack::ModifyAttack;
use runes::modify_health::ModifyHealth;
use runes::create_card::CreateCard;
use runes::summon_minion::SummonMinion;
use runes::attack::Attack;
use runes::modify_hero_health::ModifyHeroHealth;
use std::mem;
use rustc_serialize::json;

#[derive(Clone, RustcDecodable, RustcEncodable,)]
pub struct GameStateData {
    controllers: Vec<Controller>,
    minions: HashMap<UID, Minion>,
    controller_uid_to_client_id: HashMap<UID, u32>,
    client_id_to_controller_uid: HashMap<UID, u32>,
    attacked_this_turn: Vec<UID>,
    entity_count: u32,
    on_turn_player: i8,
    ai_player_copy : bool,
}

//implement_for_lua!(i32, |mut _metatable|{});

implement_for_lua!(GameStateData, |mut _metatable| {
    let mut index = _metatable.empty_array("__index");
    index.set("get_uid",
              hlua::function1(|gsd: &mut GameStateData| gsd.get_uid()));
});

impl GameStateData {
    pub fn new(aipc: bool) -> GameStateData {
        GameStateData {
            controllers: vec![],
            entity_count: 1, //this is really emportant that we start this at 1, beause new UIDS are the current value of this, but we need a value for not a UID, which is 0
            minions: HashMap::new(),
            controller_uid_to_client_id: HashMap::new(),
            client_id_to_controller_uid: HashMap::new(),
            on_turn_player: 0,
            attacked_this_turn: vec![],
            ai_player_copy : aipc,
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

    pub fn get_has_attack(&self)->Vec<UID>{
        self.attacked_this_turn.clone()
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

    pub fn get_minion_no_option(&mut self, minion_uid: UID) -> Minion {
        self.minions.get(&minion_uid).unwrap().clone()
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
        if !self.minions.contains_key(&minion.get_uid()){
            self.minions.insert(minion.get_uid(), minion);
        }
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

    pub fn get_is_ai_copy(&self)->bool{
        return self.ai_player_copy.clone();
    }

    pub fn get_all_minions_in_play(&self) -> Vec<Minion> {
        let mut mins = vec![];
        for (_, v) in self.minions.clone() {
            match v.get_minion_state() {
                EMinionState::InPlay => {
                    mins.push(v.clone());

                }
                _ => {}
            }
        }
        mins
    }
    pub fn to_json(&self)->String{
        return json::encode(self).unwrap();
    }

    pub fn reset_attacks(&mut self){
        self.attacked_this_turn = vec![];
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
    lua: Lua<'a>,

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
            game_state_data: GameStateData::new(false),
            players_ready: 0,
            team_count: 0,
            connection_number: 0,
            connections: vec![],
            rune_queue: VecDeque::new(),
            entities: HashMap::new(),
            script_engine: Engine::new(),
            lua: Lua::new(),
            game_scope: vec![],
            first_to_connect: None,
            mulligan_played_out: 0,
        };

        //this is required because we have to have to tell lua about all the special types from our game
        gs.filled_up_scripting_engine();
        return gs;
    }

    // rhai requires that we tell it about what it is going to be interacting with
    // this function does that
    pub fn filled_up_scripting_engine(&mut self) {

        //this is for a bunch of the basic functions, like print
        self.lua.openlibs();
        {
            //we do this for each of the major types that lua will deal with
            let mut minion_namepsace = self.lua.empty_array("Minion");
            minion_namepsace.set("new",
                                 hlua::function7(|id,
                                                  uid,
                                                  cost,
                                                  set,
                                                  base_attack,
                                                  base_health,
                                                  name| {
                Minion::lua_new(id, uid, cost, set, base_attack, base_health, name)
            }));

        }

        {
            let mut rune_namespace = self.lua.empty_array("Rune");
            rune_namespace.set("new_start_game", hlua::function0(|| StartGame::new()));
            rune_namespace.set("new_set_mana",
                               hlua::function2(|uid, amount| SetMana::new(uid, amount)));
            rune_namespace.set("new_set_health",
                               hlua::function2(|uid, amount| SetHealth::new(uid, amount)));
            rune_namespace.set("new_set_attack",
                               hlua::function2(|uid, amount| SetAttack::new(uid, amount)));
            rune_namespace.set("new_modify_attack",
                               hlua::function2(|uid, amount| ModifyAttack::new(uid, amount)));
            rune_namespace.set("new_modify_health",
                               hlua::function2(|uid, amount| ModifyHealth::new(uid, amount)));
            rune_namespace.set("new_modify_hero_health",
                               hlua::function2(|uid, amount| ModifyHeroHealth::new(uid, amount)));
            rune_namespace.set("new_create_card",
                               hlua::function3(|id, uid, controller_uid| {
                                   CreateCard::new(id, uid, controller_uid)
                               }));
            rune_namespace.set("new_summon_minion",
                               hlua::function3(|min_uid, controller_uid, index| {
                                   SummonMinion::new(min_uid, controller_uid, index)
                               }));
        }

        {
            let mut enum_namespace = self.lua.empty_array("RuneTypeEnum");
            enum_namespace.set("new_start_game",
                               hlua::function1(|sg| ERuneType::StartGame(sg)));
            enum_namespace.set("new_set_mana", hlua::function1(|sm| ERuneType::SetMana(sm)));
            enum_namespace.set("new_modify_attack",
                               hlua::function1(|ma| ERuneType::ModifyAttack(ma)));
            enum_namespace.set("new_modify_health",
                               hlua::function1(|mh| ERuneType::ModifyHealth(mh)));
            enum_namespace.set("new_modify_hero_health",
                               hlua::function1(|mh| ERuneType::ModifyHeroHealth(mh)));
            enum_namespace.set("new_set_health",
                               hlua::function1(|sh| ERuneType::SetHealth(sh)));
            enum_namespace.set("new_set_attack",
                               hlua::function1(|sa| ERuneType::SetAttack(sa)));
            enum_namespace.set("new_summon_minion",
                               hlua::function1(|sm| ERuneType::SummonMinion(sm)));
            enum_namespace.set("new_create_card",
                               hlua::function1(|cc| ERuneType::CreateCard(cc)));
        }


        //this is a common block I use for testing lua, so I prefer to just leave it here
        /*
        {
            let mut m : Minion = Minion::new(0, "Test".to_string(), 1, "test".to_string(), "set".to_string(), 0, 0);
            m.set_team(0);

            let mut mone : Minion = Minion::new(0, "Test".to_string(), 2, "test".to_string(), "set".to_string(), 0, 0);
 
            mone.set_team(1);
 
            let mut mtwo : Minion = Minion::new(0, "Test".to_string(), 3, "test".to_string(), "set".to_string(), 0, 0);
            mtwo.set_team(0);

            self.lua.set("enchanter", m);

            let _ = self.lua.execute::<()>("minions = {}");
            
            {
                let mut table: hlua::LuaTable<_> = self.lua.get("minions").unwrap();
                table.set(1, mone);
                table.set(2, mtwo);
                table.set("n", 2);
                
            }

            let mut tab = self.run_lua_statement::<hlua::LuaTable<_>>(
                &r#"count = 1
                    result = {}
                    team1 = enchanter:get_team()
                    index = 1
                    while count <= minions["n"] do
                        min = minions[count]
                        team2 = min:get_team()
                        if team1 == team2 thenexit

                            result[index] = min
                            index = index + 1
                        end
                        count = count + 1
                    end"#.to_string()
                    ,false).unwrap();
                    
            let unfold = tab.iter::<i32, Minion>().filter_map(|e| e).map(|(_, v)| v).collect::<Vec<Minion>>().clone();
            
            for run in unfold {
          //      run.execute();
                println!("{:?}", run);
            }
            
            panic!("Just dont need anymore");

        }
        */


    }

    fn print<T: Display>(x: &mut T) -> () {
        println!("{}", x);
    }

    pub fn add_number_to_lua(&mut self, key: String, val: u32) {
        self.lua.set(key, val);
    }
    pub fn add_integer_to_lua(&mut self, key: String, val: i32) {
        self.lua.set(key, val);
    }

    //the only function to use when you want to execute a lua function
    //you may or may not want a result from this function, if you do, you have to type it, and then also insure that the lua statment you are executing
    //places the thing you want into a varible called "result" at the end of your function
    //TODO: have this auto return the object we want as the type we want (this is a little more difficult and would require several more work per rune)
    pub fn run_lua_statement<'b, T: hlua::LuaRead<hlua::PushGuard<&'b mut hlua::Lua<'a>>>>
        (&'b mut self,
         lua_statement: &String,
         with_write_back: bool)
         -> Option<T> {

        self.lua.set("game_state_data", self.game_state_data.clone());

        let result = self.lua.execute::<()>(&lua_statement[..]);
        match result {
            Ok(_) => {}
            Err(e) => {
                panic!("{:?}", e);
            }
        }

        //this is to speed things up, but also so you can just run something in lua to generate or test something
        //without needing to worry about effecting the game_state_data
        if with_write_back == true {
            self.game_state_data = self.lua.get("game_state_data").unwrap();
        }

        self.lua.get("result")
    }

    pub fn execute_rune(&mut self, rune: Box<Rune>) {

        if self.is_rune_queue_empty() == false {
            self.add_rune_to_queue(rune)
        } else {
            self.process_rune(rune);
        }
    }
    
    pub fn stage_rune(&mut self, rune: Box<Rune>) {
        self.add_rune_to_queue(rune);
    }

    

    pub fn process_rune(&mut self, rune: Box<Rune>) {

        if !self.game_state_data.ai_player_copy{
            println!("executing rune {}", rune.to_json());
            //append_rune_to_file(rune.to_json())
        }
        rune.execute_rune(self);

        let controllers = self.game_state_data.get_controller_uids();

        for controller in controllers {
            if rune.can_see(controller, self) && !self.game_state_data.ai_player_copy {
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
    pub fn populate_deck(&mut self, controller: UID, card_ids: Vec<String>) {

        for card_id in card_ids {
            let use_uid = self.get_uid();
            let cr = CreateCard::new(card_id.clone(), use_uid, controller);
            self.execute_rune(cr.into_box());
            {
                let play_card = {

                    let minion = self.get_mut_minion(use_uid).unwrap(); //.set_team(controller.team);

                    let new_card = Card::new(minion.get_cost() as u8,
                                          ECardType::Minion,
                                          minion.get_id(),
                                          minion.get_uid(),
                                          minion.get_name(),
                                          minion.get_uid().to_string());
                        new_card
                };

                // we do this because, a minion is not a card,
                // but something placed into the field BY a card
                // on the client we tell them about the minion
                // right before we tell them they can play it
                // and so the client can tell what to display based on the uid of the
                self.get_mut_controller_by_uid(controller).unwrap().add_card_to_deck(play_card);
            }
        }
    }

    pub fn add_minion_to_minions(&mut self, minion: Minion) {
        self.game_state_data.add_minion_to_minions(minion);
    }

    pub fn parse_deck(deck_file_name: String) -> Vec<String> {
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

            //After mulligan whoever is going first will be the player who gets the options
            let controller_index = self.get_on_turn_player();

            let controller_uid =
                self.game_state_data.get_controllers()[controller_index as usize].get_uid().clone();

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
        //println!("EXECUTING OPTION {:?}", option);
        match option.option_type {
            OptionType::EAttack => {
                let attack = Attack::new(option.source_uid, option.target_uid);
                self.execute_rune(attack.into_box());
            }
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


        let mut resolve = self.resolve_state();
        while resolve {
            resolve = self.resolve_state();
        }
        let controller_index = self.get_on_turn_player();

        self.game_state_data.get_controllers()[controller_index as usize]
            .clone()
            .clear_options();

        let mut new_op = self.game_state_data.get_controllers()[controller_index as usize]
            .clone()
            .generate_options_from_every_source(self);

        new_op.push(ClientOption::new(0, 0, OptionType::EEndTurn));
        let mut_uid = self.game_state_data.get_controllers()[controller_index as usize].get_uid();
        let client_id = self.game_state_data.get_controllers()[controller_index as usize].client_id;
        

        self.get_mut_controller_by_uid(mut_uid).unwrap().set_client_options(new_op.clone());
        let op = OptionsPackage { options: new_op };
        self.report_rune_to_client(client_id, op.to_json());
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

                    let all_else = self.game_state_data.get_all_minions_in_play().clone();

                    match minion.get_function("filter_function".to_string()) {
                        Some(func) => {

                            let ok_passed = {
                                {
                                    self.lua.set("enchanter", minion.clone());
                                    let _ = self.lua.execute::<()>("minions = {}");
                                    {
                                        let mut val = 1;
                                        let mut minions_table: hlua::LuaTable<_> =
                                            self.lua.get("minions").unwrap();
                                        minions_table.set("n", all_else.len() as u32);
                                        for min in all_else {
                                            minions_table.set(val, min);
                                            val += 1;
                                        }

                                    }
                                }
                                let mut passed =
                                    self.run_lua_statement::<hlua::LuaTable<_>>(func, false)
                                        .unwrap();

                                let min_vec = passed.iter::<i32, Minion>()
                                    .filter_map(|e| e)
                                    .map(|(_, v)| v)
                                    .collect::<Vec<Minion>>()
                                    .clone();
                                min_vec
                            };

                            for get_auras in ok_passed {
                                self.get_mut_minion(get_auras.get_uid())
                                    .unwrap()
                                    .add_aura(minion.get_uid());
                            }


                        }
                        None => {
                            panic!("Was unable to get a filter function from {}",
                                   minion.get_id());
                        }
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

            //the key is a minion that at least used to have an aura
            for key in old_keys {

                let mut old_auras = HashSet::new();

                for older_keys in previous_auras.get(&key).unwrap() {
                    old_auras.insert(older_keys);
                }

                //if this matchs goes through that means that they have some auras after the remove and add process
                match current_auras.get(&key) {
                    Some(new_auras_vec) => {
                        //now we need to see those auras that they gained, and lost
                        //new auras are all the auras they have at the moment
                        //if they are not in old auras they we must actually add them
                        //if ones from old auras are not new_auras we have to actaually remove them
                        //if they are in both we do nothing
                        let mut new_auras = HashSet::new();
                        for new_auras_keys in new_auras_vec {
                            new_auras.insert(new_auras_keys);
                        }

                        let those_that_we_remove = old_auras.difference(&new_auras);
                        let those_that_we_add = new_auras.difference(&old_auras);


                        for the_removes in those_that_we_remove {

                            let loser = self.get_minion(key).unwrap().clone();
                            self.lua.set("loser", loser.clone());

                            let enchanter = self.get_minion(**the_removes).unwrap().clone();
                            self.lua.set("enchanter", enchanter.clone());

                            match enchanter.get_function("remove_aura".to_string()) {
                                Some(function) => {

                                    //we have to wrap in a closure because run_lua_statement holds a referance
                                    let runes =
                                        {
                                            let mut rune_check = self.run_lua_statement::<hlua::LuaTable<_>>(function, false).unwrap();

                                            let o = rune_check.iter::<i32, ERuneType>()
                                                .filter_map(|e| e)
                                                .map(|(_, v)| v)
                                                .collect::<Vec<ERuneType>>()
                                                .clone();
                                            o
                                        };

                                    for rune in runes {
                                        self.execute_rune(rune.unfold());
                                    }
                                }
                                None => {
                                    println!("Was unable to get a function for {}",
                                             enchanter.get_id());
                                }
                            }
                        }

                        for the_adds in those_that_we_add {

                            let getter = self.get_minion(key).unwrap().clone();
                            self.lua.set("getter", getter.clone());

                            let giver = self.get_minion(**the_adds).unwrap().clone();

                            self.lua.set("giver", giver.clone());

                            match giver.get_function("apply_aura".to_string()) {
                                Some(function) => {

                                    let runes =
                                        {
                                            let mut rune_check = self.run_lua_statement::<hlua::LuaTable<_>>(function, false).unwrap();

                                            let o = rune_check.iter::<i32, ERuneType>()
                                                .filter_map(|e| e)
                                                .map(|(_, v)| v)
                                                .collect::<Vec<ERuneType>>()
                                                .clone();
                                            o
                                        };

                                    for rune in runes {
                                        self.execute_rune(rune.unfold());
                                        redo = true;
                                    }
                                }
                                None => {
                                    println!("was unable to find a add aura function for {}",
                                             giver.get_id());
                                }
                            }
                        }
                    }
                    None => {
                        //remove all the auras that person had, they have gained no new ones, and lost all the ones they used to have
                        for oa in old_auras {

                            let loser = self.get_minion(key).unwrap().clone();
                            self.lua.set("loser", loser.clone());

                            let enchanter = self.get_minion(*oa).unwrap().clone();
                            self.lua.set("enchanter", enchanter.clone());

                            match enchanter.get_function("remove_aura".to_string()) {
                                Some(function) => {

                                    let runes =
                                        {
                                            let mut rune_check = self.run_lua_statement::<hlua::LuaTable<_>>(function, false).unwrap();

                                            let o = rune_check.iter::<i32, ERuneType>()
                                                .filter_map(|e| e)
                                                .map(|(_, v)| v)
                                                .collect::<Vec<ERuneType>>()
                                                .clone();
                                            o
                                        };

                                    for rune in runes {
                                        self.execute_rune(rune.unfold());
                                        redo = true;
                                    }
                                }
                                None => {
                                    println!("Was unable to get a function for {}",
                                             enchanter.get_id());
                                }
                            }
                        }
                    }
                }

            }
            let mut final_adds = vec![];
            let new_keys: Vec<UID> = current_auras.keys().map(|&k| k).collect();
            for new_key in new_keys {
                match previous_auras.get(&new_key) {
                    Some(_) => {}
                    None => {
                        final_adds.push(new_key.clone());
                    }
                }
            }

            for final_add in final_adds {
                let adds = current_auras.get(&final_add);
                match adds {
                    Some(adds) => {
                        //final add is the getter, uid is the giver
                        for uid in adds {

                            let getter = self.get_minion(final_add).unwrap().clone();
                            self.lua.set("getter", getter.clone());

                            let giver = self.get_minion(*uid).unwrap().clone();
                            self.lua.set("giver", giver.clone());

                            match giver.get_function("apply_aura".to_string()) {
                                Some(function) => {

                                    let runes =
                                        {
                                            let mut rune_check = self.run_lua_statement::<hlua::LuaTable<_>>(function, false).unwrap();

                                            let o = rune_check.iter::<i32, ERuneType>()
                                                .filter_map(|e| e)
                                                .map(|(_, v)| v)
                                                .collect::<Vec<ERuneType>>()
                                                .clone();
                                            o
                                        };

                                    for rune in runes {
                                        self.execute_rune(rune.unfold());
                                        redo = true;
                                    }
                                }
                                None => {
                                    println!("was unable to find a add aura function for {}",
                                             giver.get_id());
                                }
                            }



                        }

                    }
                    None => {
                        println!("Problems with getting keys for {}", final_add);
                    }
                }
            }
        }

        redo
    }

    pub fn get_controller_number(&self) -> usize {
        self.game_state_data.get_number_of_controllers()
    }

    pub fn is_ai_copy_running(&self)->bool{
        return self.game_state_data.get_is_ai_copy()
    }

    pub fn add_player_controller(&mut self, controller: Controller, deck: String) {
        
        {
            let cont_uid = controller.get_uid();
            self.game_state_data.add_player_controller(controller);
            let card_names = GameState::parse_deck(deck.clone());
            self.populate_deck(cont_uid, card_names);
        }



        if self.game_state_data.get_number_of_controllers() == 2 && !self.game_state_data.get_is_ai_copy(){

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

    pub fn has_attacked_this_turn(&self)->Vec<UID>{
        self.game_state_data.get_has_attack()
    }
    pub fn reset_attack_list(&mut self){
        self.game_state_data.reset_attacks();
    }
    
    pub fn get_game_state_data(&self)->GameStateData{
        return self.game_state_data.clone();
    }
    
    pub fn swap_gsd(&mut self, gsd: &mut GameStateData){
        mem::swap(&mut self.game_state_data, gsd);
    }
    pub fn send_msg(&self, client_id: u32, message: String){
        self.game_thread.unwrap().report_message(client_id, message.clone()); 
    }
}


pub fn append_rune_to_file(rune:String){
    let file = OpenOptions::new().create(true).write(true).append(true).open("rune_history.txt");
    match file{
        Ok(mut fil)=>{
            fil.write_all((rune+"\n").as_bytes());
            fil.sync_data();
        },
        Err(e)=>{
            println!("{}",e)
        }
    }
        
}