use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::UID;
use crate::tags_list::{FROZEN, CHARGE, WINDFURY, DIVINE_SHIELD, STEALTH, TAUNT, DEATH_RATTLE,
                TRIGGERED_EFFECT, POISON, SPELL_DAMAGE, TARGET};
use crate::runes::*;
use hlua;


#[derive(Serialize, Deserialize, Clone)]
pub struct Silence {
    pub minion_uid: UID,
}

implement_for_lua!(Silence, |mut metatable| {});

impl Silence {
    pub fn new(minion_uid: UID) -> Silence {
        Silence { minion_uid: minion_uid }
    }
}

impl Rune for Silence {
    fn execute_rune(&self, mut game_state: &mut GameState) {

        game_state.get_mut_minion(self.minion_uid).unwrap().clear_enchantments();

        //might be a good idea to do if checks if front of this, just to avoid unneeded work
        let rt = RemoveTag::new(self.minion_uid, FROZEN.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, CHARGE.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, WINDFURY.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, DIVINE_SHIELD.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, TAUNT.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, DEATH_RATTLE.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, TRIGGERED_EFFECT.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, POISON.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, SPELL_DAMAGE.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, TARGET.to_string());
        game_state.execute_rune(Box::new(rt));
        let rt = RemoveTag::new(self.minion_uid, STEALTH.to_string());
        game_state.execute_rune(Box::new(rt));

    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"Silence\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
