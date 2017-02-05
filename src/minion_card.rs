#![allow(dead_code)]

use std::collections::HashSet;
use client_option::{OptionGenerator, ClientOption, OptionType};
use tags_list::{TAUNT, STEALTH};
use game_state::GameState;
use controller::Controller;

pub type UID = u32;

pub enum EFileReadResult {
    GoodRead,
    BadFileOpen,
    BadFileRead,
}

#[derive(Clone, Debug)]
pub enum EMinionState {
    NotInPlay,
    InPlay,
    Dead
}

pub struct ProtoMinion {
    pub create_minion_function: String,
    pub battle_cry_function: String,
    pub take_damage_function: String,
    pub generate_options_function: String,
    pub target_function: String,
    pub add_aura_function: String,
    pub remove_aura_function: String
}

impl ProtoMinion {
    pub fn new(create_minion_function: String,
               battle_cry_function: String,
               take_damage_function: String, 
               generate_options_function: String,
               target_function: String,
               add_aura_function: String,
               remove_aura_function: String)
               -> ProtoMinion {
        ProtoMinion {
            create_minion_function: create_minion_function,
            battle_cry_function: battle_cry_function,
            take_damage_function: take_damage_function,
            generate_options_function: generate_options_function,
            target_function: target_function,
            add_aura_function: add_aura_function,
            remove_aura_function: remove_aura_function
        }
    }
}

#[derive(Clone, Debug)]
pub struct Minion {
    cost: u16,
    id: String,
    uid: UID,
    name: String,
    set: String,
    state: EMinionState,

    tags: HashSet<String>,
    auras: Vec<UID>,

