use crate::models::core::*;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Clone, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct KeyState {
    pub ruby: bool,
    pub emerald: bool,
    pub sapphire: bool,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Potion {
    pub base: &'static BasePotion,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Monster {
    pub base: &'static BaseMonster,
    pub creature: Creature,
    pub targetable: bool,
    pub intent: Intent,
    pub vars: Vars,
}

#[derive(PartialEq, Eq, Clone, Debug)]
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
    CardReward(Vec<(&'static str, bool)>), // true if upgraded
    ShopEntrance,
    Shop {
        cards: Vec<(&'static str, u16)>, 
        potions: Vec<(&'static str, u16)>, 
        relics: Vec<(&'static str, u16)>, 
        purge_cost: u16
     } // Last u8 is remove
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Reward {
    CardChoice,
    Gold(u8),
    Relic(Relic),
    Potion(Potion),
    EmeraldKey,
    SapphireKey(Relic),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MapState {
    pub nodes: HashMap<(i8, i8), MapNode>,
    pub floor: i8,
    pub x: i8,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MapNode {
    pub floor: i8,
    pub x: i8,
    pub next: Vec<i8>,
    pub icon: MapNodeIcon
}

#[derive(PartialEq, Eq, Clone, Debug)]
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EventState {
    pub base: &'static BaseEvent,
    pub variant: Option<&'static str>,
    pub variant_cards: Vec<Card>,
    pub variant_relic: Option<&'static str>,
    pub variant_amount: Option<u16>,
    pub available_choices: Vec<&'static str>,
    
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Creature {
    pub hp: u16,
    pub max_hp: u16,
    pub is_player: bool,
    pub position: usize,
    pub buffs: HashMap<&'static str, Buff>,
    pub block: u16,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BattleState {
    pub draw_top: Vec<Card>,
    pub draw_unknown: Vec<Card>,
    pub draw_bottom: Vec<Card>,
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CardChoiceType {
    None,
    Scry,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BattleType {
    Common,
    Elite,
    Boss,
    Event
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Orb {
    pub base: OrbType,
    pub n: u16,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Relic {
    pub base: &'static BaseRelic,
    pub vars: Vars,
    pub enabled: bool,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Buff {
    pub base: &'static BaseBuff,
    pub vars: Vars,
    pub stacked_vars: Vec<Vars>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Vars {
    pub n: i16,
    pub n_reset: i16,
    pub x: i16,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Card {
    pub base: &'static BaseCard,
    pub cost: u8,
    pub vars: Vars,
    pub upgrades: u8,
    pub bottled: bool,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GameAction<'a> {
    pub is_attack: bool,
    pub creature: &'a Creature,
    pub target: Option<usize>,
}
