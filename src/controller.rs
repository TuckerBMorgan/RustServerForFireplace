
use minion_card::UID;
use card::Card;
use std::collections::HashSet;
use rand::{thread_rng, Rng};


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
    pub uid: UID,
    pub mana: u8,
    pub base_mana: u8,
    pub team: u8,
    pub controller_state: EControllerState,
    pub client_id: u32,

    pub deck: Vec<Card>,
    pub hand: Vec<Card>,

    pub seen_cards: HashSet<UID>,

    // minions that are in the deck to start with get created,
    // but as placed here until we are they are summoned
    // the reason for this is because it allows us to refer
    // to a particular minion before it is summoned in case we need
    // to modify it, or look it up
    pub unplayed_minions: Vec<UID>,
    pub in_play_minions: Vec<UID>,
}

impl Controller {
    pub fn add_minion_to_unplayed(&mut self, minion_uid: UID) {
        self.unplayed_minions.push(minion_uid);
    }

    pub fn move_minion_from_unplayed_into_play(&mut self, minion_uid: UID) {
        let index = self.unplayed_minions.iter().position(|x| *x == minion_uid).unwrap();
        let val = self.unplayed_minions.remove(index);
        self.in_play_minions.push(val);
    }

    pub fn add_card_to_deck(&mut self, card: Card) {
        self.deck.push(card);
    }

    pub fn move_card_from_deck_to_hand(&mut self, uid: UID) {
        let index = self.deck.iter().position(|x| x.get_uid() == uid).unwrap();
        let val = self.deck.remove(index);
        self.hand.push(val);
    }

    pub fn move_card_from_hand_to_deck(&mut self, uid: UID) {
        let index = self.hand.iter().position(|x| x.get_uid() == uid).unwrap();
        let val = self.hand.remove(index);
        self.deck.push(val);
        self.shuffle_deck();
    }

    pub fn set_base_mana(&mut self, base_mana: u8) {
        self.base_mana = base_mana;
    }

    pub fn get_base_mana(&self) -> u8 {
        self.base_mana.clone()
    }

    pub fn set_mana(&mut self, mana: u8) {
        self.mana = mana;
    }

    pub fn get_mana(&self) -> u8 {
        self.mana.clone()
    }

    pub fn set_controller_state(&mut self, controller_state: EControllerState) {
        self.controller_state = controller_state;
    }

    pub fn shuffle_deck(&mut self) {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let val_1 = rng.gen_range(0, self.deck.len());
            let val_2 = rng.gen_range(0, self.deck.len());

            let hold_1 = self.deck[val_1].clone();
            let hold_2 = self.deck[val_2].clone();

            self.deck[val_1] = hold_2;
            self.deck[val_2] = hold_1;

        }
    }

    pub fn get_mut_hand(&mut self) -> &mut Vec<Card> {
        &mut self.hand
    }

    pub fn add_card_to_seen(&mut self, uid: UID) {
        self.seen_cards.insert(uid);
    }

    pub fn has_seen_card(&self, uid: UID) -> bool {
        self.seen_cards.contains(&uid)
    }

    pub fn get_card_from_deck<'a>(&'a self, uid: UID) -> Option<&'a Card> {

        for ref card in self.deck.iter() {
            if card.get_uid() == uid {
                return Some(&card);
            }
        }
        None
    }

    pub fn get_copy_of_in_play(&self) -> Vec<UID> {
        self.in_play_minions.clone()
    }

    pub fn get_n_card_uids_from_deck(&self, mut n: u32) -> Vec<UID> {

        let mut rng = thread_rng();
        let mut rets: Vec<UID> = vec![];

        for _ in 0..n {
            let val = rng.gen_range(0, self.deck.len());
            let uid = self.deck[val].get_uid();
            if rets.contains(&uid) {
                n += 1;
            } else {
                rets.push(uid);
            }
        }

        rets
    }
    pub fn get_uid(&self) -> UID {
        self.uid.clone()
    }
}
