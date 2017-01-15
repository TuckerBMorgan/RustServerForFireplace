use ::rune_vm::Rune;
use rustc_serialize::json;
use ::game_state::GameState;
use minion_card::UID;
use runes::add_tag::AddTag;
use runes::summon_minion::SummonMinion;
use tags_list::{CHARGE, SUMMONING_SICKNESS};

// the play_minion rune is called when you play a minion
// out of your hand. It will call battle_cry if it has one
// and it will remove the card from your hand
// it however wont directaly place the card into play
// it kicks off a summon minion rune after it calls battle_cry
//


#[derive(RustcDecodable, RustcEncodable)]
pub struct PlayMinion {
    pub card_uid: u32,
    pub controller_uid: u32,
    pub field_index: u8,
}

impl PlayMinion {
    pub fn new(card_uid: u32, controller_uid: u32, field_index: u8) -> PlayMinion {
        PlayMinion {
            card_uid: card_uid,
            controller_uid: controller_uid,
            field_index: field_index,
        }
    }
}

impl Rune for PlayMinion {
    fn execute_rune(&self, mut game_state: &mut GameState) {

        {
            let min = game_state.get_minion(self.card_uid).unwrap().clone();
            if !min.has_tag(CHARGE.to_string()) {
                let at = AddTag::new(self.card_uid.clone(), SUMMONING_SICKNESS.to_string());
                game_state.execute_rune(Box::new(at));
            }

            if min.get_battle_cry() != "default".to_string() {

            }
        }

        let s_r = SummonMinion::new(self.controller_uid, self.card_uid, self.field_index);
        game_state.process_rune(Box::new(s_r));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap()
    }
}
