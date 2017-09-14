use rune_vm::{Rune, ERuneType};
use minion_card::UID;
use rustc_serialize::json;
use game_state::GameState;
use hlua;


#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct ModifyHealth {
    target_uid: UID,
    amount: i32,
}

implement_for_lua!(ModifyHealth, |mut _metatable| {});

impl ModifyHealth {
    pub fn new(target_uid: UID, amount: i32) -> ModifyHealth {
        ModifyHealth {
            target_uid: target_uid,
            amount: amount,
        }
    }
}

impl Rune for ModifyHealth {
    fn execute_rune(&self, game_state: &mut GameState) {
        {
            game_state.get_mut_minion(self.target_uid).unwrap().shift_current_health(self.amount);
        }
        let min = game_state.get_minion(self.target_uid).unwrap().clone();
        match min.get_function("on_hp_change_function".to_owned()){
            Some(function) => {
                let rune_vec = {
                    game_state.add_number_to_lua("target_uid".to_string(),
                                                        self.target_uid as u32);
                    game_state.add_integer_to_lua("amount".to_string(), self.amount as i32);
                    let mut resutlt =
                        game_state.run_lua_statement::<hlua::LuaTable<_>>(&function, true)
                        .unwrap();
                    let ret = resutlt.iter::<i32, ERuneType>()
                        .filter_map(|e| e)
                        .map(|(_, v)| v)
                        .collect::<Vec<ERuneType>>()
                        .clone();
                    ret
                };
                for rune in rune_vec {
                    game_state.execute_rune(rune.unfold());
                }
            }
            _ => {}
        }


    }

    fn can_see(&self, _controller: UID, _game_state: &GameState) -> bool {
        return true;
    }

    fn to_json(&self) -> String {
        json::encode(self).unwrap().replace("{", "{\"runeType\":\"ModifyHealth\",")
    }

    fn into_box(&self) -> Box<Rune> {
        Box::new(self.clone())
    }
}
