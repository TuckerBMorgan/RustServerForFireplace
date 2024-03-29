
use crate::card::ECardType;
use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use crate::runes::*;
use hlua;

#[derive(Serialize, Deserialize, Clone)]
pub struct DealCard {
    pub card_uid: UID,
    pub controller_uid: UID,
}

implement_for_lua!(DealCard, |mut metatable| {});

impl DealCard {
    pub fn new(card_uid: UID, controller_uid: UID) -> DealCard {
        DealCard {
            card_uid: card_uid,
            controller_uid: controller_uid,
        }
    }
}

impl Rune for DealCard {
    fn execute_rune(&self, game_state: &mut GameState) {

        // game_state.get_controller_by_uid returns Option<& Controller>
        // controller.get_card_from_deck returns Option<'a Card>

        let card = game_state.get_controller_by_uid(self.controller_uid)
            .unwrap()
            .get_card_from_deck(self.card_uid)
            .unwrap()
            .clone();

        match card.get_card_type() {

            ECardType::Minion => {
                let minion =
                    game_state.get_minion(card.get_content().parse().unwrap()).unwrap().clone();
                if !game_state.get_controller_by_uid(self.controller_uid)
                    .unwrap()
                    .has_seen_card(minion.get_uid()) {

                    let c_m = ReportMinionToClient::from_minion(&minion, self.controller_uid, true);
                    game_state.execute_rune(c_m.into_box());
                }

                game_state.get_mut_controller_by_uid(self.controller_uid)
                    .unwrap()
                    .move_card_from_deck_to_hand(self.card_uid);
            } 
            _ => {}
        }
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"DealCard\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
