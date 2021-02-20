use im::Vector;
use crate::models::core::*;

#[derive(PartialEq)]
pub struct GameState {
    pub class: Class,
    pub hp: i32,
    pub max_hp: i32,
    pub floor: i32,
    pub cards: Vector<Card>,
}

#[derive(PartialEq)]
pub struct BattleState {
    
}

pub struct GamePossibility {
    pub probability: f64,
    pub state: GameState
}


pub struct GameAction {
    pub card: Card,
    pub target: Option<i32>
}

pub struct GamePossibilitySet {
    pub states: Vec<GamePossibility>
}


impl GamePossibilitySet {
    pub fn contains_state(&self, state: &GameState) -> bool {
        self.states.iter().any(|a| &a.state == state)
    }

    pub fn new(state: GameState) -> GamePossibilitySet {
        GamePossibilitySet {
            states: vec![
                GamePossibility {
                    probability: 1.0,
                    state: state,
                }
            ]
        }
    }
}