use rune_vm::Rune;
use rustc_serialize::json;
use game_state::GameState;
use minion_card::UID;
use runes::play_minion::PlayMinion;
use runes::set_mana::SetMana;
use hlua;

// the play_minion rune is called when you play a minion
// out of your hand. It will call battle_cry if it has one
// and it will remove the card from your hand
// it however wont directaly place the card into play
// it kicks off a summon minion rune after it calls battle_cry
//


#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct PlayCard {
    pub card_uid: UID,
    pub controller_uid: UID,
    pub field_index: usize,
    pub target_uid: UID,
}

impl PlayCard {
    pub fn new(card_uid: UID,
               controller_uid: UID,
               field_index: usize,
               target_uid: UID)
               -> PlayCard {
        PlayCard {
            card_uid: card_uid,
            controller_uid: controller_uid,
            field_index: field_index,
            target_uid: target_uid,
        }
    }
}

implement_for_lua!(PlayCard, |mut _metatable| {});

impl Rune for PlayCard {
    fn execute_rune(&self, mut game_state: &mut GameState) {
        let card = game_state.get_controller_by_uid(self.controller_uid)
            .unwrap()
            .get_copy_of_card_from_hand(self.card_uid);

        let card_unwrap = card.unwrap(); //.get_content().parse().unwrap().copy();

        let sm = SetMana::new(self.controller_uid, game_state.get_controller_by_uid(self.controller_uid).unwrap().get_mana() - card_unwrap.get_cost());
        game_state.execute_rune(sm.into_box());

        let content = card_unwrap.get_content();
        let parse = content.parse::<UID>();
        let parse_unwrap = parse.unwrap().clone();

        game_state.get_mut_controller_by_uid(self.controller_uid)
            .unwrap()
            .remove_card_from_hand(parse_unwrap);

        let pm = PlayMinion::new(card_unwrap.get_content().parse().unwrap(),
                                 self.controller_uid,
                                 self.field_index as usize,`
                                 self.target_uid);

        game_state.execute_rune(Box::new(pm));
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"PlayCard\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
