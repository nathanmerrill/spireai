use crate::models;
use models::choices::Choice;
use models::state::GameState;
use rand::Rng;
use rand::{seq::SliceRandom, prelude::{IteratorRandom, ThreadRng}};

pub mod appraiser;
pub mod enumerator;
pub mod evaluator;
pub mod predictor;

pub struct SpireAi {
    expected_state: Option<GamePossibility>,
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
            //predictor::verify_prediction(new_state, &expected_state);
        }
        let (choice, possibilities) = make_choice(new_state);
        self.expected_state = possibilities;

        choice
    }
}

fn make_choice(state: &GameState) -> (Choice, Option<GamePossibility>) {
    let mut max_val = f64::MIN;
    let mut best_choice = Choice::State;
    let mut best_outcome: Option<GamePossibility> = None;

    for choice in enumerator::all_choices(&state) {
        let mut possibility_set = GamePossibility::new(state.clone(), ThreadRng::default());
        predictor::predict_outcome(&choice, &mut possibility_set);
        let rating = appraiser::rate_possibility_set(&possibility_set);
        if rating > max_val {
            max_val = rating;
            best_choice = choice;
            best_outcome = Some(possibility_set);
        }
    }

    (best_choice, best_outcome)
}

pub struct GamePossibility {
    pub state: GameState,
    pub probability: f64,
    rng: ThreadRng
}

impl GamePossibility {
    fn choose<T>(&mut self, choices: Vec<T>) -> Option<T> 
    {
        let resolved_count = choices.len();
        if resolved_count != 0 {
            self.probability /= resolved_count as f64;
        }

        choices.into_iter().choose(&mut self.rng)
    }

    fn range(&mut self, max: usize) -> usize {
        self.probability /= max as f64;
        self.rng.gen_range(0..max)
    }

    fn choose_weighted<'a, T>(&mut self, choices: &'a [(T, u8)]) -> Option<&'a T> {
        if choices.is_empty() {
            None
        } else {
            let choice_sum: u8 = choices.iter().map(|(_, a)|a).sum();
            
            let selection = choices.choose_weighted(&mut self.rng, |(_, a)| *a).unwrap();

            self.probability *= selection.1 as f64/ choice_sum as f64;

            Some(&selection.0)
        }
    }

    fn choose_multiple<T>(&mut self, choices: Vec<T>, count: usize) -> Vec<T> {
        
        let resolved_count = choices.len();

        let selection = choices.into_iter().choose_multiple(&mut self.rng, count);

        self.probability /= num_integer::binomial(resolved_count, selection.len()) as f64;

        selection
    }

    fn new(state: GameState, rng: ThreadRng) -> GamePossibility {
        GamePossibility {
            state,
            rng,
            probability: 1.0
        }
    }
}

impl<'a> From<&'a mut GamePossibility> for &'a GameState {
    fn from(set: &mut GamePossibility) -> &GameState {
        &set.state
    }
}

impl<'a> From<&'a mut GamePossibility> for &'a mut GameState {
    fn from(set: &mut GamePossibility) -> &mut GameState {
        &mut set.state
    }
}


