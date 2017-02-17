#![allow(dead_code)]

use std::collections::{HashSet, HashMap};
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

    functions: HashMap<String, String>,
    /*
    battle_cry_function: String,//does the minion do something when it is placed into combat
    take_damage_function: String,//does it need to do something special when it takes damage
    generate_option_function: String,//when it is on the field what can it attack
    target_function: String,//when it is in the hand how can I play it
    add_aura_function: String, //called when a minion needs an aura applied to them
    remove_aura_function: String, //called when a minion needs a aura removed from them
    */
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
            functions: HashMap::new(),
            base_attack: base_attack,
            current_attack: base_attack,
            total_attack: base_attack,
            base_health: base_health,
            current_health: base_health,
            total_health: base_health,
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
            functions: HashMap::new(),
            state: EMinionState::NotInPlay,
            auras: vec![]
        }
    }

    pub fn add_aura(&mut self, auras_origin: UID) {
        self.auras.push(auras_origin);
    }

    pub fn get_auras(&self) -> Vec<UID>{
        self.auras.clone()
    }

    pub fn clear_auras(&mut self) {
        self.auras.clear();
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
        
        if self.current_health + amount <= self.total_health {
            self.current_health += amount;
        }
        else if self.current_health + amount > self.total_health {
            self.current_health = self.total_health;
        }
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

    pub fn set_functions(&mut self, functions: HashMap<String, String>) {
        self.functions = functions;
    }

    pub fn parse_minion_file(file_contents: String) -> HashMap<String, String> {

        let mut functions: HashMap<String, String> = HashMap::new();

        let mut blocks: Vec<&str> = file_contents.split("@@").collect();
        
        blocks.remove(0);//have to remove the "minion" from the split as it is just an identifier and not something we need

        for block in blocks.iter() {
            let name_function_pair: Vec<&str> = block.split("**").collect();
            functions.insert(name_function_pair[0].to_string(), name_function_pair[1].to_string());
        }
        functions
    }
    
    pub fn get_function(&self, name: String) -> Option<&String> {
        self.functions.get(&name)
    }
}

impl OptionGenerator for Minion {

    fn generate_options(&self, game_state: &mut GameState, current_controller: &Controller) -> Vec<ClientOption> {
        
        //this just means that the minions will follow the standard attack option rules
        if !self.functions.contains_key("generate_options_function") {

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