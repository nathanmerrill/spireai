#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
pub enum Class {
    All,
    None,
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

#[derive(PartialEq, Clone)]
pub enum Amount {
    Fixed(i16),
    X,
    NegX,
    N,
    Custom,
}

#[derive(PartialEq, Clone)]
pub enum Orb {
    Lightning,
    Dark,
    Frost,
    Plasma
}

#[derive(PartialEq, Clone)]
pub enum Stance {
    Calm,
    Wrath,
    Divinity,
    None,
    All
}

// Cards

#[derive(PartialEq, Clone)]
pub enum CardType {
    Attack, 
    Skill, 
    Power, 
    Status, 
    Curse, 
    All
}

#[derive(PartialEq, Clone)]
pub struct BaseCard {
    pub cost: i8, //-1 means X
    pub rarity: Rarity,
    pub _type: CardType,
    pub _class: Class,
    pub targeted: bool,
    pub effects: Vec<CardEffect>,
    pub on_upgrade: OnUpgrade,
    pub name: &'static str,   
    pub innate: bool,
    pub ethereal: bool,
    pub starting_n: u8,
}

#[derive(PartialEq, Clone)]
pub enum CardModifier {
    None,
    SetZeroCombatCost,
    SetZeroTurnCost,
    SetZeroCostUntilPlayed,
    Upgraded,
}

#[derive(PartialEq, Clone)]
pub enum CardReference {
    ByName(&'static str),
    CopyOf(CardLocation),
    RandomType(CardType),
    RandomRarity(Rarity),
    RandomClass(Class),
}

#[derive(PartialEq, Clone)]
pub enum CardLocation {
    This,
    DeckPile(RelativePosition),
    DrawPile(RelativePosition),
    PlayerHand(RelativePosition),
    ExhaustPile(RelativePosition),
    DiscardPile(RelativePosition),
}

// Buffs

#[derive(PartialEq, Clone)]
pub struct BaseBuff {
    pub name: &'static str,
    pub stacks: bool,
    pub is_additive: bool,
    pub is_buff: bool,
    pub starting_n: u8,
    pub reduce_at: Event,
    pub expire_at: Event,
    pub effect_at: Event,
    pub effect: Effect,
}

pub enum Activation {
    Immediate,
    Event(Event),
    Counter{
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
    Custom
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

// Monsters
pub struct MonsterSet {
    pub act: u8,
    pub monsters: Vec<BaseMonster>
}

pub struct BaseMonster {
    pub name: &'static str,
    pub hp_min: u16,
    pub hp_max: u16,
    pub moveset: MonsterMoveSet,
    pub buffs: Mon
}

pub struct MonsterMoveSet {
    starting_node: MonsterMoveNode,
    event: Event

}

pub struct MonsterMoveNode {
    _move: MonsterMove,
    next: Vec<(MonsterMoveNode, i8)>
}

pub enum MonsterMove {
    Attack(i8, i8),

}

// Rooms
#[derive(PartialEq, Clone)]
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

#[derive(PartialEq, Clone)]
pub enum Event {
    // Time-based
    TurnStart,
    TurnEnd,
    CombatEnd,
    CombatStart,

    // Targeted
    AttackDamage(EffectTarget),
    UnblockedDamage(EffectTarget),
    HpLoss(EffectTarget),
    HpChange(EffectTarget),
    Heal(EffectTarget),
    Block(EffectTarget),
    Die(EffectTarget),
    Buff(&'static str, EffectTarget),
    UnBuff(&'static str, EffectTarget),

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

#[derive(PartialEq, Clone)]
pub enum RelativePosition {
    Bottom,
    Top,
    Random,
    PlayerChoice(u8),
    All,
}

// Effects
#[derive(PartialEq, Clone)]
pub enum Effect {

    //Targeted
    Block(Amount, EffectTarget),
    Damage(Amount, EffectTarget),
    AttackDamage(Amount, EffectTarget),
    LoseHp(Amount, EffectTarget),
    AddBuff(&'static str, Amount, EffectTarget),
    AddBuffN(&'static str, Amount, EffectTarget),
    
    //Player
    Draw(Amount),
    Scry(Amount),
    AddEnergy(Amount),
    AddMaxHp(Amount),
    Heal(Amount),
    HealPercentage(u8),
    SetStance(Stance),
    ChannelOrb(Orb),
    AddGold(Amount),
    AddPotionSlot(Amount),
    AddOrbSlot(Amount),

    // Card Manipulation
    ExhaustCard(CardLocation),
    DiscardCard(CardLocation),
    MoveCard(CardLocation, CardLocation),
    SetCardModifier(CardLocation, CardModifier),
    AddCard{
        card: CardReference, 
        destination: CardLocation, 
        copies: Amount,
        modifier: CardModifier
    },
    UpgradeCard(CardLocation),
    AutoPlayCard(CardLocation),

    // Meta-scaling
    CardReward,
    AddRelic,
    ShowReward {
        potions: i8,
        cards: i8,
        gold: i8,  
        relics: i8,
    },

    //Conditionals
    IfStance(Stance, Vec<Effect>),
    IfHalfHp(EffectTarget, Vec<Effect>),
    IfStatus(EffectTarget, &'static str, Vec<Effect>),
    IfNoBlock(EffectTarget, Vec<Effect>),
    IfAttacking(EffectTarget, Vec<Effect>),
    IfBuffN(EffectTarget, &'static str, Amount, Vec<Effect>),

    // Event-based
    Cancel,
    Multiply(Amount),
    Add(Amount),
    

    //Meta
    Multiple(Vec<Effect>),
    Repeat(Amount, Box<Effect>),
    None,
    Custom,
}

#[derive(PartialEq, Clone)]
pub enum EffectTarget {
    _Self,
    RandomEnemy,
    TargetEnemy,
    AllEnemies,
    Attacker,
}

#[derive(PartialEq, Clone)]
pub enum OnUpgrade {
    SetEffects(Vec<CardEffect>),
    ReduceCost(u8),
    SearingBlow,
    Custom,
    Burn,
    Unupgradable,
    Innate,
    RemoveEthereal,
    None
}

#[derive(PartialEq, Clone)]
pub enum CardEffect {
    OnPlay(Effect),
    OnDraw(Effect),
    OnDiscard(Effect),
    OnExhaust(Effect),
    
    CustomCardCost,
    CustomPlayable,
    IfFatal(Vec<Effect>),
}