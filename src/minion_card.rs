use ::card::Card;
use std::fs::File;
use ::card::ECardType;
use regex::Regex;

pub enum EFileReadResult{
    GoodRead,
    BadFileOpen,
    BadFileRead
}

pub struct ProtoMinionCard{
    pub create_minion_function : String,
    pub battle_cry_function : String,
    pub take_damage_function : String
}

impl ProtoMinionCard {
    pub fn new(create_minion_function : String, battle_cry_function : String, take_damage_function : String) -> ProtoMinionCard{
        ProtoMinionCard{create_minion_function : create_minion_function, battle_cry_function : battle_cry_function,  take_damage_function : take_damage_function}
    }
}

#[derive(Clone)]
pub struct MinionCard {
    cost:   u16,
    id:   String,
    guid: String,
    name: String,
    set: String,

    battle_cry_function : String,
    take_damage_function : String,

    //the attack varibles, baseAttack is the default value
    //currentAttack is what we use for how much damage we do
    //totalAttack is the current ceiling for attack for the minion
    base_attack: u16,
    current_attack: u16,
    total_attack: u16,

    //the health varibles, baseHealth is the default value
    //currentHealth is how much the minion has at the moment, damage included
    //totalHealth is the current ceiling for health for the minion
    base_health: u16,
    current_health: u16,
    total_health: u16,
}

impl MinionCard {

    pub fn new(&mut self, cost: u16, id: String, 
            guid: String, name: String, set: String, 
            base_attack: u16, base_health: u16) -> MinionCard {
    
        MinionCard{cost: cost, id: id, guid: guid,
                   name: name, set: set,
                   base_attack: base_attack, current_attack: base_attack, total_attack: base_attack,
                   base_health: base_health, current_health: base_health, total_health: base_health,
                   battle_cry_function : "null".to_string(), take_damage_function : "null".to_string()}
    }
    
    //this is the version that the rhai system uses to create card
    //rhai has a limit on the number of paramaters that a function can have
    pub fn new_other(&mut self) -> MinionCard{
        MinionCard{cost: 0, id: "default".to_string(), guid: "deafult".to_string(),
                   name: "default".to_string(), set: "default".to_string(),
                   base_attack: 0, current_attack: 0, total_attack: 0,
                   base_health: 0, current_health: 0, total_health: 0,
                   battle_cry_function : "default".to_string(), take_damage_function : "default".to_string()}    

    }

    pub fn get_base_attack(&self) ->u16 {
        self.base_attack.clone()
    }

    pub fn get_current_attack(&self) -> u16 {
        self.current_attack.clone()
    }

    pub fn get_total_attack(&self) -> u16{
        self.total_attack.clone()
    }

    pub fn set_base_attack(&mut self, attack: u16){
        self.base_attack = attack;
    }

    pub fn set_current_attack(&mut self, attack: u16){
        self.current_attack = attack
    }

    pub fn set_total_attack(&mut self, attack: u16){
        self.total_attack = attack
    }

    //healper function for setting all varibles at one, used in summon minion functions in rhai
    pub fn set_attack_and_health_basics(&mut self, basic_attack : u16, basic_health : u16) {
        self.base_attack = basic_attack;
        self.total_attack = basic_attack;
        self.current_attack = basic_attack;

        self.base_health = basic_health;
        self.total_health = basic_health;
        self.current_health = basic_health;
    }

    //sets name, guid, id, and set, this is to get around the rhais function paramater limit
    pub fn set_basic_info(&mut self, name : String, guid : String, set : String, id : String, cost : u16) {
        self.name = name;
        self.guid = guid;
        self.set = set;
        self.id = id;
        self.cost = cost;
    }

    pub fn set_battle_cry(&mut self, battle_cry_function : String) {
        self.battle_cry_function = battle_cry_function;
    }

    pub fn set_take_damage(&mut self, take_damage_function : String) {
        self.take_damage_function = take_damage_function;
    }

    pub fn parse_minion_file( file_name: String ) -> Result<ProtoMinionCard, EFileReadResult> {
        use std::io::prelude::*;

        if let Ok(mut f) = File::open(file_name.clone()){
            let mut contents = String::new();


            if let Ok(r) = f.read_to_string(&mut contents){
                let functions :Vec<&str> = contents.split("@@").collect();
                
                let mut create_minion_function : String = "hold".to_string();
                let mut battle_cry_function : String = "hold".to_string();
                let mut take_damage_function : String = "hold".to_string();
                let mut i : u32 = 0;

                for function in functions{
                    if i == 0 {
                        create_minion_function = String::from(function);
                    }
                    else if i == 1 {
                        battle_cry_function = String::from(function);
                    }
                    else if i == 2 {
                        take_damage_function = String::from(function);
                    }
                    i+=1;
                }

                let proto = ProtoMinionCard::new(create_minion_function, battle_cry_function, take_damage_function);
                Ok(proto)
            }
            else{
                println!("Problem reading file{:?}", file_name);
                Err(EFileReadResult::BadFileRead)
            }
        }
        else {
            println!("Problem finding file {:?}", file_name);
            Err(EFileReadResult::BadFileOpen)
        }
    }
}