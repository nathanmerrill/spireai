use strum_macros::Display;

#[derive(PartialEq, Clone, Debug)]
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

#[derive(PartialEq, Clone, Debug, Display)]
pub enum Class {
    All,
    None,
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Amount {
    Fixed(i16),
    X,
    NegX,
    N,
    OrbCount,
    Custom,
    EnemyCount,
    Any,
    ByAsc(i16, i16, i16),
    Upgradable(i16, i16),
    Sum(Vec<Amount>),
    Mult(Vec<Amount>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Orb {
    Lightning,
    Dark,
    Frost,
    Plasma,
    Any,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Stance {
    Calm,
    Wrath,
    Divinity,
    None,
    All,
}

// Cards

#[derive(PartialEq, Clone, Debug)]
pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
    ByName(&'static str),
    All,
}

#[derive(PartialEq, Clone, Debug)]
pub struct BaseCard {
    pub cost: Amount,
    pub rarity: Rarity,
    pub _type: CardType,
    pub _class: Class,
    pub playable_if: Condition,
    pub effects: Vec<(Event, Effect)>,
    pub on_play: Vec<Effect>,
    pub on_discard: Vec<Effect>,
    pub on_draw: Vec<Effect>,
    pub on_exhaust: Vec<Effect>,
    pub on_retain: Vec<Effect>,
    pub on_turn_end: Vec<Effect>, //Happens if card is in hand, before cards are discarded
    pub name: &'static str,
    pub innate: Condition,
    pub upgradeable: Upgradable,
    pub retain: Condition,
    pub removable: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Upgradable {
    Never,
    Once,
    Infinite,
    Burn,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CardModifier {
    None,
    SetZeroCombatCost,
    SetZeroTurnCost,
    SetZeroCostUntilPlayed,
    Upgraded,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CardReference {
    ByName(&'static str),
    CopyOf(CardLocation),
    RandomType(CardType),
    RandomRarity(Rarity),
    RandomClass(Class),
}

#[derive(PartialEq, Clone, Debug)]
pub enum CardLocation {
    This,
    DeckPile(RelativePosition),
    DrawPile(RelativePosition),
    PlayerHand(RelativePosition),
    ExhaustPile(RelativePosition),
    DiscardPile(RelativePosition),
}

// Buffs

#[derive(PartialEq, Clone, Debug)]
pub struct BaseBuff {
    pub name: &'static str,
    pub stacks: bool,
    pub is_additive: bool,
    pub is_buff: bool,
    pub on_add: Effect,
    pub reduce_at: Event,
    pub expire_at: Event,
    pub effects: Vec<(Event, Effect)>,
}

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

// Relics
pub struct BaseRelic {
    pub name: &'static str,
    pub rarity: Rarity,
    pub activation: Activation,
    pub effect: Effect,
    pub disable_at: Event,
    pub class: Class,
    pub energy_relic: bool,
    pub replaces_starter: bool,
}

pub struct Act {
    easy_count: u8,
    easy_fights: Vec<(u8, MonsterSet)>,
    normal_fights: Vec<(u8, MonsterSet)>,
    elites: Vec<MonsterSet>,
    bosses: Vec<MonsterSet>,
}

pub enum MonsterSet {
    Fixed(Vec<&'static str>),
    ChooseN(u8, Vec<&'static str>),
    RandomSet(Vec<Vec<&'static str>>),
}

pub struct BaseMonster {
    pub name: &'static str,
    pub hp_range: (u16, u16),
    pub hp_range_asc: (u16, u16),
    pub moveset: Vec<MonsterMove>,
    pub move_order: Vec<Move>,
    pub n_range: (Amount, Amount),
    pub x_range: (Amount, Amount),
    pub effects: Vec<(Event, Effect)>,
}

pub enum Move {
    If(Condition, Vec<Move>, Vec<Move>),
    Loop(Vec<Move>),
    InOrder(&'static str),
    Probability(Vec<(u8, &'static str, u8)>), // Weight, name, repeats
    Event(Event),
    AfterMove(Vec<(&'static str, Move)>),
}

pub struct ProbabilisticMove {
    pub chance: Amount,
    pub move_index: u8,
    pub max_repeats: Amount,
    pub starter_asc: Option<u8>,
}

pub enum Intent {
    Attack,
    AttackBuff,
    AttackDebuff,
    AttackDefend,
    Buff,
    Debuff,
    StrongDebuff,
    Debug,
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

pub struct MonsterMove {
    pub name: &'static str,
    pub effects: Vec<Effect>,
    pub intent: Intent,
}

// Rooms
#[derive(PartialEq, Clone, Debug)]
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

// Events

#[derive(PartialEq, Clone, Debug)]
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
    Damage(Target),
    AttackDamage(Target),
    UnblockedDamage(Target),
    HpLoss(Target),
    HpChange(Target),
    HalfHp(Target),
    Heal(Target),
    Block(Target),
    Die(Target),
    Buff(&'static str, Target),
    UnBuff(&'static str, Target),
    Channel(Orb),

    // Monster
    Move(&'static str),

    // Player
    Discard,
    Exhaust,
    Scry,
    Shuffle,
    StanceChange(Stance, Stance),
    PlayCard(CardType),
    DrawCard(CardType),

    // Non-fight
    ChestOpen,
    CardReward,
    AddToDeck(CardType),
    Rest,
    RoomEnter(RoomType),
    SpendGold,
    GainGold,
    UsePotion,

    // Meta
    Never,
    Multiple(Vec<Event>),
    Custom,
}

#[derive(PartialEq, Clone, Debug)]
pub enum RelativePosition {
    Bottom,
    Top,
    Random,
    PlayerChoice(Amount),
    All, // If in a card effect, does not include the card
}

// Effects
#[derive(PartialEq, Clone, Debug)]
pub enum Effect {
    //Targeted
    Block(Amount, Target),
    Damage(Amount, Target),
    AttackDamage(Amount, Target),
    AttackDamageIfUnblocked(Amount, Target, Vec<Effect>), // N is set to amount unblocked
    AttackDamageIfFatal(Amount, Target, Vec<Effect>),
    LoseHp(Amount, Target),
    Unbuff(&'static str, Target),
    AddBuff(&'static str, Amount, Target),
    LoseStr(Amount, Target),
    HealPercentage(u8, Target),
    RemoveDebuffs(Target),
    Die(Target),
    EndTurn,

    AddX(Amount),

    SetN(Amount),
    AddN(Amount),
    ResetN,

    //Player
    Draw(Amount),
    Scry(Amount),
    AddEnergy(Amount),
    AddMaxHp(Amount),
    Heal(Amount, Target),
    SetStance(Stance),
    ChannelOrb(Orb),
    AddGold(Amount),
    AddPotionSlot(Amount),
    AddOrbSlot(Amount),
    EvokeOrb(Amount),

    // Card Manipulation
    ExhaustCard(CardLocation),
    DiscardCard(CardLocation),
    Shuffle,
    MoveCard(CardLocation, CardLocation, CardModifier),
    SetCardModifier(CardLocation, CardModifier),
    AddCard {
        card: CardReference,
        destination: CardLocation,
        copies: Amount,
        modifier: CardModifier,
    },
    UpgradeCard(CardLocation),
    AutoPlayCard(CardLocation),
    SetCardCost(CardLocation, Amount),

    // Meta-scaling
    CardReward,
    AddRelic,
    ShowReward {
        potions: i8,
        cards: i8,
        gold: i8,
        relics: i8,
    },

    // Monster
    Split(&'static str, &'static str),
    Spawn {
        choices: Vec<&'static str>,
        count: Amount,
    },

    //Meta
    If(Condition, Vec<Effect>, Vec<Effect>),
    Multiple(Vec<Effect>),
    Repeat(Amount, Box<Effect>),
    None,
    Custom,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Condition {
    Stance(Stance),
    MissingHp(Amount, Target),
    RemainingHp(Amount, Target),
    HalfHp(Target),
    Status(Target, &'static str),
    NoBlock(Target),
    Attacking(Target),
    Buff(Target, &'static str),
    BuffX(Target, &'static str, Amount), // At least this amount
    Equals(Amount, Amount),
    LessThan(Amount, Amount),
    Asc(u8),
    Act(u8),
    Dead(Target),
    InPosition(Target, u8),
    HasFriendlies(u8),
    Not(Box<Condition>),
    LastCard(CardType),
    HasCard(CardLocation, CardType),
    Upgraded,
    HasOrbSlot,
    HasDiscarded,
    Always,
    Never,
    Custom,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Target {
    _Self,
    RandomEnemy,
    TargetEnemy,
    AllEnemies,
    Attacker,
    AnyFriendly,    // Includes self
    RandomFriendly, // Self if only remaining
    Friendly(&'static str),
}
