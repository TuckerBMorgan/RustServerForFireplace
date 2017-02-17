use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use runes::add_tag::AddTag;
use runes::summon_minion::SummonMinion;
use tags_list::{CHARGE, SUMMONING_SICKNESS, TARGET};

// the play_minion rune is called when you play a minion
// out of your hand. It will call battle_cry if it has one
// and it will remove the card from your hand
// it however wont directaly place the card into play
// it kicks off a summon minion rune after it calls battle_cry
//


#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct PlayMinion {
    pub minion_uid: UID,
    pub controller_uid: UID,
    pub field_index: usize,
    pub target_uid: UID,
}

impl PlayMinion {
    pub fn new(minion_uid: UID,
               controller_uid: UID,
               field_index: usize,
               target_uid: UID)
               -> PlayMinion {
        PlayMinion {
            minion_uid: minion_uid,
            controller_uid: controller_uid,
            field_index: field_index,
            target_uid: target_uid,
        }
    }
}

impl Rune for PlayMinion {
    fn execute_rune(&self, mut game_state: &mut GameState) {

        {
            let min = game_state.get_minion(self.minion_uid).unwrap().clone();

            if min.has_tag(TARGET.to_string()) {
                //there is no reason for this statment to return anything
                game_state.run_rhai_statement::<i8>(&min.get_function("target_function".to_string())
                                                  .unwrap(),
                                              true);
            }

            if !min.has_tag(CHARGE.to_string()) {
                let at = AddTag::new(self.minion_uid.clone(), SUMMONING_SICKNESS.to_string());
                game_state.execute_rune(Box::new(at));
            }
            match min.get_function("battle_cry_function".to_string()) {
                Some(function) => {
                    game_state.run_rhai_statement::<i8>(&function, true);
                }
                _ => {}
            }
        }

        let s_r = SummonMinion::new(self.minion_uid, self.controller_uid, self.field_index as u8);
        game_state.process_rune(Box::new(s_r));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"PlayMinion\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
