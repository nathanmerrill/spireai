use crate::spireai::GamePossibilitySet;
use crate::models;
use models::state::GameState;
use models::choices::Choice;

pub fn predict_outcome(state: &GameState, choice: &Choice) -> Option<GamePossibilitySet> {
    None
}

pub fn verify_prediction<'a>(outcome: &GameState, prediction: &'a GamePossibilitySet) -> &'a GameState {
    let matches: Vec<&GameState> = prediction
        .states
        .iter()
        .map(|a| &a.state)
        .filter(|a| a == &outcome)
        .collect();
    match matches.len() {
        0 => panic!("New state did not match any of the predicted states.\n New state: {:?}.\n\n Expected states: {:?}", outcome, prediction.states),
        1 => matches.get(0).unwrap(),
        _ => panic!("New state matched multiple predicted states.\n New state: {:?}.\n\n Expected states: {:?}", outcome, prediction.states),
    }
}

