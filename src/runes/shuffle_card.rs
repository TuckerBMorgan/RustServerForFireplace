
use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use hlua;
use bson;
use bson::Document;

#[derive(RustcDecodable, RustcEncodable, Clone, Debug, Serialize, Deserialize)]
pub struct ShuffleCard {
    #[serde(with = "bson::compat::u2f")]
    pub card_uid: UID,
    #[serde(with = "bson::compat::u2f")]
    pub controller_uid: UID,
}

implement_for_lua!(ShuffleCard, |mut _metatable| {});

impl ShuffleCard {
    pub fn new(card_uid: UID, controller_uid: UID) -> ShuffleCard {
        ShuffleCard {
            card_uid: card_uid,
            controller_uid: controller_uid,
        }
    }
}

impl Rune for ShuffleCard {
    fn execute_rune(&self, game_state: &mut GameState) {

        game_state.get_mut_controller_by_uid(self.controller_uid)
            .unwrap()
            .move_card_from_hand_to_deck(self.card_uid.clone());

    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ShuffleCard\",")
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
                        d.insert("RuneType", "ShuffleCard");
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
