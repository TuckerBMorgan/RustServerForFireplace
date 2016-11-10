
use client_option::{ClientOption, EOptionType};
use game_state::GameState;
use minion_card::UID;

#[derive(Copy, Clone)]
pub enum ECardType {
    Minion,
    Spell,
    Weapon,
}

#[derive(Clone)]
pub struct Card {
    cost: u16,
    card_type: ECardType,
    id: String,
    uid: UID,
    name: String,
    //for play minion cards this is the uid of the minion
    //for spells this is the rhai file that executes the spell
    content : String
}

impl Card {
    pub fn new(cost: u16,
               card_type: ECardType,
               id: String,
               uid: UID,
               name: String,
               content : String)
               -> Card {
        Card {
            cost: cost,
            card_type: card_type,
            id: id,
            uid: uid,
            name: name,
            content: content
        }
    }

    pub fn get_cost(&self) -> u16 {
        self.cost.clone()
    }

    pub fn get_uid(&self) -> u32 {
        self.uid.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_card_type(&self) -> ECardType {
        self.card_type
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_play_options(&self, game_state: &GameState) -> Vec<ClientOption> {
        vec![]
    }

    pub fn execute_card(&mut self, game_state: &mut GameState) {
        
        match self.card_type {
            ECardType::Minion => {

            },
            ECardType::Spell => {

            },
            ECardType::Weapon => {

            }
        }

    }

    // fn set_cost(&mut self, cost: u16){
    // self.cost = cost;
    // }
    //
    // fn set_uid(&mut self, uid: String){
    // self.uid = uid;
    // }
    //
    // fn set_name(&mut self, name: String){
    // self.name = name;
    // }
    //
    // fn set_set(&mut self, set: String){
    // self.set = set;
    // }
    //
    // fn set_card_type(&mut self, card_type: ECardType){
    // self.card_type = card_type;
    // }
    //
    // fn set_id(&mut self, id: String){
    // self.id = id;
    // }
    //
}
