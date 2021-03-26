// ------------------  Fundamental types  -------------------------

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone, strum_macros::Display)]
pub enum Class {
    All,
    None,
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OrbType {
    Lightning,
    Dark,
    Frost,
    Plasma,
    Any,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stance {
    Calm,
    Wrath,
    Divinity,
    None,
    All,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

pub struct Act {
    pub easy_count: u8,
    pub easy_fights: Vec<(u8, MonsterSet)>,
    pub normal_fights: Vec<(u8, MonsterSet)>,
    pub elites: Vec<MonsterSet>,
    pub bosses: Vec<MonsterSet>,
}

// ------------------- Evalulation -------------------------------
pub enum Amount {
    Fixed(i16),
    X,
    NegX,
    N,
    OrbCount,
    Custom,
    EnemyCount,
    Any,
    PlayerBlock,
    MaxHp,
    ByAsc(i16, i16, i16),
    Upgradable(i16, i16),
    Sum(Vec<Amount>),
    Mult(Vec<Amount>),
}

pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
    ByName(&'static str),
    All,
}

pub enum Upgradable {
    Never,
    Once,
    Infinite,
    Burn,
}

pub enum CardModifier {
    None,
    SetZeroCombatCost,
    SetZeroTurnCost,
    SetZeroCostUntilPlayed,
    Upgraded,
}

pub enum CardReference {
    ByName(&'static str),
    CopyOf(CardLocation),
    RandomType(CardType, Amount), // Num choices
    RandomRarity(Rarity),
    RandomClass(Class),
}

pub enum CardLocation {
    This,
    DeckPile(RelativePosition),
    DrawPile(RelativePosition),
    PlayerHand(RelativePosition),
    ExhaustPile(RelativePosition),
    DiscardPile(RelativePosition),
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

pub enum MonsterSet {
    Fixed(Vec<&'static str>),
    ChooseN(u8, Vec<&'static str>),
    RandomSet(Vec<Vec<&'static str>>),
}

pub enum Move {
    If(Condition, Vec<Move>, Vec<Move>),
    Loop(Vec<Move>),
    InOrder(&'static str),
    Probability(Vec<(u8, &'static str, u8)>), // Weight, name, repeats
    Event(Event),
    AfterMove(Vec<(&'static str, Move)>),
}

pub struct MonsterMove {
    pub name: &'static str,
    pub effects: Vec<Effect>,
    pub intent: Intent,
}


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
    OrbDamage(Target),
    HpLoss(Target),
    HpChange(Target),
    HalfHp(Target),
    Heal(Target),
    Block(Target),
    Die(Target),
    AnyBuff(Target),
    Buff(&'static str, Target),
    UnBuff(&'static str, Target),
    Channel(OrbType),

    // Player
    Discard,
    Exhaust,
    Scry,
    Shuffle,
    StanceChange(Stance, Stance),
    PlayCard(CardType), // Sets This to be the card played
    DrawCard(CardType), // Sets This to be the card drawn
    RetainCard(CardType), // Sets This to be the card retained

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

pub enum RelativePosition {
    Bottom,
    Top,
    Random,
    PlayerChoice(Amount),
    All, // If in a card effect, does not include the card
}

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
    HealPercentage(Amount, Target),
    DamagePercentage(Amount),
    RemoveDebuffs(Target),
    RetainBlock(Amount),
    Die(Target),
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
    
    Heal(Amount, Target),
    SetStance(Stance),
    ChannelOrb(OrbType),
    AddGold(Amount),
    LoseAllGold,
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
    AddCardCost(CardLocation, Amount),
    RandomizeCost(CardLocation),
    RetainCard(CardLocation),

    // Meta-scaling
    AddRelic(&'static str),
    RandomRelic,
    ShowReward(Vec<RewardType>),
    RemoveCard(u8),
    RemoveRelic(&'static str),
    TransformCard(u8),
    TransformRandomCard(u8),
    DuplicateCard,
    RandomPotion,

    // Monster
    Split(&'static str, &'static str),
    Spawn {
        choices: Vec<&'static str>,
        count: Amount,
    },
    FakeDie,
    
    // Event-related
    Duplicate,
    Boost(Amount),
    BoostMult(Amount), // Adds/subtracts to the multiplier. In percentage units
    Cap(Amount),
    Cancel,
    Fight(Vec<&'static str>, bool), // True if elite
    ShowChoices(Vec<BaseEventChoice>),

    //Meta
    If(Condition, Vec<Effect>, Vec<Effect>),
    RandomChance(Vec<(Amount, Effect)>),
    Multiple(Vec<Effect>),
    Repeat(Amount, Box<Effect>),
    None,
    Custom,
}

pub enum RewardType {
    StandardCard,
    EliteCard,
    BossCard,
    Relic(Rarity),
    RelicName(&'static str),
    RandomRelic,
    PotionChance,
    RandomPotion,
    Gold(u8, u8),
    RandomBook,
}

#[derive(strum_macros::AsStaticStr)]
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
    HasFriendlies(u8), //Does not include fake deaths
    Not(Box<Condition>),
    LastCard(CardType),
    HasCard(CardLocation, CardType),
    Upgraded,
    HasOrbSlot,
    HasDiscarded,
    MultipleAnd(Vec<Condition>),
    MultipleOr(Vec<Condition>),
    DeckSize(u8),
    HasRelic(&'static str),
    HasGold(Amount),
    IsVariant(&'static str),  //Event variant
    Always,
    Class(Class),
    HasUpgradableCard,
    HasRemoveableCards(u8, CardType),
    Never,
    Custom,
}

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

//----------------------- Base Models ---------------------

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
impl std::fmt::Debug for BaseBuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseBuff")
            .field("name", &self.name)
            .finish()
    }
}

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
    pub targeted: Condition,
}
impl std::fmt::Debug for BaseCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseCard")
            .field("name", &self.name)
            .finish()
    }
}


pub struct BaseEvent {
    pub name: &'static str,
    pub choices: Vec<BaseEventChoice>,
    pub shrine: bool,
    pub variants: Vec<&'static str>,
}

pub struct BaseEventChoice {
    pub name: &'static str,
    pub effects: Vec<Effect>,
    pub condition: Condition,
    pub repeats: bool,
}


impl std::fmt::Debug for BaseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseEvent")
            .field("name", &self.name)
            .finish()
    }
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
impl std::fmt::Debug for BaseMonster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseMonster")
            .field("name", &self.name)
            .finish()
    }
}


pub struct BasePotion {
    pub name: &'static str,
    pub _class: Class,
    pub rarity: Rarity,
    pub on_drink: Vec<Effect>,
    pub targeted: Condition,
}
impl std::fmt::Debug for BasePotion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BasePotion")
            .field("name", &self.name)
            .finish()
    }
}


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
impl std::fmt::Debug for BaseRelic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseRelic")
            .field("name", &self.name)
            .finish()
    }
}