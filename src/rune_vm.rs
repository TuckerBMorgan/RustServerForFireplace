
use crate::game_state::GameState;
use crate::minion_card::UID;
use hlua;
use crate::runes::*;


pub trait Rune: Send {
    fn execute_rune(&self, game_state: &mut GameState);
    fn can_see(&self, controller: UID, game_state: &GameState) -> bool;
    fn to_json(&self) -> String;
    fn into_box(&self) -> Box<dyn Rune>;
}

//if we want a rune to work in lua context, just add it to this macro
implement_enum_and_unfold!(StartGame, SetMana, SetAttack, SetHealth, ModifyHealth, ModifyAttack, SetBaseMana, SummonMinion, CreateCard,);
