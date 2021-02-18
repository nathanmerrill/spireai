use std::collections::HashMap;
use crate::models::{GameState};
use crate::spireai::calculator;

use calculator::{GameCard, GameAction, GamePossibilitySet};

pub enum CardType {
    Attack, Skill, Power, Status, Curse,
}

pub enum CardRarity {
    Starter, Common, Uncommon, Rare, Special, Status, Curse,
}

pub enum CardClass {
    All,
    Ironclad,
    Silent,
    Defect,
    Watcher,
    Neutral,
    Status,
    Curse,
}

pub enum Effect {
    Block(i32, EffectTarget),
    
    Damage(i32, EffectTarget),
    DamageIfFatal(i32, EffectTarget, Vec<Effect>)

    Status(Status, i32, EffectTarget),

    Draw(i32),
    AddEnergy(i32),
    IncreaseMaxHp(i32),
    LoseHp(i32, EffectTarget),

    IfStatus(EffectTarget, Status, Vec<Effect>)
    IfAttacking(EffectTarget, Vec<Effect>)    
    
    CustomAction(fn(&GameAction, &GameState) -> GamePossibilitySet),
}

pub enum EffectTarget {
    Player,
    RandomEnemy,
    TargetEnemy,
    AllEnemies,
}

pub enum CardEffect {
    OnPlay(Effect),
    OnDraw(Effect),
    OnDiscard(Effect),

    CustomBlock(fn(&GameCard, &GameState) -> i32, EffectTarget),
    CustomDamage(fn(&GameCard, &GameState) -> i32, EffectTarget),
    CustomStatus(Status, fn(&GameCard, &GameState) -> i32, EffectTarget),
    PlayableIf(fn(&GameCard, &GameState) -> bool)
    
    Ethereal,
    Exhaust,
    Innate,

    RepeatX(Vec<CardEffect>),
    
    CustomCost(fn(&GameCard, &GameState) -> i32),

    ExhaustCard(CardReference),
    AddCard{
        card: CardReference, 
        destination: CardLocation, 
        copies: i32,
        modifier: CardModifier
    },
    MoveCard(CardReference, CardLocation),
    UpgradeCard(CardReference),
    AutoPlayCard(CardReference),
}

pub enum CardModifier {
    None,
    SetZeroCombatCost,
    SetZeroTurnCost,
    SetZeroCostUntilPlayed,
}

