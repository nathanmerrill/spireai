use crate::models::cards;
use crate::models::core::*;
use std::collections::HashMap;
use Amount::*;

impl BaseBuff {
    fn default() -> Self {
        Self {
            name: &"",
            is_additive: true,
            stacks: true,
            is_buff: true,
            expire_at: Event::Never,
            reduce_at: Event::Never,
            on_add: Effect::None,
            effects: vec![],
        }
    }
}

pub fn by_name(name: &str) -> &'static BaseBuff {
    BUFFS.get(name).unwrap_or_else(|| panic!("Unrecognized buff: {}", name))
}

lazy_static! {
    static ref BUFFS: HashMap<&'static str, BaseBuff> = {
        let mut m = HashMap::new();

        for buff in all_buffs() {
            m.insert(buff.name, buff);
        }

        m
    };
}

fn all_buffs() -> Vec<BaseBuff> {
    vec![
        BaseBuff {
            name: ACCURACY,
            effects: vec![(
                Event::PlayCard(CardType::ByName(cards::SHIV)),
                Effect::Boost(X),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: AFTER_IMAGE,
            effects: vec![(
                Event::PlayCard(CardType::All),
                Effect::Block(X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: AMPLIFY,
            effects: vec![(Event::PlayCard(CardType::Power), Effect::Duplicate)],
            expire_at: Event::BeforeEnemyMove,
            reduce_at: Event::PlayCard(CardType::Power),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ANGRY,
            effects: vec![(
                Event::AttackDamage(Target::_Self),
                Effect::AddBuff(STRENGTH, X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ARTIFACT,
            effects: vec![(
                Event::AnyBuff(Target::_Self),
                Effect::Cancel,
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ASLEEP,
            expire_at: Event::UnblockedDamage(Target::_Self),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BARRICADE,
            effects: vec![(
                Event::AfterEnemyMove,
                Effect::RetainBlock(PlayerBlock),
            )],
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BATTLE_HYMN,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddCard {
                    card: CardReference::ByName(cards::SMITE),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BEAT_OF_DEATH,
            effects: vec![(
                Event::PlayCard(CardType::All),
                Effect::Damage(X, Target::AllEnemies),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BERSERK,
            effects: vec![(Event::BeforeHandDraw, Effect::AddEnergy(X))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BIAS,
            is_buff: false,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddBuff(FOCUS, NegX, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BLASPHEMER,
            is_additive: false,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::Damage(Fixed(9999), Target::_Self),
            )],
            expire_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BLOCK_RETURN,
            is_buff: false,
            effects: vec![(
                Event::AttackDamage(Target::_Self),
                Effect::Block(X, Target::Attacker),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BLUR,
            effects: vec![(
                Event::AfterEnemyMove,
                Effect::RetainBlock(PlayerBlock),
            )],
            reduce_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BRUTALITY,
            effects: vec![
                (Event::BeforeHandDraw, Effect::LoseHp(X, Target::_Self)),
                (Event::BeforeHandDraw, Effect::Draw(X)),
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BUFFER,
            effects: vec![(Event::HpLoss(Target::_Self), Effect::Multiple(vec![Effect::Cancel, Effect::AddX(Amount::Fixed(-1))]))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: BURST,
            effects: vec![(Event::PlayCard(CardType::Skill), Effect::Duplicate)],
            reduce_at: Event::PlayCard(CardType::Skill),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: CHOKED,
            is_buff: false,
            expire_at: Event::BeforeHandDraw,
            effects: vec![(
                Event::PlayCard(CardType::All),
                Effect::LoseHp(X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: COLLECT,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddCard {
                    card: CardReference::ByName(cards::COLLECT),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: Fixed(1),
                    modifier: CardModifier::Upgraded,
                },
            )],
            reduce_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: COMBUST,
            on_add: Effect::AddN(Fixed(1)),
            effects: vec![
                (
                    Event::BeforeEnemyMove,
                    Effect::LoseHp(Amount::N, Target::_Self),
                ),
                (
                    Event::BeforeEnemyMove,
                    Effect::Damage(Amount::X, Target::AllEnemies),
                ),
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: CONFUSED,
            effects: vec![
                (Event::DrawCard(CardType::All),
                Effect::RandomizeCost(CardLocation::This)),
            ],
            is_additive: false,
            is_buff: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: CONSTRICTED,
            is_buff: false,
            effects: vec![(Event::BeforeEnemyMove, Effect::Damage(X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: CORPSE_EXPLOSION,
            effects: vec![
                (Event::Die(Target::_Self),
                Effect::Damage(Amount::Mult(vec![X, MaxHp]), Target::AnyFriendly)),
            ],
            is_buff: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: CORRUPTION, // TODO
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: CREATIVE_AI,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddCard {
                    card: CardReference::RandomType(CardType::Power, Fixed(1)),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: CURIOSITY,
            effects: vec![(
                Event::PlayCard(CardType::Power),
                Effect::AddBuff(STRENGTH, X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: CURL_UP,
            effects: vec![(
                Event::AttackDamage(Target::_Self),
                Effect::Block(X, Target::_Self),
            )],
            expire_at: Event::AttackDamage(Target::_Self),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DARK_EMBRACE,
            effects: vec![(Event::Exhaust, Effect::Draw(X))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DEMON_FORM,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddBuff(STRENGTH, X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DEVA,
            effects: vec![
                (Event::BeforeHandDraw, Effect::AddN(X)),
                (Event::BeforeHandDraw, Effect::AddEnergy(N)),
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DEVOTION,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddBuff(MANTRA, X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DEXTERITY, //TODO
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DEXTERITY_DOWN,
            is_buff: false,
            effects: vec![(
                Event::BeforeEnemyMove,
                Effect::AddBuff(DEXTERITY, NegX, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DOUBLE_DAMAGE,
            effects: vec![(
                Event::AttackDamage(Target::AllEnemies),
                Effect::BoostMult(Fixed(100)),
            )],
            reduce_at: Event::BeforeEnemyMove,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DOUBLE_TAP,
            effects: vec![(
                Event::PlayCard(CardType::Attack),
                Effect::Duplicate,
            )],
            reduce_at: Event::PlayCard(CardType::Attack),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DRAW_CARD,
            effects: vec![(Event::BeforeHandDraw, Effect::Draw(X))],
            expire_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DRAW_REDUCTION,
            is_buff: false,
            effects: vec![(Event::BeforeHandDraw, Effect::Draw(NegX))],
            expire_at: Event::BeforeEnemyMove,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: DUPLICATION,
            effects: vec![(
                Event::PlayCard(CardType::All),
                Effect::Duplicate,
            )],
            reduce_at: Event::PlayCard(CardType::All),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ECHO_FORM,
            effects: vec![
                (Event::BeforeHandDraw, Effect::SetN(Fixed(0))),
                (Event::PlayCard(CardType::Attack), Effect::If(Condition::LessThan(N, X), vec![
                    Effect::AddN(Fixed(1)),
                    Effect::Duplicate,
                ], vec![
                    Effect::AddN(Fixed(1)),
                ]))
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ELECTRO, // TODO
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ENERGIZED,
            effects: vec![(Event::BeforeHandDraw, Effect::AddEnergy(X))],
            expire_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ENRAGE,
            effects: vec![(
                Event::PlayCard(CardType::Attack),
                Effect::AddBuff(STRENGTH, X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ENTANGLED, // TODO
            is_buff: false,
            is_additive: false,
            expire_at: Event::BeforeEnemyMove,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ENVENOM,
            effects: vec![(
                Event::UnblockedDamage(Target::TargetEnemy),
                Effect::AddBuff(POISON, X, Target::TargetEnemy),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: EQUILIBRIUM,
            effects: vec![
                (Event::BeforeHandDiscard, Effect::RetainCard(CardLocation::PlayerHand(RelativePosition::All))),
            ],
            reduce_at: Event::BeforeHandDiscard,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: ESTABLISHMENT,
            effects: vec![
                (Event::RetainCard(CardType::All), Effect::AddCardCost(CardLocation::This, Fixed(-1))),
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: EVOLVE,
            effects: vec![(Event::DrawCard(CardType::Status), Effect::Draw(X))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: EXPLODE,
            reduce_at: Event::AttackDamage(Target::TargetEnemy),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FADING,
            reduce_at: Event::AfterEnemyMove,
            effects: vec![(
                Event::UnBuff(FADING, Target::_Self),
                Effect::Die(Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FASTING,
            is_buff: false,
            effects: vec![(Event::BeforeHandDraw, Effect::AddEnergy(NegX))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FEEL_NO_PAIN,
            effects: vec![(Event::Exhaust, Effect::Block(X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FIRE_BREATHING,
            effects: vec![(
                Event::Multiple(vec![
                    Event::DrawCard(CardType::Status),
                    Event::DrawCard(CardType::Status),
                ]),
                Effect::Damage(X, Target::AllEnemies),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FLAME_BARRIER,
            effects: vec![(
                Event::AttackDamage(Target::_Self),
                Effect::Damage(X, Target::Attacker),
            )],
            expire_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FLYING,
            effects: vec![
                (Event::AfterEnemyMove, Effect::Multiple(vec![
                    Effect::AddX(N),
                    Effect::SetN(Fixed(0)),
                ])),
                (Event::AttackDamage(Target::_Self), Effect::Multiple(vec![
                    Effect::BoostMult(Fixed(-50)),
                    Effect::If(Condition::Equals(X, Fixed(1)), vec![
                        Effect::SetN(Fixed(0))
                    ], vec![
                        Effect::AddN(Fixed(-1))
                    ]),
                    Effect::AddX(Fixed(-1))
                ])),
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FOCUS,  //TODO
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FORESIGHT,
            effects: vec![(Event::BeforeHandDraw, Effect::Scry(X))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FRAIL,
            effects: vec![
                (Event::Block(Target::_Self), Effect::BoostMult(Fixed(-25)))
            ],
            is_buff: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: FREE_ATTACK_POWER,  //TODO
            reduce_at: Event::PlayCard(CardType::Attack),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: HEATSINK,
            effects: vec![(Event::PlayCard(CardType::Power), Effect::Draw(X))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: HELLO,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddCard {
                    card: CardReference::RandomRarity(Rarity::Common),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: HEX,
            is_buff: false,
            effects: vec![(
                Event::Multiple(vec![
                    Event::PlayCard(CardType::Curse),
                    Event::PlayCard(CardType::Power),
                    Event::PlayCard(CardType::Skill),
                    Event::PlayCard(CardType::Status),
                ]),
                Effect::AddCard {
                    card: CardReference::ByName(cards::DAZED),
                    destination: CardLocation::DrawPile(RelativePosition::Random),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: INFINITE_BLADES,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddCard {
                    card: CardReference::ByName(cards::SHIV),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: INNATE_THIEVERY,  //TODO
            effects: vec![(Event::Damage(Target::TargetEnemy), Effect::Custom)],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: INTANGIBLE,
            effects: vec![
                (Event::Damage(Target::_Self), Effect::Cap(Fixed(1))),
                (Event::HpLoss(Target::_Self), Effect::Cap(Fixed(1))),
            ],
            reduce_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: INVINCIBLE, //TODO
            effects: vec![
                (Event::HpLoss(Target::_Self), Effect::Custom)
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: JUGGERNAUT,
            effects: vec![(
                Event::Block(Target::_Self),
                Effect::Damage(X, Target::RandomEnemy),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: LIFE_LINK,
            effects: vec![(Event::Die(Target::_Self),
                Effect::If(Condition::HasFriendlies(1), vec![
                    Effect::FakeDie,
                ], vec![
                    Effect::Die(Target::AnyFriendly)
                ])),
            ],
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: LIKE_WATER,
            effects: vec![(
                Event::BeforeEnemyMove,
                Effect::If(
                    Condition::Stance(Stance::Calm),
                    vec![Effect::Block(X, Target::_Self)],
                    vec![],
                ),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: LOCK_ON,
            is_buff: false,
            effects: vec![(Event::OrbDamage(Target::_Self), Effect::BoostMult(Fixed(50)))],
            reduce_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: LOOP,
            effects: vec![],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MACHINE_LEARNING,
            effects: vec![(Event::BeforeHandDraw, Effect::Draw(X))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MAGNETISM,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddCard {
                    card: CardReference::RandomClass(Class::None),
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MALLEABLE,
            effects: vec![
                (Event::BeforeHandDraw, Effect::SetX(Fixed(3))),
                (Event::AttackDamage(Target::_Self), Effect::Multiple(vec![
                    Effect::AddN(X),
                    Effect::AddX(Fixed(1)),
                ])),
                (Event::PlayCard(CardType::Attack), Effect::Multiple(vec![
                    Effect::Block(N, Target::_Self),
                    Effect::SetN(Fixed(0)),
                ])),
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MANTRA,
            on_add: Effect::If(
                Condition::LessThan(Fixed(10), X),
                vec![
                    Effect::AddX(Fixed(-10)),
                    Effect::SetStance(Stance::Divinity),
                ],
                vec![],
            ),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MARK,
            effects: vec![(
                Event::Buff(MARK, Target::AnyFriendly),
                Effect::LoseHp(X, Target::_Self),
            )],
            is_buff: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MASTER_REALITY,  //TODO
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MAYHEM,
            effects: vec![(
                Event::AfterHandDraw,
                Effect::AutoPlayCard(CardLocation::DrawPile(RelativePosition::Top)),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MENTAL_FORTRESS,
            effects: vec![(
                Event::StanceChange(Stance::All, Stance::All),
                Effect::Block(X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: METALLICIZE,
            effects: vec![(Event::TurnEnd, Effect::Block(X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: MODE_SHIFT,
            effects: vec![(Event::UnblockedDamage(Target::_Self), Effect::Custom)],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: NEXT_TURN_BLOCK,
            effects: vec![(Event::BeforeHandDraw, Effect::Block(X, Target::_Self))],
            expire_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: NIGHTMARE,
            is_additive: false,
            stacks: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: NIRVANA,
            effects: vec![(Event::Scry, Effect::Block(X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: NOXIOUS_FUMES,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddBuff(POISON, X, Target::AllEnemies),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: NO_BLOCK, //TODO
            is_buff: false,
            reduce_at: Event::BeforeEnemyMove,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: NO_DRAW,
            is_buff: false,
            is_additive: false,
            expire_at: Event::BeforeEnemyMove,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: OMEGA,
            effects: vec![(
                Event::BeforeEnemyMove,
                Effect::Damage(X, Target::AllEnemies),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: PAINFUL_STABS,
            effects: vec![(
                Event::UnblockedDamage(Target::TargetEnemy),
                Effect::AddCard {
                    card: CardReference::ByName(cards::WOUND),
                    destination: CardLocation::DiscardPile(RelativePosition::Bottom),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: PAINFUL_STABS,
            effects: vec![(
                Event::UnblockedDamage(Target::TargetEnemy),
                Effect::AddCard {
                    card: CardReference::ByName(cards::WOUND),
                    destination: CardLocation::DiscardPile(RelativePosition::Bottom),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: PANACHE,
            on_add: Effect::SetN(Fixed(5)),
            effects: vec![
                (Event::PlayCard(CardType::All), Effect::AddN(Fixed(-1))),
                (
                    Event::PlayCard(CardType::All),
                    Effect::If(
                        Condition::Equals(N, Fixed(0)),
                        vec![Effect::ResetN, Effect::Damage(X, Target::AllEnemies)],
                        vec![],
                    ),
                ),
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: PEN_NIB,
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: PHANTASMAL,
            reduce_at: Event::BeforeHandDraw,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddBuff(DOUBLE_DAMAGE, Fixed(1), Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: PLATED_ARMOR,
            effects: vec![(Event::BeforeEnemyMove, Effect::Block(X, Target::_Self))],
            reduce_at: Event::UnblockedDamage(Target::_Self),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: PLATED_ARMOR,
            effects: vec![(Event::BeforeHandDiscard, Effect::Block(X, Target::_Self))],
            reduce_at: Event::UnblockedDamage(Target::_Self),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: POISON,
            is_buff: false,
            effects: vec![(Event::BeforeEnemyMove, Effect::LoseHp(X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: RAGE,
            effects: vec![(
                Event::PlayCard(CardType::Attack),
                Effect::Block(X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: REACTIVE,
            effects: vec![(Event::UnblockedDamage(Target::_Self), Effect::Custom)],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: REBOUND,
            reduce_at: Event::PlayCard(CardType::All),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: REGENERATE,
            effects: vec![(Event::BeforeEnemyMove, Effect::Heal(X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: REGENERATION,
            reduce_at: Event::BeforeEnemyMove,
            effects: vec![(Event::BeforeEnemyMove, Effect::Heal(X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: REPAIR,
            effects: vec![(Event::CombatEnd, Effect::Heal(X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: RITUAL,
            effects: vec![(Event::TurnEnd, Effect::AddBuff(STRENGTH, X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: RITUAL,
            effects: vec![(Event::TurnEnd, Effect::AddBuff(STRENGTH, X, Target::_Self))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: RUPTURE,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: RUSHDOWN,
            effects: vec![(
                Event::StanceChange(Stance::All, Stance::Wrath),
                Effect::Draw(X),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SADISTIC,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SHACKLED,
            is_buff: false,
            effects: vec![(
                Event::BeforeEnemyMove,
                Effect::AddBuff(STRENGTH, X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SHARP_HIDE,
            effects: vec![(Event::Custom, Effect::Custom)],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SHIFTING,
            effects: vec![(
                Event::PlayCard(CardType::Attack),
                Effect::Damage(X, Target::Attacker),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SIMMERING_RAGE,
            is_additive: false,
            expire_at: Event::BeforeHandDraw,
            effects: vec![(Event::BeforeHandDraw, Effect::SetStance(Stance::Wrath))],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SLOW,
            is_buff: false,
            effects: vec![(Event::PlayCard(CardType::All), Effect::Custom)],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SPLIT,
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SPLIT,
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SPORE_CLOUD,
            effects: vec![(
                Event::Die(Target::_Self),
                Effect::AddBuff(VULNERABLE, X, Target::AllEnemies),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: STASIS,
            is_additive: false,
            effects: vec![(Event::Die(Target::_Self), Effect::Custom)],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: STATIC_DISCHARGE,
            effects: vec![(
                Event::UnblockedDamage(Target::_Self),
                Effect::ChannelOrb(OrbType::Lightning),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: STORM,
            effects: vec![(
                Event::PlayCard(CardType::Power),
                Effect::ChannelOrb(OrbType::Lightning),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: STRENGTH,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: STRENGTH,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: STRENGTH_DOWN,
            is_buff: false,
            effects: vec![(
                Event::BeforeEnemyMove,
                Effect::AddBuff(STRENGTH, NegX, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: STRENGTH_UP,
            effects: vec![(
                Event::AfterEnemyMove,
                Effect::AddBuff(STRENGTH, X, Target::_Self),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: STUDY,
            effects: vec![(
                Event::BeforeEnemyMove,
                Effect::AddCard {
                    card: CardReference::ByName(cards::INSIGHT),
                    destination: CardLocation::DrawPile(RelativePosition::Random),
                    copies: X,
                    modifier: CardModifier::None,
                },
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: SURROUNDED,
            is_additive: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: THE_BOMB,
            is_additive: false,
            effects: vec![(Event::BeforeEnemyMove, Effect::Custom)],
            stacks: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: THORNS,
            effects: vec![(
                Event::AttackDamage(Target::_Self),
                Effect::Damage(X, Target::Attacker),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: THOUSAND_CUTS,
            effects: vec![(
                Event::PlayCard(CardType::All),
                Effect::Damage(X, Target::AllEnemies),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: TIME_WARP,
            is_additive: false,
            reduce_at: Event::PlayCard(CardType::All),
            effects: vec![(Event::UnBuff(TIME_WARP, Target::_Self), Effect::Custom)],
            stacks: false,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: TOOLS_OF_THE_TRADE,
            effects: vec![
                (Event::AfterHandDraw, Effect::Draw(X)),
                (
                    Event::AfterHandDraw,
                    Effect::DiscardCard(CardLocation::PlayerHand(RelativePosition::PlayerChoice(
                        Fixed(1),
                    ))),
                ),
            ],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: VIGOR,
            effects: vec![(Event::PlayCard(CardType::Attack), Effect::Custom)],
            expire_at: Event::PlayCard(CardType::Attack),
            ..BaseBuff::default()
        },
        BaseBuff {
            name: VULNERABLE,
            is_buff: false,
            reduce_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: WAVE_OF_THE_HAND,
            effects: vec![(
                Event::Block(Target::_Self),
                Effect::AddBuff(WEAK, X, Target::AllEnemies),
            )],
            ..BaseBuff::default()
        },
        BaseBuff {
            name: WEAK,
            is_buff: false,
            reduce_at: Event::BeforeHandDraw,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: WELL_LAID_PLANS,
            ..BaseBuff::default()
        },
        BaseBuff {
            name: WRAITH_FORM,
            is_buff: false,
            effects: vec![(
                Event::BeforeHandDraw,
                Effect::AddBuff(DEXTERITY, NegX, Target::_Self),
            )],
            ..BaseBuff::default()
        },
    ]
}

pub const ACCURACY: &str = "Accuracy";
pub const AFTER_IMAGE: &str = "After Image";
pub const AMPLIFY: &str = "Amplify";
pub const ANGRY: &str = "Angry";
pub const ARTIFACT: &str = "Artifact";
pub const ASLEEP: &str = "Asleep";
pub const BARRICADE: &str = "Barricade";
pub const BATTLE_HYMN: &str = "Battle Hymn";
pub const BEAT_OF_DEATH: &str = "Beat Of Death";
pub const BIAS: &str = "Bias";
pub const BERSERK: &str = "Berserk";
pub const BLASPHEMER: &str = "Blasphemer";
pub const BLOCK_RETURN: &str = "Block Return";
pub const BLUR: &str = "Blur";
pub const BRUTALITY: &str = "Brutality";
pub const BUFFER: &str = "Buffer";
pub const BURST: &str = "Burst";
pub const CHOKED: &str = "Choked";
pub const COLLECT: &str = "Collect";
pub const CONFUSED: &str = "Confused";
pub const CONSTRICTED: &str = "Constricted";
pub const COMBUST: &str = "Combust";
pub const CORRUPTION: &str = "Corruption";
pub const CORPSE_EXPLOSION: &str = "Corpse Explosion";
pub const CREATIVE_AI: &str = "Creative AI";
pub const CURIOSITY: &str = "Curiosity";
pub const CURL_UP: &str = "Curl Up";
pub const DARK_EMBRACE: &str = "Dark Embrace";
pub const DEMON_FORM: &str = "Demon Form";
pub const DEVA: &str = "Deva";
pub const DEVOTION: &str = "Devotion";
pub const DEXTERITY: &str = "Dexterity";
pub const DEXTERITY_DOWN: &str = "Dexterity Down";
pub const DOUBLE_DAMAGE: &str = "Double Damage";
pub const DOUBLE_TAP: &str = "Double Tap";
pub const DRAW_CARD: &str = "Draw Card";
pub const DRAW_REDUCTION: &str = "Draw Reduction";
pub const DUPLICATION: &str = "Duplication";
pub const ECHO_FORM: &str = "Echo Form";
pub const ENERGIZED: &str = "Energized";
pub const ENRAGE: &str = "Enrage";
pub const ELECTRO: &str = "Electro";
pub const ENTANGLED: &str = "Entangled";
pub const ENVENOM: &str = "Envenom";
pub const EQUILIBRIUM: &str = "Equilibrium";
pub const ESTABLISHMENT: &str = "Establishment";
pub const EVOLVE: &str = "Evolve";
pub const EXPLODE: &str = "Explode";
pub const FADING: &str = "Fading";
pub const FASTING: &str = "Fasting";
pub const FEEL_NO_PAIN: &str = "Feel No Pain";
pub const FIRE_BREATHING: &str = "Fire Breathing";
pub const FLAME_BARRIER: &str = "Flame Barrier";
pub const FLYING: &str = "Flying";
pub const FOCUS: &str = "Focus";
pub const FORESIGHT: &str = "Foresight";
pub const FRAIL: &str = "Frail";
pub const FREE_ATTACK_POWER: &str = "Free Attack Power";
pub const HEATSINK: &str = "Heatsink";
pub const HELLO: &str = "Hello";
pub const HEX: &str = "Hex";
pub const INFINITE_BLADES: &str = "Infinite Blades";
pub const INNATE_THIEVERY: &str = "Thievery";
pub const INTANGIBLE: &str = "Intangible";
pub const INVINCIBLE: &str = "Invincible";
pub const JUGGERNAUT: &str = "Juggernaut";
pub const LIFE_LINK: &str = "Life Link";
pub const LIKE_WATER: &str = "Like Water";
pub const LOCK_ON: &str = "Lock-On";
pub const LOOP: &str = "Loop";
pub const MACHINE_LEARNING: &str = "Machine Learning";
pub const MAGNETISM: &str = "Magnetism";
pub const MALLEABLE: &str = "Malleable";
pub const MANTRA: &str = "Mantra";
pub const MARK: &str = "Mark";
pub const MASTER_REALITY: &str = "Master Reality";
pub const MAYHEM: &str = "Mayhem";
pub const MENTAL_FORTRESS: &str = "Mental Fortress";
pub const METALLICIZE: &str = "Metallicize";
pub const MODE_SHIFT: &str = "Mode Shift";
pub const NEXT_TURN_BLOCK: &str = "Next Turn Block";
pub const NIGHTMARE: &str = "Nightmare";
pub const NIRVANA: &str = "Nirvana";
pub const NO_BLOCK: &str = "No Block";
pub const NO_DRAW: &str = "No Draw";
pub const NOXIOUS_FUMES: &str = "Noxious Fumes";
pub const OMEGA: &str = "Omega";
pub const PAINFUL_STABS: &str = "Painful Stabs";
pub const PANACHE: &str = "Panache";
pub const PEN_NIB: &str = "Pen Nib";
pub const PLATED_ARMOR: &str = "Plated Armor";
pub const PHANTASMAL: &str = "Phantasmal";
pub const POISON: &str = "Poison";
pub const RAGE: &str = "Rage";
pub const REACTIVE: &str = "Reactive";
pub const REBOUND: &str = "Rebound";
pub const REGENERATION: &str = "Regeneration";
pub const REGENERATE: &str = "Regenerate";
pub const REPAIR: &str = "Repair";
pub const RITUAL: &str = "Ritual";
pub const RUSHDOWN: &str = "Rushdown";
pub const RUPTURE: &str = "Rupture";
pub const SADISTIC: &str = "Sadistic";
pub const SHACKLED: &str = "Shackled";
pub const SHARP_HIDE: &str = "Sharp Hide";
pub const SHIFTING: &str = "Shifting";
pub const SIMMERING_RAGE: &str = "Simmering Rage";
pub const SLOW: &str = "Slow";
pub const SPORE_CLOUD: &str = "Spore Cloud";
pub const SPLIT: &str = "Split";
pub const STASIS: &str = "Stasis";
pub const STATIC_DISCHARGE: &str = "Static Discharge";
pub const STORM: &str = "Storm";
pub const STRENGTH: &str = "Strength";
pub const STRENGTH_DOWN: &str = "Strength Down";
pub const STRENGTH_UP: &str = "Strength Up";
pub const STUDY: &str = "Study";
pub const SURROUNDED: &str = "Surrounded";
pub const TIME_WARP: &str = "Time Warp";
pub const THE_BOMB: &str = "The Bomb";
pub const THORNS: &str = "Thorns";
pub const THOUSAND_CUTS: &str = "Thousand Cuts";
pub const TOOLS_OF_THE_TRADE: &str = "Tools of the Trade";
pub const VIGOR: &str = "Vigor";
pub const VULNERABLE: &str = "Vulnerable";
pub const WAVE_OF_THE_HAND: &str = "Wave of the Hand";
pub const WEAK: &str = "Weakened";
pub const WELL_LAID_PLANS: &str = "Well-Laid Plans";
pub const WRAITH_FORM: &str = "Wraith Form";
