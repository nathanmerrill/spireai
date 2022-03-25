use serde::{Deserialize, Serialize};

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

#[derive(
    PartialEq, Eq, Clone, Copy, Debug, Hash, strum_macros::Display, Deserialize, Serialize,
)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum OrbType {
    Lightning,
    Dark,
    Frost,
    Plasma,
    Any,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize, Hash)]
pub enum FightType {
    Common,
    Elite {
        #[serde(default, skip_serializing_if = "is_default")]
        burning: bool 
    },
    Boss,
}

impl Default for FightType {
    fn default() -> Self {
        FightType::Common
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize, Hash)]
pub enum RoomType {
    Rest,
    Shop,
    Question,
    Fight(FightType),
    Event,
    Treasure,
    All,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ChestType {
    Large,
    Medium,
    Small,
    Boss,
}

// ------------------- Evalulation -------------------------------
#[derive(PartialEq, Eq, Hash, Clone, Debug, strum_macros::ToString, Serialize, Deserialize)]
pub enum Amount {
    ByAsc {
        #[serde(rename = "base")]
        amount: i16,
        low: i16,
        high: i16,
    },
    Upgradable {
        #[serde(rename = "base")]
        amount: i16,
        upgraded: i16,
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
    Custom,
}

impl Default for Amount {
    fn default() -> Self {
        Amount::Fixed(1)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum CardDestination {
    DrawPile(RelativePosition),
    PlayerHand,
    ExhaustPile,
    DiscardPile,
}

impl CardDestination {
    pub fn location(self) -> CardLocation {
        match self {
            CardDestination::DrawPile(_) => CardLocation::DrawPile,
            CardDestination::PlayerHand => CardLocation::PlayerHand,
            CardDestination::ExhaustPile => CardLocation::ExhaustPile,
            CardDestination::DiscardPile => CardLocation::DiscardPile,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize, Hash)]
pub enum CardLocation {
    DrawPile,
    PlayerHand,
    ExhaustPile,
    DiscardPile,
    None, // Only for cards that are to be picked
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize, Hash)]
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

impl CardType {
    pub fn matches(self, other: CardType) -> bool {
        self == CardType::All || self == other
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize, Hash)]
pub enum When {
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
    OnRecieveAttackDamage,
    OnReceiveUnblockedDamage,
    OnDealUnblockedDamage,
    OnHpLoss,
    OnHpChange,
    OnHalfHp,
    OnBlock,
    OnDie,
    OnEnemyDie,

    // Player
    Discard,
    Exhaust,
    Scry,
    Shuffle,
    PlayCard(CardType),
    DrawCard(CardType),

    // Enemy
    OnMove(String),
    OnLoseBuff(String),

    // Non-fight
    Rest,
    RoomEnter(RoomType),
    UsePotion,

    // Meta
    Never,
}

impl Default for When {
    fn default() -> Self {
        When::Never
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum DeckOperation {
    Upgrade,
    Transform,
    TransformUpgrade,
    Remove,
    Duplicate,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum EventEffect {
    ShowReward(Vec<RewardType>),
    RemoveRelic(String),
    RandomRelic,
    AddRelic(String),
    Fight {
        monsters: Vec<String>,
        room: FightType,
    },
    GameEffect(GameEffect),
    AddPotionSlot(u8),
    Custom,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum GameEffect {
    DeckOperation {
        #[serde(default, skip_serializing_if = "is_default")]
        random: bool,
        #[serde(default = "one", skip_serializing_if = "is_one")]
        count: u8,
        operation: DeckOperation,
    },
    DeckAdd(String),
    RandomPotion,
    LoseHp(Amount),
    LoseHpPercentage(Amount),
    Heal(Amount),
    AddMaxHp(Amount),
    ReduceMaxHpPercentage(Amount),    
    AddGold(Amount),
    LoseAllGold,
}


#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum BattleEffect {
    //Targeted
    Block {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    Damage {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    AttackDamage {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        #[serde(default, skip_serializing_if = "is_default")]
        times: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        if_fatal: Vec<BattleEffect>,
    },
    LoseHp {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    Unbuff(String),
    AddBuff {
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    HealPercentage {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    RemoveDebuffs,
    Die {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },

    AddX(Amount),
    SetX(Amount),

    SetN(Amount),
    AddN(Amount),
    ResetN,

    //Player
    Draw(Amount),
    Scry(Amount),
    AddEnergy(Amount),

    Heal {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    SetStance(Stance),
    ChannelOrb(OrbType),
    AddOrbSlot(Amount),
    EvokeOrb(Amount),

    ChooseCards {
        location: CardLocation,
        then: Vec<CardEffect>,
        #[serde(default, skip_serializing_if = "is_default")]
        min: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        max: Amount,
    },

    // Card Manipulation
    Shuffle,
    DoCardEffect {
        location: CardLocation,
        position: RelativePosition,
        effect: CardEffect,
    },
    SelfEffect(CardEffect),
    CreateCard {
        name: String,
        destination: CardDestination,
        #[serde(default, skip_serializing_if = "is_default")]
        then: Vec<CardEffect>,
    },

    CreateCardByType {
        destination: CardDestination,
        #[serde(rename = "type")]
        _type: CardType,
        #[serde(default, skip_serializing_if = "is_default")]
        rarity: Option<Rarity>,
        #[serde(default, skip_serializing_if = "is_default")]
        class: Option<Class>,
        #[serde(default, skip_serializing_if = "is_default")]
        then: Vec<CardEffect>,
        #[serde(default = "_true", skip_serializing_if = "is_true")]
        exclude_healing: bool,
    },

    ChooseCardByType {
        destination: CardDestination,
        #[serde(rename = "type")]
        _type: CardType,
        #[serde(default, skip_serializing_if = "is_default")]
        rarity: Option<Rarity>,
        #[serde(default, skip_serializing_if = "is_default")]
        class: Option<Class>,
        #[serde(default, skip_serializing_if = "is_default")]
        then: Vec<CardEffect>,
        #[serde(default, skip_serializing_if = "is_default")]
        choices: Amount,
        #[serde(default = "_true", skip_serializing_if = "is_true")]
        exclude_healing: bool,
    },

    GameEffect(GameEffect),

    // Monster
    Split(String, String),
    Spawn {
        choices: Vec<String>,
        #[serde(default, skip_serializing_if = "is_default")]
        count: Amount,
    },


    ShowChoices(Vec<String>),

    //Control Structures
    If {
        condition: Condition,
        #[serde(default, skip_serializing_if = "is_default")]
        then: Vec<BattleEffect>,
        #[serde(default, skip_serializing_if = "is_default")]
        _else: Vec<BattleEffect>,
    },
    RandomChance(Vec<EffectChance>),
    Repeat {
        n: Amount,
        effect: Vec<BattleEffect>,
    },
    Custom,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct EffectChance {
    pub amount: Amount,
    pub effect: Vec<BattleEffect>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
pub enum CardEffect {
    Exhaust,
    Discard,
    MoveTo(CardDestination),
    Upgrade,
    ZeroCombatCost,
    ZeroTurnCost,
    ZeroCostUntilPlayed,
    CopyTo {
        destination: CardDestination,
        #[serde(default, skip_serializing_if = "is_default")]
        then: Vec<CardEffect>,
    },
    Scry,
    AutoPlay,
    Retain,
    ReduceCost(Amount),
    Custom,
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
    Gold { min: u16, max: u16 },
    RandomBook,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Target {
    _Self,
    Player,
    RandomEnemy,
    OneEnemy,
    AllEnemies,
    Attacker,
    AnyFriendly,    // Includes self
    RandomFriendly, // Self if only remaining
}

impl Default for Target {
    fn default() -> Self {
        Target::_Self
    }
}

#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct WhenEffect {
    pub when: When,
    pub effect: Vec<BattleEffect>,
}

#[derive(PartialEq, Eq, Clone, Debug, strum_macros::AsStaticStr, Deserialize, Serialize)]
pub enum Condition {
    Stance(Stance),
    RemainingHp {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    HalfHp,
    NoBlock,
    Attacking {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    Buff {
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    BuffX {
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
    FriendlyDead(String),
    InPosition(usize),
    HasFriendlies(usize), //Does not include fake deaths
    Not(Box<Condition>),
    LastCard(CardType),
    HasCard {
        location: CardLocation,
        card: CardType,
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
    HasRemoveableCards {
        #[serde(default = "one", skip_serializing_if = "is_one")]
        count: u8,
        #[serde(default, skip_serializing_if = "is_default")]
        card_type: CardType,
    },
    OnFloor(u8),
    Never,
    Custom,
}

impl Condition {
    pub fn never() -> Self {
        Condition::Never
    }
    pub fn always() -> Self {
        Condition::Always
    }
    pub fn is_never(a: &Condition) -> bool {
        a == &Condition::Never
    }
    pub fn is_always(a: &Condition) -> bool {
        a == &Condition::Always
    }
}

pub fn is_default<T>(a: &T) -> bool
where
    T: Default + PartialEq<T>,
{
    let def: T = Default::default();
    &def == a
}

pub fn _true() -> bool {
    true
}

pub fn is_true(a: &bool) -> bool {
    a == &true
}

pub fn one() -> u8 {
    1
}

pub fn is_one(weight: &u8) -> bool {
    weight == &1
}
