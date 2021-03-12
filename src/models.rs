use serde::Deserialize;
pub mod buffs;
pub mod cards;
pub mod core;
pub mod monsters;
pub mod relics;
pub mod state;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerClass {
    Ironclad,
    Silent,
    Defect,
    Watcher,
    Other,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
    Curse,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoomPhase {
    Combat,
    Event,
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EventOption {
    pub text: String,
    pub label: String,
    pub disabled: bool,
    pub choice_index: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Event {
    pub event_name: String,
    pub event_id: String,
    pub body_text: String,
    pub options: Vec<EventOption>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Chest {
    pub chest_type: ChestType,
    pub chest_open: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Rest {
    pub has_rested: bool,
    pub rest_options: Vec<RestOption>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CardReward {
    pub cards: Vec<Card>,
    pub bowl_available: bool,
    pub skip_available: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MapChoice {
    pub current_node: Option<MapNode>,
    pub next_nodes: Option<Vec<MapNode>>,
    pub boss_available: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ShopScreen {
    pub cards: Vec<Card>,
    pub relics: Vec<Relic>,
    pub potions: Vec<Potion>,
    pub purge_available: bool,
    pub purge_cost: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Grid {
    pub cards: Vec<Card>,
    pub selected_cards: Vec<Card>,
    pub num_cards: i32,
    #[serde(default)]
    pub any_number: bool,
    pub confirm_up: bool,
    pub for_upgrade: bool,
    pub for_transform: bool,
    pub for_purge: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HandSelect {
    pub cards: Vec<Card>,
    pub selected: Vec<Card>,
    pub num_cards: i32,
    pub can_pick_zero: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GameOver {
    pub score: i32,
    pub victory: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(
    tag = "screen_type",
    content = "screen_state",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ScreenState {
    None,
    Event(Event),
    Chest(Chest),
    ShopRoom,
    Rest(Rest),
    CardReward(CardReward),
    CombatReward(Vec<RewardType>),
    Map(MapChoice),
    BossReward(Vec<Relic>),
    ShopScreen(ShopScreen),
    Grid(Grid),
    HandSelect(HandSelect),
    GameOver(GameOver),
    Complete,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChestType {
    Small,
    Medium,
    Large,
    Boss,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RewardType {
    Card,
    Gold { gold: i32 },
    Relic { relic: Relic },
    Potion { potion: Potion },
    StolenGold { gold: i32 },
    EmeraldKey,
    SapphireKey { link: Relic },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RestOption {
    Dig,
    Lift,
    Recall,
    Rest,
    Smith,
    Toke,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Response {
    pub error: Option<String>,
    pub ready_for_command: bool,
    #[serde(default)]
    pub in_game: bool,
    pub game_state: Option<GameState>,
    #[serde(default)]
    pub available_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GameState {
    pub current_action: Option<String>,
    pub current_hp: i32,
    pub max_hp: i32,
    pub floor: i32,
    pub act: i32,
    pub gold: i32,
    pub seed: i64,
    pub class: PlayerClass,
    pub ascension_level: i32,
    pub relics: Vec<Relic>,
    pub deck: Vec<Card>,
    pub map: Vec<MapNode>,
    pub potions: Vec<Potion>,
    pub act_boss: Option<String>,
    #[serde(default)]
    pub is_screen_up: bool,
    pub room_phase: RoomPhase,
    pub room_type: String,
    pub combat_state: Option<CombatState>,
    #[serde(flatten)]
    pub screen_state: ScreenState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CombatState {
    pub player: Player,
    pub monsters: Vec<Monster>,
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub exhaust_pile: Vec<Card>,
    pub hand: Vec<Card>,
    pub limbo: Vec<Card>,
    pub card_in_play: Option<Card>,
    #[serde(default)]
    pub turn: i32,
    #[serde(default)]
    pub cards_discarded_this_turn: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Intent {
    Attack,
    AttackBuff,
    AttackDebuff,
    AttackDefend,
    Buff,
    Debuff,
    StrongDebuff,
    Debug,
    Clone,
    PartialEq,
    Defend,
    DefendDebuff,
    DefendBuff,
    Escape,
    Magic,
    None,
    Sleep,
    Stun,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Monster {
    pub max_hp: i32,
    pub current_hp: i32,
    pub block: i32,
    pub powers: Vec<Power>,
    pub name: String,
    pub id: String,
    pub intent: Intent,
    pub half_dead: bool,
    pub is_gone: bool,
    pub move_id: Option<i32>,
    pub last_move_id: Option<i32>,
    pub second_last_move_id: Option<i32>,
    #[serde(default)]
    pub move_base_damage: i32,
    #[serde(default)]
    pub move_adjusted_damage: i32,
    #[serde(default)]
    pub move_hits: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Player {
    pub max_hp: i32,
    pub current_hp: i32,
    pub block: i32,
    pub powers: Vec<Power>,
    pub energy: i32,
    pub orbs: Vec<Orb>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Orb {
    pub name: String,
    pub orb_id: String,
    pub evoke_amount: i32,
    pub passive_amount: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Power {
    pub id: String,
    pub name: String,
    pub amount: i32,
    pub damage: Option<i32>,
    pub misc: Option<i32>,
    #[serde(default)]
    pub just_applied: bool,
    pub card: Option<Card>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Relic {
    pub id: String,
    pub name: String,
    pub counter: i32,
    pub price: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub rarity: CardRarity,
    pub upgrades: i32,
    pub has_target: bool,
    pub cost: i32,
    pub uuid: String,
    pub misc: Option<i32>,
    pub price: Option<i32>,
    #[serde(default)]
    pub is_playable: bool,
    #[serde(default)]
    pub exhausts: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MapNode {
    pub x: i32,
    pub y: i32,
    #[serde(default)]
    pub symbol: char,
    #[serde(default)]
    pub children: Vec<MapNode>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Potion {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub can_use: bool,
    #[serde(default)]
    pub can_discard: bool,
    #[serde(default)]
    pub requires_target: bool,
    pub price: Option<i32>,
}
