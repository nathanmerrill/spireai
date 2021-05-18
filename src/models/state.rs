use crate::models::core::*;
use std::collections::{HashMap, HashSet};

use super::{buffs::BaseBuff, cards::BaseCard, events::BaseEvent, monsters::{BaseMonster, Intent}, potions::BasePotion, relics::BaseRelic};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub map: MapState,
    pub floor_state: FloorState,
    pub battle_state: Option<BattleState>,
    pub event_state: Option<EventState>,
    pub floor: u8,
    pub act: u8,
    pub asc: u8,
    pub deck: Vec<Card>,
    pub potions: Vec<Option<Potion>>,
    pub relics: Vec<Relic>,
    pub relic_names: HashSet<String>,
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
    Event,
    EventUpgrade(u8),
    EventTransform(u8, bool), // true if it upgrades cards
    EventRemove(u8),
    Rest,
    Chest(ChestType),
    Battle,
    Map,
    GameOver,
    Rewards(Vec<Reward>),
    CardReward(Vec<(String, bool)>), // true if upgraded
    ShopEntrance,
    Shop {
        cards: Vec<(String, u16)>,
        potions: Vec<(String, u16)>,
        relics: Vec<(String, u16)>,
        purge_cost: u16,
    }, // Last u8 is remove
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
    pub icon: MapNodeIcon,
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
    pub vars: Vars,
    pub variant: Option<String>,
    pub variant_cards: Vec<Card>,
    pub variant_relic: Option<String>,
    pub variant_amount: Option<u16>,
    pub available_choices: Vec<String>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Creature {
    pub hp: u16,
    pub max_hp: u16,
    pub is_player: bool,
    pub position: usize,
    pub buffs: HashMap<String, Buff>,
    pub block: u16,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BattleState {
    pub draw_top_known: usize,
    pub draw_bottom_known: usize,
    pub draw: Vec<Card>,
    pub discard: Vec<Card>,
    pub exhaust: Vec<Card>,
    pub hand: Vec<Card>,
    pub monsters: Vec<Monster>,
    pub orbs: Vec<Orb>,
    pub orb_slots: u8,
    pub energy: u8,
    pub stance: Stance,
    pub battle_type: BattleType,
    pub card_choices: Vec<Card>,
    pub card_choice_type: CardChoiceType,
    pub discard_count: u8,
    pub last_card_played: Option<CardType>,
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
    Event,
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
