
use minion_card::{Minion, UID};
use card::Card;
use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
pub enum eControllerType {
    player,
    ai,
}

#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
pub enum eControllerState {
    mulligan,
    waiting_for_start,
    waiting_for_turn,
    in_turn,
}

#[derive(Clone)]
pub struct Controller {
    pub name: String,
    pub hero: String,
    pub controller_type: eControllerType,
    pub uid: UID,
    pub mana: u8,
    pub baseMana: u8,
    pub team: u8,
    pub controller_state: eControllerState,
    pub client_id: u32,

    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
    

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
    }

    pub fn add_card_to_deck(&mut self, card: Card) {
        self.deck.push(card);
    }

}
