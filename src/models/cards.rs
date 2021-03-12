use crate::models::buffs;
use crate::models::core::*;
use Amount::*;
use CardLocation::*;
use CardType::*;
use Class::*;
use Effect::*;
use Rarity::{Rare, Starter, Uncommon};
use RelativePosition::*;
use Target::*;

impl BaseCard {
    fn new(_class: Class, _type: CardType) -> Self {
        Self {
            name: &"",
            rarity: Rarity::Common,
            _type: _type,
            _class: _class,
            effects: vec![],
            on_play: vec![],
            on_discard: vec![],
            on_exhaust: vec![],
            on_draw: vec![],
            on_retain: vec![],
            cost: Fixed(1),
            upgradeable: Upgradeable::Once,
            innate: Condition::Never,
            ethereal: Condition::Never,
            playable_if: Condition::Always,
            retain: Condition::Never,
        }
    }

    pub fn by_name(name: &str) -> BaseCard {
        match name {
            DEFEND => BaseCard {
                name: DEFEND,
                rarity: Starter,
                on_play: vec![Block(Upgradable(5, 8), _Self)],
                ..BaseCard::new(Class::All, Skill)
            },
            STRIKE => BaseCard {
                name: STRIKE,
                rarity: Starter,
                on_play: vec![AttackDamage(Upgradable(6, 9), TargetEnemy)],
                ..BaseCard::new(Class::All, Attack)
            },
            BASH => BaseCard {
                name: BASH,
                rarity: Starter,
                on_play: vec![
                    AttackDamage(Upgradable(8, 10), TargetEnemy),
                    AddBuff(buffs::VULNERABLE, Upgradable(2, 3), TargetEnemy),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            ANGER => BaseCard {
                name: ANGER,
                on_play: vec![
                    AttackDamage(Upgradable(6, 8), TargetEnemy),
                    AddCard {
                        card: CardReference::CopyOf(This),
                        destination: DiscardPile(Bottom),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Attack)
            },
            ARMAMENTS => BaseCard {
                name: ARMAMENTS,
                on_play: vec![
                    Block(Upgradable(5, 8), _Self),
                    If(
                        Condition::Upgraded,
                        vec![UpgradeCard(PlayerHand(Random))],
                        vec![UpgradeCard(PlayerHand(RelativePosition::All))],
                    ),
                ],
                ..BaseCard::new(Ironclad, Skill)
            },
            BODY_SLAM => BaseCard {
                name: BODY_SLAM,
                on_play: vec![Block(Amount::Custom, _Self)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Ironclad, Attack)
            },
            CLASH => BaseCard {
                name: CLASH,
                playable_if: Condition::Custom,
                on_play: vec![AttackDamage(Upgradable(14, 18), TargetEnemy)],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Attack)
            },
            CLEAVE => BaseCard {
                name: CLEAVE,

                on_play: vec![AttackDamage(Upgradable(8, 11), AllEnemies)],
                ..BaseCard::new(Ironclad, Attack)
            },
            CLOTHESLINE => BaseCard {
                name: CLOTHESLINE,
                on_play: vec![
                    AttackDamage(Upgradable(12, 14), TargetEnemy),
                    AddBuff(buffs::WEAK, Upgradable(2, 3), TargetEnemy),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            FLEX => BaseCard {
                name: FLEX,
                on_play: vec![
                    AddBuff(buffs::STRENGTH, Upgradable(2, 4), _Self),
                    AddBuff(buffs::STRENGTH_DOWN, Upgradable(2, 4), _Self),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            HAVOC => BaseCard {
                name: HAVOC,
                on_play: vec![AutoPlayCard(DrawPile(Top))],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Ironclad, Skill)
            },
            HEADBUTT => BaseCard {
                name: HEADBUTT,
                on_play: vec![
                    AttackDamage(Upgradable(9, 12), TargetEnemy),
                    MoveCard(
                        DiscardPile(PlayerChoice(Fixed(1))),
                        DrawPile(Top),
                        CardModifier::None,
                    ),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            HEAVY_BLADE => BaseCard {
                name: HEAVY_BLADE,
                on_play: vec![AttackDamage(Amount::Custom, TargetEnemy)],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            IRON_WAVE => BaseCard {
                name: IRON_WAVE,
                on_play: vec![
                    AttackDamage(Upgradable(5, 7), TargetEnemy),
                    Block(Upgradable(5, 7), _Self),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            PERFECTED_STRIKE => BaseCard {
                name: PERFECTED_STRIKE,
                on_play: vec![AttackDamage(Amount::Custom, TargetEnemy)],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            SHRUG_IT_OFF => BaseCard {
                name: SHRUG_IT_OFF,
                on_play: vec![Block(Upgradable(8, 11), _Self), Draw(Fixed(1))],
                ..BaseCard::new(Ironclad, Skill)
            },
            SWORD_BOOMERANG => BaseCard {
                name: SWORD_BOOMERANG,

                on_play: vec![
                    AttackDamage(Fixed(3), RandomEnemy),
                    AttackDamage(Fixed(3), RandomEnemy),
                    AttackDamage(Fixed(3), RandomEnemy),
                    If(
                        Condition::Upgraded,
                        vec![AttackDamage(Fixed(3), RandomEnemy)],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            THUNDERCLAP => BaseCard {
                name: THUNDERCLAP,
                _type: Attack,

                on_play: vec![
                    AttackDamage(Upgradable(4, 7), AllEnemies),
                    AddBuff(buffs::VULNERABLE, Fixed(1), AllEnemies),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            TRUE_GRIT => BaseCard {
                name: TRUE_GRIT,
                on_play: vec![
                    Block(Upgradable(7, 9), _Self),
                    If(
                        Condition::Upgraded,
                        vec![ExhaustCard(PlayerHand(Random))],
                        vec![ExhaustCard(PlayerHand(PlayerChoice(Fixed(1))))],
                    ),
                ],
                ..BaseCard::new(Ironclad, Skill)
            },
            TWIN_STRIKE => BaseCard {
                name: TWIN_STRIKE,
                on_play: vec![
                    AttackDamage(Upgradable(5, 7), TargetEnemy),
                    AttackDamage(Upgradable(5, 7), TargetEnemy),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            WARCRY => BaseCard {
                name: WARCRY,
                on_play: vec![
                    Draw(Upgradable(1, 2)),
                    MoveCard(
                        PlayerHand(PlayerChoice(Fixed(1))),
                        DrawPile(Top),
                        CardModifier::None,
                    ),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            WILD_STRIKE => BaseCard {
                name: WILD_STRIKE,
                on_play: vec![
                    AttackDamage(Upgradable(12, 17), TargetEnemy),
                    AddCard {
                        card: CardReference::ByName(WOUND),
                        destination: DrawPile(Random),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            BATTLE_TRANCE => BaseCard {
                name: BATTLE_TRANCE,
                rarity: Uncommon,
                on_play: vec![
                    Draw(Upgradable(3, 4)),
                    AddBuff(buffs::NO_DRAW, Fixed(1), _Self),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            BLOOD_FOR_BLOOD => BaseCard {
                name: BLOOD_FOR_BLOOD,
                rarity: Uncommon,
                on_play: vec![AttackDamage(Upgradable(18, 22), TargetEnemy)],
                cost: Amount::Custom,
                ..BaseCard::new(Ironclad, Attack)
            },
            BLOODLETTING => BaseCard {
                name: BLOODLETTING,
                rarity: Uncommon,
                on_play: vec![LoseHp(Fixed(3), _Self), AddEnergy(Upgradable(2, 3))],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            BURNING_PACT => BaseCard {
                name: BURNING_PACT,
                rarity: Uncommon,
                on_play: vec![
                    ExhaustCard(PlayerHand(PlayerChoice(Fixed(1)))),
                    Draw(Upgradable(2, 3)),
                ],
                ..BaseCard::new(Ironclad, Skill)
            },
            CARNAGE => BaseCard {
                name: CARNAGE,
                rarity: Uncommon,
                ethereal: Condition::Always,
                on_play: vec![AttackDamage(Upgradable(20, 28), TargetEnemy)],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            COMBUST => BaseCard {
                name: COMBUST,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::COMBUST, Upgradable(5, 7), _Self)],
                ..BaseCard::new(Ironclad, Power)
            },
            DARK_EMBRACE => BaseCard {
                name: DARK_EMBRACE,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::DARK_EMBRACE, Fixed(1), _Self)],
                cost: Upgradable(2, 1),
                ..BaseCard::new(Ironclad, Power)
            },
            DISARM => BaseCard {
                name: DISARM,
                rarity: Uncommon,

                on_play: vec![
                    AddBuff(buffs::STRENGTH, Upgradable(-2, -3), TargetEnemy),
                    ExhaustCard(This),
                ],
                ..BaseCard::new(Ironclad, Skill)
            },
            DROPKICK => BaseCard {
                name: DROPKICK,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(5, 8), TargetEnemy),
                    If(
                        Condition::Status(TargetEnemy, buffs::VULNERABLE),
                        vec![AddEnergy(Fixed(1)), Draw(Fixed(1))],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            DUAL_WIELD => BaseCard {
                name: DUAL_WIELD,
                rarity: Uncommon,
                on_play: vec![AddCard {
                    card: CardReference::CopyOf(PlayerHand(PlayerChoice(Fixed(1)))),
                    destination: PlayerHand(Bottom),
                    copies: Upgradable(1, 2),
                    modifier: CardModifier::None,
                }],
                ..BaseCard::new(Ironclad, Skill)
            },
            ENTRENCH => BaseCard {
                name: ENTRENCH,
                rarity: Uncommon,
                on_play: vec![Block(Amount::Custom, _Self)],
                cost: Upgradable(2, 1),
                ..BaseCard::new(Ironclad, Skill)
            },
            EVOLVE => BaseCard {
                name: EVOLVE,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::EVOLVE, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Ironclad, Power)
            },
            FEEL_NO_PAIN => BaseCard {
                name: FEEL_NO_PAIN,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::FEEL_NO_PAIN, Upgradable(3, 4), _Self)],
                ..BaseCard::new(Ironclad, Power)
            },
            FIRE_BREATHING => BaseCard {
                name: FIRE_BREATHING,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::FIRE_BREATHING, Upgradable(6, 10), _Self)],
                ..BaseCard::new(Ironclad, Power)
            },
            FLAME_BARRIER => BaseCard {
                name: FLAME_BARRIER,
                rarity: Uncommon,
                on_play: vec![
                    Block(Upgradable(12, 16), _Self),
                    AddBuff(buffs::FLAME_BARRIER, Upgradable(4, 6), _Self),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Skill)
            },
            GHOSTLY_ARMOR => BaseCard {
                name: GHOSTLY_ARMOR,
                rarity: Uncommon,
                ethereal: Condition::Always,
                on_play: vec![Block(Upgradable(10, 13), _Self)],
                ..BaseCard::new(Ironclad, Skill)
            },
            HEMOKINESIS => BaseCard {
                name: HEMOKINESIS,
                rarity: Uncommon,
                on_play: vec![
                    LoseHp(Fixed(2), _Self),
                    AttackDamage(Upgradable(15, 20), TargetEnemy),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            INFERNAL_BLADE => BaseCard {
                name: INFERNAL_BLADE,
                rarity: Uncommon,
                on_play: vec![
                    AddCard {
                        card: CardReference::RandomType(Attack),
                        destination: PlayerHand(Bottom),
                        copies: Fixed(1),
                        modifier: CardModifier::SetZeroTurnCost,
                    },
                    ExhaustCard(This),
                ],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Ironclad, Attack)
            },
            INFLAME => BaseCard {
                name: INFLAME,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::STRENGTH, Upgradable(2, 3), _Self)],
                ..BaseCard::new(Ironclad, Power)
            },
            INTIMIDATE => BaseCard {
                name: INTIMIDATE,
                rarity: Uncommon,
                _type: Power,
                on_play: vec![
                    AddBuff(buffs::WEAK, Upgradable(1, 2), AllEnemies),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Power)
            },
            METALLICIZE => BaseCard {
                name: METALLICIZE,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::METALLICIZE, Upgradable(3, 4), _Self)],
                ..BaseCard::new(Ironclad, Power)
            },
            POWER_THROUGH => BaseCard {
                name: POWER_THROUGH,
                rarity: Uncommon,
                on_play: vec![
                    AddCard {
                        card: CardReference::ByName(WOUND),
                        destination: PlayerHand(Bottom),
                        copies: Fixed(2),
                        modifier: CardModifier::None,
                    },
                    Block(Upgradable(15, 20), _Self),
                ],
                ..BaseCard::new(Ironclad, Skill)
            },
            PUMMEL => BaseCard {
                name: PUMMEL,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Fixed(2), TargetEnemy),
                    AttackDamage(Fixed(2), TargetEnemy),
                    AttackDamage(Fixed(2), TargetEnemy),
                    AttackDamage(Fixed(2), TargetEnemy),
                    If(
                        Condition::Upgraded,
                        vec![AttackDamage(Fixed(2), TargetEnemy)],
                        vec![],
                    ),
                    ExhaustCard(This),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            RAGE => BaseCard {
                name: RAGE,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::RAGE, Upgradable(3, 5), _Self)],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            RAMPAGE => BaseCard {
                name: RAMPAGE,
                rarity: Uncommon,
                on_play: vec![AttackDamage(Amount::Custom, TargetEnemy)],
                ..BaseCard::new(Ironclad, Attack)
            },
            RECKLESS_CHARGE => BaseCard {
                name: RECKLESS_CHARGE,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(7, 10), TargetEnemy),
                    AddCard {
                        card: CardReference::ByName(DAZED),
                        destination: DrawPile(Random),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            RUPTURE => BaseCard {
                name: RUPTURE,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::RUPTURE, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Ironclad, Power)
            },
            SEARING_BLOW => BaseCard {
                name: SEARING_BLOW,
                rarity: Uncommon,
                on_play: vec![AttackDamage(Amount::Custom, TargetEnemy)],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            SECOND_WIND => BaseCard {
                name: SECOND_WIND,
                rarity: Uncommon,
                on_play: vec![Effect::Custom],
                ..BaseCard::new(Ironclad, Skill)
            },
            SEEING_RED => BaseCard {
                name: SEEING_RED,
                rarity: Uncommon,
                on_play: vec![AddEnergy(Fixed(2)), ExhaustCard(This)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Ironclad, Skill)
            },
            SENTINEL => BaseCard {
                name: SENTINEL,
                rarity: Uncommon,
                on_play: vec![Block(Upgradable(5, 8), _Self)],
                on_exhaust: vec![AddEnergy(Upgradable(2, 3))],
                ..BaseCard::new(Ironclad, Skill)
            },
            SEVER_SOUL => BaseCard {
                name: SEVER_SOUL,
                rarity: Uncommon,
                on_play: vec![
                    Effect::Custom,
                    AttackDamage(Upgradable(16, 20), TargetEnemy),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            SHOCKWAVE => BaseCard {
                name: SHOCKWAVE,
                rarity: Uncommon,
                on_play: vec![
                    AddBuff(buffs::VULNERABLE, Upgradable(3, 5), AllEnemies),
                    AddBuff(buffs::WEAK, Upgradable(3, 5), AllEnemies),
                    ExhaustCard(This),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Skill)
            },
            SPOT_WEAKNESS => BaseCard {
                name: SPOT_WEAKNESS,
                rarity: Uncommon,

                on_play: vec![If(
                    Condition::Attacking(TargetEnemy),
                    vec![AddBuff(buffs::STRENGTH, Upgradable(3, 4), _Self)],
                    vec![],
                )],
                ..BaseCard::new(Ironclad, Skill)
            },
            UPPERCUT => BaseCard {
                name: UPPERCUT,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Fixed(13), TargetEnemy),
                    AddBuff(buffs::WEAK, Upgradable(1, 2), TargetEnemy),
                    AddBuff(buffs::VULNERABLE, Upgradable(1, 2), TargetEnemy),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            WHIRLWIND => BaseCard {
                name: WHIRLWIND,
                rarity: Uncommon,

                on_play: vec![Effect::Repeat(
                    X,
                    Box::new(Effect::AttackDamage(Upgradable(5, 8), AllEnemies)),
                )],
                cost: X,
                ..BaseCard::new(Ironclad, Attack)
            },
            BARRICADE => BaseCard {
                name: BARRICADE,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::BARRICADE, Fixed(1), _Self)],
                cost: Upgradable(3, 2),
                ..BaseCard::new(Ironclad, Power)
            },
            BERSERK => BaseCard {
                name: BERSERK,
                rarity: Rare,
                on_play: vec![
                    AddBuff(buffs::VULNERABLE, Upgradable(2, 1), _Self),
                    AddBuff(buffs::BERSERK, Fixed(1), _Self),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Power)
            },
            BLUDGEON => BaseCard {
                name: BLUDGEON,
                rarity: Rare,
                on_play: vec![AttackDamage(Upgradable(32, 42), TargetEnemy)],
                cost: Fixed(3),
                ..BaseCard::new(Ironclad, Attack)
            },
            BRUTALITY => BaseCard {
                name: BRUTALITY,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::BRUTALITY, Fixed(1), _Self)],
                innate: Condition::Upgraded,
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Power)
            },
            CORRUPTION => BaseCard {
                name: CORRUPTION,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::CORRUPTION, Fixed(1), _Self)],
                cost: Upgradable(3, 2),
                ..BaseCard::new(Ironclad, Power)
            },
            DEMON_FORM => BaseCard {
                name: DEMON_FORM,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::DEMON_FORM, Upgradable(2, 3), _Self)],
                cost: Fixed(3),
                ..BaseCard::new(Ironclad, Power)
            },
            DOUBLE_TAP => BaseCard {
                name: DOUBLE_TAP,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::DOUBLE_TAP, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Ironclad, Skill)
            },
            EXHUME => BaseCard {
                name: EXHUME,
                rarity: Rare,
                on_play: vec![
                    MoveCard(
                        ExhaustPile(PlayerChoice(Fixed(1))),
                        PlayerHand(Bottom),
                        CardModifier::None,
                    ),
                    ExhaustCard(This),
                ],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Ironclad, Skill)
            },
            FEED => BaseCard {
                name: FEED,
                rarity: Rare,
                on_play: vec![
                    AttackDamageIfFatal(
                        Upgradable(10, 2),
                        TargetEnemy,
                        vec![AddMaxHp(Upgradable(3, 4))],
                    ),
                    ExhaustCard(This),
                ],
                ..BaseCard::new(Ironclad, Attack)
            },
            FIEND_FIRE => BaseCard {
                name: FIEND_FIRE,
                rarity: Rare,
                on_play: vec![Effect::Custom, ExhaustCard(This)],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            IMMOLATE => BaseCard {
                name: IMMOLATE,
                rarity: Rare,
                on_play: vec![AddCard {
                    card: CardReference::ByName(BURN),
                    destination: DiscardPile(Bottom),
                    copies: Fixed(1),
                    modifier: CardModifier::None,
                }],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Attack)
            },
            IMPERVIOUS => BaseCard {
                name: IMPERVIOUS,
                rarity: Rare,
                on_play: vec![Block(Upgradable(30, 40), _Self), ExhaustCard(This)],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Skill)
            },
            JUGGERNAUT => BaseCard {
                name: JUGGERNAUT,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::JUGGERNAUT, Upgradable(5, 7), _Self)],
                cost: Fixed(2),
                ..BaseCard::new(Ironclad, Power)
            },
            LIMIT_BREAK => BaseCard {
                name: LIMIT_BREAK,
                rarity: Rare,
                on_play: vec![
                    AddBuff(buffs::STRENGTH, Amount::Custom, _Self),
                    If(Condition::Upgraded, vec![], vec![ExhaustCard(This)]),
                ],
                ..BaseCard::new(Ironclad, Skill)
            },
            OFFERING => BaseCard {
                name: OFFERING,
                rarity: Rare,
                on_play: vec![
                    LoseHp(Fixed(6), _Self),
                    AddEnergy(Fixed(2)),
                    Draw(Upgradable(3, 5)),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Skill)
            },
            REAPER => BaseCard {
                name: REAPER,
                rarity: Rare,
                on_play: vec![Effect::Custom, ExhaustCard(This)],
                cost: Fixed(0),
                ..BaseCard::new(Ironclad, Attack)
            },
            NEUTRALIZE => BaseCard {
                name: NEUTRALIZE,
                rarity: Starter,
                on_play: vec![
                    Effect::AttackDamage(Upgradable(3, 4), TargetEnemy),
                    AddBuff(buffs::WEAK, Upgradable(1, 2), TargetEnemy),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Attack)
            },
            SURVIVOR => BaseCard {
                name: SURVIVOR,
                rarity: Starter,
                on_play: vec![
                    Block(Upgradable(8, 11), _Self),
                    DiscardCard(PlayerHand(PlayerChoice(Fixed(1)))),
                ],
                ..BaseCard::new(Silent, Skill)
            },
            ACROBATICS => BaseCard {
                name: ACROBATICS,
                on_play: vec![
                    Draw(Upgradable(3, 4)),
                    DiscardCard(PlayerHand(PlayerChoice(Fixed(1)))),
                ],
                ..BaseCard::new(Silent, Skill)
            },
            BACKFLIP => BaseCard {
                name: BACKFLIP,
                on_play: vec![Block(Upgradable(5, 8), _Self), Draw(Fixed(2))],
                ..BaseCard::new(Silent, Skill)
            },
            BANE => BaseCard {
                name: BANE,
                on_play: vec![
                    AttackDamage(Upgradable(7, 10), TargetEnemy),
                    If(
                        Condition::Buff(TargetEnemy, buffs::POISON),
                        vec![AttackDamage(Upgradable(7, 10), TargetEnemy)],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            BLADE_DANCE => BaseCard {
                name: BLADE_DANCE,
                on_play: vec![AddCard {
                    card: CardReference::ByName(SHIV),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: Upgradable(3, 4),
                    modifier: CardModifier::None,
                }],
                ..BaseCard::new(Silent, Skill)
            },
            CLOAK_AND_DAGGER => BaseCard {
                name: CLOAK_AND_DAGGER,
                on_play: vec![
                    Block(Fixed(6), _Self),
                    AddCard {
                        card: CardReference::ByName(SHIV),
                        destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                        copies: Upgradable(1, 2),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Silent, Skill)
            },
            DAGGER_SPRAY => BaseCard {
                name: CLOAK_AND_DAGGER,
                on_play: vec![
                    AttackDamage(Upgradable(4, 6), TargetEnemy),
                    AttackDamage(Upgradable(4, 6), TargetEnemy),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            DAGGER_THROW => BaseCard {
                name: DAGGER_THROW,
                on_play: vec![
                    AttackDamage(Upgradable(9, 12), TargetEnemy),
                    Draw(Fixed(1)),
                    DiscardCard(PlayerHand(PlayerChoice(Fixed(1)))),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            DEADLY_POISON => BaseCard {
                name: DEADLY_POISON,

                on_play: vec![AddBuff(buffs::POISON, Upgradable(5, 7), TargetEnemy)],
                ..BaseCard::new(Silent, Skill)
            },
            DEFLECT => BaseCard {
                name: DEFLECT,
                on_play: vec![Block(Upgradable(4, 7), _Self)],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Skill)
            },
            DODGE_AND_ROLL => BaseCard {
                name: DODGE_AND_ROLL,
                on_play: vec![
                    Block(Upgradable(4, 6), _Self),
                    AddBuff(buffs::NEXT_TURN_BLOCK, Amount::Custom, _Self),
                ],
                ..BaseCard::new(Silent, Skill)
            },
            FLYING_KNEE => BaseCard {
                name: FLYING_KNEE,
                on_play: vec![
                    AttackDamage(Upgradable(8, 11), TargetEnemy),
                    AddBuff(buffs::ENERGIZED, Fixed(1), _Self),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            OUTMANEUVER => BaseCard {
                name: OUTMANEUVER,
                on_play: vec![AddBuff(buffs::ENERGIZED, Upgradable(2, 3), _Self)],
                ..BaseCard::new(Silent, Skill)
            },
            PIERCING_WAIL => BaseCard {
                name: PIERCING_WAIL,
                on_play: vec![
                    LoseStr(Upgradable(6, 8), AllEnemies),
                    ExhaustCard(This),
                ],
                ..BaseCard::new(Silent, Skill)
            },
            POISONED_STAB => BaseCard {
                name: POISONED_STAB,
                on_play: vec![
                    AttackDamage(Upgradable(6, 8), TargetEnemy),
                    AddBuff(buffs::POISON, Upgradable(3, 4), TargetEnemy),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            PREPARED => BaseCard {
                name: PREPARED,
                on_play: vec![
                    Draw(Upgradable(1, 2)),
                    DiscardCard(PlayerHand(PlayerChoice(Upgradable(1, 2)))),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Skill)
            },
            QUICK_SLASH => BaseCard {
                name: QUICK_SLASH,
                on_play: vec![AttackDamage(Upgradable(8, 12), TargetEnemy), Draw(Fixed(1))],
                ..BaseCard::new(Silent, Attack)
            },
            SLICE => BaseCard {
                name: SLICE,
                on_play: vec![AttackDamage(Upgradable(6, 9), TargetEnemy)],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Attack)
            },
            SNEAKY_STRIKE => BaseCard {
                name: SNEAKY_STRIKE,
                on_play: vec![
                    AttackDamage(Upgradable(12, 16), TargetEnemy),
                    If(Condition::HasDiscarded, vec![AddEnergy(Fixed(2))], vec![]),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Attack)
            },
            SUCKER_PUNCH => BaseCard {
                name: SUCKER_PUNCH,
                on_play: vec![
                    AttackDamage(Upgradable(7, 8), TargetEnemy),
                    AddBuff(buffs::WEAK, Upgradable(1, 2), TargetEnemy),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            ACCURACY => BaseCard {
                name: ACCURACY,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::ACCURACY, Upgradable(4, 6), _Self)],
                ..BaseCard::new(Silent, Power)
            },
            ALL_OUT_ATTACK => BaseCard {
                name: ALL_OUT_ATTACK,
                rarity: Uncommon,

                on_play: vec![
                    AttackDamage(Upgradable(10, 14), AllEnemies),
                    DiscardCard(PlayerHand(Random)),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            BACKSTAB => BaseCard {
                name: BACKSTAB,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(11, 15), TargetEnemy),
                    ExhaustCard(This),
                ],
                innate: Condition::Always,
                ..BaseCard::new(Silent, Attack)
            },
            BLUR => BaseCard {
                name: BLUR,
                rarity: Uncommon,
                on_play: vec![
                    Block(Upgradable(5, 8), _Self),
                    AddBuff(buffs::BLUR, Fixed(1), _Self),
                ],
                ..BaseCard::new(Silent, Skill)
            },
            BOUNCING_FLASK => BaseCard {
                name: BOUNCING_FLASK,
                rarity: Uncommon,
                on_play: vec![
                    AddBuff(buffs::POISON, Fixed(3), RandomEnemy),
                    AddBuff(buffs::POISON, Fixed(3), RandomEnemy),
                    AddBuff(buffs::POISON, Fixed(3), RandomEnemy),
                    If(
                        Condition::Upgraded,
                        vec![AddBuff(buffs::POISON, Fixed(3), RandomEnemy)],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Silent, Skill)
            },
            CALCULATED_GAMBLE => BaseCard {
                name: CALCULATED_GAMBLE,
                rarity: Uncommon,
                on_play: vec![
                    Effect::Custom,
                    If(Condition::Upgraded, vec![], vec![ExhaustCard(This)]),
                ],
                ..BaseCard::new(Silent, Skill)
            },
            CALTROPS => BaseCard {
                name: CALTROPS,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::THORNS, Upgradable(3, 5), _Self)],
                ..BaseCard::new(Silent, Power)
            },
            CATALYST => BaseCard {
                name: CATALYST,
                rarity: Uncommon,

                on_play: vec![
                    AddBuff(buffs::POISON, Amount::Custom, TargetEnemy),
                    ExhaustCard(This),
                ],
                ..BaseCard::new(Silent, Skill)
            },
            CHOKE => BaseCard {
                name: CHOKE,
                rarity: Uncommon,

                on_play: vec![
                    AttackDamage(Fixed(12), TargetEnemy),
                    AddBuff(buffs::CHOKED, Upgradable(3, 5), TargetEnemy),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Attack)
            },
            CONCENTRATE => BaseCard {
                name: CONCENTRATE,
                rarity: Uncommon,
                on_play: vec![
                    DiscardCard(PlayerHand(PlayerChoice(Upgradable(3, 2)))),
                    AddEnergy(Fixed(2)),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Skill)
            },
            CRIPPLING_CLOUD => BaseCard {
                name: CRIPPLING_CLOUD,
                rarity: Uncommon,
                on_play: vec![
                    AddBuff(buffs::POISON, Upgradable(4, 7), AllEnemies),
                    AddBuff(buffs::WEAK, Fixed(2), AllEnemies),
                    ExhaustCard(This),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Skill)
            },
            DASH => BaseCard {
                name: DASH,
                rarity: Uncommon,
                on_play: vec![
                    Block(Upgradable(10, 13), _Self),
                    AttackDamage(Upgradable(10, 13), TargetEnemy),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Attack)
            },
            DISTRACTION => BaseCard {
                name: DISTRACTION,
                rarity: Uncommon,
                on_play: vec![
                    AddCard {
                        card: CardReference::RandomType(CardType::Skill),
                        destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                        copies: Fixed(1),
                        modifier: CardModifier::SetZeroTurnCost,
                    },
                    ExhaustCard(This),
                ],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Silent, Skill)
            },
            ENDLESS_AGONY => BaseCard {
                name: ENDLESS_AGONY,
                rarity: Uncommon,
                on_draw: vec![AddCard {
                    card: CardReference::CopyOf(CardLocation::This),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: Fixed(1),
                    modifier: CardModifier::None,
                }],
                on_play: vec![
                    AttackDamage(Upgradable(4, 6), TargetEnemy),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Attack)
            },
            ESCAPE_PLAN => BaseCard {
                name: ESCAPE_PLAN,
                rarity: Uncommon,
                on_play: vec![Effect::Custom],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Skill)
            },
            EVISCERATE => BaseCard {
                name: EVISCERATE,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(7, 9), TargetEnemy),
                    AttackDamage(Upgradable(7, 9), TargetEnemy),
                    AttackDamage(Upgradable(7, 9), TargetEnemy),
                ],
                cost: Amount::Custom,
                ..BaseCard::new(Silent, Attack)
            },
            EXPERTISE => BaseCard {
                name: EXPERTISE,
                rarity: Uncommon,
                on_play: vec![Draw(Amount::Custom)],
                ..BaseCard::new(Silent, Skill)
            },
            FINISHER => BaseCard {
                name: FINISHER,
                rarity: Uncommon,
                on_play: vec![Repeat(
                    Amount::Custom,
                    Box::new(AttackDamage(Upgradable(4, 6), TargetEnemy)),
                )],
                ..BaseCard::new(Silent, Attack)
            },
            FOOTWORK => BaseCard {
                name: FOOTWORK,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::DEXTERITY, Upgradable(2, 3), _Self)],
                ..BaseCard::new(Silent, Power)
            },
            HEEL_HOOK => BaseCard {
                name: HEEL_HOOK,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(5, 8), TargetEnemy),
                    If(
                        Condition::Buff(TargetEnemy, buffs::WEAK),
                        vec![AddEnergy(Fixed(1)), Draw(Fixed(1))],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            INFINITE_BLADES => BaseCard {
                name: INFINITE_BLADES,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::INFINITE_BLADES, Fixed(1), _Self)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Silent, Power)
            },
            LEG_SWEEP => BaseCard {
                name: LEG_SWEEP,
                rarity: Uncommon,

                on_play: vec![
                    AddBuff(buffs::WEAK, Upgradable(2, 3), TargetEnemy),
                    Block(Upgradable(11, 14), _Self),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Skill)
            },
            MASTERFUL_STAB => BaseCard {
                name: MASTERFUL_STAB,
                rarity: Uncommon,
                on_play: vec![AttackDamage(Upgradable(12, 16), TargetEnemy)],
                cost: Amount::Custom,
                ..BaseCard::new(Silent, Attack)
            },
            NOXIOUS_FUMES => BaseCard {
                name: NOXIOUS_FUMES,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::NOXIOUS_FUMES, Upgradable(2, 3), _Self)],
                ..BaseCard::new(Silent, Power)
            },
            PREDATOR => BaseCard {
                name: PREDATOR,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(15, 20), TargetEnemy),
                    AddBuff(buffs::DRAW_CARD, Fixed(2), _Self),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Power)
            },
            REFLEX => BaseCard {
                name: REFLEX,
                rarity: Uncommon,
                playable_if: Condition::Never,
                on_discard: vec![Draw(Upgradable(2, 3))],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Skill)
            },
            RIDDLE_WITH_HOLES => BaseCard {
                name: RIDDLE_WITH_HOLES,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(3, 4), TargetEnemy),
                    AttackDamage(Upgradable(3, 4), TargetEnemy),
                    AttackDamage(Upgradable(3, 4), TargetEnemy),
                    AttackDamage(Upgradable(3, 4), TargetEnemy),
                    AttackDamage(Upgradable(3, 4), TargetEnemy),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Attack)
            },
            SETUP => BaseCard {
                name: SETUP,
                rarity: Uncommon,
                on_play: vec![MoveCard(
                    CardLocation::PlayerHand(RelativePosition::PlayerChoice(Fixed(1))),
                    CardLocation::DrawPile(RelativePosition::Top),
                    CardModifier::SetZeroCostUntilPlayed,
                )],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Silent, Skill)
            },
            SKEWER => BaseCard {
                name: SKEWER,
                rarity: Uncommon,
                on_play: vec![Repeat(
                    X,
                    vec![AttackDamage(Upgradable(7, 10), TargetEnemy)],
                )],
                cost: X,
                ..BaseCard::new(Silent, Attack)
            },
            TACTICIAN => BaseCard {
                name: TACTICIAN,
                rarity: Uncommon,
                playable_if: Condition::Never,
                on_discard: vec![AddEnergy(Upgradable(1, 2))],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Skill)
            },
            TERROR => BaseCard {
                name: TERROR,
                rarity: Uncommon,

                on_play: vec![
                    AddBuff(buffs::VULNERABLE, Fixed(99), TargetEnemy),
                    ExhaustCard(This),
                ],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Silent, Skill)
            },
            WELL_LAID_PLANS => BaseCard {
                name: WELL_LAID_PLANS,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::WELL_LAID_PLANS, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Silent, Power)
            },
            A_THOUSAND_CUTS => BaseCard {
                name: A_THOUSAND_CUTS,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::THOUSAND_CUTS, Upgradable(1, 2), _Self)],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Power)
            },
            ADRENALINE => BaseCard {
                name: ADRENALINE,
                rarity: Rare,
                on_play: vec![
                    AddEnergy(Upgradable(1, 2)),
                    Draw(Fixed(2)),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Skill)
            },
            AFTER_IMAGE => BaseCard {
                name: AFTER_IMAGE,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::AFTER_IMAGE, Fixed(1), _Self)],
                innate: Condition::Upgraded,
                ..BaseCard::new(Silent, Power)
            },
            ALCHEMIZE => BaseCard {
                name: ALCHEMIZE,
                rarity: Rare,
                on_play: vec![Effect::Custom],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Silent, Skill)
            },
            BULLET_TIME => BaseCard {
                name: BULLET_TIME,
                rarity: Rare,
                on_play: vec![SetCardModifier(
                    CardLocation::PlayerHand(RelativePosition::All),
                    CardModifier::SetZeroTurnCost,
                )],
                cost: Upgradable(3, 2),
                ..BaseCard::new(Silent, Skill)
            },
            BURST => BaseCard {
                name: BURST,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::BURST, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Silent, Skill)
            },
            CORPSE_EXPLOSION => BaseCard {
                name: CORPSE_EXPLOSION,
                rarity: Rare,

                on_play: vec![
                    AddBuff(buffs::POISON, Upgradable(6, 9), TargetEnemy),
                    AddBuff(buffs::CORPSE_EXPLOSION, Fixed(1), TargetEnemy),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Silent, Skill)
            },
            DIE_DIE_DIE => BaseCard {
                name: DIE_DIE_DIE,
                rarity: Rare,

                on_play: vec![AttackDamage(Upgradable(13, 17), AllEnemies)],
                ..BaseCard::new(Silent, Attack)
            },
            DOPPELGANGER => BaseCard {
                name: DOPPELGANGER,
                rarity: Rare,
                on_play: vec![
                    AddBuff(
                        buffs::DRAW_CARD,
                        Sum(vec![X, Upgradable(0, 1)]),
                        Target::_Self,
                    ),
                    AddBuff(
                        buffs::ENERGIZED,
                        Sum(vec![X, Upgradable(0, 1)]),
                        Target::_Self,
                    ),
                ],
                cost: X,
                ..BaseCard::new(Silent, Skill)
            },
            ENVENOM => BaseCard {
                name: ENVENOM,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::ENVENOM, Fixed(1), Target::_Self)],
                cost: Upgradable(2, 1),
                ..BaseCard::new(Silent, Attack)
            },
            GLASS_KNIFE => BaseCard {
                name: GLASS_KNIFE,
                rarity: Rare,
                effects: vec![(
                    crate::models::core::Event::CombatStart,
                    SetN(Amount::Upgradable(8, 12)),
                )],
                on_play: vec![
                    AttackDamage(N, TargetEnemy),
                    AttackDamage(N, TargetEnemy),
                    AddN(Fixed(-2)),
                ],
                ..BaseCard::new(Silent, Attack)
            },
            GRAND_FINALE => BaseCard {
                name: GRAND_FINALE,
                rarity: Rare,
                playable_if: Condition::Custom,

                on_play: vec![AttackDamage(Upgradable(50, 60), AllEnemies)],
                cost: Fixed(0),
                ..BaseCard::new(Silent, Attack)
            },
            MALAISE => BaseCard {
                name: MALAISE,
                rarity: Rare,

                on_play: vec![
                    AddBuff(
                        buffs::STRENGTH,
                        Sum(vec![NegX, Upgradable(0, -1)]),
                        TargetEnemy,
                    ),
                    AddBuff(buffs::WEAK, Sum(vec![X, Upgradable(0, 1)]), TargetEnemy),
                    ExhaustCard(This),
                ],
                cost: X,
                ..BaseCard::new(Silent, Skill)
            },
            NIGHTMARE => BaseCard {
                name: NIGHTMARE,
                rarity: Rare,
                on_play: vec![Effect::Custom, ExhaustCard(This)],
                cost: Upgradable(3, 2),
                ..BaseCard::new(Silent, Skill)
            },
            PHANTASMAL_KILLER => BaseCard {
                name: PHANTASMAL_KILLER,
                rarity: Rare,
                on_play: vec![
                    AddBuff(buffs::PHANTASMAL, Fixed(1), _Self),
                    ExhaustCard(This),
                ],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Silent, Skill)
            },
            STORM_OF_STEEL => BaseCard {
                name: STORM_OF_STEEL,
                rarity: Rare,
                on_play: vec![Effect::Custom],
                cost: Fixed(1),
                ..BaseCard::new(Silent, Skill)
            },
            TOOLS_OF_THE_TRADE => BaseCard {
                name: TOOLS_OF_THE_TRADE,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::TOOLS_OF_THE_TRADE, Fixed(1), _Self)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Silent, Power)
            },
            UNLOAD => BaseCard {
                name: UNLOAD,
                rarity: Rare,
                on_play: vec![
                    AttackDamage(Upgradable(14, 18), TargetEnemy),
                    Effect::Custom,
                ],
                cost: Fixed(1),
                ..BaseCard::new(Silent, Attack)
            },
            WRAITH_FORM => BaseCard {
                name: WRAITH_FORM,
                rarity: Rare,
                on_play: vec![
                    AddBuff(buffs::INTANGIBLE, Upgradable(2, 3), _Self),
                    AddBuff(buffs::WRAITH_FORM, Fixed(1), _Self),
                ],
                cost: Fixed(1),
                ..BaseCard::new(Silent, Power)
            },
            DUALCAST => BaseCard {
                name: DUALCAST,
                rarity: Starter,
                on_play: vec![EvokeOrb(Fixed(2))],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Defect, Skill)
            },
            ZAP => BaseCard {
                name: ZAP,
                rarity: Starter,
                on_play: vec![ChannelOrb(Orb::Lightning)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Defect, Skill)
            },
            BALL_LIGHTNING => BaseCard {
                name: BALL_LIGHTNING,
                on_play: vec![
                    AttackDamage(Upgradable(7, 10), TargetEnemy),
                    ChannelOrb(Orb::Lightning),
                ],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Defect, Attack)
            },
            BARRAGE => BaseCard {
                name: BARRAGE,
                on_play: vec![Repeat(
                    OrbCount,
                    Box::new(AttackDamage(Upgradable(4, 6), TargetEnemy)),
                )],
                ..BaseCard::new(Defect, Attack)
            },
            BEAM_CELL => BaseCard {
                name: BEAM_CELL,
                on_play: vec![
                    AttackDamage(Upgradable(3, 4), TargetEnemy),
                    AddBuff(buffs::VULNERABLE, Upgradable(1, 2), TargetEnemy),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Attack)
            },
            CHARGE_BATTERY => BaseCard {
                name: CHARGE_BATTERY,
                on_play: vec![
                    Block(Upgradable(7, 10), _Self),
                    AddBuff(buffs::ENERGIZED, Fixed(1), _Self),
                ],
                ..BaseCard::new(Defect, Skill)
            },
            CLAW => BaseCard {
                name: CLAW,
                effects: vec![
                    (Event::CombatStart, SetN(Fixed(0))),
                    (Event::PlayCard(CardType::ByName(CLAW)), AddN(Fixed(2))),
                ],
                on_play: vec![AttackDamage(Sum(vec![N, Upgradable(3, 5)]), TargetEnemy)],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Attack)
            },
            COLD_SNAP => BaseCard {
                name: COLD_SNAP,
                on_play: vec![
                    AttackDamage(Upgradable(6, 9), TargetEnemy),
                    ChannelOrb(Orb::Frost),
                ],
                ..BaseCard::new(Defect, Attack)
            },
            COMPILE_DRIVER => BaseCard {
                name: COMPILE_DRIVER,
                on_play: vec![AttackDamage(Upgradable(7, 10), TargetEnemy), Draw(OrbCount)],
                ..BaseCard::new(Defect, Attack)
            },
            COOLHEADED => BaseCard {
                name: COOLHEADED,
                on_play: vec![ChannelOrb(Orb::Frost), Draw(Upgradable(1, 2))],
                ..BaseCard::new(Defect, Skill)
            },
            GO_FOR_THE_EYES => BaseCard {
                name: GO_FOR_THE_EYES,
                on_play: vec![
                    AttackDamage(Upgradable(3, 4), TargetEnemy),
                    AddBuff(buffs::WEAK, Upgradable(1, 2), TargetEnemy),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Attack)
            },
            HOLOGRAM => BaseCard {
                name: HOLOGRAM,
                on_play: vec![
                    Block(Upgradable(3, 5), _Self),
                    MoveCard(
                        CardLocation::DiscardPile(RelativePosition::PlayerChoice(Fixed(1))),
                        CardLocation::PlayerHand(RelativePosition::Bottom),
                        CardModifier::None,
                    ),
                    If(Condition::Upgraded, vec![], vec![ExhaustCard(This)]),
                ],
                ..BaseCard::new(Defect, Skill)
            },
            LEAP => BaseCard {
                name: LEAP,
                on_play: vec![Block(Upgradable(9, 12), _Self)],
                ..BaseCard::new(Defect, Skill)
            },
            REBOUND => BaseCard {
                name: REBOUND,
                on_play: vec![
                    AttackDamage(Upgradable(9, 12), TargetEnemy),
                    AddBuff(buffs::REBOUND, Fixed(1), _Self),
                ],
                ..BaseCard::new(Defect, Attack)
            },
            RECURSION => BaseCard {
                name: RECURSION,
                on_play: vec![Effect::Custom],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Defect, Skill)
            },
            STACK => BaseCard {
                name: STACK,
                on_play: vec![Block(Sum(vec![Amount::Custom, Upgradable(0, 3)]), _Self)],
                ..BaseCard::new(Defect, Skill)
            },
            STEAM_BARRIER => BaseCard {
                name: STEAM_BARRIER,
                effects: vec![(Event::CombatStart, SetN(Upgradable(6, 8)))],
                on_play: vec![Block(N, _Self), AddN(Fixed(-1))],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Skill)
            },
            STREAMLINE => BaseCard {
                name: STREAMLINE,
                effects: vec![(Event::CombatStart, SetN(Fixed(2)))],
                on_play: vec![
                    AttackDamage(Upgradable(15, 20), _Self),
                    If(
                        Condition::Equals(N, Fixed(0)),
                        vec![],
                        vec![AddN(Fixed(-1))],
                    ),
                ],
                cost: N,
                ..BaseCard::new(Defect, Attack)
            },
            SWEEPING_BEAM => BaseCard {
                name: SWEEPING_BEAM,
                on_play: vec![AttackDamage(Upgradable(6, 9), AllEnemies), Draw(Fixed(1))],
                ..BaseCard::new(Defect, Attack)
            },
            TURBO => BaseCard {
                name: TURBO,
                on_play: vec![
                    AddEnergy(Upgradable(2, 3)),
                    AddCard {
                        card: CardReference::ByName(VOID),
                        destination: CardLocation::DiscardPile(RelativePosition::Bottom),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Defect, Skill)
            },
            AGGREGATE => BaseCard {
                name: AGGREGATE,
                rarity: Uncommon,
                on_play: vec![AddEnergy(Amount::Custom)],
                ..BaseCard::new(Defect, Skill)
            },
            AUTO_SHIELDS => BaseCard {
                name: AUTO_SHIELDS,
                rarity: Uncommon,
                on_play: vec![If(
                    Condition::NoBlock(_Self),
                    vec![Block(Upgradable(11, 15), _Self)],
                    vec![],
                )],
                ..BaseCard::new(Defect, Skill)
            },
            BLIZZARD => BaseCard {
                name: AUTO_SHIELDS,
                rarity: Uncommon,

                effects: vec![
                    (Event::CombatStart, SetN(Fixed(0))),
                    (Event::Channel(Orb::Frost), AddN(Upgradable(2, 3))),
                ],
                on_play: vec![AttackDamage(N, AllEnemies)],
                ..BaseCard::new(Defect, Attack)
            },
            BOOT_SEQUENCE => BaseCard {
                name: AUTO_SHIELDS,
                rarity: Uncommon,
                innate: Condition::Always,
                on_play: vec![Block(Upgradable(10, 13), _Self), ExhaustCard(This)],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Skill)
            },
            BULLSEYE => BaseCard {
                name: BULLSEYE,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(8, 11), TargetEnemy),
                    AddBuff(buffs::LOCK_ON, Upgradable(2, 3), TargetEnemy),
                ],
                ..BaseCard::new(Defect, Attack)
            },
            CAPACITOR => BaseCard {
                name: CAPACITOR,
                rarity: Uncommon,
                on_play: vec![AddOrbSlot(Upgradable(2, 3))],
                ..BaseCard::new(Defect, Power)
            },
            CHAOS => BaseCard {
                name: CHAOS,
                rarity: Uncommon,
                on_play: vec![
                    ChannelOrb(Orb::Any),
                    If(Condition::Upgraded, vec![ChannelOrb(Orb::Any)], vec![]),
                ],
                ..BaseCard::new(Defect, Skill)
            },
            CHILL => BaseCard {
                name: CHILL,
                rarity: Uncommon,
                innate: Condition::Upgraded,
                on_play: vec![
                    Repeat(Amount::EnemyCount, Box::new(ChannelOrb(Orb::Frost))),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Skill)
            },
            CONSUME => BaseCard {
                name: CONSUME,
                rarity: Uncommon,
                on_play: vec![
                    AddBuff(buffs::FOCUS, Upgradable(2, 3), Target::_Self),
                    AddOrbSlot(Fixed(-1)),
                ],
                ..BaseCard::new(Defect, Skill)
            },
            DARKNESS => BaseCard {
                name: DARKNESS,
                rarity: Uncommon,
                on_play: vec![
                    ChannelOrb(Orb::Dark),
                    If(Condition::Upgraded, vec![Effect::Custom], vec![]),
                ],
                ..BaseCard::new(Defect, Skill)
            },
            DEFRAGMENT => BaseCard {
                name: DEFRAGMENT,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::FOCUS, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Defect, Power)
            },
            DOOM_AND_GLOOM => BaseCard {
                name: DOOM_AND_GLOOM,
                rarity: Uncommon,

                on_play: vec![
                    AttackDamage(Upgradable(10, 14), AllEnemies),
                    ChannelOrb(Orb::Dark),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Defect, Attack)
            },
            DOUBLE_ENERGY => BaseCard {
                name: DOUBLE_ENERGY,
                rarity: Uncommon,
                on_play: vec![AddEnergy(Amount::Custom)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Defect, Skill)
            },
            EQUILIBRIUM => BaseCard {
                name: EQUILIBRIUM,
                rarity: Uncommon,
                on_play: vec![
                    Block(Upgradable(13, 16), _Self),
                    AddBuff(buffs::EQUILIBRIUM, Fixed(1), _Self),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Defect, Skill)
            },
            FTL => BaseCard {
                name: FTL,
                rarity: Uncommon,
                effects: vec![
                    (Event::BeforeHandDraw, SetN(Upgradable(3, 4))),
                    (
                        Event::PlayCard(CardType::All),
                        If(
                            Condition::Equals(N, Fixed(0)),
                            vec![],
                            vec![AddN(Fixed(-1))],
                        ),
                    ),
                ],
                on_play: vec![
                    AttackDamage(Upgradable(5, 6), TargetEnemy),
                    If(Condition::Equals(N, Fixed(0)), vec![], vec![Draw(Fixed(1))]),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Attack)
            },
            FORCE_FIELD => BaseCard {
                name: FORCE_FIELD,
                rarity: Uncommon,
                effects: vec![
                    (Event::CombatStart, SetN(Fixed(4))),
                    (Event::PlayCard(CardType::Power), AddN(Fixed(-1))),
                ],
                on_play: vec![Block(Upgradable(12, 16), _Self)],
                cost: N,
                ..BaseCard::new(Defect, Skill)
            },
            FUSION => BaseCard {
                name: FUSION,
                rarity: Uncommon,
                on_play: vec![ChannelOrb(Orb::Plasma)],
                cost: Upgradable(2, 1),
                ..BaseCard::new(Defect, Skill)
            },
            GENETIC_ALGORITHM => BaseCard {
                name: GENETIC_ALGORITHM,
                rarity: Uncommon,
                on_play: vec![
                    Block(Upgradable(7, 10), _Self),
                    ChannelOrb(Orb::Frost),
                    ChannelOrb(Orb::Frost),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Defect, Skill)
            },
            HEATSINKS => BaseCard {
                name: HEATSINKS,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::HEATSINK, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Defect, Power)
            },
            HELLO_WORLD => BaseCard {
                name: HELLO_WORLD,
                rarity: Uncommon,
                innate: Condition::Upgraded,
                on_play: vec![AddBuff(buffs::HELLO, Fixed(1), _Self)],
                ..BaseCard::new(Defect, Power)
            },
            LOOP => BaseCard {
                name: LOOP,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::LOOP, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Defect, Power)
            },
            MELTER => BaseCard {
                name: MELTER,
                rarity: Uncommon,
                on_play: vec![
                    Effect::Custom,
                    AttackDamage(Upgradable(10, 14), TargetEnemy),
                ],
                ..BaseCard::new(Defect, Attack)
            },
            OVERCLOCK => BaseCard {
                name: OVERCLOCK,
                rarity: Uncommon,
                on_play: vec![
                    Draw(Upgradable(2, 3)),
                    AddCard {
                        card: CardReference::ByName(BURN),
                        destination: CardLocation::DiscardPile(RelativePosition::Bottom),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Defect, Skill)
            },
            RECYCLE => BaseCard {
                name: RECYCLE,
                rarity: Uncommon,
                on_play: vec![Effect::Custom],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Defect, Skill)
            },
            REINFORCED_BODY => BaseCard {
                name: REINFORCED_BODY,
                rarity: Uncommon,
                on_play: vec![Repeat(X, vec![Block(Upgradable(7, 9), _Self)])],
                cost: X,
                ..BaseCard::new(Defect, Skill)
            },
            REPROGRAM => BaseCard {
                name: REPROGRAM,
                rarity: Uncommon,
                on_play: vec![
                    AddBuff(buffs::FOCUS, Upgradable(-1, -2), _Self),
                    AddBuff(buffs::STRENGTH, Upgradable(1, 2), _Self),
                    AddBuff(buffs::DEXTERITY, Upgradable(1, 2), _Self),
                ],
                ..BaseCard::new(Defect, Skill)
            },
            RIP_AND_TEAR => BaseCard {
                name: RIP_AND_TEAR,
                rarity: Uncommon,

                on_play: vec![
                    AttackDamage(Upgradable(7, 9), RandomEnemy),
                    AttackDamage(Upgradable(7, 9), RandomEnemy),
                ],
                ..BaseCard::new(Defect, Attack)
            },
            SCRAPE => BaseCard {
                name: SCRAPE,
                rarity: Uncommon,
                on_play: vec![AttackDamage(Upgradable(7, 10), TargetEnemy), Effect::Custom],
                ..BaseCard::new(Defect, Attack)
            },
            SELF_REPAIR => BaseCard {
                name: SELF_REPAIR,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::REPAIR, Upgradable(7, 10), _Self)],
                ..BaseCard::new(Defect, Power)
            },
            SKIM => BaseCard {
                name: SKIM,
                rarity: Uncommon,
                on_play: vec![Draw(Upgradable(3, 4))],
                ..BaseCard::new(Defect, Skill)
            },
            STATIC_DISCHARGE => BaseCard {
                name: STATIC_DISCHARGE,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::STATIC_DISCHARGE, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Defect, Power)
            },
            STORM => BaseCard {
                name: STORM,
                rarity: Uncommon,
                innate: Condition::Upgraded,
                on_play: vec![AddBuff(buffs::STORM, Fixed(1), _Self)],
                ..BaseCard::new(Defect, Power)
            },
            SUNDER => BaseCard {
                name: SUNDER,
                rarity: Uncommon,
                on_play: vec![AttackDamageIfFatal(
                    Upgradable(24, 32),
                    TargetEnemy,
                    vec![AddEnergy(Fixed(3))],
                )],
                cost: Fixed(3),
                ..BaseCard::new(Defect, Attack)
            },
            TEMPEST => BaseCard {
                name: TEMPEST,
                rarity: Uncommon,
                on_play: vec![
                    Repeat(
                        Amount::Sum(vec![X, Upgradable(0, 1)]),
                        Box::new(ChannelOrb(Orb::Lightning)),
                    ),
                    ExhaustCard(This),
                ],
                cost: X,
                ..BaseCard::new(Defect, Skill)
            },
            WHITE_NOISE => BaseCard {
                name: WHITE_NOISE,
                rarity: Uncommon,
                on_play: vec![AddCard {
                    card: CardReference::RandomType(CardType::Power),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: Fixed(1),
                    modifier: CardModifier::SetZeroTurnCost,
                }],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Defect, Skill)
            },
            ALL_FOR_ONE => BaseCard {
                name: ALL_FOR_ONE,
                rarity: Rare,
                on_play: vec![
                    AttackDamage(Upgradable(10, 14), TargetEnemy),
                    Effect::Custom,
                ],
                cost: Fixed(2),
                ..BaseCard::new(Defect, Attack)
            },
            AMPLIFY => BaseCard {
                name: AMPLIFY,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::AMPLIFY, Upgradable(1, 2), _Self)],
                ..BaseCard::new(Defect, Skill)
            },
            BIASED_COGNITION => BaseCard {
                name: BIASED_COGNITION,
                rarity: Rare,
                on_play: vec![
                    AddBuff(buffs::FOCUS, Upgradable(4, 5), _Self),
                    AddBuff(buffs::BIAS, Fixed(1), _Self),
                ],
                ..BaseCard::new(Defect, Power)
            },
            BUFFER => BaseCard {
                name: BUFFER,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::BUFFER, Upgradable(1, 2), _Self)],
                cost: Fixed(2),
                ..BaseCard::new(Defect, Power)
            },
            CORE_SURGE => BaseCard {
                name: CORE_SURGE,
                rarity: Rare,
                on_play: vec![
                    AttackDamage(Upgradable(11, 15), TargetEnemy),
                    AddBuff(buffs::ARTIFACT, Fixed(1), _Self),
                ],
                ..BaseCard::new(Defect, Attack)
            },
            CREATIVE_AI => BaseCard {
                name: CREATIVE_AI,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::CREATIVE_AI, Fixed(1), _Self)],
                cost: Upgradable(3, 2),
                ..BaseCard::new(Defect, Power)
            },
            ECHO_FORM => BaseCard {
                name: ECHO_FORM,
                rarity: Rare,
                ethereal: Condition::Not(Box::new(Condition::Upgraded)),
                on_play: vec![AddBuff(buffs::ECHO_FORM, Fixed(1), _Self)],
                cost: Fixed(3),
                ..BaseCard::new(Defect, Power)
            },
            ELECTODYNAMICS => BaseCard {
                name: ELECTODYNAMICS,
                rarity: Rare,
                on_play: vec![
                    AddBuff(buffs::ELECTRO, Fixed(1), _Self),
                    Repeat(Upgradable(2, 3), Box::new(ChannelOrb(Orb::Lightning))),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Defect, Power)
            },
            FISSION => BaseCard {
                name: FISSION,
                rarity: Rare,
                on_play: vec![Effect::Custom, ExhaustCard(This)],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Skill)
            },
            HYPERBEAM => BaseCard {
                name: HYPERBEAM,
                rarity: Rare,

                on_play: vec![
                    AttackDamage(Upgradable(26, 34), AllEnemies),
                    AddBuff(buffs::FOCUS, Fixed(-3), _Self),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Defect, Attack)
            },
            MACHINE_LEARNING => BaseCard {
                name: MACHINE_LEARNING,
                rarity: Rare,
                innate: Condition::Upgraded,
                on_play: vec![AddBuff(buffs::MACHINE_LEARNING, Fixed(1), _Self)],
                ..BaseCard::new(Defect, Power)
            },
            METEOR_STRIKE => BaseCard {
                name: METEOR_STRIKE,
                rarity: Rare,
                on_play: vec![
                    Damage(Amount::Upgradable(24, 30), TargetEnemy),
                    ChannelOrb(Orb::Plasma),
                    ChannelOrb(Orb::Plasma),
                    ChannelOrb(Orb::Plasma),
                ],
                cost: Fixed(5),
                ..BaseCard::new(Defect, Attack)
            },
            MULTI_CAST => BaseCard {
                name: MULTI_CAST,
                rarity: Rare,
                on_play: vec![EvokeOrb(Sum(vec![X, Upgradable(0, 1)]))],
                cost: X,
                ..BaseCard::new(Defect, Skill)
            },
            RAINBOW => BaseCard {
                name: RAINBOW,
                rarity: Rare,
                on_play: vec![
                    ChannelOrb(Orb::Lightning),
                    ChannelOrb(Orb::Frost),
                    ChannelOrb(Orb::Dark),
                    If(Condition::Upgraded, vec![], vec![ExhaustCard(This)]),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Defect, Skill)
            },
            REBOOT => BaseCard {
                name: REBOOT,
                rarity: Rare,
                on_play: vec![
                    MoveCard(
                        CardLocation::PlayerHand(RelativePosition::All),
                        CardLocation::DrawPile(RelativePosition::Bottom),
                        CardModifier::None,
                    ),
                    MoveCard(
                        CardLocation::DiscardPile(RelativePosition::All),
                        CardLocation::DrawPile(RelativePosition::Bottom),
                        CardModifier::None,
                    ),
                    Shuffle,
                    Draw(Upgradable(4, 6)),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Skill)
            },
            SEEK => BaseCard {
                name: SEEK,
                rarity: Rare,
                on_play: vec![
                    MoveCard(
                        CardLocation::DrawPile(RelativePosition::PlayerChoice(Upgradable(1, 2))),
                        CardLocation::PlayerHand(RelativePosition::Bottom),
                        CardModifier::None,
                    ),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Defect, Skill)
            },
            THUNDER_STRIKE => BaseCard {
                name: THUNDER_STRIKE,
                rarity: Rare,
                effects: vec![
                    (Event::CombatStart, Effect::SetN(Fixed(0))),
                    (Event::Channel(Orb::Lightning), Effect::AddN(Fixed(1))),
                ],
                on_play: vec![Repeat(
                    N,
                    Box::new(AttackDamage(Upgradable(7, 9), RandomEnemy)),
                )],
                cost: Fixed(3),
                ..BaseCard::new(Defect, Skill)
            },
            ERUPTION => BaseCard {
                name: ERUPTION,
                rarity: Starter,
                on_play: vec![
                    AttackDamage(Fixed(9), TargetEnemy),
                    SetStance(Stance::Wrath),
                ],
                cost: Upgradable(2, 1),
                ..BaseCard::new(Watcher, Attack)
            },
            VIGILANCE => BaseCard {
                name: VIGILANCE,
                rarity: Starter,
                on_play: vec![Block(Upgradable(8, 12), _Self), SetStance(Stance::Calm)],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Skill)
            },
            BOWLING_BASH => BaseCard {
                name: BOWLING_BASH,
                on_play: vec![Repeat(
                    EnemyCount,
                    Box::new(AttackDamage(Upgradable(7, 10), TargetEnemy)),
                )],
                ..BaseCard::new(Watcher, Attack)
            },
            CONSECRATE => BaseCard {
                name: CONSECRATE,

                on_play: vec![AttackDamage(Upgradable(5, 8), AllEnemies)],
                ..BaseCard::new(Watcher, Attack)
            },
            CRESCENDO => BaseCard {
                name: CRESCENDO,
                retain: Condition::Always,
                on_play: vec![SetStance(Stance::Wrath), ExhaustCard(This)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Watcher, Attack)
            },
            CRUSH_JOINTS => BaseCard {
                name: CRUSH_JOINTS,
                on_play: vec![
                    AttackDamage(Upgradable(8, 11), TargetEnemy),
                    If(
                        Condition::LastCard(Skill),
                        vec![AddBuff(
                            buffs::VULNERABLE,
                            Amount::Upgradable(1, 2),
                            TargetEnemy,
                        )],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            CUT_THROUGH_FATE => BaseCard {
                name: CUT_THROUGH_FATE,
                on_play: vec![
                    AttackDamage(Upgradable(7, 9), TargetEnemy),
                    Scry(Upgradable(2, 3)),
                    Draw(Fixed(1)),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            EMPTY_BODY => BaseCard {
                name: EMPTY_BODY,
                on_play: vec![Block(Upgradable(7, 10), _Self), SetStance(Stance::None)],
                ..BaseCard::new(Watcher, Skill)
            },
            EMPTY_FIST => BaseCard {
                name: EMPTY_BODY,
                on_play: vec![
                    AttackDamage(Upgradable(9, 14), _Self),
                    SetStance(Stance::None),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            EVALUATE => BaseCard {
                name: EVALUATE,
                on_play: vec![
                    Block(Upgradable(6, 10), _Self),
                    AddCard {
                        card: CardReference::ByName(INSIGHT),
                        destination: CardLocation::DrawPile(RelativePosition::Random),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Watcher, Skill)
            },
            FLURRY_OF_BLOWS => BaseCard {
                name: FLURRY_OF_BLOWS,
                effects: vec![(
                    Event::StanceChange(Stance::All, Stance::All),
                    If(
                        Condition::Custom,
                        vec![MoveCard(
                            CardLocation::This,
                            CardLocation::PlayerHand(RelativePosition::Bottom),
                            CardModifier::None,
                        )],
                        vec![],
                    ),
                )],
                on_play: vec![AttackDamage(Upgradable(4, 6), TargetEnemy)],
                cost: Fixed(0),
                ..BaseCard::new(Watcher, Attack)
            },
            FLYING_SLEEVES => BaseCard {
                name: FLYING_SLEEVES,
                retain: Condition::Always,
                on_play: vec![
                    AttackDamage(Upgradable(4, 6), TargetEnemy),
                    AttackDamage(Upgradable(4, 6), TargetEnemy),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            FOLLOW_UP => BaseCard {
                name: FOLLOW_UP,
                on_play: vec![
                    AttackDamage(Upgradable(7, 11), TargetEnemy),
                    If(
                        Condition::LastCard(Attack),
                        vec![AddEnergy(Fixed(1))],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            HALT => BaseCard {
                name: HALT,
                on_play: vec![
                    Block(Upgradable(3, 4), _Self),
                    If(
                        Condition::Stance(Stance::Wrath),
                        vec![Block(Upgradable(9, 14), _Self)],
                        vec![],
                    ),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Watcher, Skill)
            },
            JUST_LUCKY => BaseCard {
                name: JUST_LUCKY,
                on_play: vec![
                    Scry(Upgradable(1, 2)),
                    Block(Upgradable(2, 3), _Self),
                    AttackDamage(Upgradable(3, 4), TargetEnemy),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Watcher, Attack)
            },
            PRESSURE_POINTS => BaseCard {
                name: PRESSURE_POINTS,
                on_play: vec![
                    AddBuff(buffs::MARK, Upgradable(8, 11), TargetEnemy),
                    Effect::Custom,
                ],
                ..BaseCard::new(Watcher, Skill)
            },
            PROSTRATE => BaseCard {
                name: PROSTRATE,
                on_play: vec![
                    AddBuff(buffs::MANTRA, Upgradable(2, 3), _Self),
                    Block(Fixed(4), _Self),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Watcher, Skill)
            },
            PROTECT => BaseCard {
                name: PROTECT,
                retain: Condition::Always,
                on_play: vec![Block(Upgradable(12, 16), _Self)],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Skill)
            },
            SASH_WHIP => BaseCard {
                name: SASH_WHIP,
                on_play: vec![
                    AttackDamage(Upgradable(8, 10), TargetEnemy),
                    If(
                        Condition::LastCard(Attack),
                        vec![AddBuff(buffs::WEAK, Upgradable(1, 2), TargetEnemy)],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            THIRD_EYE => BaseCard {
                name: THIRD_EYE,
                on_play: vec![Block(Upgradable(7, 9), _Self), Scry(Upgradable(3, 5))],
                ..BaseCard::new(Watcher, Skill)
            },
            TRANQUILITY => BaseCard {
                name: TRANQUILITY,
                retain: Condition::Always,
                on_play: vec![SetStance(Stance::Calm), ExhaustCard(This)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Watcher, Skill)
            },
            BATTLE_HYMN => BaseCard {
                name: BATTLE_HYMN,
                rarity: Uncommon,
                innate: Condition::Upgraded,
                on_play: vec![AddBuff(buffs::BATTLE_HYMN, Fixed(1), _Self)],
                ..BaseCard::new(Watcher, Power)
            },
            CARVE_REALITY => BaseCard {
                name: CARVE_REALITY,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(6, 10), TargetEnemy),
                    AddCard {
                        card: CardReference::ByName(SMITE),
                        destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            COLLECT => BaseCard {
                name: COLLECT,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::COLLECT, X, _Self), ExhaustCard(This)],
                cost: X,
                ..BaseCard::new(Watcher, Skill)
            },
            CONCLUDE => BaseCard {
                name: CONCLUDE,
                rarity: Uncommon,

                on_play: vec![Damage(Upgradable(12, 16), AllEnemies), EndTurn],
                ..BaseCard::new(Watcher, Attack)
            },
            DECEIVE_REALITY => BaseCard {
                name: DECEIVE_REALITY,
                rarity: Uncommon,
                on_play: vec![
                    Block(Upgradable(4, 7), _Self),
                    AddCard {
                        card: CardReference::ByName(SAFETY),
                        destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Watcher, Skill)
            },
            EMPTY_MIND => BaseCard {
                name: EMPTY_MIND,
                rarity: Uncommon,
                on_play: vec![Draw(Upgradable(2, 3)), SetStance(Stance::None)],
                ..BaseCard::new(Watcher, Skill)
            },
            FASTING => BaseCard {
                name: FASTING,
                rarity: Uncommon,
                on_play: vec![
                    AddBuff(buffs::STRENGTH, Upgradable(3, 4), _Self),
                    AddBuff(buffs::DEXTERITY, Upgradable(3, 4), _Self),
                    AddBuff(buffs::FASTING, Fixed(1), _Self),
                ],
                ..BaseCard::new(Watcher, Power)
            },
            FEAR_NO_EVIL => BaseCard {
                name: FEAR_NO_EVIL,
                rarity: Uncommon,
                on_play: vec![
                    Damage(Upgradable(8, 9), TargetEnemy),
                    If(
                        Condition::Attacking(TargetEnemy),
                        vec![SetStance(Stance::Calm)],
                        vec![],
                    ),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            FOREIGN_INFLUENCE => BaseCard {
                name: FOREIGN_INFLUENCE,
                rarity: Uncommon,
                on_play: vec![
                    If(
                        Condition::Upgraded,
                        vec![AddCard {
                            card: CardReference::RandomClass(Class::All),
                            destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                            copies: Fixed(1),
                            modifier: CardModifier::None,
                        }],
                        vec![AddCard {
                            card: CardReference::RandomClass(Class::All),
                            destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                            copies: Fixed(1),
                            modifier: CardModifier::SetZeroTurnCost,
                        }],
                    ),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Watcher, Skill)
            },
            FORESIGHT => BaseCard {
                name: FORESIGHT,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::FORESIGHT, Upgradable(3, 4), _Self)],
                ..BaseCard::new(Watcher, Power)
            },
            INDIGNATION => BaseCard {
                name: INDIGNATION,
                rarity: Uncommon,
                on_play: vec![If(
                    Condition::Stance(Stance::Wrath),
                    vec![AddBuff(buffs::VULNERABLE, Upgradable(3, 5), AllEnemies)],
                    vec![SetStance(Stance::Wrath)],
                )],
                ..BaseCard::new(Watcher, Skill)
            },
            INNER_PEACE => BaseCard {
                name: INNER_PEACE,
                rarity: Uncommon,
                on_play: vec![If(
                    Condition::Stance(Stance::Calm),
                    vec![Draw(Upgradable(3, 4))],
                    vec![SetStance(Stance::Calm)],
                )],
                ..BaseCard::new(Watcher, Skill)
            },
            LIKE_WATER => BaseCard {
                name: LIKE_WATER,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::LIKE_WATER, Upgradable(5, 7), _Self)],
                ..BaseCard::new(Watcher, Power)
            },
            MEDITATE => BaseCard {
                name: MEDITATE,
                rarity: Uncommon,
                on_play: vec![Effect::Custom],
                ..BaseCard::new(Watcher, Power)
            },
            MENTAL_FORTRESS => BaseCard {
                name: MENTAL_FORTRESS,
                rarity: Uncommon,
                on_play: vec![Effect::AddBuff(
                    buffs::MENTAL_FORTRESS,
                    Upgradable(4, 6),
                    _Self,
                )],
                ..BaseCard::new(Watcher, Power)
            },
            NIRVANA => BaseCard {
                name: NIRVANA,
                rarity: Uncommon,
                on_play: vec![Effect::AddBuff(buffs::NIRVANA, Upgradable(3, 4), _Self)],
                ..BaseCard::new(Watcher, Power)
            },
            PERSEVERANCE => BaseCard {
                name: PERSEVERANCE,
                rarity: Uncommon,
                retain: Condition::Always,
                effects: vec![(Event::CombatStart, SetN(Fixed(0)))],
                on_retain: vec![AddN(Upgradable(2, 3))],
                on_play: vec![Effect::Block(Sum(vec![N, Upgradable(5, 7)]), _Self)],
                ..BaseCard::new(Watcher, Skill)
            },
            PRAY => BaseCard {
                name: PRAY,
                rarity: Uncommon,
                on_play: vec![
                    Effect::AddBuff(buffs::MANTRA, Upgradable(3, 4), _Self),
                    Effect::AddCard {
                        card: CardReference::ByName(INSIGHT),
                        destination: CardLocation::DrawPile(RelativePosition::Random),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                ..BaseCard::new(Watcher, Skill)
            },
            REACH_HEAVEN => BaseCard {
                name: REACH_HEAVEN,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(10, 15), TargetEnemy),
                    Effect::AddCard {
                        card: CardReference::ByName(THROUGH_VIOLENCE),
                        destination: CardLocation::DrawPile(RelativePosition::Random),
                        copies: Fixed(1),
                        modifier: CardModifier::None,
                    },
                ],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Attack)
            },
            RUSHDOWN => BaseCard {
                name: RUSHDOWN,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::RUSHDOWN, Fixed(2), _Self)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Watcher, Power)
            },
            SANCTITY => BaseCard {
                name: SANCTITY,
                rarity: Uncommon,
                on_play: vec![
                    Block(Upgradable(6, 9), _Self),
                    If(Condition::LastCard(Skill), vec![Draw(Fixed(2))], vec![]),
                ],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Watcher, Skill)
            },
            SANDS_OF_TIME => BaseCard {
                name: SANDS_OF_TIME,
                rarity: Uncommon,
                retain: Condition::Always,
                on_retain: vec![SetCardCost(This, Sum(vec![X, Fixed(-1)]))],
                on_play: vec![AttackDamage(Upgradable(20, 26), TargetEnemy)],
                cost: Fixed(4),
                ..BaseCard::new(Watcher, Attack)
            },
            SIGNATURE_MOVE => BaseCard {
                name: SIGNATURE_MOVE,
                rarity: Uncommon,
                playable_if: Condition::Custom,
                on_play: vec![AttackDamage(Upgradable(30, 40), TargetEnemy)],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Attack)
            },
            SIMMERING_FURY => BaseCard {
                name: SIMMERING_FURY,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::SIMMERING_RAGE, Upgradable(2, 3), _Self)],
                ..BaseCard::new(Watcher, Skill)
            },
            STUDY => BaseCard {
                name: STUDY,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::STUDY, Upgradable(2, 1), _Self)],
                ..BaseCard::new(Watcher, Power)
            },
            SWIVEL => BaseCard {
                name: SWIVEL,
                rarity: Uncommon,
                on_play: vec![Block(Upgradable(8, 11), _Self), Effect::Custom],
                ..BaseCard::new(Watcher, Skill)
            },
            TALK_TO_THE_HAND => BaseCard {
                name: TALK_TO_THE_HAND,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(5, 7), TargetEnemy),
                    AddBuff(buffs::BLOCK_RETURN, Upgradable(2, 3), TargetEnemy),
                    ExhaustCard(This),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            TANTRUM => BaseCard {
                name: TANTRUM,
                rarity: Uncommon,
                on_play: vec![
                    Repeat(
                        Upgradable(3, 4),
                        Box::new(AttackDamage(Fixed(3), TargetEnemy)),
                    ),
                    MoveCard(This, DrawPile(RelativePosition::Random), CardModifier::None),
                    SetStance(Stance::Wrath),
                ],
                ..BaseCard::new(Watcher, Attack)
            },
            WALLOP => BaseCard {
                name: WALLOP,
                rarity: Uncommon,
                on_play: vec![AttackDamageIfUnblocked(
                    Upgradable(9, 12),
                    TargetEnemy,
                    vec![Block(N, _Self)],
                )],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Attack)
            },
            WAVE_OF_THE_HAND => BaseCard {
                name: WAVE_OF_THE_HAND,
                rarity: Uncommon,

                on_play: vec![AddBuff(
                    buffs::WAVE_OF_THE_HAND,
                    Upgradable(1, 2),
                    TargetEnemy,
                )],
                ..BaseCard::new(Watcher, Skill)
            },
            WHEEL_KICK => BaseCard {
                name: WHEEL_KICK,
                rarity: Uncommon,
                on_play: vec![
                    AttackDamage(Upgradable(15, 20), TargetEnemy),
                    Draw(Fixed(2)),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Attack)
            },
            WINDMILL_STRIKE => BaseCard {
                name: WINDMILL_STRIKE,
                rarity: Uncommon,
                retain: Condition::Always,
                effects: vec![(Event::CombatStart, SetN(Fixed(0)))],
                on_retain: vec![AddN(Upgradable(4, 5))],
                on_play: vec![AttackDamage(Sum(vec![N, Upgradable(7, 10)]), TargetEnemy)],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Attack)
            },
            WORSHIP => BaseCard {
                name: WORSHIP,
                rarity: Uncommon,
                retain: Condition::Upgraded,
                on_play: vec![AddBuff(buffs::MANTRA, Fixed(5), _Self)],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Skill)
            },
            WREATH_OF_FLAME => BaseCard {
                name: WREATH_OF_FLAME,
                rarity: Uncommon,
                on_play: vec![AddBuff(buffs::VIGOR, Upgradable(5, 8), _Self)],
                ..BaseCard::new(Watcher, Skill)
            },
            ALPHA => BaseCard {
                name: ALPHA,
                rarity: Rare,
                innate: Condition::Upgraded,
                on_play: vec![AddCard {
                    card: CardReference::ByName(BETA),
                    destination: CardLocation::DrawPile(RelativePosition::Random),
                    copies: Fixed(1),
                    modifier: CardModifier::None,
                }],
                ..BaseCard::new(Watcher, Skill)
            },
            BLASPHEMY => BaseCard {
                name: BLASPHEMY,
                rarity: Rare,
                retain: Condition::Upgraded,
                on_play: vec![
                    SetStance(Stance::Divinity),
                    AddBuff(buffs::BLASPHEMER, Fixed(1), _Self),
                    ExhaustCard(This),
                ],
                ..BaseCard::new(Watcher, Skill)
            },
            BRILLIANCE => BaseCard {
                name: BRILLIANCE,
                rarity: Rare,
                on_play: vec![AttackDamage(
                    Sum(vec![Upgradable(12, 16), Amount::Custom]),
                    TargetEnemy,
                )],
                ..BaseCard::new(Watcher, Attack)
            },
            CONJURE_BLADE => BaseCard {
                name: CONJURE_BLADE,
                rarity: Rare,
                on_play: vec![Effect::Custom],
                cost: X,
                ..BaseCard::new(Watcher, Skill)
            },
            DEUS_EX_MACHINA => BaseCard {
                name: DEUS_EX_MACHINA,
                rarity: Rare,
                playable_if: Condition::Never,
                on_draw: vec![
                    AddCard {
                        card: CardReference::ByName(MIRACLE),
                        destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                        copies: Upgradable(2, 3),
                        modifier: CardModifier::None,
                    },
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Watcher, Skill)
            },
            DEVA_FORM => BaseCard {
                name: DEVA_FORM,
                rarity: Rare,
                ethereal: Condition::Not(Box::new(Condition::Upgraded)),
                on_play: vec![AddBuff(buffs::DEVA, Fixed(1), _Self)],
                cost: Fixed(3),
                ..BaseCard::new(Watcher, Power)
            },
            DEVOTION => BaseCard {
                name: DEVOTION,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::DEVOTION, Upgradable(2, 3), _Self)],
                ..BaseCard::new(Watcher, Power)
            },
            ESTABLISHMENT => BaseCard {
                name: ESTABLISHMENT,
                rarity: Rare,
                innate: Condition::Upgraded,
                on_play: vec![AddBuff(buffs::ESTABLISHMENT, Fixed(1), _Self)],
                ..BaseCard::new(Watcher, Power)
            },
            JUDGMENT => BaseCard {
                name: JUDGMENT,
                rarity: Rare,
                innate: Condition::Upgraded,
                on_play: vec![If(
                    Condition::RemainingHp(Upgradable(31, 41), TargetEnemy),
                    vec![],
                    vec![Die(TargetEnemy)],
                )],
                ..BaseCard::new(Watcher, Skill)
            },
            LESSON_LEARNED => BaseCard {
                name: LESSON_LEARNED,
                rarity: Rare,
                on_play: vec![
                    AttackDamageIfFatal(
                        Upgradable(10, 12),
                        TargetEnemy,
                        vec![UpgradeCard(DeckPile(Random))],
                    ),
                    ExhaustCard(This),
                ],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Attack)
            },
            MASTER_REALITY => BaseCard {
                name: MASTER_REALITY,
                rarity: Rare,
                on_play: vec![AddBuff(buffs::MASTER_REALITY, Fixed(1), _Self)],
                cost: Upgradable(1, 0),
                ..BaseCard::new(Watcher, Power)
            },
            OMNISCIENCE => BaseCard {
                name: OMNISCIENCE,
                rarity: Rare,
                on_play: vec![Effect::Custom, ExhaustCard(This)],
                cost: Upgradable(4, 3),
                ..BaseCard::new(Watcher, Skill)
            },
            RAGNAROK => BaseCard {
                name: RAGNAROK,
                rarity: Rare,
                on_play: vec![Repeat(
                    Upgradable(5, 6),
                    Box::new(AttackDamage(Upgradable(5, 6), TargetEnemy)),
                )],
                cost: Fixed(3),
                ..BaseCard::new(Watcher, Attack)
            },
            SCRAWL => BaseCard {
                name: SCRAWL,
                rarity: Rare,
                on_play: vec![Draw(Amount::Custom), ExhaustCard(This)],
                cost: Fixed(3),
                ..BaseCard::new(Watcher, Skill)
            },
            SPIRIT_SHIELD => BaseCard {
                name: SPIRIT_SHIELD,
                rarity: Rare,
                on_play: vec![Block(
                    Amount::Mult(vec![Amount::Custom, Upgradable(3, 4)]),
                    _Self,
                )],
                cost: Fixed(2),
                ..BaseCard::new(Watcher, Skill)
            },
            VAULT => BaseCard {
                name: VAULT,
                rarity: Rare,
                on_play: vec![Effect::Custom, ExhaustCard(This)],
                cost: Upgradable(3, 2),
                ..BaseCard::new(Watcher, Skill)
            },
            WISH => BaseCard {
                name: WISH,
                rarity: Rare,
                on_play: vec![Effect::Custom],
                cost: Fixed(3),
                ..BaseCard::new(Watcher, Skill)
            },
            BANDAGE_UP => BaseCard {
                name: BANDAGE_UP,
                rarity: Uncommon,
                on_play: vec![Heal(Upgradable(4, 6), _Self), ExhaustCard(This)],
                cost: Fixed(0),
                ..BaseCard::new(Class::None, Skill)
            },
            BLIND => BaseCard {
                name: BLIND,
                rarity: Uncommon,
                on_play: vec![If(
                    Condition::Upgraded,
                    vec![AddBuff(buffs::WEAK, Fixed(2), AllEnemies)],
                    vec![AddBuff(buffs::WEAK, Fixed(2), TargetEnemy)],
                )],
                cost: Fixed(0),
                ..BaseCard::new(Class::None, Skill)
            },
            DARK_SHACKLES => BaseCard {
                name: DARK_SHACKLES,
                rarity: Uncommon,
                on_play: vec![
                    LoseStr(Upgradable(9, 15), TargetEnemy),
                    ExhaustCard(This),
                ],
                cost: Fixed(0),
                ..BaseCard::new(Class::None, Skill)
            },
            DEEP_BREATH => BaseCard {
                name: DEEP_BREATH,
                rarity: Uncommon,
                on_play: vec![
                    MoveCard(DiscardPile(RelativePosition::All), DrawPile(RelativePosition::All), CardModifier::None),
                    Draw(Upgradable(1, 2))
                ],
                cost: Fixed(0),
                ..BaseCard::new(Class::None, Skill)
            },
            _ => panic!("Unsupported card"),
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
pub const A_THOUSAND_CUTS: &str = "A Thousand Cuts";
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
pub const POISONED_STAB: &str = "Poisoned Stab";
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
