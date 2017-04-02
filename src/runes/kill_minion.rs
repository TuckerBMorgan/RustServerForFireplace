use rune_vm::Rune;
use minion_card::{UID, EMinionState};
use tags_list::DEATH_RATTLE;
use rustc_serialize::json;
use game_state::GameState;
use hlua;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct KillMinion {
    controller_uid: UID,
    minion_uid: UID,
}

implement_for_lua!(KillMinion, |mut _metatable| {});

impl KillMinion {
    pub fn new(controller_uid: UID, minion_uid: UID) -> KillMinion {
        KillMinion {
            controller_uid: controller_uid,
            minion_uid: minion_uid,
        }
    }
}

impl Rune for KillMinion {
    fn execute_rune(&self, game_state: &mut GameState) {

        if game_state.get_minion(self.minion_uid).unwrap().has_tag(DEATH_RATTLE.to_string()) {
            //preform deathratte
        }

        game_state.get_mut_controller_by_uid(self.controller_uid)
            .unwrap()
            .move_minion_from_play_to_graveyard(self.minion_uid);
        game_state.get_mut_minion(self.minion_uid).unwrap().set_minion_state(EMinionState::Dead);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"KillMinion\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
