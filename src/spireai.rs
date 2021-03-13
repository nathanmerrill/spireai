use log::error;

use crate::models;
use crate::models::state::*;
use models::state::GameState;

pub struct SpireAi {
    expected_state: Option<GamePossibilitySet>,
    // Neural net nodes
}

#[allow(dead_code)]
pub enum Choice {
    Start {
        player_class: models::core::Class,
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
    State,
}

impl SpireAi {
    pub fn new() -> SpireAi {
        SpireAi {
            expected_state: None,
        }
    }

    pub fn choose(&mut self, state: &GameState) -> Choice {
        match &self.expected_state {
            Some(expected) => verify_state(state, &expected),
            None => {}
        }
        let choice = make_choice(state);
        self.expected_state = predict_outcome(state, &choice);

        return choice;
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

fn verify_state(outcome: &GameState, prediction: &GamePossibilitySet) {
    if prediction.states.iter().all(|a| &a.state != outcome) {
        error!(
            "New state does not match expected state.  New state: {:?}",
            outcome
        );
        panic!()
    }
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

fn predict_outcome(state: &GameState, choice: &Choice) -> Option<GamePossibilitySet> {
    panic!("Not implemented")
}
