use crate::models;
use models::state::GameState;
use models::choices::Choice;

pub mod evaluator;
pub mod enumerator;
pub mod predictor;
pub mod appraiser;

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
        let state: &GameState = match &self.expected_state {
            Some(expected) => predictor::verify_prediction(new_state, &expected),
            None => new_state,
        };

        let choice = make_choice(state);
        self.expected_state = predictor::predict_outcome(state, &choice);

        return choice;
    }
}

fn make_choice(state: &GameState) -> Choice {
    let mut max_val = f64::MIN;
    let mut best_choice = Choice::State;

    for choice in enumerator::all_choices(&state) {
        match predictor::predict_outcome(state, &choice) {
            Some(outcome) => {
                let rating = appraiser::rate_possibility_set(outcome);
                if rating > max_val {
                    max_val = rating;
                    best_choice = choice
                }
            }
            None => {
                if max_val == f64::MIN {
                    best_choice = choice
                }
            }
        }
    }

    best_choice
}


#[derive(PartialEq, Clone, Debug)]
pub struct GamePossibility {
    pub probability: f64,
    pub state: GameState,
}

pub struct GamePossibilitySet {
    pub states: Vec<GamePossibility>,
}

impl GamePossibilitySet {
    /*
    pub fn new(state: GameState) -> Self {
        let mut states = Vector::new();
        states.push_back(GamePossibility {
            probability: 1.0,
            state: state,
        });
        Self {
            states: states
        }
    }
    */
}
