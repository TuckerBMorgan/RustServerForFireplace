use ::rune_vm::Rune;
use minion_card::{UID};
use tags_list::DEATH_RATTLE;
use rustc_serialize::json;
use game_state::GameState;


#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct ModifyHealth {
    targe_uid: UID,
    amount: u16
}

impl ModifyHealth {
    pub fn new(controller_uid: UID, minion_uid: UID) -> ModifyHealth {
        ModifyHealth {
            controller_uid : controller_uid,
            minion_uid: minion_uid
        }
    }
}

impl Rune for ModifyHealth {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_minion(self.targe_uid).set_current_health(self.amount);    
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ModifyHealth\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(*self.clone())
    }
}