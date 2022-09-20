use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use hlua;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetHealth {
    card_uid: UID,
    amount: u32,
}

implement_for_lua!(SetHealth, |mut metatable| {});

impl SetHealth {
    pub fn new(card_uid: UID, amount: u32) -> SetHealth {
        SetHealth {
            card_uid: card_uid,
            amount: amount,
        }
    }
}

impl Rune for SetHealth {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        //TODO: Make this work the way it is supposed to


        game_state.get_mut_minion(self.card_uid).unwrap().set_total_health(self.amount);



    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"SetHealth\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
