use crate::rune_vm::Rune;
use serde::{Deserialize, Serialize};
use crate::game_state::GameState;
use crate::minion_card::{UID, Minion, EMinionState};
use hlua;

use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateCard {
    card_id: String,
    uid: UID,
    controller_uid: UID,
}

implement_for_lua!(CreateCard, |mut metatable| {});

impl CreateCard {
    #[allow(dead_code)]
    pub fn new(card_id: String, uid: UID, controller_uid: UID) -> CreateCard {

        CreateCard {
            card_id: card_id,
            uid: uid,
            controller_uid: controller_uid,
        }
    }
}

impl Rune for CreateCard {
    fn execute_rune(&self, game_state: &mut GameState) {
        println!("{}", "content/cards/".to_string() + &self.card_id.clone() +
                               &".lua".to_string());
        let mut f = File::open("content/cards/".to_string() + &self.card_id.clone() +
                               &".lua".to_string())
            .unwrap();

        let mut contents = String::new();
        let _result = f.read_to_string(&mut contents);
        let spl: Vec<&str> = contents.split("@@").collect();
        if spl[0].contains("minion") {
            let proto_minion = Minion::parse_minion_file(contents.clone());
            game_state.add_number_to_lua("give_uid".to_string(), self.uid);
            let mut minion = game_state.run_lua_statement::<Minion>(&proto_minion.get(&"create_minion_function".to_string()).unwrap(), true).unwrap();
            let team =
                game_state.get_mut_controller_by_uid(self.controller_uid).unwrap().get_team();

            minion.set_team(team);
            minion.set_minion_state(EMinionState::NotInPlay);
            minion.set_functions(proto_minion);

            game_state.get_mut_controller_by_uid(self.controller_uid)
                .unwrap()
                .add_minion_to_unplayed(minion.get_uid());
            game_state.add_minion_to_minions(minion);
        }
    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return false;
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap().replace("{", "{\"runeType\":\"CreateCard\",")
    }

    fn into_box(&self) -> Box<dyn Rune> {
        Box::new(self.clone())
    }
}
