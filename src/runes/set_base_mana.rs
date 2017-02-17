use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;



#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct SetBaseMana {
    controller_uid: UID,
    base_mana: u8,
}

impl SetBaseMana {
    pub fn new(controller_uid: UID, base_mana: u8) -> SetBaseMana {
        SetBaseMana {
            controller_uid: controller_uid,
            base_mana: base_mana,
        }
    }
}

impl Rune for SetBaseMana {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        game_state.get_mut_controller_by_uid(self.controller_uid)
            .unwrap()
            .set_base_mana(self.base_mana);
    }


    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"SetBaseMana\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
