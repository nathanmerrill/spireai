use im::Vector;

use crate::models::core::{ChestType, DeckOperation};

use super::{core::{CardOffer, RewardState}, battle::BattleState, game::GameState, shop::ShopState, probability::Probability, event::EventState};



#[derive(PartialEq, Clone, Debug)]
pub struct GamePossibility {
    pub state: FloorState,
    pub probability: Probability,
}


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
//The goal here is not to enumerate every possible screen state, but the states that the AI will hit (e.g. once the map has been viewed, no returning)
pub enum FloorState {
    Event(EventState),
    Rest(RestState),
    Chest(ChestState),
    Battle(BattleState),
    BattleOver(BattleOverState),
    GameOver(bool),
    Shop(ShopState),
    Map(GameState),
    Menu
}

impl FloorState {
    pub fn game_state(&self) -> &GameState {
        match self {
            FloorState::Event(state) => &state.game_state,
            FloorState::Rest(state) => &state.game_state,
            FloorState::Chest(state) => &state.game_state,
            FloorState::Battle(state) => &state.game_state,
            FloorState::BattleOver(state) => &state.game_state,
            FloorState::Shop(state) => &state.game_state,
            FloorState::Map(state) => state,
            FloorState::GameOver(_) => panic!("No game state in GameOver"),
            FloorState::Menu => panic!("No game state in Menu"),
        }
    }

    pub fn game_state_mut(&mut self) -> &mut GameState {
        match self {
            FloorState::Event(state) => &mut state.game_state,
            FloorState::Rest(state) => &mut state.game_state,
            FloorState::Chest(state) => &mut state.game_state,
            FloorState::Battle(state) => &mut state.game_state,
            FloorState::BattleOver(state) => &mut state.game_state,
            FloorState::Shop(state) => &mut state.game_state,
            FloorState::Map(state) => state,
            FloorState::GameOver(_) => panic!("No game state in GameOver"),
            FloorState::Menu => panic!("No game state in Menu"),
        }
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct BattleOverState {
    pub game_state: GameState,
    pub rewards: RewardState,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct RestState {
    pub screen_state: RestScreenState,
    pub game_state: GameState,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum RestScreenState {
    IShouldRest,
    Toke,
    DreamCatch(Vector<CardOffer>),
    Smith,
    Dig(RewardState),
    DeckSelect(DeckOperation),
    Proceed
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ChestState {
    pub chest: ChestType,
    pub rewards: Option<RewardState>,  // Taking tiny house replaces this rewards list
    pub game_state: GameState,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy,Debug)]
pub struct KeyState {
    pub ruby: bool,
    pub emerald: bool,
    pub sapphire: bool,
}
