use crate::models::core::*;
use im::Vector;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub floor: u8,
    pub act: u8,
    pub asc: u8,
    pub deck: Vector<Rc<Card>>,
    pub screen: ScreenState,
    pub potions: Vec<Potion>,
    pub relics: HashMap<&'static str, Relic>,
    pub player: Creature,
    pub room: RoomType,
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
    pub creature: Creature,
    pub targetable: bool,
    pub intent: Intent,
    pub vars: Vars,
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base)
            && self.creature == other.creature
            && self.targetable == other.targetable
            && self.intent == other.intent
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ScreenState {
    Battle(BattleState),
    None,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Creature {
    pub hp: u16,
    pub max_hp: u16,
    pub position: u8,
    pub is_player: bool,
    pub buffs: HashMap<&'static str, Buff>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BattleState {
    pub draw: Vector<Rc<Card>>,
    pub discard: Vector<Rc<Card>>,
    pub exhaust: Vector<Rc<Card>>,
    pub hand: Vector<Rc<Card>>,
    pub monsters: Vec<Monster>,
    pub orbs: Vec<Orb>,
    pub energy: u8,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Orb {
    pub base: OrbType,
    pub n: u16,
}

#[derive(Clone, Debug)]
pub struct Relic {
    pub base: &'static BaseRelic,
    pub vars: Vars,
}

impl PartialEq for Relic {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base) && self.vars.n == other.vars.n
    }
}

#[derive(Clone, Debug)]
pub struct Buff {
    pub base: &'static BaseBuff,
    pub vars: Vars,
}

impl PartialEq for Buff {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base) && self.vars.n == other.vars.n
    }
}

#[derive(Clone, Debug)]
pub struct Vars {
    pub n: u8,
    pub n_reset: u8,
    pub x: u8,
}

#[derive(Clone, Debug)]
pub struct Card {
    pub base: &'static BaseCard,
    pub cost: u8,
    pub vars: Vars,
    pub upgrades: u8,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base)
            && self.cost == other.cost
            && self.upgrades == other.upgrades
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct GamePossibility {
    pub probability: f64,
    pub state: GameState,
}

pub struct GameAction<'a> {
    pub is_attack: bool,
    pub creature: &'a Creature,
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
