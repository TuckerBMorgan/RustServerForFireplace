use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use minion_card::UID;



#[derive(RustcDecodable, RustcEncodable)]
pub struct AddTag {
    pub minion_uid: UID,
    pub tag: String,
}

impl AddTag {
    pub fn new(minion_uid: UID, tag: String) -> AddTag {
        AddTag {
            minion_uid: minion_uid,
            tag: tag,
        }
    }
}

impl Rune for AddTag {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        let mut minion = game_state.get_mut_minion(self.minion_uid);
        match minion {
            Some(minion) => {
                minion.add_tag_to(self.tag.to_string().clone());
            }
            None => {
                println!("We could not find the minion with the UID {}",
                         self.minion_uid)
            }
        }
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"AddTag\",")
    }
}
