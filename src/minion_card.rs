use ::card::Card;
use ::card::ECardType;

#[derive(Clone)]
pub struct MinionCard {
    cost:   u16,
    card_type: ECardType,
    id:   String,
    guid: String,
    name: String,
    set: String,

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

    fn new(&mut self, cost: u16, card_type: ECardType, id: String, 
            guid: String, name: String, set: String, 
            base_attack: u16, current_attack: u16, total_attack: u16,
            base_health: u16, current_health: u16, total_health: u16) -> MinionCard {
    
        MinionCard{cost: cost, card_type: card_type, id: id, guid: guid,
                   name: name, set: set,
                   base_attack: base_attack, current_attack: current_attack, total_attack: total_attack,
                   base_health: base_health, current_health: current_health, total_health: total_health}
    }

    fn get_base_attack(&self) ->u16 {
        self.base_attack.clone()
    }

    fn get_current_attack(&self) -> u16 {
        self.current_attack.clone()
    }

    fn get_total_attack(&self) -> u16{
        self.total_attack.clone()
    }

    fn set_base_attack(&mut self, attack: u16){
        self.base_attack = attack;
    }

    fn set_current_attack(&mut self, attack: u16){
        self.current_attack = attack
    }

    fn set_total_attack(&mut self, attack: u16){
        self.total_attack = attack
    }

}