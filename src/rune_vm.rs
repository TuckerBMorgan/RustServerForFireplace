
use game_state::GameState;

pub trait Rune: Send {
    fn execute_rune(&self, game_state: &mut GameState);
    fn can_see(&self, controller: u32, game_state: &GameState) -> bool;
    fn to_json(&self) -> String;
}

pub fn execute_rune(rune: Box<Rune>, game_state: &mut GameState) {

    if game_state.is_rune_queue_empty() == false {
        game_state.add_rune_to_queue(rune);
    } else {
        process_rune(rune, game_state);
    }
}

pub fn process_rune(rune: Box<Rune>, mut game_state: &mut GameState) {
     rune.execute_rune(&mut game_state);
     let controllers = game_state.get_controller_client_id();
    
     for controller in controllers {
    
     if rune.can_see(controller, game_state){
            game_state.report_rune_to_client(controller.clone(),rune.to_json());
        }
     }
}
