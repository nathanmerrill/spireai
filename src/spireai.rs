use crate::comm::interop;
use crate::comm::request::GameState as CommState;
use crate::state::floor::{GamePossibility, FloorState};
use crate::{
    models,
    state::{game::GameState, probability::Probability},
};
use ::std::hash::{Hash, Hasher};
use im::{HashMap, HashSet};
use itertools::Itertools;
use models::choices::Choice;
use rand::Rng;
use std::cmp::Ordering;
use std::rc::Rc;
use uuid::Uuid;

pub mod appraiser;
pub mod enumerator;
pub mod predictor;
pub mod references;

pub struct SpireAi {
    state: Rc<FloorState>,
    last_choice: Option<Choice>, // Neural net nodes
    tree: MonteCarloTree,
    pub uuid_map: HashMap<String, Uuid>,
}

impl SpireAi {
    pub fn new(state: FloorState) -> SpireAi {
        SpireAi {
            state: Rc::new(state),
            last_choice: None,
            tree: MonteCarloTree {
                choices: HashMap::new(),
            },
            uuid_map: HashMap::new(),
        }
    }

    pub fn choose(&mut self, comm_state: &Option<CommState>) -> Choice {
        if let Some(choice) = self.last_choice.clone() {
            if let Some(matching_state) = self.find_match(&choice, comm_state) {
                let mut new_state = matching_state.as_ref().clone();
                interop::update_state(comm_state, &mut new_state);
                self.state = Rc::new(new_state);
            } else {
                panic!("No matching state found!")
            }
        }

        let next_choice = search(&mut self.tree, self.state.clone(), 50000, 10, 2.0f64.sqrt());

        self.last_choice = Some(next_choice.clone());

        next_choice
    }

    fn find_match(&mut self, choice: &Choice, comm_state: &Option<CommState>) -> Option<Rc<FloorState>> {
        if let Some(choices) = self.tree.choices.get(&self.state) {
            if let Some(outcomes) = choices.get(choice) {
                for outcome in outcomes.outcomes.iter() {
                    if interop::state_matches(
                        comm_state,
                        outcome.state.as_ref(),
                        &mut self.uuid_map,
                    ) {
                        return Some(outcome.state.clone());
                    }
                }
            }
        }
        None
    }
}

#[derive(PartialEq, Clone)]
struct MonteCarloTree {
    choices: HashMap<Rc<FloorState>, HashMap<Choice, ChoiceOutcomes>>,
}

#[derive(PartialEq, Clone)]
struct ChoiceOutcomes {
    outcomes: HashSet<Outcome>,
    evaluation_count: u32,
}

#[derive(Clone)]
struct Outcome {
    state: Rc<FloorState>,
    probability: f64,
    evaluation: f64,
}

impl Hash for Outcome {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state)
    }
}

impl PartialEq for Outcome {
    fn eq(&self, other: &Outcome) -> bool {
        self.state.eq(&other.state)
    }
}

impl Eq for Outcome {}

fn search(
    tree: &mut MonteCarloTree,
    state: Rc<FloorState>,
    iterations: usize,
    max_depth: usize,
    exploration_factor: f64,
) -> Choice {
    for _ in 0..iterations {
        descend(tree, state.clone(), max_depth, exploration_factor)
    }

    //Prune tree to remove game states that are no longer relevant

    best_choice(tree, state, 0.0)
}

fn total_probability(outcomes: &ChoiceOutcomes) -> f64 {
    let total_probability: f64 = outcomes.outcomes.iter().map(|a| a.probability).sum();
    if total_probability > 1.00001 {
        panic!("Total probability greater than 1!")
    }
    total_probability
}

fn score_outcomes(outcomes: &ChoiceOutcomes) -> f64 {
    let total_probability = total_probability(outcomes);
    outcomes
        .outcomes
        .iter()
        .map(|a| a.evaluation * a.probability / total_probability)
        .sum()
}

fn best_choice(tree: &MonteCarloTree, state: Rc<FloorState>, exploration_factor: f64) -> Choice {
    tree.choices[&state]
        .iter()
        .map(|(choice, outcomes)| {
            let score = score_outcomes(outcomes);
            let exploration_score = if exploration_factor != 0.0 {
                score * (exploration_factor / (outcomes.evaluation_count as f64)).sqrt()
            } else {
                score
            };
            (choice, exploration_score)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Less))
        .unwrap()
        .0
        .clone()
}

fn descend(
    tree: &mut MonteCarloTree,
    mut state: Rc<FloorState>,
    max_depth: usize,
    exploration_factor: f64,
) {
    for _ in 0..max_depth {
        tree.choices.entry(state.clone()).or_insert_with(|| {
            enumerator::all_choices(state.as_ref())
                .into_iter()
                .map(|c| {
                    let mut evaluation = ChoiceOutcomes {
                        outcomes: HashSet::new(),
                        evaluation_count: 1,
                    };

                    resolve_choice(state.as_ref(), c.clone(), &mut evaluation);

                    (c, evaluation)
                })
                .collect()
        });

        let choice = best_choice(tree, state.clone(), exploration_factor);

        let possible_outcomes = tree
            .choices
            .get_mut(&state)
            .unwrap()
            .get_mut(&choice)
            .unwrap();

        let total_probability = total_probability(possible_outcomes);
        state = if total_probability > 0.99999 {
            let mut option: f64 = rand::thread_rng().gen();
            possible_outcomes
                .outcomes
                .iter()
                .find_or_last(|a| {
                    if option < a.probability {
                        true
                    } else {
                        option -= a.probability;
                        false
                    }
                })
                .unwrap()
                .state
                .clone()
        } else {
            resolve_choice(state.as_ref(), choice, possible_outcomes)
        };

        // Use state to back-propagate evaluation function
    }
}

fn evaluate(state: &FloorState) -> f64 {
    let game_state = state.game_state();
    game_state.map.floor as f64 * 100.0 + game_state.hp.amount as f64
    // Neural net
}

fn resolve_choice(
    state: &FloorState,
    choice: Choice,
    possible_outcomes: &mut ChoiceOutcomes,
) -> Rc<FloorState> {
    let mut possibility = GamePossibility {
        state: state.clone(),
        probability: Probability::new(),
    };

    predictor::predict_outcome(choice, &mut possibility);

    let state = Rc::new(possibility.state);

    let mut outcome = Outcome {
        state: state.clone(),
        probability: possibility.probability.probability,
        evaluation: 0.0,
    };

    if !possible_outcomes.outcomes.contains(&outcome) {
        outcome.evaluation = evaluate(state.as_ref());
        possible_outcomes.outcomes.insert(outcome);
    }

    possible_outcomes.evaluation_count += 1;

    state
}
