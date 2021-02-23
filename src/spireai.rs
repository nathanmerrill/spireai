use crate::models;

use crate::models::core::*;
use models::GameState;
use crate::models::state::*;

pub struct SpireAi {
    expected_state: Option<GamePossibilitySet>
}

#[allow(dead_code)]
pub enum Choice {
    Start {
        player_class: models::PlayerClass,
        ascension: Option<u8>,
    },
    Potion {
        should_use: bool,
        slot: u8,
        target_index: Option<u8>,
    },
    Play {
        card_index: u8,
        target_index: Option<u8>,
    },
    End,
    Choose {
        choice_index: u8,
    },
    Proceed,
    Return,
    State
}

impl SpireAi {
    pub fn new() -> SpireAi {
        SpireAi {
            expected_state: None,
        }
    }

    pub fn choose(&mut self, state: &GameState) -> Choice {
        self.verify_state(state);
        let choice = make_choice(state);
        let outcome = predict_outcome(state, &choice);
        self.expected_state = Some(outcome);

        BaseBuff::by_name("");
        BaseCard::by_name("");
        
        return choice;
    }


    fn verify_state(&mut self, new_state: &GameState) {
        panic!("Not implemented");
        /*
        if let Some(state) = &self.expected_state {
            if !state.contains_state(new_state) {
                error!("New state does not match expected state.  New state: {:?}", new_state);
                panic!()
            }
        }*/
    }

    /*
    fn handle_event(&mut self, event: &Event) -> Choice{
        match event.options.len() {
            0 => {
                error!("Unable to handle event with no choices.");
                panic!();
            }
            1 => return Choice::Choose {choice_index: 0},
            _ => {}
        }

        return match event.event_id.as_str() {
            "Neow Event" => {
                Choice::Choose {choice_index: 0}
            }
            _ => {
                error!("Unhandled event: {}", event.event_id);
                panic!();
            }
        }
    }
    */
    
}

fn make_choice(state: &GameState) -> Choice {
    panic!("Not implemented")
    /*
    match state.room_phase {
        models::RoomPhase::Combat => {

        }
        models::RoomPhase::Complete => {

        }
        
    }

    return match state {
        //ScreenState::Event(ref event) => self.handle_event(event),

        _ => {
            Choice::Choose {choice_index: 0}
        }
    };*/
}

fn handle_combat(state: &GameState) -> Choice {
    panic!("Not implemented")

}

fn predict_outcome(state: &GameState, choice: &Choice) -> GamePossibilitySet {
    panic!("Not implemented")
}