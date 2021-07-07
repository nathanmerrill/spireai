use crate::comm::request::GameState as CommState;
use crate::{
    models,
    state::{game::GameState, probability::Probability},
};
use models::choices::Choice;

use self::evaluator::GamePossibility;

pub mod appraiser;
pub mod enumerator;
pub mod evaluator;
pub mod predictor;
pub mod references;

pub struct SpireAi {
    state: GameState,
    last_choice: Option<Choice>, // Neural net nodes
}

impl SpireAi {
    pub fn new(state: GameState) -> SpireAi {
        SpireAi {
            state,
            last_choice: None,
        }
    }

    pub fn choose(&mut self, new_state: &CommState) -> Choice {
        if let Some(choice) = &self.last_choice {
            self.state = find_match(&self.state, choice, new_state);
        }

        make_choice(&self.state)
    }
}

fn find_match(state: &GameState, choice: &Choice, comm_state: &CommState) -> GameState {
    unimplemented!()
}

fn make_choice(state: &GameState) -> Choice {
    let mut max_val = f64::MIN;
    let mut best_choice = Choice::State;

    for choice in enumerator::all_choices(&state) {
        let mut possibility = GamePossibility {
            state: state.clone(),
            probability: Probability::new(),
        };

        predictor::predict_outcome(&choice, &mut possibility);

        //let rating = appraiser::r(&possibility_set);
        let rating = 0_f64;
        if rating > max_val {
            max_val = rating;
            best_choice = choice;
        }
    }

    best_choice
}
