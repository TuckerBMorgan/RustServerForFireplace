use rune_vm::Rune;
use minion_card::{UID, EMinionState};
use rustc_serialize::json;
use game_state::GameState;


#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct Destroy {
    target_uid: UID,
}

impl Destroy {
    pub fn new(target_uid: UID) -> Destroy {
        Destroy { target_uid: target_uid }
    }
}

impl Rune for Destroy {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_minion(self.target_uid)
            .unwrap()
            .set_minion_state(EMinionState::MarkForDestroy);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"Destroy\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
