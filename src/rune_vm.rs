
use game_state::GameState;
use minion_card::UID;
use hlua;
use runes::start_game::StartGame;
use runes::set_mana::SetMana;
use runes::set_health::SetHealth;
use runes::set_attack::SetAttack;

pub trait Rune: Send {
    fn execute_rune(&self, game_state: &mut GameState);
    fn can_see(&self, controller: UID, game_state: &GameState) -> bool;
    fn to_json(&self) -> String;
    fn into_box(&self) -> Box<Rune>;
}

#[derive(Clone, Debug)]
pub enum ERuneType { 
    StartGame(StartGame),
    SetMana(SetMana),
    SetAttack(SetAttack),
    SetHealth(SetHealth)
}

impl ERuneType {
    pub fn unfold(&self) -> Box<Rune> {
        match *self {
            ERuneType::SetAttack(ref start_game_stuff) => {
                return start_game_stuff.into_box();
            },
            ERuneType::SetHealth(ref set_health_rune) => {
                return set_health_rune.into_box();
            },
            ERuneType::SetMana(ref set_mana_rune) => {
                return set_mana_rune.into_box();
            },
            ERuneType::StartGame(ref start_game_rune) => {
                return start_game_rune.into_box();
            }
        }
    }
}
implement_for_lua!(ERuneType, |mut _metatable| {});