pub enum CardReference {
    ByName(&'static str),
    InLocation(i32, CardLocation),
    This,
    AllInLocation(CardLocation),
    RandomType(CardType)
}

pub enum CardLocation {
    Deck,
    DrawRandom,
    DrawTop,
    DrawBottom,
    DrawChoose,
    HandChoose,
    HandRandom,
    HandBottom,
    Exhaust,
    Discard,
}

pub enum OnUpgrade {
    SetEffects(Vec<CardEffect>),
    ReduceCost(i32),
    Armaments,
    Custom,
    Burn,
    Unupgradable,
    None
}

pub struct BaseCard {
    pub cost: i32, //-1 means X
    pub rarity: CardRarity,
    pub _type: CardType,
    pub _class: CardClass,
    pub effects: Vec<CardEffect>,
    pub on_upgrade: OnUpgrade,
    pub name: &'static str,    
}

const cards: HashMap<&str, &BaseCard> = vec![
    BaseCard {
        name: DEFEND,
        rarity: CardRarity::Starter,
        _type: CardType::Skill,
        _class: CardClass::All,
        effects: vec![CardEffect::Block(5)],
        on_upgrade: OnUpgrade::SetEffects(vec![CardEffect::Block(8)]),
        cost: 1,
    },
    BaseCard {
        name: STRIKE,
        rarity: CardRarity::Starter,
        _type: CardType::Attack,
        _class: CardClass::All,
        effects: vec![CardEffect::Damage(6)],
        on_upgrade: OnUpgrade::SetEffects(vec![CardEffect::Damage(9)]),
        cost: 1,
    },
    BaseCard {
        name: BASH,
        rarity: CardRarity::Starter,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(8), 
            CardEffect::TargetStatus(Status::Vulnerable, 2),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(10), 
            CardEffect::TargetStatus(Status::Vulnerable, 2),
        ]),
        cost: 2,
    },
    BaseCard {
        name: ANGER,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(6),
            CardEffect::AddCard{
                card: CardReference::This, 
                destination: CardLocation::Discard, 
                copies: 1,
                modifier: CardModifier::None
            }
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(8), 
            CardEffect::AddCard{
                card: CardReference::This, 
                destination: CardLocation::Discard, 
                copies: 1,
                modifier: CardModifier::None
            }
        ]),
        cost: 0,
    },
    BaseCard {
        name: ARMAMENTS,
        rarity: CardRarity::Common,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Block(5),
            CardEffect::UpgradeCard(CardReference::InLocation(1, CardLocation::HandChoose))
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(8), 
            CardEffect::UpgradeCard(CardReference::AllInLocation(CardLocation::HandChoose))
        ]),
        cost: 1,
    },
    BaseCard {
        name: BODY_SLAM,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomDamage(calculator::body_slam_damage)
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        cost: 1,
    },
    BaseCard {
        name: CLASH,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::PlayableIf(calculator::clash_playable),
            CardEffect::Damage(14), 
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::PlayableIf(calculator::clash_playable),
            CardEffect::Damage(18), 
        ]),
        cost: 0,
    },
    BaseCard {
        name: CLEAVE,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AoeDamage(8),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::AoeDamage(11),
        ]),
        cost: 1,
    },
    BaseCard {
        name: CLOTHESLINE,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(12),
            CardEffect::TargetStatus(Status::Weak, 2),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(14),
            CardEffect::TargetStatus(Status::Weak, 3),
        ]),
        cost: 2,
    },
    BaseCard {
        name: FLEX,
        rarity: CardRarity::Common,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::TargetStatus(Status::Str, 2),
            CardEffect::TargetStatus(Status::StrDown, 2),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::TargetStatus(Status::Str, 4),
            CardEffect::TargetStatus(Status::StrDown, 4),
        ]),
        cost: 0,
    },
    BaseCard {
        name: HAVOC,
        rarity: CardRarity::Common,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AutoPlayCard(CardReference::InLocation(1, CardLocation::DrawTop))
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        cost: 1,
    },
    BaseCard {
        name: HEADBUTT,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(9), 
            CardEffect::MoveCard(CardReference::InLocation(1, CardLocation::Discard), CardLocation::DrawTop)
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(12),
            CardEffect::MoveCard(CardReference::InLocation(1, CardLocation::Discard), CardLocation::DrawTop)
        ]),
        cost: 1,
    },
    BaseCard {
        name: HEAVY_BLADE,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomDamage(calculator::heavy_blade_damage), 
        ],
        on_upgrade: OnUpgrade::None,
        cost: 2,
    },
    BaseCard {
        name: IRON_WAVE,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(5),
            CardEffect::Block(5),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(7),
            CardEffect::Block(7),
        ]),
        cost: 1,
    },
    BaseCard {
        name: PERFECTED_STRIKE,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomDamage(calculator::perfected_strike_damage), 
        ],
        on_upgrade: OnUpgrade::None,
        cost: 2,
    },
    BaseCard {
        name: SHRUG_IT_OFF,
        rarity: CardRarity::Common,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Block(8),
            CardEffect::Draw(1),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Block(11),
            CardEffect::Draw(1),
        ]),
        cost: 1,
    },
    BaseCard {
        name: SWORD_BOOMERANG,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::RandomTargetDamage(3),
            CardEffect::RandomTargetDamage(3),
            CardEffect::RandomTargetDamage(3),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::RandomTargetDamage(3),
            CardEffect::RandomTargetDamage(3),
            CardEffect::RandomTargetDamage(3),
            CardEffect::RandomTargetDamage(3),
        ]),
        cost: 1,
    },
    BaseCard {
        name: THUNDERCLAP,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AoeDamage(4),
            CardEffect::AoeStatus(Status::Vulnerable, 1),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::AoeDamage(7),
            CardEffect::AoeStatus(Status::Vulnerable, 1),
        ]),
        cost: 1,
    },
    BaseCard {
        name: TRUE_GRIT,
        rarity: CardRarity::Common,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Block(7),
            CardEffect::ExhaustCard(CardReference::InLocation(1, CardLocation::HandRandom))
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Block(9),
            CardEffect::ExhaustCard(CardReference::InLocation(1, CardLocation::HandChoose))
        ]),
        cost: 1,
    },
    BaseCard {
        name: TWIN_STRIKE,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(5),
            CardEffect::Damage(5),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(7),
            CardEffect::Damage(7),
        ]),
        cost: 1,
    },
    BaseCard {
        name: WARCRY,
        rarity: CardRarity::Common,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Draw(1),
            CardEffect::MoveCard(CardReference::InLocation(1, CardLocation::HandChoose), CardLocation::DrawTop),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Draw(2),
            CardEffect::MoveCard(CardReference::InLocation(1, CardLocation::HandChoose), CardLocation::DrawTop),
            CardEffect::Exhaust,
        ]),
        cost: 0,
    },
    BaseCard {
        name: WILD_STRIKE,
        rarity: CardRarity::Common,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(12),
            CardEffect::AddCard{
                card: CardReference::ByName(WOUND), 
                destination: CardLocation::DrawRandom, 
                copies: 1,
                modifier: CardModifier::None
            },
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(17),
            CardEffect::AddCard{
                card: CardReference::ByName(WOUND), 
                destination: CardLocation::DrawRandom, 
                copies: 1,
                modifier: CardModifier::None
            },
        ]),
        cost: 1,
    },
    BaseCard {
        name: BATTLE_TRANCE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Draw(3),
            CardEffect::SelfStatus(Status::NoDraw, 1),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Draw(4),
            CardEffect::SelfStatus(Status::NoDraw, 1),
        ]),
        cost: 0,
    },
    BaseCard {
        name: BLOOD_FOR_BLOOD,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomCost(calculator::blood_for_blood_cost),
            CardEffect::Damage(18),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::CustomCost(calculator::blood_for_blood_cost),
            CardEffect::Damage(22),
        ]),
        cost: 4,
    },
    BaseCard {
        name: BLOODLETTING,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::LoseHp(3),
            CardEffect::AddEnergy(2),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::LoseHp(3),
            CardEffect::AddEnergy(3),
        ]),
        cost: 0,
    },
    BaseCard {
        name: BURNING_PACT,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::ExhaustCard(CardReference::InLocation(1, CardLocation::HandChoose)),
            CardEffect::Draw(2),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::ExhaustCard(CardReference::InLocation(1, CardLocation::HandChoose)),
            CardEffect::Draw(3),
        ]),
        cost: 1,
    },
    BaseCard {
        name: CARNAGE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(20),
            CardEffect::Ethereal,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(28),
            CardEffect::Ethereal,
        ]),
        cost: 2,
    },
    BaseCard {
        name: COMBUST,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Combust, 5),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Combust, 7),
        ]),
        cost: 1,
    },
    BaseCard {
        name: COMBUST,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::DarkEmbrance, 1),
        ],
        on_upgrade: OnUpgrade::ReduceCost(1),
        cost: 2,
    },
    BaseCard {
        name: DISARM,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::TargetStatus(Status::Str, -2),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::TargetStatus(Status::Str, -3),
            CardEffect::Exhaust,
        ]),
        cost: 1,
    },
    BaseCard {
        name: DROPKICK,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(5),
            CardEffect::IfTargetStatus(Status::Vulnerable, vec![
                CardEffect::AddEnergy(1),
                CardEffect::Draw(1),
            ])
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(8),
            CardEffect::IfTargetStatus(Status::Vulnerable, vec![
                CardEffect::AddEnergy(1),
                CardEffect::Draw(1),
            ])
        ]),
        cost: 1,
    },
    BaseCard {
        name: DUAL_WIELD,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AddCard{
                card: CardReference::InLocation(1, CardLocation::HandChoose), 
                destination: CardLocation::HandBottom, 
                copies: 1,
                modifier: CardModifier::None
            },
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::AddCard{
                card: CardReference::InLocation(1, CardLocation::HandChoose), 
                destination: CardLocation::HandBottom, 
                copies: 2,
                modifier: CardModifier::None
            },
        ]),
        cost: 1,
    },
    BaseCard {
        name: ENTRENCH,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomBlock(calculator::entrench_block)
        ],
        on_upgrade: OnUpgrade::ReduceCost(1),
        cost: 2,
    },
    BaseCard {
        name: EVOLVE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Evolve, 1)
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Evolve, 2)
        ]),
        cost: 1,
    },
    BaseCard {
        name: FEEL_NO_PAIN,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::FeelNoPain, 3)
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::FeelNoPain, 4)
        ]),
        cost: 1,
    },
    BaseCard {
        name: FIRE_BREATHING,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::FireBreathing, 6)
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::FireBreathing, 10)
        ]),
        cost: 1,
    },
    BaseCard {
        name: FLAME_BARRIER,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Block(12),
            CardEffect::SelfStatus(Status::FlameBarrier, 4)
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Block(16),
            CardEffect::SelfStatus(Status::FlameBarrier, 6)
        ]),
        cost: 2,
    },
    BaseCard {
        name: GHOSTLY_ARMOR,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Block(10),
            CardEffect::Ethereal,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Block(13),
            CardEffect::Ethereal,
        ]),
        cost: 1,
    },
    BaseCard {
        name: HEMOKINESIS,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::LoseHp(2),
            CardEffect::Damage(15),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::LoseHp(2),
            CardEffect::Damage(20),
        ]),
        cost: 1,
    },
    BaseCard {
        name: INFERNAL_BLADE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AddCard{
                card: CardReference::RandomType(CardType::Attack), 
                destination: CardLocation::HandBottom, 
                copies: 1,
                modifier: CardModifier::SetZeroTurnCost
            },
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        cost: 1,
    },
    BaseCard {
        name: INFLAME,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Str, 2),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Str, 3),
        ]),
        cost: 1,
    },
    BaseCard {
        name: INTIMIDATE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AoeStatus(Status::Weak, 1),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::AoeStatus(Status::Weak, 2),
            CardEffect::Exhaust,
        ]),
        cost: 0,
    },
    BaseCard {
        name: METALLICIZE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Metallicize, 3),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Metallicize, 4),
        ]),
        cost: 1,
    },
    BaseCard {
        name: POWER_THROUGH,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AddCard{
                card: CardReference::ByName(WOUND), 
                destination: CardLocation::HandBottom, 
                copies: 2,
                modifier: CardModifier::None
            },
            CardEffect::Block(15),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::AddCard{
                card: CardReference::ByName(WOUND), 
                destination: CardLocation::HandBottom, 
                copies: 2,
                modifier: CardModifier::None
            },
            CardEffect::Block(20),
        ]),
        cost: 1,
    },
    BaseCard {
        name: PUMMEL,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(2),
            CardEffect::Damage(2),
            CardEffect::Damage(2),
            CardEffect::Damage(2),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(2),
            CardEffect::Damage(2),
            CardEffect::Damage(2),
            CardEffect::Damage(2),
            CardEffect::Damage(2),
            CardEffect::Exhaust,
        ]),
        cost: 1,
    },
    BaseCard {
        name: RAGE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Rage, 3),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Rage, 5),
        ]),
        cost: 0,
    },
    BaseCard {
        name: RAMPAGE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomAction(calculator::rampage_effect),
        ],
        on_upgrade: OnUpgrade::None,
        cost: 1,
    },
    BaseCard {
        name: RECKLESS_CHARGE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(7),
            CardEffect::AddCard{
                card: CardReference::ByName(DAZED), 
                destination: CardLocation::DrawRandom, 
                copies: 1,
                modifier: CardModifier::None
            },
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(10),
            CardEffect::AddCard{
                card: CardReference::ByName(DAZED), 
                destination: CardLocation::DrawRandom, 
                copies: 1,
                modifier: CardModifier::None
            },
        ]),
        cost: 1,
    },
    BaseCard {
        name: RUPTURE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Rupture, 1),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Rupture, 2),
        ]),
        cost: 1,
    },
    BaseCard {
        name: SEARING_BLOW,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomDamage(calculator::searing_blow_damage),
        ],
        on_upgrade: OnUpgrade::None,
        cost: 2,
    },
    BaseCard {
        name: SECOND_WIND,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomAction(calculator::second_wind_effect),
        ],
        on_upgrade: OnUpgrade::None,
        cost: 1,
    },
    BaseCard {
        name: SEEING_RED,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AddEnergy(2),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        cost: 1,
    },
    BaseCard {
        name: SENTINEL,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Block(5),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Block(8),
        ]),
        cost: 1,
    },
    BaseCard {
        name: SEVER_SOUL,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomAction(calculator::sever_soul_effect),
            CardEffect::Damage(16),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(20),
        ]),
        cost: 2,
    },
    BaseCard {
        name: SHOCKWAVE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AoeStatus(Status::Vulnerable, 3),
            CardEffect::AoeStatus(Status::Weak, 3),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::AoeStatus(Status::Vulnerable, 5),
            CardEffect::AoeStatus(Status::Weak, 5),
            CardEffect::Exhaust,
        ]),
        cost: 2,
    },
    BaseCard {
        name: SPOT_WEAKNESS,
        rarity: CardRarity::Uncommon,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::IfTargetAttacking(vec![
                CardEffect::SelfStatus(Status::Str, 3)
            ]),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::IfTargetAttacking(vec![
                CardEffect::SelfStatus(Status::Str, 4)
            ]),
        ]),
        cost: 1,
    },
    BaseCard {
        name: UPPERCUT,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(13),
            CardEffect::TargetStatus(Status::Weak, 1),
            CardEffect::TargetStatus(Status::Vulnerable, 1),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(13),
            CardEffect::TargetStatus(Status::Weak, 2),
            CardEffect::TargetStatus(Status::Vulnerable, 2),
        ]),
        cost: 2,
    },
    BaseCard {
        name: WHIRLWIND,
        rarity: CardRarity::Uncommon,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::RepeatX(vec![
                CardEffect::AoeDamage(5)
            ])
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::RepeatX(vec![
                CardEffect::AoeDamage(8)
            ])
        ]),
        cost: -1,
    },
    BaseCard {
        name: BARRICADE,
        rarity: CardRarity::Rare,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Barricade, 1)
        ],
        on_upgrade: OnUpgrade::ReduceCost(2),
        cost: 3,
    },
    BaseCard {
        name: BERSERK,
        rarity: CardRarity::Rare,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Vulnerable, 2),
            CardEffect::SelfStatus(Status::Beserk, 1),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Vulnerable, 1),
            CardEffect::SelfStatus(Status::Beserk, 1),
        ]),
        cost: 0,
    },
    BaseCard {
        name: BLUDGEON,
        rarity: CardRarity::Rare,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Damage(32),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Damage(42),
        ]),
        cost: 3,
    },
    BaseCard {
        name: BRUTALITY,
        rarity: CardRarity::Rare,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Brutality, 1),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Brutality, 1),
            CardEffect::Innate,
        ]),
        cost: 0,
    },
    BaseCard {
        name: CORRUPTION,
        rarity: CardRarity::Rare,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Corruption, 1),
        ],
        on_upgrade: OnUpgrade::ReduceCost(2),
        cost: 3,
    },
    BaseCard {
        name: DEMON_FORM,
        rarity: CardRarity::Rare,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::DemonForm, 2),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::DemonForm, 3),
        ]),
        cost: 3,
    },
    BaseCard {
        name: DOUBLE_TAP,
        rarity: CardRarity::Rare,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::DoubleTap, 1),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::DoubleTap, 2),
        ]),
        cost: 1,
    },
    BaseCard {
        name: EXHUME,
        rarity: CardRarity::Rare,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::MoveCard(CardReference::InLocation(1, CardLocation::Exhaust), CardLocation::HandBottom),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        cost: 1,
    },
    BaseCard {
        name: FEED,
        rarity: CardRarity::Rare,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::IfDamageFatal(10, vec![CardEffect::IncreaseMaxHp(3)]),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::IfDamageFatal(12, vec![CardEffect::IncreaseMaxHp(4)]),
            CardEffect::Exhaust,
        ]),
        cost: 1,
    },
    BaseCard {
        name: FIEND_FIRE,
        rarity: CardRarity::Rare,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomAction(calculator::fiend_fire_effect),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::None,
        cost: 2,
    },
    BaseCard {
        name: IMMOLATE,
        rarity: CardRarity::Rare,
        _type: CardType::Attack,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::AddCard{
                card: CardReference::ByName(BURN), 
                destination: CardLocation::Discard, 
                copies: 1,
                modifier: CardModifier::None,
            },
        ],
        on_upgrade: OnUpgrade::None,
        cost: 2,
    },
    BaseCard {
        name: IMPERVIOUS,
        rarity: CardRarity::Rare,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::Block(30),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::Block(40),
            CardEffect::Exhaust,
        ]),
        cost: 2,
    },
    BaseCard {
        name: JUGGERNAUT,
        rarity: CardRarity::Rare,
        _type: CardType::Power,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::SelfStatus(Status::Juggernaut, 5),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::SelfStatus(Status::Juggernaut, 7),
        ]),
        cost: 2,
    },
    BaseCard {
        name: LIMIT_BREAK,
        rarity: CardRarity::Rare,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomAction(calculator::limit_break_effect),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::CustomAction(calculator::limit_break_effect),
        ]),
        cost: 1,
    },
    BaseCard {
        name: LIMIT_BREAK,
        rarity: CardRarity::Rare,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomAction(calculator::limit_break_effect),
            CardEffect::Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::CustomAction(calculator::limit_break_effect),
        ]),
        cost: 1,
    },
    BaseCard {
        name: OFFERING,
        rarity: CardRarity::Rare,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::LoseHp(6),
            CardEffect::AddEnergy(2),
            CardEffect::Draw(3),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CardEffect::LoseHp(6),
            CardEffect::AddEnergy(2),
            CardEffect::Draw(5),
        ]),
        cost: 0,
    },
    BaseCard {
        name: REAPER,
        rarity: CardRarity::Rare,
        _type: CardType::Skill,
        _class: CardClass::Ironclad,
        effects: vec![
            CardEffect::CustomAction(calculator::reaper_effect),
        ],
        on_upgrade: OnUpgrade::None,
        cost: 0,
    },
].iter().map(|a| (a.name, a)).collect();

