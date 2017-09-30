
use game_state::GameState;
use minion_card::UID;
use hlua;
use runes::start_game::StartGame;
use runes::set_mana::SetMana;
use runes::set_health::SetHealth;
use runes::set_attack::SetAttack;
use runes::set_base_mana::SetBaseMana;
use runes::modify_attack::ModifyAttack;
use runes::modify_health::ModifyHealth;
use runes::summon_minion::SummonMinion;
use runes::create_card::CreateCard;
use runes::modify_hero_health::ModifyHeroHealth;
use runes::end_game::EndGame;
use bson::Document;


pub trait Rune: Send {
    fn execute_rune(&self, game_state: &mut GameState);
    fn can_see(&self, controller: UID, game_state: &GameState) -> bool;
    fn to_json(&self) -> String;
    fn into_box(&self) -> Box<Rune>;
    fn to_bson_doc(&self, game_name: String, count: usize) -> Document;
}

//if we want a rune to work in lua context, just add it to this macro
implement_enum_and_unfold!(StartGame, SetMana, SetAttack, SetHealth, ModifyHealth, ModifyAttack, SetBaseMana, SummonMinion, CreateCard, ModifyHeroHealth, EndGame, );
