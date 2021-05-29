use crate::models;
use itertools::Itertools;
use models::choices::Choice;
use models::state::GameState;
use rand::Rng;
use rand::{seq::SliceRandom, prelude::{IteratorRandom, ThreadRng}};

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
        let mut possibility_set = GamePossibilitySet::new(state.clone(), ThreadRng::default());
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

pub struct GamePossibilitySet {
    pub state: GameState,
    pub probability: f64,
    rng: ThreadRng
}

impl GamePossibilitySet {
    fn choose<'a, T>(&mut self, choices: impl IntoIterator<Item = T>) -> Option<T> 
    {
        let resolved = choices.into_iter().collect_vec();
        let resolved_count = resolved.len();
        if resolved_count != 0 {
            self.probability /= resolved_count as f64;
        }

        resolved.into_iter().choose(&mut self.rng)
    }

    fn range(&mut self, max: usize) -> usize {
        self.probability /= max as f64;
        self.rng.gen_range(0..max)
    }

    fn choose_weighted<'a, T>(&mut self, choices: &'a [(T, u8)]) -> Option<&'a T> {
        if choices.len() == 0 {
            None
        } else {
            let selection = choices.choose_weighted(&mut self.rng, |(_, a)| *a).unwrap();
            let choice_sum: u8 = choices.iter().map(|(_, a)|a).sum();

            self.probability *= selection.1 as f64/ choice_sum as f64;

            Some(&selection.0)
        }
    }

    fn choose_multiple<'a, T>(&mut self, choices: impl IntoIterator<Item = &'a T>, count: usize) -> Vec<&'a T> {
        
        let mut resolved = choices.into_iter().collect_vec();
        let resolved_count = resolved.len();

        let selection: Vec<&T> = resolved.into_iter().choose_multiple(&mut self.rng, count);

        self.probability /= num_integer::binomial(resolved_count, selection.len()) as f64;

        selection
    }

    fn new(state: GameState, rng: ThreadRng) -> GamePossibilitySet {
        GamePossibilitySet {
            state,
            rng,
            probability: 1.0
        }
    }
}

impl<'a> From<&'a mut GamePossibilitySet> for &'a GameState {
    fn from(set: &mut GamePossibilitySet) -> &GameState {
        &set.state
    }
}

impl<'a> From<&'a mut GamePossibilitySet> for &'a mut GameState {
    fn from(set: &mut GamePossibilitySet) -> &mut GameState {
        &mut set.state
    }
}


