use crate::models::GameState;
use crate::models::cards::CardType; 
use crate::spireai::calculator::GamePossibilitySet;

pub enum Effect {
    Block(i32, EffectTarget),
    Damage(i32, EffectTarget),
    LoseHp(i32, EffectTarget),

    SetStatus(&'static str, i32, EffectTarget),

    Draw(i32),
    Scry(i32),
    AddEnergy(i32),
    IncreaseMaxHp(i32),

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

    IfStatus(EffectTarget, &'static str, Vec<Effect>),
    IfAttacking(EffectTarget, Vec<Effect>),
    
    Custom(fn(&GameState) -> GamePossibilitySet),
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
    RandomType(CardType)
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