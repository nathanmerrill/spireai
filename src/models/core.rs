#[derive(PartialEq)]
pub enum Orb {
    Lightning,
    Dark,
    Frost,
    Plasma
}

pub struct Status {
    pub name: &'static str,
    pub stacks: bool,
    pub is_additive: bool,
    pub is_buff: bool,
    pub starting_n: i32,
    pub reduce_at: Event,
    pub expire_at: Event,
    pub effect_at: Event,
    pub effect: Effect,
}

#[derive(PartialEq)]
pub enum Rarity {
    Starter, 
    Common, 
    Uncommon, 
    Rare, 
    Special,
    Event,
    Status, 
    Curse,
}

#[derive(PartialEq)]
pub enum Stance {
    Calm,
    Wrath,
    Divinity,
    None,
    All
}

#[derive(PartialEq)]
pub enum Class {
    All,
    None,
    Ironclad,
    Silent,
    Defect,
    Watcher,
}

#[derive(PartialEq)]
pub enum CardType {
    Attack, 
    Skill, 
    Power, 
    Status, 
    Curse, 
    All
}

fn create(){
}

#[derive(PartialEq, Clone)]
pub struct Card {
    base: &'static BaseCard,
}

#[derive(PartialEq)]
pub struct BaseCard {
    pub cost: i32, //-1 means X
    pub rarity: Rarity,
    pub _type: CardType,
    pub _class: Class,
    pub targeted: bool,
    pub effects: Vec<CardEffect>,
    pub on_upgrade: OnUpgrade,
    pub name: &'static str,   
    pub innate: bool,
    pub ethereal: bool,
    pub starting_n: i32,
}

#[derive(PartialEq)]
pub enum Amount {
    Fixed(i32),
    X,
    NegX,
    N,
    Custom,
}

#[derive(PartialEq)]
pub enum Effect {
    Block(Amount, EffectTarget),
    Damage(Amount, EffectTarget),
    AttackDamage(Amount, EffectTarget),
    LoseHp(Amount, EffectTarget),
    SetStatus(&'static str, Amount, EffectTarget),
    IncreaseStatusN(&'static str, Amount, EffectTarget),
    
    AddMantra(Amount),
    Draw(Amount),
    Scry(Amount),
    AddEnergy(Amount),
    IncreaseMaxHp(Amount),
    Heal(Amount),
    SetStance(Stance),
    ChannelOrb(Orb),

    ExhaustCard(CardLocation),
    DiscardCard(CardLocation),
    MoveCard(CardLocation, CardLocation),

    AddCard{
        card: CardReference, 
        destination: CardLocation, 
        copies: Amount,
        modifier: CardModifier
    },
    
    UpgradeCard(CardLocation),
    AutoPlayCard(CardLocation),

    IfStance(Stance, Vec<Effect>),
    IfStatus(EffectTarget, &'static str, Vec<Effect>),
    IfAttacking(EffectTarget, Vec<Effect>),
    IfStatusN(EffectTarget, &'static str, Amount, Vec<Effect>),

    Multiple(Vec<Effect>),
    Repeat(Amount, Box<Effect>),
    None,
    Custom,
}

pub enum Event {
    TurnStart,
    TurnEnd,
    OnAttackDamage,
    OnUnblockedDamage,
    OnTargetUnblockedDamage,
    OnHpLoss,
    OnBlock,
    OnDiscard,
    OnExhaust,
    OnScry,
    OnCombatEnd,
    OnStanceChange(Stance, Stance),
    Never,
    OnCard(CardType),
    OnDraw(CardType),
    Multiple(Vec<Event>),
}

#[derive(PartialEq)]
pub enum EffectTarget {
    _Self,
    RandomEnemy,
    TargetEnemy,
    AllEnemies,
    Attacker,
}

#[derive(PartialEq)]
pub enum CardModifier {
    None,
    SetZeroCombatCost,
    SetZeroTurnCost,
    SetZeroCostUntilPlayed,
    Upgraded,
}

#[derive(PartialEq)]
pub enum CardReference {
    ByName(&'static str),
    CopyOf(CardLocation),
    RandomType(CardType),
    RandomRarity(Rarity),
    RandomClass(Class),
}

#[derive(PartialEq)]
pub enum RelativePosition {
    Bottom,
    Top,
    Random,
    PlayerChoice(i32),
    All,
}

#[derive(PartialEq)]
pub enum CardLocation {
    This,
    DeckPile,
    DrawPile(RelativePosition),
    PlayerHand(RelativePosition),
    ExhaustPile(RelativePosition),
    DiscardPile(RelativePosition),
}

#[derive(PartialEq)]
pub enum CardEffect {
    OnPlay(Effect),
    OnDraw(Effect),
    OnDiscard(Effect),
    OnExhaust(Effect),

    RepeatX(Vec<Effect>),
    CustomCardCost,
    CustomPlayable,
    IfFatal(Vec<Effect>),
}

#[derive(PartialEq)]
pub enum OnUpgrade {
    SetEffects(Vec<CardEffect>),
    ReduceCost(i32),
    SearingBlow,
    Custom,
    Burn,
    Unupgradable,
    Innate,
    RemoveEthereal,
    None
}