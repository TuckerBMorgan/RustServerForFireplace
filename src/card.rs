
use game_state::GameState;
use minion_card::UID;
use client_option::{OptionGenerator, ClientOption, OptionType};
use tags_list::{TARGET};
use controller::Controller;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum ECardType {
    Minion,
    Spell,
    Weapon,
}

#[derive(Clone)]
pub struct Card {
    cost: u8,
    card_type: ECardType,
    id: String,
    uid: UID,
    name: String,
    // for play minion cards this is the uid of the minion
    // for spells this is the rhai file that executes the spell
    content: String,
}

impl Card {
    pub fn new(cost: u8,
               card_type: ECardType,
               id: String,
               uid: UID,
               name: String,
               content: String)
               -> Card {
        Card {
            cost: cost,
            card_type: card_type,
            id: id,
            uid: uid,
            name: name,
            content: content,
        }
    }

    pub fn _get_cost(&self) -> u8 {
        self.cost.clone()
    }

    pub fn get_uid(&self) -> u32 {
        self.uid.clone()
    }

    pub fn _get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_card_type(&self) -> ECardType {
        self.card_type
    }

    pub fn _get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
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

impl OptionGenerator for Card {
    fn generate_options(&self, game_state : &mut GameState, controller: &Controller) -> Vec<ClientOption> {
        if controller.get_mana() >= self.cost {
            if !self.content.contains("default") {
                let minion = game_state.get_minion(self.content.parse().unwrap()).unwrap().clone();
                if minion.has_tag(TARGET.to_string()) {
                    return game_state.run_rhai_statement::<Vec<ClientOption>>(&minion.get_target_function(), false);
                }
                else {
                    let mut co = vec![];
                    co.push(ClientOption::new(self.uid, 0, OptionType::EPlayCard));
                    return co;
                }
            }
            let mut co = vec![];
            co.push(ClientOption::new(self.uid, 0, OptionType::EPlayCard));
            return co;
        }
        return vec![];
    }
}