    battle_cry_function: String,//does the minion do something when it is placed into combat
    take_damage_function: String,//does it need to do something special when it takes damage
    generate_option_function: String,//when it is on the field what can it attack
    target_function: String,//when it is in the hand how can I play it
    add_aura_function: String, //called when a minion needs an aura applied to them
    remove_aura_function: String, //called when a minion needs a aura removed from them

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
    pub fn new(cost: u16,
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
            tags: HashSet::new(),
            base_attack: base_attack,
            current_attack: base_attack,
            total_attack: base_attack,
            base_health: base_health,
            current_health: base_health,
            total_health: base_health,
            battle_cry_function: "null".to_string(),
            take_damage_function: "null".to_string(),
            generate_option_function: "null".to_string(),
            target_function: "null".to_string(),
            add_aura_function: "null".to_string(),
            remove_aura_function: "null".to_string(),
            state: EMinionState::NotInPlay,
            auras: vec![]
        }
    }

    // this is the version that the rhai system uses to create card
    // rhai has a limit on the number of paramaters that a function can have
    pub fn new_other() -> Minion {
        Minion {
            cost: 0,
            id: "default".to_string(),
            uid: 0,
            name: "default".to_string(),
            set: "default".to_string(),
            tags: HashSet::new(),
            base_attack: 0,
            current_attack: 0,
            total_attack: 0,
            base_health: 0,
            current_health: 0,
            total_health: 0,
            battle_cry_function: "default".to_string(),
            take_damage_function: "default".to_string(),
            generate_option_function: "default".to_string(),
            target_function: "default".to_string(),
            add_aura_function: "default".to_string(),
            remove_aura_function: "default".to_string(),
            state: EMinionState::NotInPlay,
            auras: vec![]
        }
    }

    pub fn add_aura(&mut self, auras_origin: UID) {
        self.auras.push(auras_origin);
    }

    pub fn set_minion_state(&mut self, new_state: EMinionState){
        self.state = new_state;
    }

    pub fn add_tag_to(&mut self, tag: String) {
        self.tags.insert(tag.clone());
    }

    pub fn remove_tag(&mut self, tag: String) {
        self.tags.remove(&tag);
    }

    pub fn has_tag(&self, tag: String) -> bool {
        self.tags.contains(&tag)
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

    pub fn set_current_health(&mut self, amount: u16) {
        self.current_health = amount;
    }

    pub fn get_current_health(&self) -> u16 {
        self.current_health.clone()
    }

    pub fn get_total_health(&self) -> u16 {
        self.total_health.clone()
    }

    pub fn set_total_health(&mut self, amount: u16) {
        self.total_health = amount;
        if self.current_health > self.total_health {
            self.current_health = self.total_health.clone();
        }
    }

    // healper function for setting all varibles at one, used in summon minion functions in rhai
    #[allow(dead_code)]
    pub fn set_attack_and_health_basics(&mut self, basic_attack: i64, basic_health: i64) {
        self.base_attack = basic_attack as u16;
        self.total_attack = basic_attack as u16;
        self.current_attack = basic_attack as u16;

        self.base_health = basic_health as u16;
        self.total_health = basic_health as u16;
        self.current_health = basic_health as u16;
    }

    pub fn set_uid(&mut self, uid: i64) {
        self.uid = uid as u32;
    }

    // sets name, uid, id, and set, this is to get around the rhais function paramater limit
    // also for whatever reason magic numbers in rhai as default i64 type, and there is
    // type conversion within rhai, so we take our u16, and cast them from i64s when we use them
    #[allow(dead_code)]
    pub fn set_basic_info(&mut self, name: String, uid: UID, set: String, id: String, cost: i64) {
        self.name = name;
        self.uid = uid;
        self.set = set;
        self.id = id;
        self.cost = cost as u16;
    }

    pub fn get_battle_cry(&self) -> String {
        self.battle_cry_function.clone()
    }

    pub fn _get_take_damage(&self) -> String {
        self.take_damage_function.clone()
    }

    pub fn get_target_function(&self) -> String {
        self.target_function.clone()   
    }

    pub fn set_proto_minion_function(&mut self, proto_minion: ProtoMinion){
        self.battle_cry_function = proto_minion.battle_cry_function;
        self.take_damage_function = proto_minion.take_damage_function;
        self.generate_option_function = proto_minion.generate_options_function;
        self.target_function = proto_minion.target_function;
        self.add_aura_function = proto_minion.add_aura_function;
        self.remove_aura_function = proto_minion.remove_aura_function;
    }

    pub fn parse_minion_file(file_contents: String) -> Result<ProtoMinion, EFileReadResult> {

        let functions: Vec<&str> = file_contents.split("@@").collect();

        let mut create_minion_function: String = "hold".to_string();
        let mut battle_cry_function: String = "hold".to_string();
        let mut take_damage_function: String = "hold".to_string();
        let mut generate_options_function: String = "hold".to_string();
        let mut target_function: String = "hold".to_string();
        let mut add_aura_function: String = "hold".to_string();
        let mut remove_aura_function: String = "hold".to_string();

        let mut i: u32 = 0;

        for function in functions {
            if i == 1 {
                create_minion_function = String::from(function);
            } else if i == 2 {
                battle_cry_function = String::from(function);
            } else if i == 3 {
                take_damage_function = String::from(function);
            } else if i == 4 {
                generate_options_function = String::from(function);
            } else if i == 5 {
                target_function = String::from(function);
            } else if i == 6 {
                add_aura_function = String::from(function);
            } else if i == 7 {
                remove_aura_function = String::from(function);
            }
            i += 1;
        }

        let proto = ProtoMinion::new(create_minion_function,
                                     battle_cry_function,
                                     take_damage_function,
                                     generate_options_function,
                                     target_function,
                                     add_aura_function,
                                     remove_aura_function);
        Ok(proto)
    }
}

impl OptionGenerator for Minion {

    fn generate_options(&self, game_state: &mut GameState, current_controller: &Controller) -> Vec<ClientOption> {
        
        //this just means that the minions will follow the standard attack option rules
        if self.generate_option_function.contains("default") {

            let other_controller = game_state.get_other_controller(current_controller.get_uid());

            let in_play = other_controller.get_copy_of_in_play();

            let mut taunts = vec![];
            let mut not_taunts = vec![];

            for uid in in_play {
                let min = game_state.get_minion(uid);
                match min {
                    Some(min) => {
                        if min.has_tag(TAUNT.to_string()) {
                            taunts.push(uid.clone());
                        }
                        else if !min.has_tag(STEALTH.to_string()) {
                            not_taunts.push(uid.clone());
                        }
                    },
                    _ => {

                    }
                }
            }
            let mut use_uids = vec![];

            //normal attack options cannot ignore taunts
            if taunts.len() != 0 {
                for uids in taunts {
                    use_uids.push(uids.clone());
                }
            }
            else {
                for uids in not_taunts {
                    use_uids.push(uids.clone());
                }
            }
            let mut client_options = vec![];
            for uids in use_uids {
                let co = ClientOption::new(self.uid.clone(), uids.clone(), OptionType::EAttack);
                client_options.push(co);
            }
            client_options
        }
        else {
            //TODO: make this runa rhai statment 
            vec![]
        }


    }

}