
use minion_card::UID;
use card::Card;
use std::collections::HashSet;


#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
pub enum EControllerType {
    Player,
    AI,
}

#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
pub enum EControllerState {
    Mulligan,
    WaitingForStart,
    WaitingForTurn,
    InTurn,
}

#[derive(Clone)]
pub struct Controller {
    pub name: String,
    pub hero: String,
    pub controller_type: EControllerType,
    pub uid: UID,
    pub mana: u8,
    pub base_mana: u8,
    pub team: u8,
    pub controller_state: EControllerState,
    pub client_id: u32,

    pub deck: Vec<Card>,
    pub hand: Vec<Card>,

    pub seen_cards : HashSet<UID>,
    
    // minions that are in the deck to start with get created,
    // but as placed here until we are they are summoned
    // the reason for this is because it allows us to refer
    // to a particular minion before it is summoned in case we need
    // to modify it, or look it up
    pub unplayed_minions: Vec<UID>,
    pub in_play_minions: Vec<UID>
}

impl Controller {

    pub fn add_minion_to_unplayed(&mut self, minion_uid: UID) {
        self.unplayed_minions.push(minion_uid);
    }

    pub fn move_minion_from_unplayed_into_play(&mut self, minion_uid : UID) {
        let index = self.unplayed_minions.iter().position(|x| *x == minion_uid).unwrap();
        let val = self.unplayed_minions.remove(index);
        self.in_play_minions.push(val);
    }

    pub fn add_card_to_deck(&mut self, card: Card) {
        self.deck.push(card);
    }
     
    pub fn add_card_to_seen(&mut self, uid : UID) {
        self.seen_cards.insert(uid);
    }

    pub fn has_seen_card(&self, uid : UID) -> bool {
        self.seen_cards.contains(&uid)
    }

    pub fn get_card_from_deck<'a>(&'a self, uid : UID) -> Option<&'a Card>{
        
        for ref card in self.deck.iter() {
            if card.get_uid() == uid {
                return Some(&card);
            }
        }
        None
    }
}
