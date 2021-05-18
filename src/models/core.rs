use serde::{Deserialize, Serialize};
use serde;

// ------------------  Fundamental types  -------------------------

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Rarity {
    Starter,
    Common,
    Uncommon,
    Rare,
    Special,
    Event,
    Status,
    Curse,
    Shop,
    Boss,
}

impl Default for Rarity {
    fn default() -> Self {
        Rarity::Common
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, strum_macros::Display, Deserialize, Serialize)]
pub enum Class {
    All,
    None,
    Ironclad,
    Silent,
    Defect,
    Watcher,
    Curse,
}

impl Default for Class {
    fn default() -> Self {
        Class::All
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum OrbType {
    Lightning,
    Dark,
    Frost,
    Plasma,
    Any,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Stance {
    Calm,
    Wrath,
    Divinity,
    None,
    All,
}
impl Default for Stance {
    fn default() -> Self {
        Stance::All
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum RoomType {
    Rest,
    Shop,
    Question,
    Battle,
    HallwayFight,
    Event,
    Elite,
    Boss,
    Treasure,
    All,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Intent {
    Attack,
    AttackBuff,
    AttackDebuff,
    AttackDefend,
    Buff,
    Debuff,
    StrongDebuff,
    Defend,
    DefendDebuff,
    DefendBuff,
    Escape,
    None,
    Sleep,
    Stun,
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ChestType {
    Large,
    Medium,
    Small,
    Boss,
}

// ------------------- Evalulation -------------------------------
#[derive(PartialEq, Eq, Clone, Debug, strum_macros::ToString, Serialize, Deserialize)]
pub enum Amount {
    ByAsc{
        #[serde(rename="base")]
        amount: i16, 
        low: i16, 
        high: i16
    },
    Upgradable{
        #[serde(rename="base")]
        amount: i16, 
        upgraded: i16
    },
    Fixed(i16),
    Sum(Vec<Amount>),
    Mult(Vec<Amount>),
    X,
    NegX,
    N,
    OrbCount,
    EnemyCount,
    PlayerBlock,
    MaxHp,
    Custom
}

impl Default for Amount {
    fn default() -> Self {
        Amount::Fixed(1)
    }
}


#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
    All,
}

impl Default for CardType {
    fn default() -> Self {
        CardType::All
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum CardLocation {
    DeckPile,
    DrawPile,
    PlayerHand,
    ExhaustPile,
    DiscardPile,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Activation {
    Immediate,
    Event(Event),
    Counter {
        increment: Event,
        reset: Event,
        auto_reset: bool,
        target: u8,
    },
    Uses {
        use_when: Event,
        uses: u8,
    },
    WhenEnabled {
        //Activation is triggered before any enable/disable checks
        activated_at: Event,
        enabled_at: Event,
        disabled_at: Event,
    },
    Custom,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum MonsterSet {
    Fixed(Vec<String>),
    ChooseN{
        n: u8, 
        choices: Vec<String>
    },
    RandomSet(Vec<Vec<String>>),
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct ProbabilisticMove {
    #[serde(default="one", skip_serializing_if = "is_one")]
    pub weight: u8,
    pub name: String,
    #[serde(default="one", skip_serializing_if = "is_one")]
    pub max_repeats: u8,
}

fn one() -> u8 {
    1
}

fn is_one(weight: &u8) -> bool {
    weight == &1
}


#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Move {
    If{condition: Condition, then: Vec<Move>, _else: Vec<Move>},
    Loop(Vec<Move>),
    InOrder(String),
    Probability(Vec<ProbabilisticMove>), // Weight, name, repeats
    Event(Event),
    AfterMove(Vec<(String, Move)>),
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct MonsterMove {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<Effect>,
    pub intent: Intent,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Event {
    // Time-based
    BeforeHandDraw,
    AfterHandDraw,
    BeforeHandDiscard,
    BeforeEnemyMove, // After discarding cards
    AfterEnemyMove,
    TurnEnd, // Target-specific
    CombatEnd,
    CombatStart,

    // Targeted
    OnAttackDamage{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    OnUnblockedDamage{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    OnHpLoss{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    OnHpChange{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    OnHalfHp,
    OnBlock{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    OnDie{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    OnBuff{
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    OnUnBuff{
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },

    // Player
    Discard,
    Exhaust,
    Scry,
    Shuffle,
    StanceChange{
        #[serde(default, skip_serializing_if = "is_default")]
        from: Stance, 
        #[serde(default, skip_serializing_if = "is_default")]
        to: Stance
    },
    PlayCard(CardType),
    DrawCard(CardType),

    // Non-fight
    ChestOpen,
    CardReward,
    Rest,
    RoomEnter(RoomType),
    UsePotion,

    // Meta
    Never,
    Multiple(Vec<Event>),
    Custom,
}

impl Default for Event {
    fn default() -> Self {
        Event::Never
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum RelativePosition {
    Bottom,
    Top,
    Random,
    All, // If in a card effect, does not include the card
}

impl Default for RelativePosition {
    fn default() -> Self {
        RelativePosition::Bottom
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Effect {
    //Targeted
    Block{
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    Damage{
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    AttackDamage{
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        #[serde(default, skip_serializing_if = "is_default")]
        if_fatal: Vec<Effect>,
    },
    LoseHp{
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    Unbuff{
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    AddBuff{
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    HealPercentage{
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    DamagePercentage(Amount),
    RemoveDebuffs{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    RetainBlock,
    Die{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    EndTurn,

    AddX(Amount),
    SetX(Amount),

    SetN(Amount),
    AddN(Amount),
    ResetN,

    //Player
    Draw(Amount),
    Scry(Amount),
    AddEnergy(Amount),
    AddMaxHp(Amount),
    ReduceMaxHpPercentage(Amount),

    Heal{
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    SetStance(Stance),
    ChannelOrb(OrbType),
    AddGold(Amount),
    LoseAllGold,
    AddPotionSlot(Amount),
    AddOrbSlot(Amount),
    EvokeOrb(Amount),

    ChooseCards {
        location: CardLocation,
        then: CardEffectGroup,
        #[serde(default, skip_serializing_if = "is_default")]
        min: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        max: Amount
    },

    // Card Manipulation
    Shuffle,
    DoCardEffect{location: CardLocation, position: RelativePosition, effect: CardEffect},
    SelfEffect(CardEffect),
    CreateCard {
        name: String,
        location: CardLocation,
        #[serde(default, skip_serializing_if = "is_default")]
        position: RelativePosition,
        #[serde(default, skip_serializing_if = "is_default")]
        then: CardEffectGroup,
    },

    CreateCardByType {
        location: CardLocation,
        #[serde(rename="type")]
        _type: CardType,
        #[serde(default, skip_serializing_if = "is_default")]
        _rarity: Option<Rarity>,
        #[serde(default, skip_serializing_if = "is_default")]
        _class: Option<Class>,
        #[serde(default, skip_serializing_if = "is_default")]
        position: RelativePosition,
        #[serde(default, skip_serializing_if = "is_default")]
        then: CardEffectGroup,
    },

    ChooseCardByType {
        location: CardLocation,
        #[serde(rename="type")]
        _type: CardType,
        #[serde(default, skip_serializing_if = "is_default")]
        _rarity: Option<Rarity>,
        #[serde(default, skip_serializing_if = "is_default")]
        _class: Option<Class>,
        #[serde(default, skip_serializing_if = "is_default")]
        position: RelativePosition,
        #[serde(default, skip_serializing_if = "is_default")]
        then: CardEffectGroup,
        #[serde(default, skip_serializing_if = "is_default")]
        choices: Amount,
    },

    // Meta-scaling
    AddRelic(String),
    RandomRelic,
    ShowReward(Vec<RewardType>),
    RemoveCard(u8),
    RemoveRelic(String),
    TransformCard(u8),
    TransformRandomCard(u8),
    DuplicateCard,
    RandomPotion,
    UpgradeRandomCard(u8),
    UpgradeCard,

    // Monster
    Split(String, String),
    Spawn {
        choices: Vec<String>,
        #[serde(default, skip_serializing_if = "is_default")]
        count: Amount,
    },
    FakeDie,

    Fight{
        monsters: Vec<String>, 
        room: RoomType
    },

    ShowChoices(Vec<String>),

    //Meta
    If{
        condition: Condition, 
        #[serde(default, skip_serializing_if = "is_default")]
        then: EffectGroup, 
        #[serde(default, skip_serializing_if = "is_default")]
        _else: EffectGroup
    },
    RandomChance(Vec<EffectChance>),
    Repeat {
        n: Amount, 
        effect: EffectGroup
    },
    Custom,
}


#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct EffectChance {
    pub amount: Amount,
    pub effect: EffectGroup
}


#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum CardEffectGroup {
    Multiple(Vec<CardEffect>),
    Single(Box<CardEffect>),
    None,
}

impl Default for CardEffectGroup {
    fn default() -> Self {
        CardEffectGroup::None
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum EffectGroup {
    Multiple(Vec<Effect>),
    Single(Box<Effect>),
    None,
}
impl Default for EffectGroup {
    fn default() -> Self {
        EffectGroup::None
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum CardEffect {
    Exhaust,
    Discard,
    MoveTo{
        location: CardLocation,
        #[serde(default, skip_serializing_if = "is_default")]
        position: RelativePosition
    },
    Upgrade,
    ZeroCombatCost,
    ZeroTurnCost,
    ZeroCostUntilPlayed,
    CopyTo{
        location: CardLocation, 
        #[serde(default, skip_serializing_if = "is_default")]
        position: RelativePosition, 
        #[serde(default, skip_serializing_if = "is_default")]
        then: CardEffectGroup
    },
    AutoPlay,
    Retain,
    ReduceCost(Amount),
    Custom
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum RewardType {
    StandardCard,
    EliteCard,
    ColorlessCard,
    Relic(Rarity),
    RelicName(String),
    RandomRelic,
    RandomPotion,
    Gold{
        min:u8, 
        max:u8
    },
    RandomBook,
}

#[derive(PartialEq, Eq, Clone, Debug, strum_macros::AsStaticStr, Deserialize, Serialize)]
pub enum Condition {
    Stance(Stance),
    RemainingHp{
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    HalfHp,
    Status{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target, 
        status: String
    },
    NoBlock,
    Attacking{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    Buff{
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    BuffX{
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    }, // At least this amount
    Equals(Amount, Amount),
    LessThan(Amount, Amount),
    Asc(u8),
    Act(u8),
    Dead{
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    InPosition(usize),
    HasFriendlies(usize), //Does not include fake deaths
    Not(Box<Condition>),
    LastCard(CardType),
    HasCard{
        location: CardLocation, 
        card: CardType
    },
    Upgraded,
    HasOrbSlot,
    HasDiscarded,
    MultipleAnd(Vec<Condition>),
    MultipleOr(Vec<Condition>),
    HasRelic(String),
    HasGold(Amount),
    IsVariant(String), //Event variant
    Always,
    Class(Class),
    HasUpgradableCard,
    HasRemoveableCards{
        #[serde(default = "one", skip_serializing_if = "is_one")]
        count: u8, 
        #[serde(default, skip_serializing_if = "is_default")]
        card_type: CardType
    },
    OnFloor(u8),
    Never,
    Custom,
}

impl Condition {
    fn never() -> Self {
        Condition::Never
    }
    fn always() -> Self {
        Condition::Always
    }
    fn is_never(a: &Condition) -> bool {
        return a == &Condition::Never
    }
    fn is_always(a: &Condition) -> bool {
        return a == &Condition::Always
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Target {
    _Self,
    RandomEnemy,
    TargetEnemy,
    AllEnemies,
    Attacker,
    AnyFriendly,    // Includes self
    RandomFriendly, // Self if only remaining
    Friendly(String),
}

impl Default for Target {
    fn default() -> Self {
        Target::_Self
    }
}

#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct ProbabilisticFight {
    pub probability: u8,
    pub set: MonsterSet
}

//----------------------- Base Models ---------------------
#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Act {
    pub num: u8,
    pub easy_count: u8,
    pub easy_fights: Vec<ProbabilisticFight>,
    pub normal_fights: Vec<ProbabilisticFight>,
    pub elites: Vec<MonsterSet>,
    pub bosses: Vec<MonsterSet>,
    pub events: Vec<String>,
}
impl std::fmt::Debug for Act {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Act").field("act", &self.num).finish()
    }
}

#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct EventEffect {
    pub event: Event,
    pub effect: EffectGroup
}

#[derive(Clone, Eq, Deserialize, Serialize)]
pub struct BaseBuff {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub repeats: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub singular: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub debuff: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_add: EffectGroup,
    #[serde(default, skip_serializing_if = "is_default")]
    pub reduce_at: Event,
    #[serde(default, skip_serializing_if = "is_default")]
    pub expire_at: Event,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<EventEffect>,
}
impl std::fmt::Debug for BaseBuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseBuff")
            .field("name", &self.name)
            .finish()
    }
}
impl PartialEq for BaseBuff {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Eq, Clone, Deserialize, Serialize)]
pub struct BaseCard {
    pub name: String,
    #[serde(rename="type")]
    pub _type: CardType,
    #[serde(rename="class")]
    pub _class: Class,
    #[serde(default, skip_serializing_if = "is_default")]
    pub cost: Amount,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rarity: Rarity,
    #[serde(default = "Condition::always", skip_serializing_if = "Condition::is_always")]
    pub playable_if: Condition,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_start: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_play: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_discard: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_draw: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_exhaust: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_retain: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_turn_end: Vec<Effect>, //Happens if card is in hand, before cards are discarded
    #[serde(default = "Condition::never", skip_serializing_if = "Condition::is_never")]
    pub innate: Condition,
    #[serde(default = "Condition::never", skip_serializing_if = "Condition::is_never")]
    pub retain: Condition,
    #[serde(default = "Condition::never", skip_serializing_if = "Condition::is_never")]
    pub targeted: Condition,
}
impl std::fmt::Debug for BaseCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseCard")
            .field("name", &self.name)
            .finish()
    }
}
impl PartialEq for BaseCard {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn is_default<T>(a: &T) -> bool
    where T: Default + PartialEq<T>
{
    let def: T = Default::default();
    &def == a
}

fn _true() -> bool {
    true
}

fn is_true(a: &bool) -> bool {
    a == &true
}


#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct BaseEvent {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub choices: Vec<BaseEventChoice>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub shrine: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub variants: Vec<String>,
    #[serde(default = "Condition::always", skip_serializing_if = "Condition::is_always")]
    pub condition: Condition,
}
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct BaseEventChoice {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<Effect>,
    #[serde(default = "Condition::always", skip_serializing_if = "Condition::is_always")]
    pub condition: Condition,
    #[serde(default = "_true", skip_serializing_if = "is_true")]
    pub initial: bool,
}
impl std::fmt::Debug for BaseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseEvent")
            .field("name", &self.name)
            .finish()
    }
}
impl PartialEq for BaseEvent {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Range {
    pub min: Amount,
    pub max: Amount,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SimpleRange {
    pub min: u16,
    pub max: u16,
}

#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct BaseMonster {
    pub name: String,
    pub hp_range: SimpleRange,
    pub hp_range_asc: SimpleRange,
    pub moveset: Vec<MonsterMove>,
    pub move_order: Vec<Move>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub n_range: Option<Range>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub x_range: Option<Range>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<EventEffect>,
}
impl std::fmt::Debug for BaseMonster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseMonster")
            .field("name", &self.name)
            .finish()
    }
}
impl PartialEq for BaseMonster {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct BasePotion {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub _class: Class,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rarity: Rarity,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_drink: Vec<Effect>,
    #[serde(default = "Condition::never", skip_serializing_if = "Condition::is_never")]
    pub targeted: Condition,
}
impl std::fmt::Debug for BasePotion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BasePotion")
            .field("name", &self.name)
            .finish()
    }
}
impl PartialEq for BasePotion {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct BaseRelic {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rarity: Rarity,
    pub activation: Activation,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effect: EffectGroup,
    #[serde(default, skip_serializing_if = "is_default")]
    pub disable_at: Event,
    #[serde(default, skip_serializing_if = "is_default")]
    pub class: Class,
    #[serde(default, skip_serializing_if = "is_default")]
    pub energy_relic: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub replaces_starter: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub starting_x: i16,
}
impl std::fmt::Debug for BaseRelic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseRelic")
            .field("name", &self.name)
            .finish()
    }
}
impl PartialEq for BaseRelic {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
