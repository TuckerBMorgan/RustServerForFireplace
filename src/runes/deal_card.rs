
use ::card::{Card, ECardType};
use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use minion_card::{UID};
use std::collections::HashMap;
use rustc_serialize::json::Json;
use runes::create_minion::CreateMinion;

#[derive(RustcDecodable, RustcEncodable)]
pub struct DealCard {
    pub card_uid: UID,
    pub controller_uid: UID,
}

impl DealCard {
    pub fn new(card_uid: UID,
               controller_uid: UID,
               field_index: u8)
               -> DealCard {
        DealCard {
            card_uid: card_uid,
            controller_uid: controller_uid,
        }
    }
}

impl Rune for DealCard {

    fn execute_rune(&self, game_state: &mut GameState) {
        let controller = game_state.get_controller_by_uid(self.controller_uid);    

        match controller {
            Some(controller) => {
                
                let card = controller.get_card_from_deck(self.card_uid).unwrap();

                match card.get_card_type() {
                    ECardType::Minion => {
                        if !controller.has_seen_card(self.card_uid) {

                            
                        }
                    },
                    _ => {

                    }
                }

            },
            None => {
                println!("Was unable to find controller in SummonMinion with uuid of {}", self.controller_uid);
            }

        }
    }

    fn can_see(&self, _controller: u32, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap()
    }
}
