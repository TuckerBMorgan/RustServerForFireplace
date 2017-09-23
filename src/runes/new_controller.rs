
use rune_vm::Rune;
use runes::create_card::CreateCard;
use rustc_serialize::json;
use minion_card::UID;
use game_state::GameState;
use std::collections::HashSet;
use controller::{EControllerState, Controller};
use hlua;
use runes::summon_minion::SummonMinion;


#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct NewController {
    pub uid: UID,
    pub hero: String,
    pub client_id: u32,
    pub deck: String,
    pub is_me: bool,
}

impl NewController {
    #[allow(dead_code)]
    pub fn new(uid: UID, hero: String, client_id: u32, deck: String, is_me: bool) -> NewController {
        NewController {
            uid: uid,
            hero: hero,
            client_id: client_id,
            deck: deck,
            is_me: is_me,
        }
    }
}

implement_for_lua!(NewController, |mut _metatable| {});

impl Rune for NewController {
    fn execute_rune(&self, game_state: &mut GameState) {
        println!("New Controller with client_id {}", self.client_id.clone());
            let new_controller = Controller {
            name: "controller".to_string(),
            hero: self.hero.clone(),
            uid: self.uid,
            base_mana: 0,
            mana: 0,
            life: 30,
            total_life: 30,
            team: game_state.get_team() as u32,
            controller_state: EControllerState::WaitingForStart,
            current_options: vec![],
            deck: vec![],
            hand: vec![],
            unplayed_minions: vec![],
            in_play_minions: vec![],
            client_id: self.client_id.clone(),
            seen_cards: HashSet::new(),
            played_cards: vec![],
            graveyard: vec![],
        };

        game_state.add_player_controller(new_controller, self.deck.clone());
        game_state.execute_rune(CreateCard::new("basic/hunter".to_owned(), self.uid, self.uid).into_box());

        //new_controller.unplayed_minions.push(new_controller.uid);
        
        //game_state.get_mut_controller_by_uid(self.uid).unwrap().add_card_to_deck(play_card);
        //game_state.execute_rune(DealCard::new(self.uid, self.uid).into_box());
        game_state.execute_rune(SummonMinion::new(self.uid, self.uid, 0).into_box());
        

        //game_state.stage_rune(PlayCard::new(self.uid, self.uid, 0, 0).into_box());
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"NewController\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
