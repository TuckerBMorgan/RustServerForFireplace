
use minion_card::Minion;
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
    pub guid: u32,
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
    pub unplayed_minions: Vec<Minion>,
}

impl Controller {
    pub fn add_minion_to_unplayed(&mut self, minion: Minion) {
        self.unplayed_minions.push(minion);
    }

    pub fn add_card_to_deck(&mut self, card: Card) {
        self.deck.push(card);
    }
}
