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
    Dead,
    MarkForDestroy,
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
    enchantments: Vec<UID>,
    functions: HashMap<String, String>,

    // the attack varibles, baseAttack is the default value
    // currentAttack is what we use for how much damage we do
    // totalAttack is the current ceiling for attack for the minion
    base_attack: u16,
    current_attack: u16,
    total_attack: u16,

    // the health varibles, baseHealth is the default value
    // currentHealth is how much the minion has at the moment, damage included
    // totalHealth is the current ceiling for health for the minion
    base_health: i32,
    current_health: i32,
    total_health: i32,

    spell_damage: u8,
}

impl Minion {
    pub fn new(cost: u16,
               id: String,
               uid: UID,
               name: String,
               set: String,
               base_attack: u16,
               base_health: i32)
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
            auras: vec![],
            enchantments: vec![],
            spell_damage: 0,
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
            auras: vec![],
            enchantments: vec![],
            spell_damage: 0,
        }
    }

    pub fn add_aura(&mut self, auras_origin: UID) {
        self.auras.push(auras_origin);
    }

    pub fn get_auras(&self) -> Vec<UID> {
        self.auras.clone()
    }

    pub fn clear_auras(&mut self) {
        self.auras.clear();
    }

    pub fn add_enchantment(&mut self, enchantment_giver: UID) {
        self.enchantments.push(enchantment_giver);
    }

    pub fn get_enchantments(&self) -> Vec<UID> {
        self.enchantments.clone()
    }

    pub fn clear_enchantments(&mut self) {
        self.enchantments.clear();
    }

    pub fn set_minion_state(&mut self, new_state: EMinionState) {
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

    pub fn get_base_health(&self) -> i32 {
        self.base_health.clone()
    }

    pub fn set_current_health(&mut self, amount: i32) {

        if self.current_health + amount <= self.total_health {
            self.current_health += amount;
        } else if self.current_health + amount > self.total_health {
            self.current_health = self.total_health;
        }
    }

    pub fn get_current_health(&self) -> i32 {
        self.current_health.clone()
    }

    pub fn get_total_health(&self) -> i32 {
        self.total_health.clone()
    }

    pub fn set_total_health(&mut self, amount: i32) {
        self.total_health = amount;
        if self.current_health > self.total_health {
            self.current_health = self.total_health.clone();
        }
    }

    pub fn set_spell_damage(&mut self, amount: i64) {
        self.spell_damage = amount as u8;
    }

    // healper function for setting all varibles at one, used in summon minion functions in rhai
    #[allow(dead_code)]
    pub fn set_attack_and_health_basics(&mut self, basic_attack: i64, basic_health: i64) {
        self.base_attack = basic_attack as u16;
        self.total_attack = basic_attack as u16;
        self.current_attack = basic_attack as u16;

        self.base_health = basic_health as i32;
        self.total_health = basic_health as i32;
        self.current_health = basic_health as i32;
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

        blocks.remove(0); //have to remove the "minion" from the split as it is just an identifier and not something we need

        for block in blocks.iter() {
            let name_function_pair: Vec<&str> = block.split("**").collect();
            functions.insert(name_function_pair[0].to_string().trim().to_string(),
                             name_function_pair[1].to_string());
        }
        functions
    }

    pub fn get_function(&self, name: String) -> Option<&String> {
        self.functions.get(&name)
    }
}

impl OptionGenerator for Minion {
    fn generate_options(&self,
                        game_state: &mut GameState,
                        current_controller: &Controller)
                        -> Vec<ClientOption> {
        println!("I got here");
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
                        } else if !min.has_tag(STEALTH.to_string()) {
                            not_taunts.push(uid.clone());
                        }
                    }
                    _ => {}
                }
            }
            let mut use_uids = vec![];

            //normal attack options cannot ignore taunts
            if taunts.len() != 0 {
                for uids in taunts {
                    use_uids.push(uids.clone());
                }
            } else {
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
        } else {
            //TODO: make this runa rhai statment
            vec![]
        }


    }
}
