use crate::models::core::*;
use im::Vector;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub hp: u16,
    pub max_hp: u16,
    pub floor: u8,
    pub deck: Vector<Rc<Card>>,
    pub screen: ScreenState,
    pub potions: Vec<Potion>,
}

#[derive(Clone, Debug)]
pub struct Potion {
    pub base: &'static BasePotion,
}

impl PartialEq for Potion {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base)
    }
}

#[derive(Clone, Debug)]
pub struct Monster {
    pub base: &'static BaseMonster,
    pub hp: u16,
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base) && self.hp == other.hp
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ScreenState {
    Battle(BattleState),
    None,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BattleState {
    pub draw: Vector<Rc<Card>>,
    pub discard: Vector<Rc<Card>>,
    pub exhaust: Vector<Rc<Card>>,
    pub hand: Vector<Rc<Card>>,
    pub monsters: Vec<Monster>,
}

pub struct Relic {}

pub struct Buff {}

#[derive(Clone, Debug)]
pub struct Card {
    pub base: &'static BaseCard,
    pub cost: u8,
    pub n: u16,
    pub n_reset: u16,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base) && self.cost == other.cost
    }
}

#[derive(PartialEq, Clone)]
pub struct GamePossibility {
    pub probability: f64,
    pub state: GameState,
}

pub struct GameAction {
    pub card: Card,
    pub target: Option<u8>,
}

pub struct GamePossibilitySet {
    pub states: Vector<Rc<GamePossibility>>,
}

impl GamePossibilitySet {
    /*
    pub fn contains_state(&self, state: &GameState) -> bool {
        self.states.iter().any(|a| &a.state == state)
    }

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
