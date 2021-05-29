use std::ops::{Index, IndexMut};

use im::{HashMap, Vector};
use uuid::Uuid;

use crate::{models::core::*, spireai::evaluator::{CardReference, CreatureReference}};

use super::{buffs::BaseBuff, cards::BaseCard, events::BaseEvent, monsters::{BaseMonster, Intent}, potions::BasePotion, relics::BaseRelic};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub map: MapState,
    pub floor_state: FloorState,
    pub battle_state: BattleState,
    pub event_state: Option<EventState>,
    pub floor: u8,
    pub act: u8,
    pub asc: u8,
    pub deck: UuidVector<Card>,
    pub potions: Vector<Option<Potion>>,
    pub relics: UuidVector<Relic>,
    pub relic_whens: HashMap<When, Vector<Uuid>>,
    pub relic_names: HashMap<String, Uuid>,
    pub player: Creature,
    pub gold: u16,
    pub keys: Option<KeyState>,
    pub won: Option<bool>,
}

impl GameState {
    pub fn cards(&self) -> impl Iterator<Item=CardReference> + '_ {
        self.deck.uuids.iter().map(|u| CardReference::Deck(*u))
    }
}

pub trait Keyed {
    fn get_key(&self) -> Uuid;
}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct UuidVector<A> 
    where A: Clone + Keyed
{
    pub uuids: Vector<Uuid>,
    pub items: HashMap<Uuid, A>
}

impl<A> UuidVector<A> 
    where A: Clone + Keyed
{
    pub fn get(&self, uuid: Uuid) -> &A {
        self.items.get(&uuid).unwrap()
    }

    pub fn get_mut(&mut self, uuid: Uuid) -> &mut A {
        self.items.get_mut(&uuid).unwrap()
    }

    pub fn add(&mut self, item: A) {
        let uuid = item.get_key();
        self.uuids.push_back(uuid);
        self.items.insert(uuid, item);
    }

    pub fn remove(&mut self, index: usize) -> A {
        let uuid = self.uuids.remove(index);
        self.items.remove(&uuid).unwrap()
    }

    pub fn new() -> UuidVector<A> {
        UuidVector {
            uuids: Vector::new(),
            items: HashMap::new()
        }
    }
}

impl<A> Index<usize> for UuidVector<A> 
    where A: Clone + Keyed
{
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[&self.uuids[index]]
    }
}

impl<A> IndexMut<usize> for UuidVector<A> 
    where A: Clone + Keyed
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[&self.uuids[index]]
    }
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
    pub position: usize,
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
    Rewards(Vector<Reward>),
    CardReward(Vector<(String, bool)>), // true if upgraded
    ShopEntrance,
    Shop {
        cards: Vector<(String, u16)>,
        potions: Vector<(String, u16)>,
        relics: Vector<(String, u16)>,
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
    pub buffs_when: HashMap<When, Vector<Uuid>>,
    pub buff_names: HashMap<String, Uuid>,
    pub buffs: UuidVector<Buff>,
    pub block: u16,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BattleState {
    pub active: bool,
    pub draw_top_known: Vector<Uuid>,
    pub draw_bottom_known: Vector<Uuid>,
    pub draw: HashMap<Uuid, Card>,
    pub discard: UuidVector<Card>,
    pub exhaust: UuidVector<Card>,
    pub hand: UuidVector<Card>,
    pub monsters: Vector<Monster>,
    pub orbs: Vector<Orb>,
    pub orb_slots: u8,
    pub energy: u8,
    pub stance: Stance,
    pub battle_type: BattleType,
    pub card_choices: UuidVector<Card>,
    pub card_choice_type: CardChoiceType,
    pub discard_count: u8,
    pub play_count: u8,
    pub last_card_played: Option<CardType>,
}

impl<'a> BattleState {
    pub fn discard(&'a self) -> impl Iterator<Item=CardReference> + 'a {
        self.discard.uuids.iter().map(|u| CardReference::Discard(*u))
    }
    pub fn exhaust(&'a self) -> impl Iterator<Item=CardReference> + 'a {
        self.discard.uuids.iter().map(|u| CardReference::Exhaust(*u))
    }
    pub fn hand(&'a self) -> impl Iterator<Item=CardReference> + 'a {
        self.discard.uuids.iter().map(|u| CardReference::Hand(*u))
    }
    pub fn draw(&'a self) -> impl Iterator<Item=CardReference> + 'a {
        self.draw.keys().map(|u| CardReference::Hand(*u))
    }
    pub fn available_monsters(&self) -> Vector<CreatureReference> {
        self.monsters.iter()
            .filter(|m| m.targetable)
            .map(|m| CreatureReference::Creature(m.position))
            .collect()
    }
}

impl Default for BattleState {
    fn default() -> Self {
        BattleState {
            active: false,
            draw_top_known: Vector::new(),
            draw_bottom_known: Vector::new(),
            draw: HashMap::new(),
            discard: UuidVector::new(),
            exhaust: UuidVector::new(),
            hand: UuidVector::new(),
            monsters: Vector::new(),
            orbs: Vector::new(),
            orb_slots: 0,
            energy: 0,
            stance: Stance::None,
            battle_type: BattleType::Common,
            card_choices: UuidVector::new(),
            card_choice_type: CardChoiceType::None,
            discard_count: 0,
            play_count: 0,
            last_card_played: None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CardChoiceType {
    None,
    Scry,
    AddToLocation(CardLocation, RelativePosition, Vec<CardEffect>)
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
    pub uuid: Uuid,
    pub vars: Vars,
    pub enabled: bool,
}

impl Keyed for Relic {
    fn get_key(&self) -> Uuid {
        self.uuid
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Buff {
    pub base: &'static BaseBuff,
    pub uuid: Uuid,
    pub vars: Vars,
    pub stacked_vars: Vec<Vars>,
}

impl Keyed for Buff {
    fn get_key(&self) -> Uuid {
        self.uuid
    }
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
    pub uuid: Uuid,
    pub vars: Vars,
    pub upgrades: u8,
    pub bottled: bool,
}

impl Keyed for Card {
    fn get_key(&self) -> Uuid {
        self.uuid
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GameAction {
    pub is_attack: bool,
    pub creature: CreatureReference,
    pub target: Option<usize>,
}
