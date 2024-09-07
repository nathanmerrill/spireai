use crate::comm::interop;
use crate::comm::request::GameState as CommState;
use crate::state::floor::{FloorState, GamePossibility};
use crate::{models, state::probability::Probability};
use im::{HashMap, HashSet};
use itertools::Itertools;
use models::choices::Choice;
use num::complex::ComplexFloat;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rustc_hash::FxHasher;
use std::cmp::Ordering;
use std::hash::{BuildHasher, Hash};
use std::rc::Rc;
use uuid::Uuid;

pub mod appraiser;
pub mod enumerator;
pub mod predictor;
pub mod references;

pub struct SpireAi {
    last_choice: Option<Choice>,
    tree: MonteCarloTree,
    pub uuid_map: HashMap<String, Uuid>,
}

impl SpireAi {
    pub fn new(state: FloorState) -> SpireAi {
        SpireAi {
            last_choice: None,
            tree: MonteCarloTree::new(Rc::new(Box::new(state))),
            uuid_map: HashMap::new(),
        }
    }

    pub fn update_state(&mut self, comm_state: &Option<CommState>) {
        if let Some(choice) = self.last_choice {
            if let Some(mut matching_state) = self.find_match(&choice, comm_state) {
                interop::update_state(comm_state, matching_state.as_mut());
                self.tree.new_root(matching_state);
            } else {
                panic!("No matching state found!")
            }
        }
    }

    pub fn choose(&mut self) -> Option<Choice> {
        let choice = self.tree.make_choice();
        self.last_choice = choice;
        choice
    }

    fn find_match(&mut self, choice: &Choice, comm_state: &Option<CommState>) -> Option<GameState> {
        if let Some(outcomes) = self.tree.root.get_outcomes(choice) {
            for outcome in outcomes.outcomes.keys() {
                if interop::state_matches(comm_state, outcome, &mut self.uuid_map) {
                    return Some(*outcome);
                }
            }
        }
        None
    }
}

type GameState = Rc<Box<FloorState>>;

#[derive(PartialEq, Clone)]
struct MonteCarloTree {
    root: MonteCarloNode,
    nodes: HashMap<GameState, MonteCarloNode, FxBuildHasher>,
}

impl MonteCarloTree {
    pub fn new(state: GameState) -> Self {
        let root = MonteCarloNode::new(state, 0);
        let mut nodes = HashMap::with_hasher(FxBuildHasher::default());
        Self {
            root,
            nodes,
        }
    }

    pub fn new_root(&mut self, root: GameState) {
        let old_depth = self.root.depth;
        if let Some(node) = self.nodes.remove(&root) {
            self.root = node;
            self.nodes.retain(|_, v| v.depth > old_depth)
        } else {
            *self = MonteCarloTree::new(root);
        }
    }

    pub fn make_choice(&self) -> Option<Choice> {
        self.root.choose().map(|a| a.choice)
    }

    pub fn explore(&mut self) -> Vec<(Choice, GameState)> {
        let mut current_node = &self.root;
        let mut path: Vec<(Choice, GameState)> = vec![];
        loop {
            if let Some(choice) = current_node.choose() {
                
                let outcome = choice.predict_outcome(current_node.game);
                let mut inserted = false;
                current_node = self.nodes.entry(outcome).or_insert_with(|| {
                    inserted = true;
                    MonteCarloNode::new(outcome, current_node.depth + 1, )
                });
                path.push((choice.choice, current_node.game));
                
                if inserted
                {
                    return path;
                }
            }
        }
    }
}

#[derive(PartialEq, Clone)]
struct MonteCarloNode {
    game: GameState,
    depth: usize,
    visits: f64,
    eval: f64,
    children: Vec<ChoiceOutcomes>,
}

impl MonteCarloNode {
    pub fn new(state: GameState, depth: usize) -> Self {
        let eval = evaluate(&state);
        let mut children: Vec<_> = enumerator::all_choices(&state)
            .into_iter()
            .map(ChoiceOutcomes::new)
            .collect();

        // Shuffle children to make exploration of choices balanced
        children.shuffle(&mut rand::thread_rng());

        Self {
            game: state,
            depth,
            visits: 0.0,
            eval,
            children,
        }
    }

    pub fn get_outcomes(&self, choice: &Choice) -> Option<&ChoiceOutcomes> {
        self.children.iter().find(|f| &f.choice == choice)
    }

    pub fn choose(&self) -> Option<&ChoiceOutcomes> {
        let total_visits_factor = self.visits.ln() * 2.0;
        self.children.iter()
            .map(|child| (child, child.eval(total_visits_factor)))
            .reduce(|child1, child2| {
                if child1.1 < child2.1 {
                    child2
                } else {
                    child1
                }
            })
            .map(|a| a.0)
    }
}

#[derive(PartialEq, Clone)]
struct ChoiceOutcomes {
    choice: Choice,
    outcomes: HashMap<GameState, f64>,
    visits: f64,
    fully_evaluated: bool,
}

impl ChoiceOutcomes {
    fn new(choice: Choice) -> Self {
        Self {
            choice,
            outcomes: HashMap::new(),
            visits: 0.0,
            fully_evaluated: false,
        }
    }

    fn total_probability(&self) -> f64 {
        let total_probability: f64 = self.outcomes.values().sum();
        assert!(
            total_probability < 1.00001,
            "Total probability greater than 1!"
        );
        total_probability
    }

    fn eval(&self, total_visits_factor: f64) -> f64 {
        if self.outcomes.is_empty() {
            return f64::MAX;
        }

        let mut total_probability: f64 = 0.0;
        let mut total_eval: f64 = 0.0;
        for (outcome, probability) in self.outcomes {
            total_probability += probability;
            total_eval += outcome.eval * outcome.probability;
        }

        total_eval / total_probability + (total_visits_factor / self.visits).sqrt()
    }

    fn predict_outcome(&mut self, state: GameState) -> GameState {
        if self.fully_evaluated {
            let mut remaining = rand::thread_rng().gen_range(0.0..1.0);
            for (state, val) in self.outcomes {
                remaining -= val;
                if remaining < 0.00001 {
                    return state;
                }
            }

            panic!("Expected total probability to be greater than the random range!")
        }
        let mut possibility = GamePossibility {
            state: state.clone(),
            probability: Probability::new(),
        };

        predictor::predict_outcome(self.choice, &mut possibility);

        let probability = possibility.probability.probability;
        let state = Rc::new(Box::new(possibility.state));

        let existing_probability = *self.outcomes.entry(state).or_insert(probability);

        assert!(
            (probability - existing_probability).abs() < 0.0001,
            "Probabilities of equal states are different!"
        );

        if self.total_probability() > 0.99999 {
            self.fully_evaluated = true
        }

        state
    }
}

fn evaluate(state: &FloorState) -> f64 {
    let game_state = state.game_state();
    game_state.map.floor as f64 * 100.0 + game_state.hp.amount as f64
    // Neural net
}

#[derive(Default)]
struct FxBuildHasher;

impl BuildHasher for FxBuildHasher {
    type Hasher = FxHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FxHasher::default()
    }
}

#[cfg(test)]
mod test {
    use crate::{models::choices::Choice, state::floor::FloorState};

    use super::SpireAi;

    #[test]
    fn test_start() {
        let mut ai = SpireAi::new(FloorState::Menu);
        let choice = ai.choose(&None);
        assert!(matches!(choice, Choice::Start { .. }))
    }
}
