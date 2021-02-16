use crate::models;

use models::GameState;
use models::cards;

struct GamePossibility {
    probability: f64,
    state: GameState
}

pub struct GameAction {
    card: GameCard,
    target: Option<i32>
}

pub struct GamePossibilitySet {
    states: Vec<GamePossibility>
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

pub struct GameCard {
    pub base_card: cards::BaseCard,
    pub meta_number: Option<i32>,
    pub upgrades: i32,
} 

// Playables

pub fn clash_playable(card: &GameCard, state: &GameState) -> bool {
    return state.combat_state.unwrap().hand.iter().all(|a| a.card_type == models::CardType::Attack)
}

// Calculations

pub fn body_slam_damage(card: &GameCard, state: &GameState) -> i32 {
    return state.combat_state.unwrap().player.block
}

pub fn heavy_blade_damage(card: &GameCard, state: &GameState) -> i32 {
    let strength: i32 = state.combat_state.unwrap().player.powers.iter().find(|a| a.id == "Strength".to_string()).map(|a| a.amount).unwrap_or(0);
    if card.upgrades == 0 {
        return 14 + 2*strength
    } else {
        return 14 + 4*strength
    }
}

pub fn perfected_strike_damage(card: &GameCard, state: &GameState) -> i32 {
    let combat_state = state.combat_state.unwrap();
    let num_strikes = 
        (combat_state.discard_pile.iter().filter(|a| a.id.contains("Strike")).count() + 
        combat_state.hand.iter().filter(|a| a.id.contains("Strike")).count() + 
        combat_state.draw_pile.iter().filter(|a| a.id.contains("Strike")).count()) as i32;

    if card.upgrades == 0 {
        return 6 + 2*num_strikes
    } else {
        return 6 + 3*num_strikes
    }    
}

pub fn blood_for_blood_cost(card: &GameCard, state: &GameState) -> i32 {
    card.base_card.cost;
}

pub fn entrench_block(card: &GameCard, state: &GameState) -> i32 {
    
}

pub fn searing_blow_damage(card: &GameCard, state: &GameState) -> i32 {
    
}

// Custom effects
pub fn rampage_effect(action: &GameAction, state: &GameState) -> GamePossibilitySet {
    
}

pub fn second_wind_effect(action: &GameAction, state: &GameState) -> GamePossibilitySet {
    
}

pub fn sever_soul_effect(action: &GameAction, state: &GameState) -> GamePossibilitySet {
    
}

pub fn fiend_fire_effect(action: &GameAction, state: &GameState) -> GamePossibilitySet {
    
}

pub fn limit_break_effect(action: &GameAction, state: &GameState) -> GamePossibilitySet {
    
}

pub fn reaper_effect(action: &GameAction, state: &GameState) -> GamePossibilitySet {
    
}