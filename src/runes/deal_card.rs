
use card::ECardType;
use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use runes::report_minion_to_client::ReportMinionToClient;
use hlua;
use bson;
use bson::Document;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct DealCard {
    #[serde(with = "bson::compat::u2f")]
    pub card_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    pub controller_uid: UID,
}

implement_for_lua!(DealCard, |mut _metatable| {});

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
        //println!("DC to {}", json::encode(&game_state.get_game_state_data().get_is_ai_copy()).unwrap());
        //println!("DC to {}", json::encode(&game_state.get_controller_by_uid(self.controller_uid).unwrap().deck).unwrap());

        //println!("DEAL DEBUG CONTROLLER {:?}", self.controller_uid);
        //println!("DEAL DEBUG CARD {:?}", self.card_uid);
        //println!("DEAL DEBUG {:?}", game_state.get_controller_by_uid(self.controller_uid));

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
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"DealCard\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }

    fn to_bson_doc(&self, game_name: String, count: usize) -> Document{
        let mut doc = bson::to_bson(&self);
        match doc{
            Ok(document)=>{
                match document{
                    bson::Bson::Document(mut d)=>{
                        d.insert("game", game_name);
                        d.insert("RuneCount", count as u64);
                        d.insert("RuneType", "DealCard");
                        return d
                    },
                    _=>{}
                }
            },
            Err(e)=>{
                return Document::new();
            }
        }
        return Document::new();
    }
}
