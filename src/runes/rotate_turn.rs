use ::rune_vm::Rune;
use ::game_state::GameState;
use minion_card::UID;
use ::tags_list::*;
use runes::remove_tag::RemoveTag;
use runes::set_base_mana::SetBaseMana;
use runes::set_mana::SetMana;
use runes::deal_card::DealCard;
use ::controller::EControllerState;

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct RotateTurn {

}

impl RotateTurn {
    pub fn new() -> RotateTurn {
        RotateTurn {}
    }
}

impl Rune for RotateTurn {
    fn execute_rune(&self, mut game_state: &mut GameState) {

        let mut turn_index = game_state.get_on_turn_player().clone();
        let in_play = game_state.get_controller_by_index(turn_index as usize).get_copy_of_in_play();
        let mut remove_tag_runes = vec![];

        for uids in in_play {

            let minion = game_state.get_minion(uids).unwrap();

            if minion.has_tag(SUMMONING_SICKNESS.to_string()) {
                remove_tag_runes.push(RemoveTag::new(uids.clone(), SUMMONING_SICKNESS.to_string().clone()));
            }
        }

        for rune in remove_tag_runes {
            game_state.execute_rune(Box::new(rune));
        }

        game_state.get_mut_controller_by_index(turn_index as usize)
            .set_controller_state(EControllerState::WaitingForTurn);

        turn_index += 1;
        if turn_index == 2 {
            turn_index = 0;
        }

        game_state.set_on_turn_player(turn_index);

        let new_mana =
            game_state.get_controller_by_index(turn_index as usize).get_base_mana().clone() + 1;
        let sbm = SetBaseMana::new(game_state.get_controller_by_index(turn_index as usize)
                                       .get_uid()
                                       .clone(),
                                   new_mana);
        let sm = SetMana::new(game_state.get_controller_by_index(turn_index as usize).get_uid(),
                              new_mana);

        game_state.execute_rune(Box::new(sbm));
        game_state.execute_rune(Box::new(sm));

        let uids = game_state.get_controller_by_index(turn_index as usize)
            .get_n_card_uids_from_deck(1)
            .clone();

        let dc = DealCard::new(uids[0].clone(), 
                                    game_state.get_controller_by_index(turn_index as usize)
                                   .get_uid()
                                   .clone());
        game_state.execute_rune(Box::new(dc));
    }
    
    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        "{\"runeType\":\"RotateTurn\"}".to_string()
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
