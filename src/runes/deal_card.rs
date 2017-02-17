
use card::ECardType;
use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use runes::create_minion::CreateMinion;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct DealCard {
    pub card_uid: UID,
    pub controller_uid: UID,
}

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

        // would like a better way of doing this, but for now -\_(-.-)_/-, blame the borrow checker


        // game_state.get_controller_by_uid returns Option<& Controller>
        // controller.get_card_from_deck returns Option<'a Card>

        let card = game_state.get_controller_by_uid(self.controller_uid)
            .unwrap()
            .get_card_from_deck(self.card_uid)
            .unwrap()
            .clone();

        match card.get_card_type() {

            ECardType::Minion => {

                if !game_state.get_controller_by_uid(self.controller_uid)
                    .unwrap()
                    .has_seen_card(self.card_uid) {
                    let minion =
                        game_state.get_minion(card.get_content().parse().unwrap()).unwrap().clone();
                    let c_m = CreateMinion::from_minion(&minion, self.controller_uid);
                    game_state.execute_rune(Box::new(c_m));
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
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"DealCard\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