const ACCURACY: &str = "Accuracy";
const ACROBATICS: &str = "Acrobatics";
const ADRENALINE: &str = "Adrenaline";
const AFTER_IMAGE: &str = "After Image";
const AGGREGATE: &str = "Aggregate";
const ALCHEMIZE: &str = "Alchemize";
const ALL_FOR_ONE: &str = "All for One";
const ALL_OUT_ATTACK: &str = "All-Out Attack";
const ALPHA: &str = "Alpha";
const AMPLIFY: &str = "Amplify";
const ANGER: &str = "Anger";
const APOTHEOSIS: &str = "Apotheosis";
const APPARITION: &str = "Apparition";
const ARMAMENTS: &str = "Armaments";
const ASCENDERS_BANE: &str = "Ascender's Bane";
const ATHOUSANDCUTS: &str = "A Thousand Cuts";
const AUTO_SHIELDS: &str = "Auto Shields";
const BACKFLIP: &str = "Backflip";
const BACKSTAB: &str = "Backstab";
const BALL_LIGHTNING: &str = "Ball Lightning";
const BANDAGE_UP: &str = "Bandage Up";
const BANE: &str = "Bane";
const BARRAGE: &str = "Barrage";
const BARRICADE: &str = "Barricade";
const BASH: &str = "Bash";
const BATTLE_HYMN: &str = "Battle Hymn";
const BATTLE_TRANCE: &str = "Battle Trance";
const BEAM_CELL: &str = "Beam Cell";
const BERSERK: &str = "Berserk";
const BETA: &str = "Beta";
const BIASED_COGNITION: &str = "Biased Cognition";
const BITE: &str = "Bite";
const BLADE_DANCE: &str = "Blade Dance";
const BLASPHEMY: &str = "Blasphemy";
const BLIND: &str = "Blind";
const BLIZZARD: &str = "Blizzard";
const BLOODLETTING: &str = "Bloodletting";
const BLOOD_FOR_BLOOD: &str = "Blood for Blood";
const BLUDGEON: &str = "Bludgeon";
const BLUR: &str = "Blur";
const BODY_SLAM: &str = "Body Slam";
const BOOT_SEQUENCE: &str = "Boot Sequence";
const BOUNCING_FLASK: &str = "Bouncing Flask";
const BOWLING_BASH: &str = "Bowling Bash";
const BRILLIANCE: &str = "Brilliance";
const BRUTALITY: &str = "Brutality";
const BUFFER: &str = "Buffer";
const BULLET_TIME: &str = "Bullet Time";
const BULLSEYE: &str = "Bullseye";
const BURN: &str = "Burn";
const BURNING_PACT: &str = "Burning Pact";
const BURN_PLUS: &str = "Burn+";
const BURST: &str = "Burst";
const CALCULATED_GAMBLE: &str = "Calculated Gamble";
const CALTROPS: &str = "Caltrops";
const CAPACITOR: &str = "Capacitor";
const CARNAGE: &str = "Carnage";
const CARVE_REALITY: &str = "Carve Reality";
const CATALYST: &str = "Catalyst";
const CHAOS: &str = "Chaos";
const CHARGE_BATTERY: &str = "Charge Battery";
const CHILL: &str = "Chill";
const CHOKE: &str = "Choke";
const CHRYSALIS: &str = "Chrysalis";
const CLASH: &str = "Clash";
const CLAW: &str = "Claw";
const CLEAVE: &str = "Cleave";
const CLOAK_AND_DAGGER: &str = "Cloak and Dagger";
const CLOTHESLINE: &str = "Clothesline";
const CLUMSY: &str = "Clumsy";
const COLD_SNAP: &str = "Cold Snap";
const COLLECT: &str = "Collect";
const COMBUST: &str = "Combust";
const COMPILE_DRIVER: &str = "Compile Driver";
const CONCENTRATE: &str = "Concentrate";
const CONCLUDE: &str = "Conclude";
const CONJURE_BLADE: &str = "Conjure Blade";
const CONSECRATE: &str = "Consecrate";
const CONSUME: &str = "Consume";
const COOLHEADED: &str = "Coolheaded";
const CORE_SURGE: &str = "Core Surge";
const CORPSE_EXPLOSION: &str = "Corpse Explosion";
const CORRUPTION: &str = "Corruption";
const CREATIVE_AI: &str = "Creative AI";
const CRESCENDO: &str = "Crescendo";
const CRIPPLING_CLOUD: &str = "Crippling Cloud";
const CRUSH_JOINTS: &str = "Crush Joints";
const CURSE_OF_THE_BELL: &str = "Curse of the Bell";
const CUT_THROUGH_FATE: &str = "Cut Through Fate";
const DAGGER_SPRAY: &str = "Dagger Spray";
const DAGGER_THROW: &str = "Dagger Throw";
const DARKNESS: &str = "Darkness";
const DARK_EMBRACE: &str = "Dark Embrace";
const DARK_SHACKLES: &str = "Dark Shackles";
const DASH: &str = "Dash";
const DAZED: &str = "Dazed";
const DEADLY_POISON: &str = "Deadly Poison";
const DECAY: &str = "Decay";
const DECEIVE_REALITY: &str = "Deceive Reality";
const DEEP_BREATH: &str = "Deep Breath";
const DEFEND: &str = "Defend";
const DEFLECT: &str = "Deflect";
const DEFRAGMENT: &str = "Defragment";
const DEMON_FORM: &str = "Demon Form";
const DEUS_EX_MACHINA: &str = "Deus Ex Machina";
const DEVA_FORM: &str = "Deva Form";
const DEVOTION: &str = "Devotion";
const DIE_DIE_DIE: &str = "Die Die Die";
const DISARM: &str = "Disarm";
const DISCOVERY: &str = "Discovery";
const DISTRACTION: &str = "Distraction";
const DODGE_AND_ROLL: &str = "Dodge and Roll";
const DOOM_AND_GLOOM: &str = "Doom and Gloom";
const DOPPELGANGER: &str = "Doppelganger";
const DOUBLE_ENERGY: &str = "Double Energy";
const DOUBLE_TAP: &str = "Double Tap";
const DOUBT: &str = "Doubt";
const DRAMATIC_ENTRANCE: &str = "Dramatic Entrance";
const DROPKICK: &str = "Dropkick";
const DUALCAST: &str = "Dualcast";
const DUAL_WIELD: &str = "Dual Wield";
const ECHO_FORM: &str = "Echo Form";
const ELECTODYNAMICS: &str = "Electodynamics";
const EMPTY_BODY: &str = "Empty Body";
const EMPTY_FIST: &str = "Empty Fist";
const EMPTY_MIND: &str = "Empty Mind";
const ENDLESS_AGONY: &str = "Endless Agony";
const ENLIGHTENMENT: &str = "Enlightenment";
const ENTRENCH: &str = "Entrench";
const ENVENOM: &str = "Envenom";
const EQUILIBRIUM: &str = "Equilibrium";
const ERUPTION: &str = "Eruption";
const ESCAPE_PLAN: &str = "Escape Plan";
const ESTABLISHMENT: &str = "Establishment";
const EVALUATE: &str = "Evaluate";
const EVISCERATE: &str = "Eviscerate";
const EVOLVE: &str = "Evolve";
const EXHUME: &str = "Exhume";
const EXPERTISE: &str = "Expertise";
const EXPUNGER: &str = "Expunger";
const FASTING: &str = "Fasting";
const FEAR_NO_EVIL: &str = "Fear No Evil";
const FEED: &str = "Feed";
const FEEL_NO_PAIN: &str = "Feel No Pain";
const FIEND_FIRE: &str = "Fiend Fire";
const FINESSE: &str = "Finesse";
const FINISHER: &str = "Finisher";
const FIRE_BREATHING: &str = "Fire Breathing";
const FISSION: &str = "Fission";
const FLAME_BARRIER: &str = "Flame Barrier";
const FLASH_OF_STEEL: &str = "Flash of Steel";
const FLECHETTES: &str = "Flechettes";
const FLEX: &str = "Flex";
const FLURRY_OF_BLOWS: &str = "Flurry of Blows";
const FLYING_KNEE: &str = "Flying Knee";
const FLYING_SLEEVES: &str = "Flying Sleeves";
const FOLLOW_UP: &str = "Follow Up";
const FOOTWORK: &str = "Footwork";
const FORCE_FIELD: &str = "Force Field";
const FOREIGN_INFLUENCE: &str = "Foreign Influence";
const FORESIGHT: &str = "Foresight";
const FORETHOUGHT: &str = "Forethought";
const FTL: &str = "FTL";
const FUSION: &str = "Fusion";
const GENETIC_ALGORITHM: &str = "Genetic Algorithm";
const GHOSTLY_ARMOR: &str = "Ghostly Armor";
const GLACIER: &str = "Glacier";
const GLASS_KNIFE: &str = "Glass Knife";
const GOOD_INSTINCTS: &str = "Good Instincts";
const GO_FOR_THE_EYES: &str = "Go for the Eyes";
const GRAND_FINALE: &str = "Grand Finale";
const HALT: &str = "Halt";
const HAND_OF_GREED: &str = "Hand of Greed";
const HAVOC: &str = "Havoc";
const HEADBUTT: &str = "Headbutt";
const HEATSINKS: &str = "Heatsinks";
const HEAVY_BLADE: &str = "Heavy Blade";
const HEEL_HOOK: &str = "Heel Hook";
const HELLO_WORLD: &str = "Hello World";
const HEMOKINESIS: &str = "Hemokinesis";
const HOLOGRAM: &str = "Hologram";
const HYPERBEAM: &str = "Hyperbeam";
const IMMOLATE: &str = "Immolate";
const IMPATIENCE: &str = "Impatience";
const IMPERVIOUS: &str = "Impervious";
const INDIGNATION: &str = "Indignation";
const INFERNAL_BLADE: &str = "Infernal Blade";
const INFINITE_BLADES: &str = "Infinite Blades";
const INFLAME: &str = "Inflame";
const INJURY: &str = "Injury";
const INNER_PEACE: &str = "Inner Peace";
const INSIGHT: &str = "Insight";
const INTIMIDATE: &str = "Intimidate";
const IRON_WAVE: &str = "Iron Wave";
const JACK_OF_ALL_TRADES: &str = "Jack of All Trades";
const JAX: &str = "Jax";
const JUDGMENT: &str = "Judgment";
const JUGGERNAUT: &str = "Juggernaut";
const JUST_LUCKY: &str = "Just Lucky";
const LEAP: &str = "Leap";
const LEG_SWEEP: &str = "Leg Sweep";
const LESSON_LEARNED: &str = "Lesson Learned";
const LIKE_WATER: &str = "Like Water";
const LIMIT_BREAK: &str = "Limit Break";
const LOOP: &str = "Loop";
const MACHINE_LEARNING: &str = "Machine Learning";
const MADNESS: &str = "Madness";
const MAGNETISM: &str = "Magnetism";
const MALAISE: &str = "Malaise";
const MASTERFUL_STAB: &str = "Masterful Stab";
const MASTER_OF_STRATEGY: &str = "Master of Strategy";
const MASTER_REALITY: &str = "Master Reality";
const MAYHEM: &str = "Mayhem";
const MEDITATE: &str = "Meditate";
const MELTER: &str = "Melter";
const MENTAL_FORTRESS: &str = "Mental Fortress";
const METALLICIZE: &str = "Metallicize";
const METAMORPHOSIS: &str = "Metamorphosis";
const METEOR_STRIKE: &str = "Meteor Strike";
const MIND_BLAST: &str = "Mind Blast";
const MIRACLE: &str = "Miracle";
const MULTI_CAST: &str = "Multi-Cast";
const NECRONOMICURSE: &str = "Necronomicurse";
const NEUTRALIZE: &str = "Neutralize";
const NIGHTMARE: &str = "Nightmare";
const NIRVANA: &str = "Nirvana";
const NORMALITY: &str = "Normality";
const NOXIOUS_FUMES: &str = "Noxious Fumes";
const OFFERING: &str = "Offering";
const OMEGA: &str = "Omega";
const OMNISCIENCE: &str = "Omniscience";
const OUTMANEUVER: &str = "Outmaneuver";
const OVERCLOCK: &str = "Overclock";
const PAIN: &str = "Pain";
const PANACEA: &str = "Panacea";
const PANACHE: &str = "Panache";
const PANIC_BUTTON: &str = "Panic Button";
const PARASITE: &str = "Parasite";
const PERFECTED_STRIKE: &str = "Perfected Strike";
const PERSEVERANCE: &str = "Perseverance";
const PHANTASMAL_KILLER: &str = "Phantasmal Killer";
const PIERCING_WAIL: &str = "Piercing Wail";
const POMMEL_STRIKE: &str = "Pommel Strike";
const POSIONED_STAB: &str = "Posioned Stab";
const POWER_THROUGH: &str = "Power Through";
const PRAY: &str = "Pray";
const PREDATOR: &str = "Predator";
const PREPARED: &str = "Prepared";
const PRESSURE_POINTS: &str = "Pressure Points";
const PRIDE: &str = "Pride";
const PROSTRATE: &str = "Prostrate";
const PROTECT: &str = "Protect";
const PUMMEL: &str = "Pummel";
const PURITY: &str = "Purity";
const QUICK_SLASH: &str = "Quick Slash";
const RAGE: &str = "Rage";
const RAGNAROK: &str = "Ragnarok";
const RAINBOW: &str = "Rainbow";
const RAMPAGE: &str = "Rampage";
const REACH_HEAVEN: &str = "Reach Heaven";
const REAPER: &str = "Reaper";
const REBOOT: &str = "Reboot";
const REBOUND: &str = "Rebound";
const RECKLESS_CHARGE: &str = "Reckless Charge";
const RECURSION: &str = "Recursion";
const RECYCLE: &str = "Recycle";
const REFLEX: &str = "Reflex";
const REGRET: &str = "Regret";
const REINFORCED_BODY: &str = "Reinforced Body";
const REPROGRAM: &str = "Reprogram";
const RIDDLE_WITH_HOLES: &str = "Riddle with Holes";
const RIP_AND_TEAR: &str = "Rip and Tear";
const RITUAL_DAGGER: &str = "Ritual Dagger";
const RUPTURE: &str = "Rupture";
const RUSHDOWN: &str = "Rushdown";
const SADISTIC_NATURE: &str = "Sadistic Nature";
const SAFETY: &str = "Safety";
const SANCTITY: &str = "Sanctity";
const SANDS_OF_TIME: &str = "Sands of Time";
const SASH_WHIP: &str = "Sash Whip";
const SCRAPE: &str = "Scrape";
const SCRAWL: &str = "Scrawl";
const SEARING_BLOW: &str = "Searing Blow";
const SECOND_WIND: &str = "Second Wind";
const SECRET_TECHIQUE: &str = "Secret Techique";
const SECRET_WEAPON: &str = "Secret Weapon";
const SEEING_RED: &str = "Seeing Red";
const SEEK: &str = "Seek";
const SELF_REPAIR: &str = "Self Repair";
const SENTINEL: &str = "Sentinel";
const SETUP: &str = "Setup";
const SEVER_SOUL: &str = "Sever Soul";
const SHAME: &str = "Shame";
const SHIV: &str = "Shiv";
const SHOCKWAVE: &str = "Shockwave";
const SHRUG_IT_OFF: &str = "Shrug It Off";
const SIGNATURE_MOVE: &str = "Signature Move";
const SIMMERING_FURY: &str = "Simmering Fury";
const SKEWER: &str = "Skewer";
const SKIM: &str = "Skim";
const SLICE: &str = "Slice";
const SLIMED: &str = "Slimed";
const SMITE: &str = "Smite";
const SNEAKY_STRIKE: &str = "Sneaky Strike";
const SPIRIT_SHIELD: &str = "Spirit Shield";
const SPOT_WEAKNESS: &str = "Spot Weakness";
const STACK: &str = "Stack";
const STATIC_DISCHARGE: &str = "Static Discharge";
const STEAM_BARRIER: &str = "Steam Barrier";
const STORM: &str = "Storm";
const STORM_OF_STEEL: &str = "Storm of Steel";
const STREAMLINE: &str = "Streamline";
const STRIKE: &str = "Strike";
const STUDY: &str = "Study";
const SUCKER_PUNCH: &str = "Sucker Punch";
const SUNDER: &str = "Sunder";
const SURVIVOR: &str = "Survivor";
const SWEEPING_BEAM: &str = "Sweeping Beam";
const SWIFT_STRIKE: &str = "Swift Strike";
const SWIVEL: &str = "Swivel";
const SWORD_BOOMERANG: &str = "Sword Boomerang";
const TACTICIAN: &str = "Tactician";
const TALK_TO_THE_HAND: &str = "Talk to the Hand";
const TANTRUM: &str = "Tantrum";
const TEMPEST: &str = "Tempest";
const TERROR: &str = "Terror";
const THE_BOMB: &str = "The Bomb";
const THINKING_AHEAD: &str = "Thinking Ahead";
const THIRD_EYE: &str = "Third Eye";
const THROUGH_VIOLENCE: &str = "Through Violence";
const THUNDERCLAP: &str = "Thunderclap";
const THUNDER_STRIKE: &str = "Thunder Strike";
const TOOLS_OF_THE_TRADE: &str = "Tools of the Trade";
const TRANQUILITY: &str = "Tranquility";
const TRANSMUTATION: &str = "Transmutation";
const TRIP: &str = "Trip";
const TRUE_GRIT: &str = "True Grit";
const TURBO: &str = "Turbo";
const TWIN_STRIKE: &str = "Twin Strike";
const UNLOAD: &str = "Unload";
const UPPERCUT: &str = "Uppercut";
const VAULT: &str = "Vault";
const VIGILANCE: &str = "Vigilance";
const VIOLENCE: &str = "Violence";
const VOID: &str = "Void";
const WALLOP: &str = "Wallop";
const WARCRY: &str = "Warcry";
const WAVE_OF_THE_HAND: &str = "Wave of the Hand";
const WEAVE: &str = "Weave";
const WELL_LAID_PLANS: &str = "Well-Laid Plans";
const WHEEL_KICK: &str = "Wheel Kick";
const WHIRLWIND: &str = "Whirlwind";
const WHITE_NOISE: &str = "White Noise";
const WILD_STRIKE: &str = "Wild Strike";
const WINDMILL_STRIKE: &str = "Windmill Strike";
const WISH: &str = "Wish";
const WORSHIP: &str = "Worship";
const WOUND: &str = "Wound";
const WRAITH_FORM: &str = "Wraith Form";
const WREATH_OF_FLAME: &str = "Wreath of Flame";
const WRITHE: &str = "Writhe";
const ZAP: &str = "Zap";