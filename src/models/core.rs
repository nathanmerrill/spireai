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

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize, Hash)]
pub enum RoomType {
    Rest,
    Shop,
    Question,
    HallwayFight,
    Event,
    Elite,
    Boss,
    Treasure,
    All,
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

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum CardLocation {
    DeckPile,
    DrawPile,
    PlayerHand,
    ExhaustPile,
    DiscardPile,
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

    // Non-fight
    Rest,
    RoomEnter(RoomType),
    UsePotion,

    // Meta
    Never,
    Multiple(Vec<When>),
}

impl Default for When {
    fn default() -> Self {
        When::Never
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
        if_fatal: EffectGroup,
    },
    LoseHp {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    Unbuff {
        buff: String,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
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
    DamagePercentage(Amount),
    RemoveDebuffs {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    RetainBlock,
    Die {
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

    Heal {
        #[serde(default, skip_serializing_if = "is_default")]
        amount: Amount,
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
    SetStance(Stance),
    ChannelOrb(OrbType),
    AddGold(Amount),
    LoseAllGold,
    AddPotionSlot(u8),
    AddOrbSlot(Amount),
    EvokeOrb(Amount),

    ChooseCards {
        location: CardLocation,
        then: CardEffectGroup,
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
        location: CardLocation,
        #[serde(default, skip_serializing_if = "is_default")]
        position: RelativePosition,
        #[serde(default, skip_serializing_if = "is_default")]
        then: CardEffectGroup,
    },

    CreateCardByType {
        location: CardLocation,
        #[serde(rename = "type")]
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
        #[serde(rename = "type")]
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

    Fight {
        monsters: Vec<String>,
        room: RoomType,
    },

    ShowChoices(Vec<String>),

    //Meta
    If {
        condition: Condition,
        #[serde(default, skip_serializing_if = "is_default")]
        then: EffectGroup,
        #[serde(default, skip_serializing_if = "is_default")]
        _else: EffectGroup,
    },
    RandomChance(Vec<EffectChance>),
    Repeat {
        n: Amount,
        effect: EffectGroup,
    },
    Custom,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct EffectChance {
    pub amount: Amount,
    pub effect: EffectGroup,
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
    MoveTo {
        location: CardLocation,
        #[serde(default, skip_serializing_if = "is_default")]
        position: RelativePosition,
    },
    Upgrade,
    ZeroCombatCost,
    ZeroTurnCost,
    ZeroCostUntilPlayed,
    CopyTo {
        location: CardLocation,
        #[serde(default, skip_serializing_if = "is_default")]
        position: RelativePosition,
        #[serde(default, skip_serializing_if = "is_default")]
        then: CardEffectGroup,
    },
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
    Gold { min: u8, max: u8 },
    RandomBook,
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
#[derive(PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct WhenEffect {
    pub when: When,
    pub effect: EffectGroup,
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
    Status {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
        status: String,
    },
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
    Dead {
        #[serde(default, skip_serializing_if = "is_default")]
        target: Target,
    },
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