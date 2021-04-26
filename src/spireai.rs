use crate::models;
use models::choices::Choice;
use models::state::GameState;
use rand::Rng;

pub mod appraiser;
pub mod enumerator;
pub mod evaluator;
pub mod predictor;

pub struct SpireAi {
    expected_state: Option<GamePossibilitySet>,
    // Neural net nodes
}

impl SpireAi {
    pub fn new() -> SpireAi {
        SpireAi {
            expected_state: None,
        }
    }

    pub fn choose(&mut self, new_state: &GameState) -> Choice {
        if let Some(expected_state) = &self.expected_state {
            predictor::verify_prediction(new_state, &expected_state);
        }
        let (choice, possibilities) = make_choice(new_state);
        self.expected_state = possibilities;

        choice
    }
}

fn make_choice(state: &GameState) -> (Choice, Option<GamePossibilitySet>) {
    let mut max_val = f64::MIN;
    let mut best_choice = Choice::State;
    let mut best_outcome: Option<GamePossibilitySet> = None;

    for choice in enumerator::all_choices(&state) {
        let prediction = predictor::predict_outcome(state, &choice);
        let rating = appraiser::rate_possibility_set(&prediction);
        if rating > max_val {
            max_val = rating;
            best_choice = choice;
            best_outcome = Some(prediction);
        }
    }

    (best_choice, best_outcome)
}

type GamePossibilitySet = (GameState, f64);
