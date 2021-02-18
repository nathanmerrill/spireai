use std::collections::HashMap;
use crate::models::{GameState};
use crate::spireai::calculator;
use crate::models::statuses;

use crate::models::effects::*;
use Effect::*;
use EffectTarget::*;
use CardLocation::*;
use CardEffect::*;

use calculator::{GameCard, GameAction, GamePossibilitySet};

#[derive(PartialEq)]
pub enum CardType {
    Attack, Skill, Power, Status, Curse, All
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

pub const Exhaust: CardEffect = OnPlay(ExhaustCard(CardLocation::This));

pub enum CardEffect {
    OnPlay(Effect),
    OnDraw(Effect),
    OnDiscard(Effect),
    OnExhaust(Effect),

    RepeatX(Vec<Effect>),
    IfFatal(Vec<Effect>),
    
    PlayableIf(fn(&GameCard, &GameState) -> bool),
    CustomOnPlay(fn(&GameAction, &GameState) -> GamePossibilitySet),
    CustomCost(fn(&GameCard, &GameState) -> i32),
}

pub enum OnUpgrade {
    SetEffects(Vec<CardEffect>),
    ReduceCost(i32),
    Armaments,
    Custom,
    Burn,
    Unupgradable,
    Innate,
    RemoveEthereal,
    None
}

pub struct BaseCard {
    pub cost: i32, //-1 means X
    pub rarity: CardRarity,
    pub _type: CardType,
    pub _class: CardClass,
    pub targeted: bool,
    pub effects: Vec<CardEffect>,
    pub on_upgrade: OnUpgrade,
    pub name: &'static str,   
    pub innate: bool,
    pub ethereal: bool, 
}

impl BaseCard {
    fn default(_class: CardClass, _type: CardType) -> Self {
        Self {
            name: &"",
            rarity: CardRarity::Common,
            _type: _type,
            _class: _class,
            targeted: _type == CardType::Attack,
            effects: vec![],
            on_upgrade: OnUpgrade::None,
            cost: 1,
            innate: false,
            ethereal: false,
        }
    }
}

pub const cards: HashMap<&str, &BaseCard> = vec![
    BaseCard {
        name: DEFEND,
        rarity: CardRarity::Starter,
        effects: vec![OnPlay(Block(5, _Self))],
        on_upgrade: OnUpgrade::SetEffects(vec![OnPlay(Block(8, _Self))]),
        ..BaseCard::default(CardClass::All, CardType::Skill)
    },
    BaseCard {
        name: STRIKE,
        rarity: CardRarity::Starter,
        effects: vec![OnPlay(AttackDamage(6, TargetEnemy))],
        on_upgrade: OnUpgrade::SetEffects(vec![OnPlay(AttackDamage(9, TargetEnemy))]),
        ..BaseCard::default(CardClass::All, CardType::Attack)
    },
    BaseCard {
        name: BASH,
        rarity: CardRarity::Starter,
        effects: vec![
            OnPlay(AttackDamage(8, TargetEnemy)), 
            OnPlay(SetStatus(statuses::VULNERABLE, 2, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(10, TargetEnemy)), 
            OnPlay(SetStatus(statuses::VULNERABLE, 3, TargetEnemy)),
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: ANGER,
        effects: vec![
            OnPlay(AttackDamage(6, TargetEnemy)),
            OnPlay(AddCard{
                card: CardReference::CopyOf(CardLocation::This), 
                destination: DiscardPile(RelativePosition::Bottom), 
                copies: 1,
                modifier: CardModifier::None
            })
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(8, TargetEnemy)), 
            OnPlay(AddCard{
                card: CardReference::CopyOf(CardLocation::This), 
                destination: DiscardPile(RelativePosition::Bottom), 
                copies: 1,
                modifier: CardModifier::None
            })
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: ARMAMENTS,
        effects: vec![
            OnPlay(Block(5, _Self)),
            OnPlay(UpgradeCard(PlayerHand(RelativePosition::Random))),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(8, TargetEnemy)), 
            OnPlay(UpgradeCard(PlayerHand(RelativePosition::All))),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: BODY_SLAM,
        effects: vec![
            CustomOnPlay(calculator::body_slam_damage)
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: CLASH,
        effects: vec![
            PlayableIf(calculator::clash_playable),
            OnPlay(AttackDamage(14, TargetEnemy)), 
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            PlayableIf(calculator::clash_playable),
            OnPlay(AttackDamage(18, TargetEnemy)), 
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: CLEAVE,
        targeted: false,
        effects: vec![
            OnPlay(AttackDamage(8, AllEnemies)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(11, AllEnemies)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: CLOTHESLINE,
        effects: vec![
            OnPlay(AttackDamage(12, TargetEnemy)),
            OnPlay(SetStatus(statuses::WEAK, 2, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(14, TargetEnemy)),
            OnPlay(SetStatus(statuses::WEAK, 3, TargetEnemy)),
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: FLEX,
        effects: vec![
            OnPlay(SetStatus(statuses::STRENGTH, 2, _Self)),
            OnPlay(SetStatus(statuses::STRENGTH_DOWN, 2, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::STRENGTH, 4, _Self)),
            OnPlay(SetStatus(statuses::STRENGTH_DOWN, 4, _Self)),
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: HAVOC,
        effects: vec![
            OnPlay(AutoPlayCard(DrawPile(RelativePosition::Top))),
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: HEADBUTT,
        effects: vec![
            OnPlay(AttackDamage(9, TargetEnemy)), 
            OnPlay(MoveCard(DiscardPile(RelativePosition::PlayerChoice(1)), CardLocation::DrawPile(RelativePosition::Top))),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(12, TargetEnemy)),
            OnPlay(MoveCard(DiscardPile(RelativePosition::PlayerChoice(1)), CardLocation::DrawPile(RelativePosition::Top))),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: HEAVY_BLADE,
        effects: vec![
            CustomOnPlay(calculator::heavy_blade_damage), 
        ],
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: IRON_WAVE,
        effects: vec![
            OnPlay(AttackDamage(5, TargetEnemy)),
            OnPlay(Block(5, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(7, TargetEnemy)),
            OnPlay(Block(7, _Self)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: PERFECTED_STRIKE,
        effects: vec![
            CustomOnPlay(calculator::perfected_strike_damage), 
        ],
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: SHRUG_IT_OFF,
        effects: vec![
            OnPlay(Block(8, _Self)),
            OnPlay(Draw(1)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(Block(11, _Self)),
            OnPlay(Draw(1)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: SWORD_BOOMERANG,
        targeted: false,
        effects: vec![
            OnPlay(AttackDamage(3, RandomEnemy)),
            OnPlay(AttackDamage(3, RandomEnemy)),
            OnPlay(AttackDamage(3, RandomEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(3, RandomEnemy)),
            OnPlay(AttackDamage(3, RandomEnemy)),
            OnPlay(AttackDamage(3, RandomEnemy)),
            OnPlay(AttackDamage(3, RandomEnemy)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: THUNDERCLAP,
        _type: CardType::Attack,
        targeted: false,
        effects: vec![
            OnPlay(AttackDamage(4, AllEnemies)),
            OnPlay(SetStatus(statuses::VULNERABLE, 1, AllEnemies)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(7, AllEnemies)),
            OnPlay(SetStatus(statuses::VULNERABLE, 1, AllEnemies)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: TRUE_GRIT,
        effects: vec![
            OnPlay(Block(7, _Self)),
            OnPlay(ExhaustCard(CardLocation::PlayerHand(RelativePosition::Random))),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(Block(9, _Self)),
            OnPlay(ExhaustCard(CardLocation::PlayerHand(RelativePosition::PlayerChoice(1)))),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: TWIN_STRIKE,
        effects: vec![
            OnPlay(AttackDamage(5, TargetEnemy)),
            OnPlay(AttackDamage(5, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(7, TargetEnemy)),
            OnPlay(AttackDamage(7, TargetEnemy)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: WARCRY,
        effects: vec![
            OnPlay(Draw(1)),
            OnPlay(MoveCard(
                CardLocation::PlayerHand(RelativePosition::PlayerChoice(1)), 
                CardLocation::DrawPile(RelativePosition::Top))
            ),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(Draw(2)),
            OnPlay(MoveCard(
                CardLocation::PlayerHand(RelativePosition::PlayerChoice(1)), 
                CardLocation::DrawPile(RelativePosition::Top))
            ),
            Exhaust,
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: WILD_STRIKE,
        effects: vec![
            OnPlay(AttackDamage(12, TargetEnemy)),
            OnPlay(AddCard{
                card: CardReference::ByName(WOUND), 
                destination: CardLocation::DrawPile(RelativePosition::Random), 
                copies: 1,
                modifier: CardModifier::None
            }),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(17, TargetEnemy)),
            OnPlay(AddCard{
                card: CardReference::ByName(WOUND), 
                destination: CardLocation::DrawPile(RelativePosition::Random), 
                copies: 1,
                modifier: CardModifier::None
            }),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: BATTLE_TRANCE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(Draw(3)),
            OnPlay(SetStatus(statuses::NO_DRAW, 1, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(Draw(4)),
            OnPlay(SetStatus(statuses::NO_DRAW, 1, _Self)),
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: BLOOD_FOR_BLOOD,
        rarity: CardRarity::Uncommon,
        effects: vec![
            CustomCost(calculator::blood_for_blood_cost),
            OnPlay(AttackDamage(18, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CustomCost(calculator::blood_for_blood_cost),
            OnPlay(AttackDamage(22, TargetEnemy)),
        ]),
        cost: 4,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: BLOODLETTING,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(LoseHp(3, _Self)),
            OnPlay(AddEnergy(2)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(LoseHp(3, _Self)),
            OnPlay(AddEnergy(3)),
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: BURNING_PACT,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(ExhaustCard(CardLocation::PlayerHand(RelativePosition::PlayerChoice(1)))),
            OnPlay(Draw(2)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(ExhaustCard(CardLocation::PlayerHand(RelativePosition::PlayerChoice(1)))),
            OnPlay(Draw(3)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: CARNAGE,
        rarity: CardRarity::Uncommon,
        ethereal: true,
        effects: vec![
            OnPlay(AttackDamage(20, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(28, TargetEnemy)),
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: COMBUST,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::COMBUST, 5, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::COMBUST, 7, _Self)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: COMBUST,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::DARK_EMBRACE, 1, _Self)),
        ],
        on_upgrade: OnUpgrade::ReduceCost(1),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: DISARM,
        rarity: CardRarity::Uncommon,
        targeted: true,
        effects: vec![
            OnPlay(SetStatus(statuses::STRENGTH, -2, TargetEnemy)),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::STRENGTH, -3, TargetEnemy)),
            Exhaust,
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: DROPKICK,
        rarity: CardRarity::Uncommon,
        targeted: true,
        effects: vec![
            OnPlay(AttackDamage(5, TargetEnemy)),
            OnPlay(IfStatus(TargetEnemy, statuses::VULNERABLE, vec![
                AddEnergy(1),
                Draw(1),
            ]))
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(8, TargetEnemy)),
            OnPlay(IfStatus(TargetEnemy, statuses::VULNERABLE, vec![
                AddEnergy(1),
                Draw(1),
            ]))
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: DUAL_WIELD,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(AddCard{
                card: CardReference::CopyOf(CardLocation::PlayerHand(RelativePosition::PlayerChoice(1))), 
                destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                copies: 1,
                modifier: CardModifier::None
            }),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AddCard{
                card: CardReference::CopyOf(CardLocation::PlayerHand(RelativePosition::PlayerChoice(1))), 
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: 2,
                modifier: CardModifier::None
            }),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: ENTRENCH,
        rarity: CardRarity::Uncommon,
        effects: vec![
            CustomOnPlay(calculator::entrench_block)
        ],
        on_upgrade: OnUpgrade::ReduceCost(1),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: EVOLVE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::EVOLVE, 1, _Self))
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::EVOLVE, 2, _Self))
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: FEEL_NO_PAIN,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::FEEL_NO_PAIN, 3, _Self))
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::FEEL_NO_PAIN, 4, _Self))
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: FIRE_BREATHING,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::FIRE_BREATHING, 6, _Self))
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::FIRE_BREATHING, 10, _Self))
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: FLAME_BARRIER,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(Block(12, _Self)),
            OnPlay(SetStatus(statuses::FLAME_BARRIER, 4, _Self))
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(Block(16, _Self)),
            OnPlay(SetStatus(statuses::FLAME_BARRIER, 6, _Self))
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: GHOSTLY_ARMOR,
        rarity: CardRarity::Uncommon,
        targeted: false,
        ethereal: true,
        effects: vec![
            OnPlay(Block(10, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(Block(13, _Self)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: HEMOKINESIS,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(LoseHp(2, _Self)),
            OnPlay(AttackDamage(15, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(LoseHp(2, _Self)),
            OnPlay(AttackDamage(20, TargetEnemy)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: INFERNAL_BLADE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(AddCard{
                card: CardReference::RandomType(CardType::Attack),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                copies: 1,
                modifier: CardModifier::SetZeroTurnCost
            }),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: INFLAME,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::STRENGTH, 2, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::STRENGTH, 3, _Self)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: INTIMIDATE,
        rarity: CardRarity::Uncommon,
        _type: CardType::Power,
        effects: vec![
            OnPlay(SetStatus(statuses::WEAK, 1, AllEnemies)),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::WEAK, 2, AllEnemies)),
            Exhaust,
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: METALLICIZE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::METALLICIZE, 3, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::METALLICIZE, 4, _Self)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: POWER_THROUGH,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(AddCard{
                card: CardReference::ByName(WOUND), 
                destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                copies: 2,
                modifier: CardModifier::None
            }),
            OnPlay(Block(15, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AddCard{
                card: CardReference::ByName(WOUND), 
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: 2,
                modifier: CardModifier::None
            }),
            OnPlay(Block(20, _Self)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: PUMMEL,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(AttackDamage(2, TargetEnemy)),
            OnPlay(AttackDamage(2, TargetEnemy)),
            OnPlay(AttackDamage(2, TargetEnemy)),
            OnPlay(AttackDamage(2, TargetEnemy)),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(2, TargetEnemy)),
            OnPlay(AttackDamage(2, TargetEnemy)),
            OnPlay(AttackDamage(2, TargetEnemy)),
            OnPlay(AttackDamage(2, TargetEnemy)),
            OnPlay(AttackDamage(2, TargetEnemy)),
            Exhaust,
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: RAGE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::RAGE, 3, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::RAGE, 5, _Self)),
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: RAMPAGE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            CustomOnPlay(calculator::rampage_effect),
        ],
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: RECKLESS_CHARGE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(AttackDamage(7, TargetEnemy)),
            OnPlay(AddCard{
                card: CardReference::ByName(DAZED), 
                destination: CardLocation::DrawPile(RelativePosition::Random), 
                copies: 1,
                modifier: CardModifier::None
            }),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(10, TargetEnemy)),
            OnPlay(AddCard{
                card: CardReference::ByName(DAZED), 
                destination: CardLocation::DrawPile(RelativePosition::Random), 
                copies: 1,
                modifier: CardModifier::None
            }),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: RUPTURE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::RUPTURE, 1, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::RUPTURE, 2, _Self)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: SEARING_BLOW,
        rarity: CardRarity::Uncommon,
        effects: vec![
            CustomOnPlay(calculator::searing_blow_damage),
        ],
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: SECOND_WIND,
        rarity: CardRarity::Uncommon,
        effects: vec![
            CustomOnPlay(calculator::second_wind_effect),
        ],
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: SEEING_RED,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(AddEnergy(2)),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: SENTINEL,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(Block(5, _Self)),
            OnExhaust(AddEnergy(2)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(Block(8, _Self)),
            OnExhaust(AddEnergy(3)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: SEVER_SOUL,
        rarity: CardRarity::Uncommon,
        effects: vec![
            CustomOnPlay(calculator::sever_soul_effect),
            OnPlay(AttackDamage(16, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CustomOnPlay(calculator::sever_soul_effect),
            OnPlay(AttackDamage(20, TargetEnemy)),
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: SHOCKWAVE,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(SetStatus(statuses::VULNERABLE, 3, AllEnemies)),
            OnPlay(SetStatus(statuses::WEAK, 3, AllEnemies)),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::VULNERABLE, 5, AllEnemies)),
            OnPlay(SetStatus(statuses::WEAK, 5, AllEnemies)),
            Exhaust,
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: SPOT_WEAKNESS,
        rarity: CardRarity::Uncommon,
        targeted: true,
        effects: vec![
            OnPlay(IfAttacking(TargetEnemy, vec![
                SetStatus(statuses::STRENGTH, 3, _Self)
            ])),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(IfAttacking(TargetEnemy, vec![
                SetStatus(statuses::STRENGTH, 4, _Self)
            ])),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: UPPERCUT,
        rarity: CardRarity::Uncommon,
        effects: vec![
            OnPlay(AttackDamage(13, TargetEnemy)),
            OnPlay(SetStatus(statuses::WEAK, 1, TargetEnemy)),
            OnPlay(SetStatus(statuses::VULNERABLE, 1, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(13, TargetEnemy)),
            OnPlay(SetStatus(statuses::WEAK, 2, TargetEnemy)),
            OnPlay(SetStatus(statuses::VULNERABLE, 2, TargetEnemy)),
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: WHIRLWIND,
        rarity: CardRarity::Uncommon,
        targeted: false,
        effects: vec![
            RepeatX(vec![
                AttackDamage(5, AllEnemies)
            ])
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            RepeatX(vec![
                AttackDamage(8, AllEnemies)
            ])
        ]),
        cost: -1,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: BARRICADE,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(SetStatus(statuses::BARRICADE, 1, _Self)),
        ],
        on_upgrade: OnUpgrade::ReduceCost(2),
        cost: 3,
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: BERSERK,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(SetStatus(statuses::VULNERABLE, 2, _Self)),
            OnPlay(SetStatus(statuses::BERSERK, 1, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::VULNERABLE, 1, _Self)),
            OnPlay(SetStatus(statuses::BERSERK, 1, _Self)),
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: BLUDGEON,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(AttackDamage(32, TargetEnemy)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(42, TargetEnemy)),
        ]),
        cost: 3,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: BRUTALITY,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(SetStatus(statuses::BRUTALITY, 1, _Self)),
        ],
        on_upgrade: OnUpgrade::Innate,
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: CORRUPTION,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(SetStatus(statuses::CORRUPTION, 1, _Self)),
        ],
        on_upgrade: OnUpgrade::ReduceCost(2),
        cost: 3,
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: DEMON_FORM,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(SetStatus(statuses::DEMON_FORM, 2, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::DEMON_FORM, 3, _Self)),
        ]),
        cost: 3,
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: DOUBLE_TAP,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(SetStatus(statuses::DOUBLE_TAP, 1, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::DOUBLE_TAP, 2, _Self)),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: EXHUME,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(MoveCard(
                CardLocation::ExhaustPile(RelativePosition::PlayerChoice(1)), 
                CardLocation::PlayerHand(RelativePosition::Bottom))
            ),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::ReduceCost(0),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: FEED,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(AttackDamage(10, TargetEnemy)),
            IfFatal(vec![IncreaseMaxHp(3)]),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(AttackDamage(12, TargetEnemy)),
            IfFatal(vec![IncreaseMaxHp(3)]),
            Exhaust,
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: FIEND_FIRE,
        rarity: CardRarity::Rare,
        effects: vec![
            CustomOnPlay(calculator::fiend_fire_effect),
            Exhaust,
        ],
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: IMMOLATE,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(AddCard{
                card: CardReference::ByName(BURN), 
                destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                copies: 1,
                modifier: CardModifier::None,
            }),
        ],
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
    BaseCard {
        name: IMPERVIOUS,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(Block(30, _Self)),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(Block(40, _Self)),
            Exhaust,
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: JUGGERNAUT,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(SetStatus(statuses::JUGGERNAUT, 5, _Self)),
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(SetStatus(statuses::JUGGERNAUT, 7, _Self)),
        ]),
        cost: 2,
        ..BaseCard::default(CardClass::Ironclad, CardType::Power)
    },
    BaseCard {
        name: LIMIT_BREAK,
        rarity: CardRarity::Rare,
        effects: vec![
            CustomOnPlay(calculator::limit_break_effect),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            CustomOnPlay(calculator::limit_break_effect),
        ]),
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: OFFERING,
        rarity: CardRarity::Rare,
        effects: vec![
            OnPlay(LoseHp(6, _Self)),
            OnPlay(AddEnergy(2)),
            OnPlay(Draw(3)),
            Exhaust,
        ],
        on_upgrade: OnUpgrade::SetEffects(vec![
            OnPlay(LoseHp(6, _Self)),
            OnPlay(AddEnergy(2)),
            OnPlay(Draw(5)),
            Exhaust,
        ]),
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Skill)
    },
    BaseCard {
        name: REAPER,
        rarity: CardRarity::Rare,
        effects: vec![
            CustomOnPlay(calculator::reaper_effect),
            Exhaust,
        ],
        cost: 0,
        ..BaseCard::default(CardClass::Ironclad, CardType::Attack)
    },
].iter().map(|a| (a.name, a)).collect();

pub const ACCURACY: &str = "Accuracy";
pub const ACROBATICS: &str = "Acrobatics";
pub const ADRENALINE: &str = "Adrenaline";
pub const AFTER_IMAGE: &str = "After Image";
pub const AGGREGATE: &str = "Aggregate";
pub const ALCHEMIZE: &str = "Alchemize";
pub const ALL_FOR_ONE: &str = "All for One";
pub const ALL_OUT_ATTACK: &str = "All-Out Attack";
pub const ALPHA: &str = "Alpha";
pub const AMPLIFY: &str = "Amplify";
pub const ANGER: &str = "Anger";
pub const APOTHEOSIS: &str = "Apotheosis";
pub const APPARITION: &str = "Apparition";
pub const ARMAMENTS: &str = "Armaments";
pub const ASCENDERS_BANE: &str = "Ascender's Bane";
pub const ATHOUSANDCUTS: &str = "A Thousand Cuts";
pub const AUTO_SHIELDS: &str = "Auto Shields";
pub const BACKFLIP: &str = "Backflip";
pub const BACKSTAB: &str = "Backstab";
pub const BALL_LIGHTNING: &str = "Ball Lightning";
pub const BANDAGE_UP: &str = "Bandage Up";
pub const BANE: &str = "Bane";
pub const BARRAGE: &str = "Barrage";
pub const BARRICADE: &str = "Barricade";
pub const BASH: &str = "Bash";
pub const BATTLE_HYMN: &str = "Battle Hymn";
pub const BATTLE_TRANCE: &str = "Battle Trance";
pub const BEAM_CELL: &str = "Beam Cell";
pub const BERSERK: &str = "Berserk";
pub const BETA: &str = "Beta";
pub const BIASED_COGNITION: &str = "Biased Cognition";
pub const BITE: &str = "Bite";
pub const BLADE_DANCE: &str = "Blade Dance";
pub const BLASPHEMY: &str = "Blasphemy";
pub const BLIND: &str = "Blind";
pub const BLIZZARD: &str = "Blizzard";
pub const BLOODLETTING: &str = "Bloodletting";
pub const BLOOD_FOR_BLOOD: &str = "Blood for Blood";
pub const BLUDGEON: &str = "Bludgeon";
pub const BLUR: &str = "Blur";
pub const BODY_SLAM: &str = "Body Slam";
pub const BOOT_SEQUENCE: &str = "Boot Sequence";
pub const BOUNCING_FLASK: &str = "Bouncing Flask";
pub const BOWLING_BASH: &str = "Bowling Bash";
pub const BRILLIANCE: &str = "Brilliance";
pub const BRUTALITY: &str = "Brutality";
pub const BUFFER: &str = "Buffer";
pub const BULLET_TIME: &str = "Bullet Time";
pub const BULLSEYE: &str = "Bullseye";
pub const BURN: &str = "Burn";
pub const BURNING_PACT: &str = "Burning Pact";
pub const BURN_PLUS: &str = "Burn+";
pub const BURST: &str = "Burst";
pub const CALCULATED_GAMBLE: &str = "Calculated Gamble";
pub const CALTROPS: &str = "Caltrops";
pub const CAPACITOR: &str = "Capacitor";
pub const CARNAGE: &str = "Carnage";
pub const CARVE_REALITY: &str = "Carve Reality";
pub const CATALYST: &str = "Catalyst";
pub const CHAOS: &str = "Chaos";
pub const CHARGE_BATTERY: &str = "Charge Battery";
pub const CHILL: &str = "Chill";
pub const CHOKE: &str = "Choke";
pub const CHRYSALIS: &str = "Chrysalis";
pub const CLASH: &str = "Clash";
pub const CLAW: &str = "Claw";
pub const CLEAVE: &str = "Cleave";
pub const CLOAK_AND_DAGGER: &str = "Cloak and Dagger";
pub const CLOTHESLINE: &str = "Clothesline";
pub const CLUMSY: &str = "Clumsy";
pub const COLD_SNAP: &str = "Cold Snap";
pub const COLLECT: &str = "Collect";
pub const COMBUST: &str = "Combust";
pub const COMPILE_DRIVER: &str = "Compile Driver";
pub const CONCENTRATE: &str = "Concentrate";
pub const CONCLUDE: &str = "Conclude";
pub const CONJURE_BLADE: &str = "Conjure Blade";
pub const CONSECRATE: &str = "Consecrate";
pub const CONSUME: &str = "Consume";
pub const COOLHEADED: &str = "Coolheaded";
pub const CORE_SURGE: &str = "Core Surge";
pub const CORPSE_EXPLOSION: &str = "Corpse Explosion";
pub const CORRUPTION: &str = "Corruption";
pub const CREATIVE_AI: &str = "Creative AI";
pub const CRESCENDO: &str = "Crescendo";
pub const CRIPPLING_CLOUD: &str = "Crippling Cloud";
pub const CRUSH_JOINTS: &str = "Crush Joints";
pub const CURSE_OF_THE_BELL: &str = "Curse of the Bell";
pub const CUT_THROUGH_FATE: &str = "Cut Through Fate";
pub const DAGGER_SPRAY: &str = "Dagger Spray";
pub const DAGGER_THROW: &str = "Dagger Throw";
pub const DARKNESS: &str = "Darkness";
pub const DARK_EMBRACE: &str = "Dark Embrace";
pub const DARK_SHACKLES: &str = "Dark Shackles";
pub const DASH: &str = "Dash";
pub const DAZED: &str = "Dazed";
pub const DEADLY_POISON: &str = "Deadly Poison";
pub const DECAY: &str = "Decay";
pub const DECEIVE_REALITY: &str = "Deceive Reality";
pub const DEEP_BREATH: &str = "Deep Breath";
pub const DEFEND: &str = "Defend";
pub const DEFLECT: &str = "Deflect";
pub const DEFRAGMENT: &str = "Defragment";
pub const DEMON_FORM: &str = "Demon Form";
pub const DEUS_EX_MACHINA: &str = "Deus Ex Machina";
pub const DEVA_FORM: &str = "Deva Form";
pub const DEVOTION: &str = "Devotion";
pub const DIE_DIE_DIE: &str = "Die Die Die";
pub const DISARM: &str = "Disarm";
pub const DISCOVERY: &str = "Discovery";
pub const DISTRACTION: &str = "Distraction";
pub const DODGE_AND_ROLL: &str = "Dodge and Roll";
pub const DOOM_AND_GLOOM: &str = "Doom and Gloom";
pub const DOPPELGANGER: &str = "Doppelganger";
pub const DOUBLE_ENERGY: &str = "Double Energy";
pub const DOUBLE_TAP: &str = "Double Tap";
pub const DOUBT: &str = "Doubt";
pub const DRAMATIC_ENTRANCE: &str = "Dramatic Entrance";
pub const DROPKICK: &str = "Dropkick";
pub const DUALCAST: &str = "Dualcast";
pub const DUAL_WIELD: &str = "Dual Wield";
pub const ECHO_FORM: &str = "Echo Form";
pub const ELECTODYNAMICS: &str = "Electodynamics";
pub const EMPTY_BODY: &str = "Empty Body";
pub const EMPTY_FIST: &str = "Empty Fist";
pub const EMPTY_MIND: &str = "Empty Mind";
pub const ENDLESS_AGONY: &str = "Endless Agony";
pub const ENLIGHTENMENT: &str = "Enlightenment";
pub const ENTRENCH: &str = "Entrench";
pub const ENVENOM: &str = "Envenom";
pub const EQUILIBRIUM: &str = "Equilibrium";
pub const ERUPTION: &str = "Eruption";
pub const ESCAPE_PLAN: &str = "Escape Plan";
pub const ESTABLISHMENT: &str = "Establishment";
pub const EVALUATE: &str = "Evaluate";
pub const EVISCERATE: &str = "Eviscerate";
pub const EVOLVE: &str = "Evolve";
pub const EXHUME: &str = "Exhume";
pub const EXPERTISE: &str = "Expertise";
pub const EXPUNGER: &str = "Expunger";
pub const FASTING: &str = "Fasting";
pub const FEAR_NO_EVIL: &str = "Fear No Evil";
pub const FEED: &str = "Feed";
pub const FEEL_NO_PAIN: &str = "Feel No Pain";
pub const FIEND_FIRE: &str = "Fiend Fire";
pub const FINESSE: &str = "Finesse";
pub const FINISHER: &str = "Finisher";
pub const FIRE_BREATHING: &str = "Fire Breathing";
pub const FISSION: &str = "Fission";
pub const FLAME_BARRIER: &str = "Flame Barrier";
pub const FLASH_OF_STEEL: &str = "Flash of Steel";
pub const FLECHETTES: &str = "Flechettes";
pub const FLEX: &str = "Flex";
pub const FLURRY_OF_BLOWS: &str = "Flurry of Blows";
pub const FLYING_KNEE: &str = "Flying Knee";
pub const FLYING_SLEEVES: &str = "Flying Sleeves";
pub const FOLLOW_UP: &str = "Follow Up";
pub const FOOTWORK: &str = "Footwork";
pub const FORCE_FIELD: &str = "Force Field";
pub const FOREIGN_INFLUENCE: &str = "Foreign Influence";
pub const FORESIGHT: &str = "Foresight";
pub const FORETHOUGHT: &str = "Forethought";
pub const FTL: &str = "FTL";
pub const FUSION: &str = "Fusion";
pub const GENETIC_ALGORITHM: &str = "Genetic Algorithm";
pub const GHOSTLY_ARMOR: &str = "Ghostly Armor";
pub const GLACIER: &str = "Glacier";
pub const GLASS_KNIFE: &str = "Glass Knife";
pub const GOOD_INSTINCTS: &str = "Good Instincts";
pub const GO_FOR_THE_EYES: &str = "Go for the Eyes";
pub const GRAND_FINALE: &str = "Grand Finale";
pub const HALT: &str = "Halt";
pub const HAND_OF_GREED: &str = "Hand of Greed";
pub const HAVOC: &str = "Havoc";
pub const HEADBUTT: &str = "Headbutt";
pub const HEATSINKS: &str = "Heatsinks";
pub const HEAVY_BLADE: &str = "Heavy Blade";
pub const HEEL_HOOK: &str = "Heel Hook";
pub const HELLO_WORLD: &str = "Hello World";
pub const HEMOKINESIS: &str = "Hemokinesis";
pub const HOLOGRAM: &str = "Hologram";
pub const HYPERBEAM: &str = "Hyperbeam";
pub const IMMOLATE: &str = "Immolate";
pub const IMPATIENCE: &str = "Impatience";
pub const IMPERVIOUS: &str = "Impervious";
pub const INDIGNATION: &str = "Indignation";
pub const INFERNAL_BLADE: &str = "Infernal Blade";
pub const INFINITE_BLADES: &str = "Infinite Blades";
pub const INFLAME: &str = "Inflame";
pub const INJURY: &str = "Injury";
pub const INNER_PEACE: &str = "Inner Peace";
pub const INSIGHT: &str = "Insight";
pub const INTIMIDATE: &str = "Intimidate";
pub const IRON_WAVE: &str = "Iron Wave";
pub const JACK_OF_ALL_TRADES: &str = "Jack of All Trades";
pub const JAX: &str = "Jax";
pub const JUDGMENT: &str = "Judgment";
pub const JUGGERNAUT: &str = "Juggernaut";
pub const JUST_LUCKY: &str = "Just Lucky";
pub const LEAP: &str = "Leap";
pub const LEG_SWEEP: &str = "Leg Sweep";
pub const LESSON_LEARNED: &str = "Lesson Learned";
pub const LIKE_WATER: &str = "Like Water";
pub const LIMIT_BREAK: &str = "Limit Break";
pub const LOOP: &str = "Loop";
pub const MACHINE_LEARNING: &str = "Machine Learning";
pub const MADNESS: &str = "Madness";
pub const MAGNETISM: &str = "Magnetism";
pub const MALAISE: &str = "Malaise";
pub const MASTERFUL_STAB: &str = "Masterful Stab";
pub const MASTER_OF_STRATEGY: &str = "Master of Strategy";
pub const MASTER_REALITY: &str = "Master Reality";
pub const MAYHEM: &str = "Mayhem";
pub const MEDITATE: &str = "Meditate";
pub const MELTER: &str = "Melter";
pub const MENTAL_FORTRESS: &str = "Mental Fortress";
pub const METALLICIZE: &str = "Metallicize";
pub const METAMORPHOSIS: &str = "Metamorphosis";
pub const METEOR_STRIKE: &str = "Meteor Strike";
pub const MIND_BLAST: &str = "Mind Blast";
pub const MIRACLE: &str = "Miracle";
pub const MULTI_CAST: &str = "Multi-Cast";
pub const NECRONOMICURSE: &str = "Necronomicurse";
pub const NEUTRALIZE: &str = "Neutralize";
pub const NIGHTMARE: &str = "Nightmare";
pub const NIRVANA: &str = "Nirvana";
pub const NORMALITY: &str = "Normality";
pub const NOXIOUS_FUMES: &str = "Noxious Fumes";
pub const OFFERING: &str = "Offering";
pub const OMEGA: &str = "Omega";
pub const OMNISCIENCE: &str = "Omniscience";
pub const OUTMANEUVER: &str = "Outmaneuver";
pub const OVERCLOCK: &str = "Overclock";
pub const PAIN: &str = "Pain";
pub const PANACEA: &str = "Panacea";
pub const PANACHE: &str = "Panache";
pub const PANIC_BUTTON: &str = "Panic Button";
pub const PARASITE: &str = "Parasite";
pub const PERFECTED_STRIKE: &str = "Perfected Strike";
pub const PERSEVERANCE: &str = "Perseverance";
pub const PHANTASMAL_KILLER: &str = "Phantasmal Killer";
pub const PIERCING_WAIL: &str = "Piercing Wail";
pub const POMMEL_STRIKE: &str = "Pommel Strike";
pub const POSIONED_STAB: &str = "Posioned Stab";
pub const POWER_THROUGH: &str = "Power Through";
pub const PRAY: &str = "Pray";
pub const PREDATOR: &str = "Predator";
pub const PREPARED: &str = "Prepared";
pub const PRESSURE_POINTS: &str = "Pressure Points";
pub const PRIDE: &str = "Pride";
pub const PROSTRATE: &str = "Prostrate";
pub const PROTECT: &str = "Protect";
pub const PUMMEL: &str = "Pummel";
pub const PURITY: &str = "Purity";
pub const QUICK_SLASH: &str = "Quick Slash";
pub const RAGE: &str = "Rage";
pub const RAGNAROK: &str = "Ragnarok";
pub const RAINBOW: &str = "Rainbow";
pub const RAMPAGE: &str = "Rampage";
pub const REACH_HEAVEN: &str = "Reach Heaven";
pub const REAPER: &str = "Reaper";
pub const REBOOT: &str = "Reboot";
pub const REBOUND: &str = "Rebound";
pub const RECKLESS_CHARGE: &str = "Reckless Charge";
pub const RECURSION: &str = "Recursion";
pub const RECYCLE: &str = "Recycle";
pub const REFLEX: &str = "Reflex";
pub const REGRET: &str = "Regret";
pub const REINFORCED_BODY: &str = "Reinforced Body";
pub const REPROGRAM: &str = "Reprogram";
pub const RIDDLE_WITH_HOLES: &str = "Riddle with Holes";
pub const RIP_AND_TEAR: &str = "Rip and Tear";
pub const RITUAL_DAGGER: &str = "Ritual Dagger";
pub const RUPTURE: &str = "Rupture";
pub const RUSHDOWN: &str = "Rushdown";
pub const SADISTIC_NATURE: &str = "Sadistic Nature";
pub const SAFETY: &str = "Safety";
pub const SANCTITY: &str = "Sanctity";
pub const SANDS_OF_TIME: &str = "Sands of Time";
pub const SASH_WHIP: &str = "Sash Whip";
pub const SCRAPE: &str = "Scrape";
pub const SCRAWL: &str = "Scrawl";
pub const SEARING_BLOW: &str = "Searing Blow";
pub const SECOND_WIND: &str = "Second Wind";
pub const SECRET_TECHIQUE: &str = "Secret Techique";
pub const SECRET_WEAPON: &str = "Secret Weapon";
pub const SEEING_RED: &str = "Seeing Red";
pub const SEEK: &str = "Seek";
pub const SELF_REPAIR: &str = "Self Repair";
pub const SENTINEL: &str = "Sentinel";
pub const SETUP: &str = "Setup";
pub const SEVER_SOUL: &str = "Sever Soul";
pub const SHAME: &str = "Shame";
pub const SHIV: &str = "Shiv";
pub const SHOCKWAVE: &str = "Shockwave";
pub const SHRUG_IT_OFF: &str = "Shrug It Off";
pub const SIGNATURE_MOVE: &str = "Signature Move";
pub const SIMMERING_FURY: &str = "Simmering Fury";
pub const SKEWER: &str = "Skewer";
pub const SKIM: &str = "Skim";
pub const SLICE: &str = "Slice";
pub const SLIMED: &str = "Slimed";
pub const SMITE: &str = "Smite";
pub const SNEAKY_STRIKE: &str = "Sneaky Strike";
pub const SPIRIT_SHIELD: &str = "Spirit Shield";
pub const SPOT_WEAKNESS: &str = "Spot Weakness";
pub const STACK: &str = "Stack";
pub const STATIC_DISCHARGE: &str = "Static Discharge";
pub const STEAM_BARRIER: &str = "Steam Barrier";
pub const STORM: &str = "Storm";
pub const STORM_OF_STEEL: &str = "Storm of Steel";
pub const STREAMLINE: &str = "Streamline";
pub const STRIKE: &str = "Strike";
pub const STUDY: &str = "Study";
pub const SUCKER_PUNCH: &str = "Sucker Punch";
pub const SUNDER: &str = "Sunder";
pub const SURVIVOR: &str = "Survivor";
pub const SWEEPING_BEAM: &str = "Sweeping Beam";
pub const SWIFT_STRIKE: &str = "Swift Strike";
pub const SWIVEL: &str = "Swivel";
pub const SWORD_BOOMERANG: &str = "Sword Boomerang";
pub const TACTICIAN: &str = "Tactician";
pub const TALK_TO_THE_HAND: &str = "Talk to the Hand";
pub const TANTRUM: &str = "Tantrum";
pub const TEMPEST: &str = "Tempest";
pub const TERROR: &str = "Terror";
pub const THE_BOMB: &str = "The Bomb";
pub const THINKING_AHEAD: &str = "Thinking Ahead";
pub const THIRD_EYE: &str = "Third Eye";
pub const THROUGH_VIOLENCE: &str = "Through Violence";
pub const THUNDERCLAP: &str = "Thunderclap";
pub const THUNDER_STRIKE: &str = "Thunder Strike";
pub const TOOLS_OF_THE_TRADE: &str = "Tools of the Trade";
pub const TRANQUILITY: &str = "Tranquility";
pub const TRANSMUTATION: &str = "Transmutation";
pub const TRIP: &str = "Trip";
pub const TRUE_GRIT: &str = "True Grit";
pub const TURBO: &str = "Turbo";
pub const TWIN_STRIKE: &str = "Twin Strike";
pub const UNLOAD: &str = "Unload";
pub const UPPERCUT: &str = "Uppercut";
pub const VAULT: &str = "Vault";
pub const VIGILANCE: &str = "Vigilance";
pub const VIOLENCE: &str = "Violence";
pub const VOID: &str = "Void";
pub const WALLOP: &str = "Wallop";
pub const WARCRY: &str = "Warcry";
pub const WAVE_OF_THE_HAND: &str = "Wave of the Hand";
pub const WEAVE: &str = "Weave";
pub const WELL_LAID_PLANS: &str = "Well-Laid Plans";
pub const WHEEL_KICK: &str = "Wheel Kick";
pub const WHIRLWIND: &str = "Whirlwind";
pub const WHITE_NOISE: &str = "White Noise";
pub const WILD_STRIKE: &str = "Wild Strike";
pub const WINDMILL_STRIKE: &str = "Windmill Strike";
pub const WISH: &str = "Wish";
pub const WORSHIP: &str = "Worship";
pub const WOUND: &str = "Wound";
pub const WRAITH_FORM: &str = "Wraith Form";
pub const WREATH_OF_FLAME: &str = "Wreath of Flame";
pub const WRITHE: &str = "Writhe";
pub const ZAP: &str = "Zap";