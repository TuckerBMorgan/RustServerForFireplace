#![allow(dead_code)]

use std::collections::HashSet;

pub type UID = u32;

pub enum EFileReadResult {
    GoodRead,
    BadFileOpen,
    BadFileRead,
}

pub struct ProtoMinion {
    pub create_minion_function: String,
    pub battle_cry_function: String,
    pub take_damage_function: String,
}

impl ProtoMinion {
    pub fn new(create_minion_function: String,
               battle_cry_function: String,
               take_damage_function: String)
               -> ProtoMinion {
        ProtoMinion {
            create_minion_function: create_minion_function,
            battle_cry_function: battle_cry_function,
            take_damage_function: take_damage_function,
        }
    }
}

#[derive(Clone)]
pub struct Minion {
    cost: u16,
    id: String,
    uid: UID,
    name: String,
    set: String,

    tags : HashSet<String>,

    battle_cry_function: String,
    take_damage_function: String,

    // the attack varibles, baseAttack is the default value
    // currentAttack is what we use for how much damage we do
    // totalAttack is the current ceiling for attack for the minion
    base_attack: u16,
    current_attack: u16,
    total_attack: u16,

    // the health varibles, baseHealth is the default value
    // currentHealth is how much the minion has at the moment, damage included
    // totalHealth is the current ceiling for health for the minion
    base_health: u16,
    current_health: u16,
    total_health: u16,
}

impl Minion {
    pub fn new(&mut self,
               cost: u16,
               id: String,
               uid: UID,
               name: String,
               set: String,
               base_attack: u16,
               base_health: u16)
               -> Minion {

        Minion {
            cost: cost,
            id: id,
            uid: uid,
            name: name,
            set: set,
            tags : HashSet::new(),
            base_attack: base_attack,
            current_attack: base_attack,
            total_attack: base_attack,
            base_health: base_health,
            current_health: base_health,
            total_health: base_health,
            battle_cry_function: "null".to_string(),
            take_damage_function: "null".to_string(),
        }
    }

    // this is the version that the rhai system uses to create card
    // rhai has a limit on the number of paramaters that a function can have
    pub fn new_other(&mut self) -> Minion {
        Minion {
            cost: 0,
            id: "default".to_string(),
            uid: 0,
            name: "default".to_string(),
            set: "default".to_string(),
            tags : HashSet::new(),
            base_attack: 0,
            current_attack: 0,
            total_attack: 0,
            base_health: 0,
            current_health: 0,
            total_health: 0,
            battle_cry_function: "default".to_string(),
            take_damage_function: "default".to_string(),
        }
    }

    pub fn add_tag_to(&mut self, tag : String) {
        self.tags.insert(tag.clone());
    }

    pub fn get_cost(&self) -> u16 {
        self.cost.clone()
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_uid(&self) -> u32 {
        self.uid.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_set(&self) -> String {
        self.set.clone()
    }

    pub fn get_base_attack(&self) -> u16 {
        self.base_attack.clone()
    }

    pub fn get_current_attack(&self) -> u16 {
        self.current_attack.clone()
    }

    pub fn get_total_attack(&self) -> u16 {
        self.total_attack.clone()
    }

    pub fn set_base_attack(&mut self, attack: u16) {
        self.base_attack = attack;
    }

    pub fn set_current_attack(&mut self, attack: u16) {
        self.current_attack = attack
    }

    pub fn set_total_attack(&mut self, attack: u16) {
        self.total_attack = attack
    }

    pub fn get_base_health(&self) -> u16 {
        self.base_health.clone()
    }

    pub fn get_current_health(&self) -> u16 {
        self.current_health.clone()
    }

    pub fn get_total_health(&self) -> u16 {
        self.total_health.clone()
    }

    // healper function for setting all varibles at one, used in summon minion functions in rhai
    #[allow(dead_code)]
    pub fn set_attack_and_health_basics(&mut self, basic_attack: u16, basic_health: u16) {
        self.base_attack = basic_attack;
        self.total_attack = basic_attack;
        self.current_attack = basic_attack;

        self.base_health = basic_health;
        self.total_health = basic_health;
        self.current_health = basic_health;
    }

    // sets name, uid, id, and set, this is to get around the rhais function paramater limit
    #[allow(dead_code)]
    pub fn set_basic_info(&mut self,
                          name: String,
                          uid: u32,
                          set: String,
                          id: String,
                          cost: u16) {
        self.name = name;
        self.uid = uid;
        self.set = set;
        self.id = id;
        self.cost = cost;
    }

    pub fn set_battle_cry(&mut self, battle_cry_function: String) {
        self.battle_cry_function = battle_cry_function;
    }

    pub fn get_battle_cry(& self) ->String {
        self.battle_cry_function.clone()
    }

    pub fn set_take_damage(&mut self, take_damage_function: String) {
        self.take_damage_function = take_damage_function;
    }

    pub fn _get_take_damage(& self) -> String {
        self.take_damage_function.clone()
    }

    
    pub fn parse_minion_file(file_contents: String) -> Result<ProtoMinion, EFileReadResult> {
                let functions: Vec<&str> = file_contents.split("@@").collect();

                let mut create_minion_function: String = "hold".to_string();
                let mut battle_cry_function: String = "hold".to_string();
                let mut take_damage_function: String = "hold".to_string();
                let mut i: u32 = 0;

                for function in functions {
                    if i == 1 {
                        create_minion_function = String::from(function);
                    } else if i == 2 {
                        battle_cry_function = String::from(function);
                    } else if i == 3 {
                        take_damage_function = String::from(function);
                    }
                    i += 1;
                }

                let proto = ProtoMinion::new(create_minion_function,
                                             battle_cry_function,
                                             take_damage_function);
                Ok(proto)
    }
}
