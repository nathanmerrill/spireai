use crate::models::core::*;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub map: MapState,
    pub floor_state: FloorState,
    pub battle_state: Option<BattleState>,
    pub act: u8,
    pub asc: u8,
    pub deck: Vec<Card>,
    pub potions: Vec<Option<Potion>>,
    pub relics: Vec<Relic>,
    pub relic_names: HashSet<&'static str>,
    pub player: Creature,
    pub gold: u16,
    pub keys: Option<KeyState>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct KeyState {
    pub ruby: bool,
    pub emerald: bool,
    pub sapphire: bool,
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
pub enum FloorState {
    Event(EventState),
    EventUpgrade(u8),
    EventTransform(u8, bool), // true if it upgrades cards
    EventRemove(u8),
    Rest,
    Chest(ChestType),
    Battle,
    Map,
    GameOver,
    Rewards(Vec<Reward>),
    CardReward(Vec<Card>),
    ShopEntrance,
    Shop(Vec<(Card, u16)>, Vec<(Relic, u16)>, Vec<(Potion, u16)>, u16) // Last u8 is remove
}

#[derive(PartialEq, Clone, Debug)]
pub enum Reward {
    CardChoice,
    Gold(u8),
    Relic(Relic),
    Potion(Potion),
    EmeraldKey,
    SapphireKey(Relic),
}

#[derive(PartialEq, Clone, Debug)]
pub struct MapState {
    pub nodes: HashMap<(i8, i8), MapNode>,
    pub floor: i8,
    pub x: i8,
}

#[derive(PartialEq, Clone, Debug)]
pub struct MapNode {
    pub floor: i8,
    pub x: i8,
    pub next: Vec<i8>,
    pub icon: MapNodeIcon
}

#[derive(PartialEq, Clone, Debug)]
pub enum MapNodeIcon {
    Question,
    Elite,
    BurningElite,
    Campfire,
    Boss,
    Monster,
    Shop,
    Chest,
}

#[derive(Clone, Debug)]
pub struct EventState {
    pub base: &'static BaseEvent,
    pub variant: Option<&'static str>,
    pub variant_cards: Vec<Card>,
    pub variant_relic: Option<&'static str>,
    pub variant_amount: Option<u16>,
    pub available_choices: Vec<&'static str>,
    
}

impl PartialEq for EventState {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum ChestType {
    Large,
    Medium,
    Small,
    Boss
}



#[derive(PartialEq, Clone, Debug)]
pub struct Creature {
    pub hp: u16,
    pub max_hp: u16,
    pub position: u8,
    pub is_player: bool,
    pub buffs: HashMap<&'static str, Buff>,
    pub block: u16,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BattleState {
    pub draw: Vec<Card>,
    pub discard: Vec<Card>,
    pub exhaust: Vec<Card>,
    pub hand: Vec<Card>,
    pub monsters: Vec<Monster>,
    pub orbs: Vec<Orb>,
    pub energy: u8,
    pub stance: Stance,
    pub battle_type: BattleType,
    pub card_choices: Vec<Card>,
    pub card_choice_type: CardChoiceType,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CardChoiceType {
    None,
    Scry,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BattleType {
    Common,
    Elite,
    Boss,
    Event
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
    pub id: String,
    pub cost: u8,
    pub vars: Vars,
    pub upgrades: u8,
    pub bottled: bool,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.base, other.base)
            && self.cost == other.cost
            && self.upgrades == other.upgrades
    }
}

pub struct GameAction<'a> {
    pub is_attack: bool,
    pub creature: &'a Creature,
    pub target: Option<u8>,
}
