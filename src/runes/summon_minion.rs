
use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::{UID, EMinionState};
use hlua;
use runes::report_minion_to_client::ReportMinionToClient;

#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct SummonMinion {
    pub minion_uid: UID,
    pub controller_uid: UID,
    pub field_index: u8,
}

implement_for_lua!(SummonMinion, |mut _metatable| {});

impl SummonMinion {
    pub fn new(minion_uid: UID, controller_uid: UID, field_index: u8) -> SummonMinion {
        SummonMinion {
            minion_uid: minion_uid,
            controller_uid: controller_uid,
            field_index: field_index,
        }
    }
}

impl Rune for SummonMinion {
    fn execute_rune(&self, game_state: &mut GameState) {

        {
            game_state.get_mut_minion(self.minion_uid)
                .unwrap()
                .set_minion_state(EMinionState::InPlay);
        }
        println!("{}", self.minion_uid);
        if !game_state.get_mut_controller_by_uid(self.controller_uid).unwrap().has_seen_card(self.minion_uid) {
            let rmtc = ReportMinionToClient::from_minion(game_state.get_minion(self.minion_uid).unwrap(), self.controller_uid, false);
            game_state.execute_rune(Box::new(rmtc));
        }

        let controller = game_state.get_mut_controller_by_uid(self.controller_uid);

        match controller {
            Some(controller) => {
                if self.field_index == 0 {
                    controller.move_minion_from_unplayed_into_play(self.minion_uid);
                } else {
                    controller.move_minion_from_unplayed_into_play_with_index(self.minion_uid,
                                                                              self.field_index as usize);
                }
            }
            None => {
                println!("Was unable to find controller in SummonMinion with uid of {}",
                         self.controller_uid);
            }
        }
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"SummonMinion\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
