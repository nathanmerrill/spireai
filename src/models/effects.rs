use crate::models::GameState;
use crate::models::cards::{CardType, CardRarity, CardClass}; 
use crate::spireai::calculator::GamePossibilitySet;

pub enum Effect {
    Block(i32, EffectTarget),
    Damage(i32, EffectTarget),
    AttackDamage(i32, EffectTarget),
    LoseHp(i32, EffectTarget),
    SetStatus(&'static str, i32, EffectTarget),
    AddMantra(i32),
    Draw(i32),
    Scry(i32),
    AddEnergy(i32),
    IncreaseMaxHp(i32),
    Heal(i32),
    SetStance(Stance),
    ChannelOrb(Orb),

    ExhaustCard(CardLocation),
    DiscardCard(CardLocation),
    MoveCard(CardLocation, CardLocation),

    AddCard{
        card: CardReference, 
        destination: CardLocation, 
        copies: i32,
        modifier: CardModifier
    },
    
    UpgradeCard(CardLocation),
    AutoPlayCard(CardLocation),

    IfStance(Stance, Vec<Effect>),
    IfStatus(EffectTarget, &'static str, Vec<Effect>),
    IfAttacking(EffectTarget, Vec<Effect>),
    
    Custom(fn(&GameState) -> GamePossibilitySet),

    Multiple(Vec<Effect>),
    None,
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

pub enum Orb {
    Lightning,
    Dark,
    Frost,
    Plasma
}


pub enum Stance {
    Calm,
    Wrath,
    Divinity,
    None,
    All
}

pub enum EffectTarget {
    _Self,
    RandomEnemy,
    TargetEnemy,
    AllEnemies,
    Attacker,
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
    RandomType(CardType),
    RandomRarity(CardRarity),
    RandomClass(CardClass),
}

pub enum RelativePosition {
    Bottom,
    Top,
    Random,
    PlayerChoice(i32),
    All,
}

pub enum CardLocation {
    This,
    DeckPile,
    DrawPile(RelativePosition),
    PlayerHand(RelativePosition),
    ExhaustPile(RelativePosition),
    DiscardPile(RelativePosition),
}