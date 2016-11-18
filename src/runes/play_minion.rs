use ::card::Card;
use ::rune_vm::{Rune, process_rune};
use rustc_serialize::json;
use ::minion_card::Minion;
use ::game_state::GameState;
use std::collections::HashMap;
use rustc_serialize::json::Json;


use runes::summon_minion::SummonMinion;

/*the play_minion rune is called when you play a minion
 *out of your hand. It will call battle_cry if it has one
 *and it will remove the card from your hand
 *it however wont directaly place the card into play
 *it kicks off a summon minion rune after it calls battle_cry
 */ 

#[derive(RustcDecodable, RustcEncodable)]
pub struct PlayMinion {
    pub card_uid: u32,
    pub controller_uid: u32,
    pub field_index : u8,
}

impl PlayMinion {
    pub fn new(card_uid: u32,
               controller_uid: u32,
               field_index: u8)
               -> PlayMinion {
        PlayMinion {
            card_uid: card_uid,
            controller_uid: controller_uid,
            field_index: field_index
        }
    }
}

impl Rune for PlayMinion {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        
        {
            let min = game_state.get_minion(self.card_uid);
            match min {
                Some(min) => {
                    if min.get_battle_cry() != "default".to_string() {
                        
                    }
                },
                None => {
                    println!("Could not find minion with uid {}", self.card_uid);
                }
            }
        }

        let s_r = SummonMinion::new(self.controller_uid, self.card_uid, self.field_index);
        process_rune(Box::new(s_r), &mut game_state);
    }

    fn can_see(&self, controller: u32, game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap()
    }
}