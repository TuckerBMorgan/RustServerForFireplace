
use minion_card::MinionCard;
use card::Card;
use std::collections::HashMap;

#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
pub enum eControllerType
{
    player,
    ai
}

#[derive(RustcDecodable, RustcEncodable, Copy, Clone)]
pub enum eControllerState
{
    mulligan,
    waiting_for_start,
    waiting_for_turn,
    in_turn,
}

#[derive(Clone)]
pub struct Controller{

    pub name : String,
    pub hero : String,
    pub controller_type : eControllerType,
    pub guid : u32,
    pub mana : u8,
    pub baseMana : u8,
    pub team : u8,
    pub controller_state : eControllerState,
    pub client_id : u32,

    pub deck: Vec<Card>,
    pub hand: Vec<Card>,
//    pub in_play: Vec<Box<Card>>,
//   pub graveyard : Vec<Box<Card>>,
//  pub seen_cards : HashMap<String, Box<Card>>

}
