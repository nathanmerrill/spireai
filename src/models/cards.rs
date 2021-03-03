use crate::models::buffs;
use crate::models::core::*;
use Class::*;
use CardType::*;
use Rarity::*;
use Effect::*;
use Target::*;
use CardLocation::*;
use CardEffect::*;
use Amount::*;
use RelativePosition::*;

pub const EXHAUST: CardEffect = OnPlay(ExhaustCard(This));

impl BaseCard {
    fn new(_class: Class, _type: CardType) -> Self {
        let targeted: bool = &_type == &Attack;
        Self {
            name: &"",
            rarity: Common,
            _type: _type,
            _class: _class,
            targeted: targeted,
            effects: vec![],
            on_upgrade: OnUpgrade::None,
            starting_n: 0,
            cost: 1,
            innate: false,
            ethereal: false,
        }
    }
    
    pub fn by_name(name: &str) -> BaseCard {
        match name {
            DEFEND => BaseCard { 
                name: DEFEND, 
                rarity: Starter,
                effects: vec![OnPlay(Block(Fixed(5), _Self))],
                on_upgrade: OnUpgrade::SetEffects(vec![OnPlay(Block(Fixed(8), _Self))]),
                ..BaseCard::new(Class::All, Skill)
            },
            STRIKE => BaseCard { 
                name: STRIKE, 
                rarity: Starter,
                effects: vec![OnPlay(AttackDamage(Fixed(6), TargetEnemy))],
                on_upgrade: OnUpgrade::SetEffects(vec![OnPlay(AttackDamage(Fixed(9), TargetEnemy))]),
                ..BaseCard::new(Class::All, Attack)
            },
            BASH => BaseCard { 
                name: BASH, 
                rarity: Starter,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(8), TargetEnemy)), 
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(2), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(10), TargetEnemy)), 
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(3), TargetEnemy)),
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            ANGER => BaseCard { 
                name: ANGER, 
                effects: vec![
                    OnPlay(AttackDamage(Fixed(6), TargetEnemy)),
                    OnPlay(AddCard{
                        card: CardReference::CopyOf(This), 
                        destination: DiscardPile(Bottom), 
                        copies: Fixed(1),
                        modifier: CardModifier::None
                    })
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(8), TargetEnemy)), 
                    OnPlay(AddCard{
                        card: CardReference::CopyOf(This), 
                        destination: DiscardPile(Bottom), 
                        copies: Fixed(1),
                        modifier: CardModifier::None
                    })
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Attack)
            },
            ARMAMENTS => BaseCard { 
                name: ARMAMENTS, 
                effects: vec![
                    OnPlay(Block(Fixed(5), _Self)),
                    OnPlay(UpgradeCard(PlayerHand(Random))),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(8), TargetEnemy)), 
                    OnPlay(UpgradeCard(PlayerHand(RelativePosition::All))),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            BODY_SLAM => BaseCard { 
                name: BODY_SLAM, 
                effects: vec![
                    OnPlay(Block(Amount::Custom, _Self)),
                ],
                on_upgrade: OnUpgrade::ReduceCost(0),
                ..BaseCard::new(Ironclad, Attack)
            },
            CLASH => BaseCard { 
                name: CLASH, 
                effects: vec![
                    CustomPlayable,
                    OnPlay(AttackDamage(Fixed(14), TargetEnemy)), 
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    CustomPlayable,
                    OnPlay(AttackDamage(Fixed(18), TargetEnemy)), 
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Attack)
            },
            CLEAVE => BaseCard { 
                name: CLEAVE, 
                targeted: false,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(8), AllEnemies)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(11), AllEnemies)),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            CLOTHESLINE => BaseCard { 
                name: CLOTHESLINE, 
                effects: vec![
                    OnPlay(AttackDamage(Fixed(12), TargetEnemy)),
                    OnPlay(AddBuff(buffs::WEAK, Fixed(2), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(14), TargetEnemy)),
                    OnPlay(AddBuff(buffs::WEAK, Fixed(3), TargetEnemy)),
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            FLEX => BaseCard { 
                name: FLEX, 
                effects: vec![
                    OnPlay(AddBuff(buffs::STRENGTH, Fixed(2), _Self)),
                    OnPlay(AddBuff(buffs::STRENGTH_DOWN, Fixed(2), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::STRENGTH, Fixed(4), _Self)),
                    OnPlay(AddBuff(buffs::STRENGTH_DOWN, Fixed(4), _Self)),
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Skill)
            },
            HAVOC => BaseCard { 
                name: HAVOC, 
                effects: vec![
                    OnPlay(AutoPlayCard(DrawPile(Top))),
                ],
                on_upgrade: OnUpgrade::ReduceCost(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            HEADBUTT => BaseCard { 
                name: HEADBUTT, 
                effects: vec![
                    OnPlay(AttackDamage(Fixed(9), TargetEnemy)), 
                    OnPlay(MoveCard(DiscardPile(PlayerChoice(1)), DrawPile(Top))),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(12), TargetEnemy)),
                    OnPlay(MoveCard(DiscardPile(PlayerChoice(1)), DrawPile(Top))),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            HEAVY_BLADE => BaseCard { 
                name: HEAVY_BLADE, 
                effects: vec![
                    OnPlay(AttackDamage(Amount::Custom, TargetEnemy)),
                ],
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            IRON_WAVE => BaseCard { 
                name: IRON_WAVE, 
                effects: vec![
                    OnPlay(AttackDamage(Fixed(5), TargetEnemy)),
                    OnPlay(Block(Fixed(5), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(7), TargetEnemy)),
                    OnPlay(Block(Fixed(7), _Self)),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            PERFECTED_STRIKE => BaseCard { 
                name: PERFECTED_STRIKE, 
                effects: vec![
                    OnPlay(AttackDamage(Amount::Custom, TargetEnemy)),
                ],
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            SHRUG_IT_OFF => BaseCard { 
                name: SHRUG_IT_OFF, 
                effects: vec![
                    OnPlay(Block(Fixed(8), _Self)),
                    OnPlay(Draw(Fixed(1))),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Block(Fixed(11), _Self)),
                    OnPlay(Draw(Fixed(1))),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            SWORD_BOOMERANG => BaseCard { 
                name: SWORD_BOOMERANG, 
                targeted: false,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(3), RandomEnemy)),
                    OnPlay(AttackDamage(Fixed(3), RandomEnemy)),
                    OnPlay(AttackDamage(Fixed(3), RandomEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(3), RandomEnemy)),
                    OnPlay(AttackDamage(Fixed(3), RandomEnemy)),
                    OnPlay(AttackDamage(Fixed(3), RandomEnemy)),
                    OnPlay(AttackDamage(Fixed(3), RandomEnemy)),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            THUNDERCLAP => BaseCard { 
                name: THUNDERCLAP, 
                _type: Attack,
                targeted: false,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(4), AllEnemies)),
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(1), AllEnemies)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(7), AllEnemies)),
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(1), AllEnemies)),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            TRUE_GRIT => BaseCard { 
                name: TRUE_GRIT, 
                effects: vec![
                    OnPlay(Block(Fixed(7), _Self)),
                    OnPlay(ExhaustCard(PlayerHand(Random))),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Block(Fixed(9), _Self)),
                    OnPlay(ExhaustCard(PlayerHand(PlayerChoice(1)))),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            TWIN_STRIKE => BaseCard { 
                name: TWIN_STRIKE, 
                effects: vec![
                    OnPlay(AttackDamage(Fixed(5), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(5), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(7), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(7), TargetEnemy)),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            WARCRY => BaseCard { 
                name: WARCRY, 
                effects: vec![
                    OnPlay(Draw(Fixed(1))),
                    OnPlay(MoveCard(
                        PlayerHand(PlayerChoice(1)), 
                        DrawPile(Top))
                    ),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Draw(Fixed(2))),
                    OnPlay(MoveCard(
                        PlayerHand(PlayerChoice(1)), 
                        DrawPile(Top))
                    ),
                    EXHAUST,
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Skill)
            },
            WILD_STRIKE => BaseCard { 
                name: WILD_STRIKE, 
                effects: vec![
                    OnPlay(AttackDamage(Fixed(12), TargetEnemy)),
                    OnPlay(AddCard{
                        card: CardReference::ByName(WOUND), 
                        destination: DrawPile(Random), 
                        copies: Fixed(1),
                        modifier: CardModifier::None
                    }),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(17), TargetEnemy)),
                    OnPlay(AddCard{
                        card: CardReference::ByName(WOUND), 
                        destination: DrawPile(Random), 
                        copies: Fixed(1),
                        modifier: CardModifier::None
                    }),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            BATTLE_TRANCE => BaseCard { 
                name: BATTLE_TRANCE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(Draw(Fixed(3))),
                    OnPlay(AddBuff(buffs::NO_DRAW, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Draw(Fixed(4))),
                    OnPlay(AddBuff(buffs::NO_DRAW, Fixed(1), _Self)),
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Skill)
            },
            BLOOD_FOR_BLOOD => BaseCard { 
                name: BLOOD_FOR_BLOOD, 
                rarity: Uncommon,
                effects: vec![
                    CustomCardCost,
                    OnPlay(AttackDamage(Fixed(18), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    CustomCardCost,
                    OnPlay(AttackDamage(Fixed(22), TargetEnemy)),
                ]),
                cost: 4,
                ..BaseCard::new(Ironclad, Attack)
            },
            BLOODLETTING => BaseCard { 
                name: BLOODLETTING, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(LoseHp(Fixed(3), _Self)),
                    OnPlay(AddEnergy(Fixed(2))),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(LoseHp(Fixed(3), _Self)),
                    OnPlay(AddEnergy(Fixed(3))),
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Skill)
            },
            BURNING_PACT => BaseCard { 
                name: BURNING_PACT, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(ExhaustCard(PlayerHand(PlayerChoice(1)))),
                    OnPlay(Draw(Fixed(2))),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(ExhaustCard(PlayerHand(PlayerChoice(1)))),
                    OnPlay(Draw(Fixed(3))),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            CARNAGE => BaseCard { 
                name: CARNAGE, 
                rarity: Uncommon,
                ethereal: true,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(20), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(28), TargetEnemy)),
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            COMBUST => BaseCard { 
                name: COMBUST, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::COMBUST, Fixed(5), _Self)),
                    OnPlay(AddBuffN(buffs::COMBUST, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::COMBUST, Fixed(7), _Self)),
                    OnPlay(AddBuffN(buffs::COMBUST, Fixed(1), _Self)),
                ]),
                ..BaseCard::new(Ironclad, Power)
            },
            DARK_EMBRACE => BaseCard { 
                name: DARK_EMBRACE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::DARK_EMBRACE, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::ReduceCost(1),
                cost: 2,
                ..BaseCard::new(Ironclad, Power)
            },
            DISARM => BaseCard { 
                name: DISARM, 
                rarity: Uncommon,
                targeted: true,
                effects: vec![
                    OnPlay(AddBuff(buffs::STRENGTH, Fixed(-2), TargetEnemy)),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::STRENGTH, Fixed(-3), TargetEnemy)),
                    EXHAUST,
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            DROPKICK => BaseCard { 
                name: DROPKICK, 
                rarity: Uncommon,
                targeted: true,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(5), TargetEnemy)),
                    OnPlay(If(Condition::Status(TargetEnemy, buffs::VULNERABLE), vec![
                        AddEnergy(Fixed(1)),
                        Draw(Fixed(1)),
                    ]))
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(8), TargetEnemy)),
                    OnPlay(If(Condition::Status(TargetEnemy, buffs::VULNERABLE), vec![
                        AddEnergy(Fixed(1)),
                        Draw(Fixed(1)),
                    ]))
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            DUAL_WIELD => BaseCard { 
                name: DUAL_WIELD, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddCard{
                        card: CardReference::CopyOf(PlayerHand(PlayerChoice(1))), 
                        destination: PlayerHand(Bottom), 
                        copies: Fixed(1),
                        modifier: CardModifier::None
                    }),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddCard{
                        card: CardReference::CopyOf(PlayerHand(PlayerChoice(1))), 
                        destination: PlayerHand(Bottom),
                        copies: Fixed(2),
                        modifier: CardModifier::None
                    }),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            ENTRENCH => BaseCard { 
                name: ENTRENCH, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(Block(Amount::Custom, _Self))
                ],
                on_upgrade: OnUpgrade::ReduceCost(1),
                cost: 2,
                ..BaseCard::new(Ironclad, Skill)
            },
            EVOLVE => BaseCard { 
                name: EVOLVE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::EVOLVE, Fixed(1), _Self))
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::EVOLVE, Fixed(2), _Self))
                ]),
                ..BaseCard::new(Ironclad, Power)
            },
            FEEL_NO_PAIN => BaseCard { 
                name: FEEL_NO_PAIN, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::FEEL_NO_PAIN, Fixed(3), _Self))
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::FEEL_NO_PAIN, Fixed(4), _Self))
                ]),
                ..BaseCard::new(Ironclad, Power)
            },
            FIRE_BREATHING => BaseCard { 
                name: FIRE_BREATHING, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::FIRE_BREATHING, Fixed(6), _Self))
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::FIRE_BREATHING, Fixed(10), _Self))
                ]),
                ..BaseCard::new(Ironclad, Power)
            },
            FLAME_BARRIER => BaseCard { 
                name: FLAME_BARRIER, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(Block(Fixed(12), _Self)),
                    OnPlay(AddBuff(buffs::FLAME_BARRIER, Fixed(4), _Self))
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Block(Fixed(16), _Self)),
                    OnPlay(AddBuff(buffs::FLAME_BARRIER, Fixed(6), _Self))
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Skill)
            },
            GHOSTLY_ARMOR => BaseCard { 
                name: GHOSTLY_ARMOR, 
                rarity: Uncommon,
                targeted: false,
                ethereal: true,
                effects: vec![
                    OnPlay(Block(Fixed(10), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Block(Fixed(13), _Self)),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            HEMOKINESIS => BaseCard { 
                name: HEMOKINESIS, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(LoseHp(Fixed(2), _Self)),
                    OnPlay(AttackDamage(Fixed(15), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(LoseHp(Fixed(2), _Self)),
                    OnPlay(AttackDamage(Fixed(20), TargetEnemy)),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            INFERNAL_BLADE => BaseCard { 
                name: INFERNAL_BLADE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddCard{
                        card: CardReference::RandomType(Attack),
                        destination: PlayerHand(Bottom), 
                        copies: Fixed(1),
                        modifier: CardModifier::SetZeroTurnCost
                    }),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::ReduceCost(0),
                ..BaseCard::new(Ironclad, Attack)
            },
            INFLAME => BaseCard { 
                name: INFLAME, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::STRENGTH, Fixed(2), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::STRENGTH, Fixed(3), _Self)),
                ]),
                ..BaseCard::new(Ironclad, Power)
            },
            INTIMIDATE => BaseCard { 
                name: INTIMIDATE, 
                rarity: Uncommon,
                _type: Power,
                effects: vec![
                    OnPlay(AddBuff(buffs::WEAK, Fixed(1), AllEnemies)),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::WEAK, Fixed(2), AllEnemies)),
                    EXHAUST,
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Power)
            },
            METALLICIZE => BaseCard { 
                name: METALLICIZE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::METALLICIZE, Fixed(3), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::METALLICIZE, Fixed(4), _Self)),
                ]),
                ..BaseCard::new(Ironclad, Power)
            },
            POWER_THROUGH => BaseCard { 
                name: POWER_THROUGH, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddCard{
                        card: CardReference::ByName(WOUND), 
                        destination: PlayerHand(Bottom), 
                        copies: Fixed(2),
                        modifier: CardModifier::None
                    }),
                    OnPlay(Block(Fixed(15), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddCard{
                        card: CardReference::ByName(WOUND), 
                        destination: PlayerHand(Bottom),
                        copies: Fixed(2),
                        modifier: CardModifier::None
                    }),
                    OnPlay(Block(Fixed(20), _Self)),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            PUMMEL => BaseCard { 
                name: PUMMEL, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    OnPlay(AttackDamage(Fixed(2), TargetEnemy)),
                    EXHAUST,
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            RAGE => BaseCard { 
                name: RAGE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::RAGE, Fixed(3), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::RAGE, Fixed(5), _Self)),
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Skill)
            },
            RAMPAGE => BaseCard { 
                name: RAMPAGE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AttackDamage(Amount::Custom, TargetEnemy)),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            RECKLESS_CHARGE => BaseCard { 
                name: RECKLESS_CHARGE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(7), TargetEnemy)),
                    OnPlay(AddCard{
                        card: CardReference::ByName(DAZED), 
                        destination: DrawPile(Random), 
                        copies: Fixed(1),
                        modifier: CardModifier::None
                    }),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(10), TargetEnemy)),
                    OnPlay(AddCard{
                        card: CardReference::ByName(DAZED), 
                        destination: DrawPile(Random), 
                        copies: Fixed(1),
                        modifier: CardModifier::None
                    }),
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            RUPTURE => BaseCard { 
                name: RUPTURE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::RUPTURE, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::RUPTURE, Fixed(2), _Self)),
                ]),
                ..BaseCard::new(Ironclad, Power)
            },
            SEARING_BLOW => BaseCard { 
                name: SEARING_BLOW, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AttackDamage(Amount::Custom, TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SearingBlow,
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            SECOND_WIND => BaseCard { 
                name: SECOND_WIND, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(Effect::Custom),
                ],
                ..BaseCard::new(Ironclad, Skill)
            },
            SEEING_RED => BaseCard { 
                name: SEEING_RED, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddEnergy(Fixed(2))),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::ReduceCost(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            SENTINEL => BaseCard { 
                name: SENTINEL, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(Block(Fixed(5), _Self)),
                    OnExhaust(AddEnergy(Fixed(2))),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Block(Fixed(8), _Self)),
                    OnExhaust(AddEnergy(Fixed(3))),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            SEVER_SOUL => BaseCard { 
                name: SEVER_SOUL, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(Effect::Custom),
                    OnPlay(AttackDamage(Fixed(16), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Effect::Custom),
                    OnPlay(AttackDamage(Fixed(20), TargetEnemy)),
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            SHOCKWAVE => BaseCard { 
                name: SHOCKWAVE, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(3), AllEnemies)),
                    OnPlay(AddBuff(buffs::WEAK, Fixed(3), AllEnemies)),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(5), AllEnemies)),
                    OnPlay(AddBuff(buffs::WEAK, Fixed(5), AllEnemies)),
                    EXHAUST,
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Skill)
            },
            SPOT_WEAKNESS => BaseCard { 
                name: SPOT_WEAKNESS, 
                rarity: Uncommon,
                targeted: true,
                effects: vec![
                    OnPlay(If(Condition::Attacking(TargetEnemy), vec![
                        AddBuff(buffs::STRENGTH, Fixed(3), _Self)
                    ])),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(If(Condition::Attacking(TargetEnemy), vec![
                        AddBuff(buffs::STRENGTH, Fixed(4), _Self)
                    ])),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            UPPERCUT => BaseCard { 
                name: UPPERCUT, 
                rarity: Uncommon,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(13), TargetEnemy)),
                    OnPlay(AddBuff(buffs::WEAK, Fixed(1), TargetEnemy)),
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(1), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(13), TargetEnemy)),
                    OnPlay(AddBuff(buffs::WEAK, Fixed(2), TargetEnemy)),
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(2), TargetEnemy)),
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            WHIRLWIND => BaseCard { 
                name: WHIRLWIND, 
                rarity: Uncommon,
                targeted: false,
                effects: vec![
                    OnPlay(
                        Effect::Repeat(X, Box::new(Effect::AttackDamage(Fixed(5), AllEnemies)))
                    )
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(
                        Effect::Repeat(X, Box::new(Effect::AttackDamage(Fixed(8), AllEnemies)))
                    )
                ]),
                cost: -1,
                ..BaseCard::new(Ironclad, Attack)
            },
            BARRICADE => BaseCard { 
                name: BARRICADE, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddBuff(buffs::BARRICADE, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::ReduceCost(2),
                cost: 3,
                ..BaseCard::new(Ironclad, Power)
            },
            BERSERK => BaseCard { 
                name: BERSERK, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(2), _Self)),
                    OnPlay(AddBuff(buffs::BERSERK, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::VULNERABLE, Fixed(1), _Self)),
                    OnPlay(AddBuff(buffs::BERSERK, Fixed(1), _Self)),
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Power)
            },
            BLUDGEON => BaseCard { 
                name: BLUDGEON, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(32), TargetEnemy)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(42), TargetEnemy)),
                ]),
                cost: 3,
                ..BaseCard::new(Ironclad, Attack)
            },
            BRUTALITY => BaseCard { 
                name: BRUTALITY, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddBuff(buffs::BRUTALITY, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::Innate,
                cost: 0,
                ..BaseCard::new(Ironclad, Power)
            },
            CORRUPTION => BaseCard { 
                name: CORRUPTION, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddBuff(buffs::CORRUPTION, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::ReduceCost(2),
                cost: 3,
                ..BaseCard::new(Ironclad, Power)
            },
            DEMON_FORM => BaseCard { 
                name: DEMON_FORM, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddBuff(buffs::DEMON_FORM, Fixed(2), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::DEMON_FORM, Fixed(3), _Self)),
                ]),
                cost: 3,
                ..BaseCard::new(Ironclad, Power)
            },
            DOUBLE_TAP => BaseCard { 
                name: DOUBLE_TAP, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddBuff(buffs::DOUBLE_TAP, Fixed(1), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::DOUBLE_TAP, Fixed(2), _Self)),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            EXHUME => BaseCard { 
                name: EXHUME, 
                rarity: Rare,
                effects: vec![
                    OnPlay(MoveCard(
                        ExhaustPile(PlayerChoice(1)), 
                        PlayerHand(Bottom))
                    ),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::ReduceCost(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            FEED => BaseCard { 
                name: FEED, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AttackDamage(Fixed(10), TargetEnemy)),
                    IfFatal(vec![AddMaxHp(Fixed(3))]),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AttackDamage(Fixed(12), TargetEnemy)),
                    IfFatal(vec![AddMaxHp(Fixed(3))]),
                    EXHAUST,
                ]),
                ..BaseCard::new(Ironclad, Attack)
            },
            FIEND_FIRE => BaseCard { 
                name: FIEND_FIRE, 
                rarity: Rare,
                effects: vec![
                    OnPlay(Effect::Custom),
                    EXHAUST,
                ],
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            IMMOLATE => BaseCard { 
                name: IMMOLATE, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddCard{
                        card: CardReference::ByName(BURN), 
                        destination: DiscardPile(Bottom), 
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    }),
                ],
                cost: 2,
                ..BaseCard::new(Ironclad, Attack)
            },
            IMPERVIOUS => BaseCard { 
                name: IMPERVIOUS, 
                rarity: Rare,
                effects: vec![
                    OnPlay(Block(Fixed(30), _Self)),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(Block(Fixed(40), _Self)),
                    EXHAUST,
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Skill)
            },
            JUGGERNAUT => BaseCard { 
                name: JUGGERNAUT, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddBuff(buffs::JUGGERNAUT, Fixed(5), _Self)),
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::JUGGERNAUT, Fixed(7), _Self)),
                ]),
                cost: 2,
                ..BaseCard::new(Ironclad, Power)
            },
            LIMIT_BREAK => BaseCard { 
                name: LIMIT_BREAK, 
                rarity: Rare,
                effects: vec![
                    OnPlay(AddBuff(buffs::STRENGTH, Amount::Custom, _Self)),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(AddBuff(buffs::STRENGTH, Amount::Custom, _Self)),
                ]),
                ..BaseCard::new(Ironclad, Skill)
            },
            OFFERING => BaseCard { 
                name: OFFERING, 
                rarity: Rare,
                effects: vec![
                    OnPlay(LoseHp(Fixed(6), _Self)),
                    OnPlay(AddEnergy(Fixed(2))),
                    OnPlay(Draw(Fixed(3))),
                    EXHAUST,
                ],
                on_upgrade: OnUpgrade::SetEffects(vec![
                    OnPlay(LoseHp(Fixed(6), _Self)),
                    OnPlay(AddEnergy(Fixed(2))),
                    OnPlay(Draw(Fixed(5))),
                    EXHAUST,
                ]),
                cost: 0,
                ..BaseCard::new(Ironclad, Skill)
            },
            REAPER => BaseCard { 
                name: REAPER, 
                rarity: Rare,
                effects: vec![
                    OnPlay(Effect::Custom),
                    EXHAUST,
                ],
                cost: 0,
                ..BaseCard::new(Ironclad, Attack)
            },
            _ => panic!("Unsupported card")
        }
    }
}

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