use im::{HashMap, HashSet, Vector};
use uuid::Uuid;

use crate::{models::core::*, spireai::evaluator::{CardReference, CardStorage, CreatureReference}};

use super::{buffs::BaseBuff, cards::BaseCard, events::BaseEvent, monsters::{BaseMonster, Intent, MonsterMove}, potions::BasePotion, relics::BaseRelic};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub map: MapState,
    pub floor_state: FloorState,
    pub battle_state: BattleState,
    pub event_state: Option<EventState>,
    pub card_stasis: HashMap<Uuid, Card>,
    pub card_choices: Vector<CardReference>,
    pub card_choice_range: Option<(usize, usize)>,
    pub card_choice_type: CardChoiceType,
    pub floor: u8,
    pub act: u8,
    pub asc: u8,
    pub deck: HashMap<Uuid, Card>,
    pub potions: Vector<Option<Potion>>,
    pub relics: HashMap<Uuid, Relic>,
    pub relic_whens: HashMap<When, Vector<Uuid>>,
    pub relic_names: HashMap<String, Uuid>,
    pub player: Creature,
    pub gold: u16,
    pub keys: Option<KeyState>,
    pub won: Option<bool>,
}

impl GameState {
    pub fn deck(&self) -> impl Iterator<Item=CardReference> + '_ {
        self.deck.iter().map(|(u, c)| CardReference{uuid: *u, storage: CardStorage::Deck, base: c.base})
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
    pub uuid: Uuid,
    pub creature: Creature,
    pub position: usize,
    pub targetable: bool,
    pub intent: Intent,
    pub vars: Vars,
    pub whens: HashMap<When, &'static String>,
    pub phase: usize,
    pub index: usize,
    pub current_move: &'static MonsterMove,
    pub last_move: Option<&'static MonsterMove>,
    pub last_move_count: u8,
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
    pub buffs: HashMap<Uuid, Buff>,
    pub block: u16,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BattleState {
    pub active: bool,
    pub cards: HashMap<Uuid, Card>,
    pub draw: HashSet<Uuid>,
    pub draw_top_known: Vector<Uuid>,
    pub draw_bottom_known: Vector<Uuid>,
    pub discard: HashSet<Uuid>,
    pub exhaust: HashSet<Uuid>,
    pub hand: HashSet<Uuid>,
    pub monsters: HashMap<Uuid, Monster>,
    pub orbs: Vector<Orb>,
    pub orb_slots: u8,
    pub energy: u8,
    pub stance: Stance,
    pub battle_type: BattleType,
    pub discard_count: u8,
    pub play_count: u8,
    pub hp_loss_count: u8,
    pub power_count: u8,
    pub last_card_played: Option<CardType>,
}

impl<'a> BattleState {
    pub fn discard(&'a self) -> impl Iterator<Item=CardReference> + 'a {
        self.discard.iter().map(move |u| CardReference{uuid: *u, storage: CardStorage::Battle, base: self.cards[u].base})
    }
    pub fn exhaust(&'a self) -> impl Iterator<Item=CardReference> + 'a {
        self.exhaust.iter().map(move |u| CardReference{uuid: *u, storage: CardStorage::Battle, base: self.cards[u].base})
    }
    pub fn hand(&'a self) -> impl Iterator<Item=CardReference> + 'a {
        self.hand.iter().map(move |u| CardReference{uuid: *u, storage: CardStorage::Battle, base: self.cards[u].base})
    }
    pub fn draw(&'a self) -> impl Iterator<Item=CardReference> + 'a {
        self.draw.iter().map(move |u| CardReference{uuid: *u, storage: CardStorage::Battle, base: self.cards[u].base})
    }
    pub fn available_monsters(&'a self) -> impl Iterator<Item = CreatureReference> + 'a {
        self.monsters.values()
            .filter(|m| m.targetable)
            .map(|m| CreatureReference::Creature(m.uuid))
    }
}

impl Default for BattleState {
    fn default() -> Self {
        BattleState {
            active: false,
            draw_top_known: Vector::new(),
            draw_bottom_known: Vector::new(),
            draw: HashSet::new(),
            discard: HashSet::new(),
            exhaust: HashSet::new(),
            hand: HashSet::new(),
            cards: HashMap::new(),
            monsters: HashMap::new(),
            orbs: Vector::new(),
            orb_slots: 0,
            energy: 0,
            stance: Stance::None,
            battle_type: BattleType::Common,
            discard_count: 0,
            play_count: 0,
            hp_loss_count: 0,
            power_count: 0,
            last_card_played: None
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum CardChoiceType {
    None,
    Scry,
    Then(&'static Vec<CardEffect>),
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Buff {
    pub base: &'static BaseBuff,
    pub uuid: Uuid,
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
    pub base_cost: u8,
    pub cost_until_played: bool,
    pub uuid: Uuid,
    pub vars: Vars,
    pub retain: bool,
    pub upgrades: u8,
    pub bottled: bool,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GameAction {
    pub is_attack: bool,
    pub creature: CreatureReference,
    pub target: Option<CreatureReference>,
}
