use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use hlua;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct SetMana {
    controller_uid: UID,
    mana: u8,
}

implement_for_lua!(SetMana, |mut _metatable| {});

impl SetMana {
    pub fn new(controller_uid: UID, mana: u8) -> SetMana {
        SetMana {
            controller_uid: controller_uid,
            mana: mana,
        }
    }
 
    pub fn to_rune(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}

impl Rune for SetMana {
    fn execute_rune(&self, game_state: &mut GameState) {
        game_state.get_mut_controller_by_uid(self.controller_uid).unwrap().set_mana(self.mana);
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"SetMana\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
