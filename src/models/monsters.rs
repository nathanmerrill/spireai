use crate::models::core::*;
use crate::models::cards;
use crate::models::buffs;
use Amount::*;

impl BaseMonster {
    pub fn default() -> Self {
        Self {
            name: "",
            hp_range: (0, 0),
            hp_range_asc: (0, 0),
            n_range: (Fixed(0), Fixed(0)),
            x_range: (Fixed(0), Fixed(0)),
            moveset: vec![],
            move_order: vec![],
            effects: vec![],
        }
    }

    pub fn by_name(name: &str) -> BaseMonster {
        match name {
            ACID_SLIME_L => {
                Self {
                    hp_range: (65, 68),
                    hp_range_asc: (69, 72),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::SPLIT, Fixed(1), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: CORROSIVE_SPIT,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(11, 12, 12), Target::TargetEnemy),
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::SLIMED), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: Fixed(2),
                                    modifier: CardModifier::None,
                                }
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: LICK,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(2), Target::TargetEnemy)
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![Effect::AttackDamage(ByAsc(16, 18, 18), Target::TargetEnemy)],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SPLIT,
                            effects: vec![
                                Effect::Split(ACID_SLIME_M, ACID_SLIME_M)
                            ],
                            intent: Intent::Unknown,
                        },
                    ],
                    move_order: vec![
                        Move::If(Condition::Asc(17), 
                            vec![Move::Loop(vec![
                                Move::Probability(vec![
                                    (40, CORROSIVE_SPIT, 2),
                                    (30, TACKLE, 2),
                                    (30, LICK, 1),
                                ]),
                            ])],
                            vec![Move::Loop(vec![
                                Move::Probability(vec![
                                    (30, CORROSIVE_SPIT, 2),
                                    (40, TACKLE, 1),
                                    (30, LICK, 2),
                            ])])],
                        ),
                        Move::Event(Event::HalfHp(Target::_Self)),
                        Move::InOrder(SPLIT),
                    ],
                    ..BaseMonster::default()
                }
            },
            ACID_SLIME_M => {
                Self {
                    hp_range: (28, 32),
                    hp_range_asc: (29, 34),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::SPLIT, Fixed(1), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: CORROSIVE_SPIT,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::SLIMED), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None,
                                }
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: LICK,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(1), Target::TargetEnemy)
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![Effect::AttackDamage(ByAsc(10, 12, 12), Target::TargetEnemy)],
                            intent: Intent::Attack,
                        }
                    ],
                    move_order: vec![
                        Move::If(Condition::Asc(17), 
                            vec![Move::Loop(vec![
                                Move::Probability(vec![
                                    (40, CORROSIVE_SPIT, 2),
                                    (30, TACKLE, 2),
                                    (30, LICK, 1),
                            ])])], 
                            vec![Move::Loop(vec![
                                Move::Probability(vec![
                                    (30, CORROSIVE_SPIT, 2),
                                    (40, TACKLE, 1),
                                    (30, LICK, 2),
                            ])])],
                        ),
                    ],
                    ..BaseMonster::default()
                }
            },
            ACID_SLIME_S => {
                Self {
                    hp_range: (8, 12),
                    hp_range_asc: (9, 13),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::SPLIT, Fixed(1), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: LICK,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(1), Target::TargetEnemy)
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![Effect::AttackDamage(ByAsc(3, 4, 4), Target::TargetEnemy)],
                            intent: Intent::Attack,
                        }
                    ],
                    move_order: vec![
                        Move::If(Condition::Asc(17), vec![
                            Move::InOrder(LICK)
                        ], vec![]),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (50, LICK, 1),
                                (50, TACKLE, 1),
                        ])]),
                    ],
                    ..BaseMonster::default()
                }
            },
            AWAKENED_ONE => {                
                Self {
                    hp_range: (300, 300),
                    hp_range_asc: (320, 320),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::CURIOSITY, ByAsc(1, 1, 2), Target::_Self)),
                        
                        (Event::CombatStart, Effect::AddBuff(buffs::STRENGTH, ByAsc(0, 2, 2), Target::_Self)),
                        (Event::CombatStart, Effect::AddBuff(buffs::REGENERATE, ByAsc(10, 10, 15), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: SLASH,
                            effects: vec![
                                Effect::AttackDamage(Fixed(20), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SOUL_STRIKE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: REBIRTH,
                            effects: vec![
                                Effect::RemoveDebuffs(Target::_Self),
                                Effect::HealPercentage(100, Target::_Self)
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: DARK_ECHO,
                            effects: vec![
                                Effect::AttackDamage(Fixed(40), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SLUDGE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(18), Target::TargetEnemy),
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::VOID), 
                                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None,
                                }
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(SLASH),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (75, SLASH, 2),
                                (25, SOUL_STRIKE, 1),
                        ])]),
                        Move::Event(Event::Die(Target::_Self)),
                        Move::InOrder(REBIRTH),
                        Move::InOrder(DARK_ECHO),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (50, SLUDGE, 2),
                                (50, TACKLE, 2),
                        ])]),
                    ],
                    ..BaseMonster::default()
                }
            },
            BEAR => {
                Self {
                    hp_range: (38, 42),
                    hp_range_asc: (40, 44),
                    moveset: vec![
                        MonsterMove {
                            name: BEAR_HUG,
                            effects: vec![Effect::AddBuff(buffs::DEXTERITY, ByAsc(-2, -2, -4), Target::TargetEnemy)],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: LUNGE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(9), Target::TargetEnemy),
                                Effect::Block(ByAsc(9, 10, 10), Target::_Self),
                            ],
                            intent: Intent::AttackDefend,
                        },
                        MonsterMove {
                            name: MAUL,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(18, 20, 20), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(BEAR_HUG),
                        Move::Loop(vec![
                            Move::InOrder(LUNGE),
                            Move::InOrder(MAUL),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            BOOK_OF_STABBING => {
                Self {
                    hp_range: (160, 162),
                    hp_range_asc: (168, 172),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::PAINFUL_STABS, Fixed(1), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: MULTI_STAB,
                            effects: vec![Effect::Custom],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SINGLE_STAB,
                            effects: vec![Effect::AttackDamage(ByAsc(21, 24, 24), Target::TargetEnemy)],
                            intent: Intent::Attack,
                        }
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (85, MULTI_STAB, 2),
                                (15, MULTI_STAB, 1),
                        ])]),
                    ],
                    ..BaseMonster::default()
                }
            },
            BLUE_SLAVER => {
                Self {
                    hp_range: (46, 50),
                    hp_range_asc: (48, 52),
                    moveset: vec![
                        MonsterMove {
                            name: STAB,
                            effects: vec![Effect::AttackDamage(ByAsc(12, 13, 13), Target::TargetEnemy)],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: RAKE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                                Effect::AddBuff(buffs::WEAK, ByAsc(1, 1, 2), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        }
                    ],
                    move_order: vec![
                        Move::If(Condition::Asc(17), 
                            vec![Move::Loop(vec![
                                Move::Probability(vec![
                                    (60, STAB, 2),
                                    (40, RAKE, 1),
                            ])])],
                            vec![Move::Loop(vec![
                                Move::Probability(vec![
                                    (60, STAB, 2),
                                (40, RAKE, 2),
                            ])])],
                        ),
                        
                    ],
                    ..BaseMonster::default()
                }
            },
            BRONZE_AUTOMATON => {
                Self {
                    hp_range: (300, 300),
                    hp_range_asc: (320, 320),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::ARTIFACT, Fixed(3), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: SPAWN_ORBS,
                            effects: vec![
                                Effect::Spawn {
                                    choices: vec![BRONZE_ORB],
                                    count: Fixed(2),
                                },
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: FLAIL,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: BOOST,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 4, 4), Target::_Self),
                                Effect::Block(ByAsc(9, 12, 12), Target::_Self),
                            ],
                            intent: Intent::DefendBuff,
                        },
                        MonsterMove {
                            name: HYPERBEAM,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(45, 50, 50), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: STUNNED,
                            effects: vec![],
                            intent: Intent::Stun,
                        },                        
                    ],
                    move_order: vec![
                        Move::InOrder(SPAWN_ORBS),
                        Move::Loop(vec![
                            Move::InOrder(FLAIL),
                            Move::InOrder(BOOST),
                            Move::InOrder(FLAIL),
                            Move::InOrder(BOOST),
                            Move::InOrder(HYPERBEAM),
                            Move::If(Condition::Asc(19), vec![
                                Move::InOrder(BOOST)
                            ], vec![
                                Move::InOrder(STUNNED)
                            ])
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            BRONZE_ORB => {
                Self {
                    hp_range: (52, 58),
                    hp_range_asc: (54, 60),
                    moveset: vec![
                        MonsterMove {
                            name: STASIS,
                            effects: vec![
                                Effect::Custom,
                            ],
                            intent: Intent::StrongDebuff
                        },
                        MonsterMove {
                            name: BEAM,
                            effects: vec![
                                Effect::AttackDamage(Fixed(8), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SUPPORT_BEAM,
                            effects: vec![
                                Effect::Block(Fixed(12), Target::Friendly(BRONZE_AUTOMATON)),
                            ],
                            intent: Intent::Defend,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (150, STASIS, 1),
                                (15, BEAM, 0),
                                (35, SUPPORT_BEAM, 0),
                        ])]),
                        Move::Event(Event::Buff(buffs::STASIS, Target::_Self)),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (30, BEAM, 2),
                                (70, SUPPORT_BEAM, 2),
                        ])]),
                    ],
                    ..BaseMonster::default()
                }
            },
            BYRD => {
                Self {
                    hp_range: (25, 31),
                    hp_range_asc: (26, 33),
                    effects: vec![(Event::CombatStart, Effect::AddBuff(buffs::FLYING, ByAsc(3, 3, 4), Target::_Self))],
                    moveset: vec![
                        MonsterMove {
                            name: CAW,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, Fixed(1), Target::_Self),
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: PECK,
                            effects: vec![
                                Effect::AttackDamage(Fixed(1), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(1), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(1), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(1), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(1), Target::TargetEnemy),
                                Effect::If(Condition::Asc(2), vec![
                                    Effect::AttackDamage(Fixed(1), Target::TargetEnemy)
                                ], vec![])
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SWOOP,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(12, 14, 14), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: FLY,
                            effects: vec![
                                Effect::AddBuff(buffs::FLYING, ByAsc(3, 3, 4), Target::_Self),
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: HEADBUTT,
                            effects: vec![
                                Effect::AttackDamage(Fixed(3), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::Probability(vec![
                            (125, PECK, 1),
                            (75, CAW, 1),
                        ]),
                        Move::Loop(vec![
                            Move::Loop(vec![
                                Move::Probability(vec![
                                    (50, PECK, 2),
                                    (20, SWOOP, 1),
                                    (30, CAW, 1),
                            ])]),
                            Move::Event(Event::UnBuff(buffs::FLYING, Target::_Self)),
                            Move::InOrder(STUNNED),
                            Move::InOrder(HEADBUTT),
                            Move::InOrder(FLY),
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            CENTURION => {
                Self {
                    hp_range: (76, 80),
                    hp_range_asc: (78, 83),
                    moveset: vec![
                        MonsterMove {
                            name: SLASH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(12, 14, 14), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: FURY,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(6, 7, 7), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(6, 7, 7), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(6, 7, 7), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: DEFEND,
                            effects: vec![
                                Effect::If(Condition::Dead(Target::Friendly(MYSTIC)), vec![
                                    Effect::Block(ByAsc(15, 15, 20), Target::_Self),
                                ], vec![]),
                                Effect::Block(ByAsc(15, 15, 20), Target::Friendly(MYSTIC)),
                            ],
                            intent: Intent::Defend,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::HasFriendlies(1), vec![
                                Move::Probability(vec![
                                    (65, DEFEND, 0),
                                    (35, SLASH, 0),
                                ])
                            ], vec![
                                Move::Probability(vec![
                                    (65, FURY, 0),
                                    (35, SLASH, 0),
                                ])
                            ])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            CHOSEN => {
                Self {
                    hp_range: (76, 80),
                    hp_range_asc: (78, 83),
                    moveset: vec![
                        MonsterMove {
                            name: POKE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(5, 6, 6), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(5, 6, 6), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: ZAP,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(18, 21, 21), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: DEBILITATE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 12, 12), Target::TargetEnemy),
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: DRAIN,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(3), Target::TargetEnemy),
                                Effect::AddBuff(buffs::STRENGTH, Fixed(3), Target::_Self),
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: HEX,
                            effects: vec![
                                Effect::AddBuff(buffs::HEX, Fixed(1), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::If(Condition::Asc(17), vec![], vec![
                            Move::InOrder(POKE),
                        ]),
                        Move::InOrder(HEX),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (50, DEBILITATE, 0),
                                (50, DRAIN, 0),
                            ]),
                            Move::Probability(vec![
                                (60, POKE, 0),
                                (40, ZAP, 0),
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            CORRUPT_HEART => {
                Self {
                    hp_range: (750, 750),
                    hp_range_asc: (800, 800),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::BEAT_OF_DEATH, ByAsc(1, 1, 2), Target::_Self)),
                        (Event::CombatStart, Effect::AddBuff(buffs::INVINCIBLE, ByAsc(300, 300, 200), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: DEBILITATE,
                            effects: vec![
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), Target::TargetEnemy),
                                Effect::AddBuff(buffs::WEAK, Fixed(2), Target::TargetEnemy),
                                Effect::AddBuff(buffs::FRAIL, Fixed(2), Target::TargetEnemy),
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::DAZED), 
                                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None
                                },
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::SLIMED), 
                                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None
                                },
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::WOUND), 
                                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None
                                },
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::BURN), 
                                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None
                                },
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::VOID), 
                                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None
                                },
                            ],
                            intent: Intent::StrongDebuff
                        },
                        MonsterMove {
                            name: BLOOD_SHOTS,
                            effects: vec![
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                Effect::If(Condition::Asc(4), vec![
                                    Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                    Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                    Effect::AttackDamage(Fixed(2), Target::TargetEnemy),
                                ], vec![]),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: ECHO,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(40, 45, 45), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: BUFF,
                            effects: vec![
                                Effect::Custom,
                            ],
                            intent: Intent::StrongDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(DEBILITATE),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (50, BLOOD_SHOTS, 1),
                                (50, ECHO, 1),
                            ]),
                            Move::Probability(vec![
                                (50, BLOOD_SHOTS, 1),
                                (50, ECHO, 1),
                            ]),
                            Move::InOrder(BUFF),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            CULTIST => {
                Self {
                    hp_range: (48, 54),
                    hp_range_asc: (50, 56),
                    moveset: vec![
                        MonsterMove {
                            name: INCANTATION,
                            effects: vec![
                                Effect::AddBuff(buffs::RITUAL, ByAsc(3, 4, 5), Target::_Self)
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: DARK_STRIKE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(INCANTATION),
                        Move::Loop(vec![
                            Move::InOrder(DARK_STRIKE),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            DAGGER => {
                Self {
                    hp_range: (20, 25),
                    hp_range_asc: (20, 25),
                    moveset: vec![
                        MonsterMove {
                            name: STAB,
                            effects: vec![
                                Effect::AttackDamage(Fixed(9), Target::TargetEnemy),
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::WOUND), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None,
                                }
                            ],
                            intent: Intent::AttackDebuff
                        },
                        MonsterMove {
                            name: EXPLODE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(25), Target::TargetEnemy),
                                Effect::Die(Target::_Self),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(STAB),
                        Move::InOrder(EXPLODE),
                    ],
                    ..BaseMonster::default()
                }
            },
            DARKLING => {
                Self {
                    hp_range: (48, 56),
                    hp_range_asc: (50, 59),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::LIFE_LINK, Fixed(1), Target::_Self)),
                    ],
                    n_range: (ByAsc(7, 9, 9), ByAsc(9, 11, 11)),
                    moveset: vec![
                        MonsterMove {
                            name: NIP,
                            effects: vec![
                                Effect::AttackDamage(N, Target::TargetEnemy),
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: CHOMP,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(8, 9, 9), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(8, 9, 9), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: HARDEN,
                            effects: vec![
                                Effect::Block(Fixed(12), Target::_Self),
                                Effect::If(Condition::Asc(17), vec![
                                    Effect::AddBuff(buffs::STRENGTH, Fixed(2), Target::_Self)
                                ], vec![])
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: REINCARNATE,
                            effects: vec![
                                Effect::HealPercentage(50, Target::_Self),
                            ],
                            intent: Intent::Buff,
                        },
                        MonsterMove {
                            name: REGROW,
                            effects: vec![],
                            intent: Intent::Unknown,
                        },
                    ],
                    move_order: vec![
                        Move::Probability(vec![
                            (50, NIP, 1),
                            (50, HARDEN, 1),
                        ]),
                        Move::Loop(vec![
                            Move::If(Condition::InPosition(Target::_Self, 1), vec![
                                Move::Loop(vec![
                                    Move::Probability(vec![
                                        (50, NIP, 2),
                                        (50, HARDEN, 1),
                                    ]),
                                ])
                            ], vec![
                                Move::Loop(vec![
                                    Move::Probability(vec![
                                        (30, NIP, 2),
                                        (40, CHOMP, 1),
                                        (30, HARDEN, 1),
                                    ]),
                                ]),
                            ]),
                            Move::Event(Event::Die(Target::_Self)),
                            Move::InOrder(REGROW),
                            Move::InOrder(REINCARNATE),
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            DECA => {
                Self {
                    hp_range: (250, 250),
                    hp_range_asc: (265, 265),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::ARTIFACT, ByAsc(2, 2, 3), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: SQUARE_OF_PROTECTION,
                            effects: vec![
                                Effect::Block(Fixed(16), Target::_Self),
                                Effect::Block(Fixed(16), Target::Friendly(DONU)),
                                Effect::If(Condition::Asc(19), vec![
                                    Effect::AddBuff(buffs::PLATED_ARMOR, Fixed(3), Target::_Self),
                                    Effect::AddBuff(buffs::PLATED_ARMOR, Fixed(3), Target::Friendly(DONU)),
                                ], vec![])
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: BEAM,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 12, 12), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(10, 12, 12), Target::TargetEnemy),
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::DAZED), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: Fixed(2),
                                    modifier: CardModifier::None,
                                },
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(BEAM),
                            Move::InOrder(SQUARE_OF_PROTECTION),
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            DONU => {
                Self {
                    hp_range: (250, 250),
                    hp_range_asc: (265, 265),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::ARTIFACT, ByAsc(2, 2, 3), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: CIRCLE_OF_POWER,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, Fixed(3), Target::_Self),
                                Effect::AddBuff(buffs::STRENGTH, Fixed(3), Target::Friendly(DECA)),
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: BEAM,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 12, 12), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(10, 12, 12), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(CIRCLE_OF_POWER),
                            Move::InOrder(BEAM),
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            EXPLODER => {
                Self {
                    hp_range: (30, 30),
                    hp_range_asc: (30, 35),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::EXPLODE, Fixed(3), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: SLAM,
                            effects: vec![
                                Effect::AttackDamage(Fixed(9), Target::TargetEnemy),
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: EXPLODE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(30), Target::TargetEnemy),
                                Effect::Die(Target::_Self),
                            ],
                            intent: Intent::Unknown,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(SLAM),
                        Move::InOrder(SLAM),
                        Move::InOrder(EXPLODE),
                    ],
                    ..BaseMonster::default()
                }
            },
            FAT_GREMLIN => {
                Self {
                    hp_range: (13, 17),
                    hp_range_asc: (14, 18),
                    moveset: vec![
                        MonsterMove {
                            name: SMASH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(4, 5, 5), Target::TargetEnemy),
                                Effect::AddBuff(buffs::WEAK, Fixed(1), Target::TargetEnemy),
                                Effect::If(Condition::Asc(17), vec![
                                    Effect::AddBuff(buffs::FRAIL, Fixed(1), Target::TargetEnemy)
                                ], vec![])
                            ],
                            intent: Intent::Attack
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(SMASH)
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            FUNGI_BEAST => {
                Self {
                    hp_range: (22, 28),
                    hp_range_asc: (24, 28),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::SPORE_CLOUD, Fixed(2), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: BITE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: GROW,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 4, 5), Target::_Self),
                            ],
                            intent: Intent::Buff
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (60, BITE, 2),
                                (40, GROW, 1)
                            ])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            GIANT_HEAD => {
                Self {
                    hp_range: (500, 500),
                    hp_range_asc: (520, 520),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::SLOW, Fixed(1), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: COUNT,
                            effects: vec![
                                Effect::AttackDamage(Fixed(13), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: GLARE,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(1), Target::TargetEnemy),
                            ],
                            intent: Intent::Debuff
                        },
                        MonsterMove {
                            name: IT_IS_TIME,
                            effects: vec![
                                Effect::AttackDamage(Custom, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                    ],
                    move_order: vec![
                        Move::Probability(vec![
                            (50, COUNT, 2),
                            (50, GLARE, 2),
                        ]),
                        Move::Probability(vec![
                            (50, COUNT, 2),
                            (50, GLARE, 2),
                        ]),
                        Move::Probability(vec![
                            (50, COUNT, 2),
                            (50, GLARE, 2),
                        ]),
                        Move::If(Condition::Asc(18), vec![], vec![
                            Move::Probability(vec![
                                (50, COUNT, 2),
                                (50, GLARE, 2),
                            ]),
                        ]),
                        Move::Loop(vec![
                            Move::InOrder(IT_IS_TIME),
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            GREEN_LOUSE => {
                Self {
                    hp_range: (11, 17),
                    hp_range_asc: (12, 18),
                    n_range: (ByAsc(3, 4, 9), ByAsc(7, 8, 12)),
                    x_range: (ByAsc(5, 6, 6), ByAsc(7, 8, 8)),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::CURL_UP, N, Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: BITE,
                            effects: vec![
                                Effect::AttackDamage(X, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: SPIT_WEB,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(2), Target::TargetEnemy),
                            ],
                            intent: Intent::Debuff
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::Asc(17), vec![
                                Move::Probability(vec![
                                    (25, SPIT_WEB, 1),
                                    (75, BITE, 2),
                                ]),
                            ], vec![
                                Move::Probability(vec![
                                    (25, SPIT_WEB, 2),
                                    (75, BITE, 2),
                                ]),
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            GREMLIN_LEADER => {
                Self {
                    hp_range: (140, 148),
                    hp_range_asc: (145, 155),
                    moveset: vec![
                        MonsterMove {
                            name: ENCOURAGE,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 4, 5), Target::AnyFriendly),
                                Effect::Block(ByAsc(6, 6, 10), Target::AnyFriendly),
                            ],
                            intent: Intent::DefendBuff
                        },
                        MonsterMove {
                            name: RALLY,
                            effects: vec![
                                Effect::Spawn{
                                    choices: vec![
                                        FAT_GREMLIN,
                                        MAD_GREMLIN,
                                        SHIELD_GREMLIN,
                                        SNEAKY_GREMLIN,
                                        GREMLIN_WIZARD,
                                    ],
                                    count: Fixed(2),
                                }
                            ],
                            intent: Intent::Unknown
                        },
                        MonsterMove {
                            name: STAB,
                            effects: vec![
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::HasFriendlies(0), vec![
                                Move::Probability(vec![
                                    (75, RALLY, 1),
                                    (25, STAB, 1)
                                ])
                            ], vec![
                                Move::If(Condition::HasFriendlies(1), vec![
                                    Move::Probability(vec![
                                        (25, RALLY, 1),
                                        (25, STAB, 1),
                                        (15, ENCOURAGE, 1),
                                    ])
                                ], vec![ 
                                    Move::Probability(vec![
                                        (66, ENCOURAGE, 1),
                                        (34, STAB, 1),
                                    ])
                                ]),
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            GREMLIN_NOB => {
                Self {
                    hp_range: (82, 86),
                    hp_range_asc: (85, 90),
                    moveset: vec![
                        MonsterMove {
                            name: BELLOW,
                            effects: vec![
                                Effect::AddBuff(buffs::ENRAGE, ByAsc(2, 2, 3), Target::_Self),
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: RUSH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(14, 16, 16), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: SKULL_BASH,
                            effects: vec![
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(BELLOW),
                        Move::If(Condition::Asc(18), vec![
                            Move::Loop(vec![
                                Move::InOrder(SKULL_BASH),
                                Move::InOrder(RUSH),
                                Move::InOrder(RUSH),
                            ])
                        ], vec![
                            Move::Loop(vec![
                                Move::Probability(vec![
                                    (33, SKULL_BASH, 0),
                                    (67, RUSH, 2),
                                ])
                            ])
                        ]),
                    ],
                    
                    ..BaseMonster::default()
                }
            },
            GREMLIN_WIZARD => {
                Self {
                    hp_range: (23, 25),
                    hp_range_asc: (22, 26),
                    moveset: vec![
                        MonsterMove {
                            name: CHARGING,
                            effects: vec![],
                            intent: Intent::Unknown
                        },
                        MonsterMove {
                            name: ULTIMATE_BLAST,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(25, 30, 30), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(CHARGING),
                            Move::InOrder(CHARGING),
                            Move::If(Condition::Asc(17), vec![
                                Move::Loop(vec![
                                    Move::InOrder(ULTIMATE_BLAST),
                                ])
                            ], vec![
                                Move::InOrder(ULTIMATE_BLAST)
                            ])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            HEXAGHOST => {
                Self {
                    hp_range: (250, 250),
                    hp_range_asc: (264, 264),
                    n_range: (Custom, Custom),
                    moveset: vec![
                        MonsterMove {
                            name: ACTIVATE,
                            effects: vec![],
                            intent: Intent::Unknown
                        },
                        MonsterMove {
                            name: DIVIDER,
                            effects: vec![
                                Effect::AttackDamage(N, Target::TargetEnemy),
                                Effect::AttackDamage(N, Target::TargetEnemy),
                                Effect::AttackDamage(N, Target::TargetEnemy),
                                Effect::AttackDamage(N, Target::TargetEnemy),
                                Effect::AttackDamage(N, Target::TargetEnemy),
                                Effect::AttackDamage(N, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: INFERNO,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(2, 3, 3), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(2, 3, 3), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(2, 3, 3), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(2, 3, 3), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(2, 3, 3), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(2, 3, 3), Target::TargetEnemy),
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::BURN), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: Fixed(3),
                                    modifier: CardModifier::Upgraded,
                                },
                                Effect::Custom,
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: SEAR,
                            effects: vec![
                                Effect::AttackDamage(Fixed(6), Target::TargetEnemy),
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::BURN), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: ByAsc(1, 1, 2),
                                    modifier: CardModifier::None,
                                },
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(5, 6, 6), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(5, 6, 6), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: INFLAME,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(2, 2, 3), Target::_Self),
                                Effect::Block(Fixed(12), Target::_Self),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(ACTIVATE),
                        Move::InOrder(DIVIDER),
                        Move::Loop(vec![
                            Move::InOrder(SEAR),
                            Move::InOrder(TACKLE),
                            Move::InOrder(SEAR),
                            Move::InOrder(INFLAME),
                            Move::InOrder(TACKLE),
                            Move::InOrder(SEAR),
                            Move::InOrder(INFERNO),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            JAW_WORM => {
                Self {
                    hp_range: (40, 44),
                    hp_range_asc: (42, 46),
                    effects: vec![(Event::CombatStart, Effect::If(Condition::Act(3), vec![
                        Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 4, 5), Target::_Self),
                        Effect::Block(ByAsc(6, 6, 9), Target::_Self),
                    ], vec![]))],
                    moveset: vec![
                        MonsterMove {
                            name: CHOMP,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(11, 12, 12), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: THRASH,
                            effects: vec![
                                Effect::AttackDamage(Fixed(7), Target::TargetEnemy),
                                Effect::Block(Fixed(5), Target::_Self),
                            ],
                            intent: Intent::AttackDefend,
                        },
                        MonsterMove {
                            name: BELLOW,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 4, 5), Target::_Self),
                                Effect::Block(ByAsc(6, 6, 9), Target::_Self),
                            ],
                            intent: Intent::DefendBuff,
                        },
                    ],
                    move_order: vec![
                        Move::If(Condition::Act(3), vec![], vec![
                            Move::InOrder(CHOMP),
                        ]),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (45, BELLOW, 1),
                                (30, THRASH, 2),
                                (25, CHOMP, 1),
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            LAGAVULIN => {
                Self {
                    hp_range: (109, 111),
                    hp_range_asc: (112, 115),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::METALLICIZE, Fixed(8), Target::_Self)),
                        (Event::CombatStart, Effect::AddBuff(buffs::ASLEEP, Fixed(1), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: ATTACK,
                            effects: vec![
                                Effect::Debuff(buffs::METALLICIZE, Target::_Self),
                                Effect::Debuff(buffs::ASLEEP, Target::_Self),
                                Effect::AttackDamage(ByAsc(18, 20, 20), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: SIPHON_SOUL,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(-1, -1, -2), Target::TargetEnemy),
                                Effect::AddBuff(buffs::DEXTERITY, ByAsc(-1, -1, -2), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: SLEEP,
                            effects: vec![],
                            intent: Intent::Sleep,
                        },
                        MonsterMove {
                            name: STUNNED,
                            effects: vec![
                                Effect::Debuff(buffs::METALLICIZE, Target::_Self),
                                Effect::Debuff(buffs::ASLEEP, Target::_Self),
                            ],
                            intent: Intent::Stun,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(SLEEP), 
                        Move::InOrder(SLEEP),
                        Move::InOrder(SLEEP),
                        Move::Event(Event::UnblockedDamage(Target::_Self)),
                        Move::If(Condition::Buff(Target::_Self, buffs::ASLEEP), vec![], vec![
                            Move::InOrder(STUNNED)
                        ]),
                        Move::Loop(vec![
                            Move::InOrder(ATTACK),
                            Move::InOrder(ATTACK),
                            Move::InOrder(SIPHON_SOUL),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            LOOTER => {
                Self {
                    hp_range: (44, 48),
                    hp_range_asc: (46, 50),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::INNATE_THIEVERY, ByAsc(15, 15, 20), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: MUG,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: LUNGE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(12, 14, 14), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: SMOKE_BOMB,
                            effects: vec![
                                Effect::Block(Fixed(6), Target::_Self),
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: ESCAPE,
                            effects: vec![
                                Effect::Custom,
                            ],
                            intent: Intent::Escape,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(MUG), 
                        Move::InOrder(MUG),
                        Move::Probability(vec![
                            (50, LUNGE, 1),
                            (50, SMOKE_BOMB, 1)
                        ]),
                        Move::Probability(vec![
                            (0, ESCAPE, 1),
                            (100, SMOKE_BOMB, 1),
                        ]),
                        Move::InOrder(ESCAPE),
                    ],
                    ..BaseMonster::default()
                }
            },
            MAD_GREMLIN => {
                Self {
                    hp_range: (20, 24),
                    hp_range_asc: (21, 25),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::ANGRY, ByAsc(1, 1, 2), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: SCRATCH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(4, 5, 5), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(SCRATCH)
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            MUGGER => {
                Self {
                    hp_range: (48, 52),
                    hp_range_asc: (50, 54),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::INNATE_THIEVERY, ByAsc(15, 15, 20), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: MUG,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: LUNGE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(16, 18, 18), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: SMOKE_BOMB,
                            effects: vec![
                                Effect::Block(ByAsc(11, 11, 17), Target::_Self),
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: ESCAPE,
                            effects: vec![
                                Effect::Custom,
                            ],
                            intent: Intent::Escape,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(MUG), 
                        Move::InOrder(MUG),
                        Move::Probability(vec![
                            (50, LUNGE, 1),
                            (50, SMOKE_BOMB, 1)
                        ]),
                        Move::Probability(vec![
                            (100, SMOKE_BOMB, 1),
                        ]),
                        Move::InOrder(ESCAPE),
                    ],
                    ..BaseMonster::default()
                }
            },
            MYSTIC => {
                Self {
                    hp_range: (48, 56),
                    hp_range_asc: (50, 58),
                    moveset: vec![
                        MonsterMove {
                            name: HEAL,
                            effects: vec![
                                Effect::Heal(ByAsc(16, 16, 20), Target::AnyFriendly),
                                Effect::Heal(ByAsc(16, 16, 20), Target::_Self),
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: BUFF,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(2, 3, 4), Target::AnyFriendly),
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: ATTACK_DEBUFF,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(8, 9, 9), Target::TargetEnemy),
                                Effect::AddBuff(buffs::FRAIL, Fixed(2), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::MissingHp(ByAsc(16, 16, 21), Target::AnyFriendly), vec![
                                Move::If(Condition::Asc(17),vec![
                                    Move::Probability(vec![
                                        (100, HEAL, 2)
                                    ])
                                ], vec![
                                    Move::Probability(vec![
                                        (100, HEAL, 3)
                                    ])
                                ])
                            ], vec![
                                Move::Probability(vec![
                                    (40, BUFF, 2),
                                    (60, ATTACK_DEBUFF, 2),
                                ]),
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            NEMESIS => {
                Self {
                    hp_range: (185, 185),
                    hp_range_asc: (200, 200),
                    effects: vec![
                        (Event::Custom, Effect::AddBuff(buffs::INTANGIBLE, Fixed(1), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: DEBUFF,
                            effects: vec![
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::BURN), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: ByAsc(3, 3, 5),
                                    modifier: CardModifier::None,
                                },
                            ],
                            intent: Intent::Debuff
                        },
                        MonsterMove {
                            name: ATTACK,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(6, 7, 7), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(6, 7, 7), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(6, 7, 7), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: SCYTHE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(45), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::Probability(vec![
                            (50, DEBUFF, 1),
                            (50, ATTACK, 1)
                        ]),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (35, DEBUFF, 1),
                                (35, ATTACK, 2),
                                (30, SCYTHE, 1),
                            ])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            ORB_WALKER => {
                Self {
                    hp_range: (90, 96),
                    hp_range_asc: (92, 102),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::STRENGTH_UP, ByAsc(3, 3, 5), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: LASER,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::BURN), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None,
                                },
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::BURN), 
                                    destination: CardLocation::DrawPile(RelativePosition::Random), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None,
                                },
                            ],
                            intent: Intent::AttackDebuff
                        },
                        MonsterMove {
                            name: CLAW,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(15, 16, 16), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (60, LASER, 2),
                                (40, CLAW, 2),
                            ])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            POINTY => {
                Self {
                    hp_range: (30, 30),
                    hp_range_asc: (34, 34),
                    moveset: vec![
                        MonsterMove {
                            name: ATTACK,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(5, 6, 6), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(5, 6, 6), Target::TargetEnemy),
                            ],
                            intent: Intent::Buff
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(ATTACK),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            RED_LOUSE => {
                Self {
                    hp_range: (10, 15),
                    hp_range_asc: (11, 16),
                    n_range: (ByAsc(3, 4, 9), ByAsc(7, 8, 12)),
                    x_range: (ByAsc(5, 6, 6), ByAsc(7, 8, 8)),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::CURL_UP, N, Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: BITE,
                            effects: vec![
                                Effect::AttackDamage(X, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: GROW,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 3, 4), Target::_Self),
                            ],
                            intent: Intent::Debuff
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::Asc(17), vec![
                                Move::Probability(vec![
                                    (25, GROW, 1),
                                    (75, BITE, 2),
                                ]),
                            ], vec![
                                Move::Probability(vec![
                                    (25, GROW, 2),
                                    (75, BITE, 2),
                                ]),
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            RED_SLAVER => {
                Self {
                    hp_range: (46, 50),
                    hp_range_asc: (48, 52),
                    moveset: vec![
                        MonsterMove {
                            name: STAB,
                            effects: vec![Effect::AttackDamage(ByAsc(13, 14, 14), Target::TargetEnemy)],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SCRAPE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(8, 9, 9), Target::TargetEnemy),
                                Effect::AddBuff(buffs::VULNERABLE, ByAsc(1, 1, 2), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: ENTANGLE,
                            effects: vec![
                                Effect::AddBuff(buffs::ENTANGLED, Fixed(1), Target::TargetEnemy),
                            ],
                            intent: Intent::Debuff,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(STAB),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (25, ENTANGLE, 1),
                                (75, SCRAPE, 2),
                            ]),
                            Move::If(Condition::Asc(17), vec![], vec![
                                Move::Probability(vec![
                                    (25, ENTANGLE, 1),
                                    (75, SCRAPE, 2),
                                ]),
                            ]),
                            Move::Probability(vec![
                                (25, ENTANGLE, 1),
                                (75, STAB, 2),
                            ]),
                        ]),
                        Move::Event(Event::Buff(buffs::ENTANGLED, Target::AllEnemies)),
                        Move::Loop(vec![
                            Move::If(Condition::Asc(17), 
                            vec![
                                Move::Probability(vec![
                                    (55, SCRAPE, 1),
                                    (45, STAB, 2),
                            ])],
                            vec![
                                Move::Probability(vec![
                                    (55, SCRAPE, 2),
                                    (45, STAB, 2),
                            ])],
                        )]),
                        
                    ],
                    ..BaseMonster::default()
                }
            },
            REPTOMANCER => {
                Self {
                    hp_range: (180, 190),
                    hp_range_asc: (190, 200),
                    moveset: vec![
                        MonsterMove {
                            name: SUMMON,
                            effects: vec![
                                Effect::Spawn {
                                    choices: vec![
                                        DAGGER,
                                    ],
                                    count: ByAsc(1, 1, 2)
                                }
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: SNAKE_STRIKE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(13, 16, 16), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(13, 16, 16), Target::TargetEnemy),
                                Effect::AddBuff(buffs::WEAK, Fixed(1), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: BIG_BITE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(30, 34, 34), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(SUMMON),
                        Move::Loop(vec![
                            Move::If(Condition::HasFriendlies(3), vec![
                                Move::Probability(vec![
                                    (67, SNAKE_STRIKE, 1),
                                    (33, BIG_BITE, 1),
                                ]),
                            ], vec![Move::If(Condition::HasFriendlies(4), vec![
                                Move::Probability(vec![
                                    (67, SNAKE_STRIKE, 1),
                                    (33, BIG_BITE, 1),
                                ]),
                            ], vec![
                                Move::Probability(vec![
                                    (33, SNAKE_STRIKE, 1),
                                    (33, BIG_BITE, 1),
                                    (33, SUMMON, 2),
                                ]),
                            ])])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            REPULSOR => {
                Self {
                    hp_range: (29, 35),
                    hp_range_asc: (31, 38),
                    moveset: vec![
                        MonsterMove {
                            name: BASH,
                            effects: vec![
                                Effect::AttackDamage(Fixed(11), Target::TargetEnemy),
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: REPULSE,
                            effects: vec![
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::DAZED), 
                                    destination: CardLocation::DeckPile(RelativePosition::Random), 
                                    copies: Fixed(2),
                                    modifier: CardModifier::None,

                                }
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (80, REPULSE, 0),
                                (20, BASH, 1),
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            ROMEO => {
                Self {
                    hp_range: (35, 39),
                    hp_range_asc: (37, 41),
                    moveset: vec![
                        MonsterMove {
                            name: MOCK,
                            effects: vec![],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: AGONIZING_SLASH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 12, 13), Target::TargetEnemy),
                                Effect::AddBuff(buffs::WEAK, ByAsc(2, 2, 3), Target::TargetEnemy)
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: CROSS_SLASH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(15, 17, 17), Target::TargetEnemy),
                            ],  
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(MOCK),
                        Move::Loop(vec![
                            Move::InOrder(AGONIZING_SLASH),
                            Move::InOrder(CROSS_SLASH),
                            Move::If(Condition::Asc(17), vec![
                                Move::InOrder(CROSS_SLASH),
                            ], vec![])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            SENTRY => {
                Self {
                    hp_range: (38, 42),
                    hp_range_asc: (39, 45),
                    moveset: vec![
                        MonsterMove {
                            name: BEAM,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(9, 10, 10), Target::TargetEnemy),
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: BOLT,
                            effects: vec![
                                Effect::AddCard {            
                                    card: CardReference::ByName(cards::DAZED), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: ByAsc(2, 2, 3),
                                    modifier: CardModifier::None,
                                }
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::If(Condition::InPosition(Target::_Self, 1), vec![
                            Move::InOrder(BEAM)
                        ], vec![
                            Move::InOrder(BOLT)
                        ]),
                        Move::Probability(vec![
                            (50, BEAM, 1),
                            (50, BOLT, 1)
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            SHELLED_PARASITE => {
                Self {
                    hp_range: (68, 72),
                    hp_range_asc: (70, 75),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::PLATED_ARMOR, Fixed(14), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: DOUBLE_STRIKE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(6, 7, 7), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(6, 7, 7), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SUCK,
                            effects: vec![
                                Effect::Custom,
                            ],
                            intent: Intent::AttackBuff,
                        },
                        MonsterMove {
                            name: FELL,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(18, 21, 21), Target::TargetEnemy),
                                Effect::AddBuff(buffs::FRAIL, Fixed(2), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: STUNNED,
                            effects: vec![],
                            intent: Intent::Stun,
                        },
                    ],
                    move_order: vec![
                        Move::If(Condition::Asc(17), vec![
                            Move::InOrder(FELL),
                        ], vec![
                            Move::Probability(vec![
                                (50, SUCK, 1),
                                (50, DOUBLE_STRIKE, 1),
                            ]),
                        ]),
                        Move::Loop(vec![
                            Move::Loop(vec![
                                Move::Probability(vec![
                                    (40, DOUBLE_STRIKE, 2),
                                    (40, SUCK, 2),
                                    (20, FELL, 1),
                                ])
                            ]),
                            Move::Event(Event::UnBuff(buffs::PLATED_ARMOR, Target::_Self)),
                            Move::InOrder(STUNNED),
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            SHIELD_GREMLIN => {
                Self {
                    hp_range: (12, 15),
                    hp_range_asc: (13, 17),
                    moveset: vec![
                        MonsterMove {
                            name: PROTECT,
                            effects: vec![
                                Effect::Block(ByAsc(7, 8, 11), Target::RandomFriendly),
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: SHIELD_BASH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(6, 8, 8), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::HasFriendlies(0), vec![
                                Move::InOrder(SHIELD_BASH),
                            ], vec![
                                Move::InOrder(PROTECT)
                            ]),
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            SLIME_BOSS => {
                Self {
                    hp_range: (12, 15),
                    hp_range_asc: (13, 17),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::SPLIT, Fixed(1), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: GOOP_SPRAY,
                            effects: vec![
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::SLIMED), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: ByAsc(3, 3, 5),
                                    modifier: CardModifier::None,
                                }
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: PREPARING,
                            effects: vec![],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: SLAM,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(35, 38, 38), Target::TargetEnemy)
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: SPLIT,
                            effects: vec![
                                Effect::Split(ACID_SLIME_L, SPIKE_SLIME_L)
                            ],
                            intent: Intent::Unknown,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(GOOP_SPRAY),
                            Move::InOrder(PREPARING),
                            Move::InOrder(SLAM),
                        ]),
                        Move::Event(Event::HalfHp(Target::_Self)),
                        Move::InOrder(SPLIT)
                    ],
                    ..BaseMonster::default()
                }
            },
            SNAKE_PLANT => {
                Self {
                    hp_range: (75, 79),
                    hp_range_asc: (78, 82),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::MALLEABLE, Fixed(3), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: CHOMP,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: ENFEEBLING_SPORES,
                            effects: vec![
                                Effect::AddBuff(buffs::FRAIL, Fixed(2), Target::TargetEnemy),
                                Effect::AddBuff(buffs::WEAK, Fixed(2), Target::TargetEnemy),
                            ],
                            intent: Intent::Unknown,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (65, CHOMP, 2),
                                (35, ENFEEBLING_SPORES, 1)
                            ])
                        ]),
                        Move::If(Condition::Asc(17), vec![
                            Move::AfterMove(vec![
                                (ENFEEBLING_SPORES, Move::Loop(vec![
                                    Move::InOrder(CHOMP),
                                    Move::InOrder(CHOMP),
                                    Move::InOrder(ENFEEBLING_SPORES),
                                ]))
                            ])
                        ], vec![])                        
                    ],
                    ..BaseMonster::default()
                }
            },
            SNECKO => {
                Self {
                    hp_range: (114, 120),
                    hp_range_asc: (120, 125),
                    moveset: vec![
                        MonsterMove {
                            name: PERPLEXING_GLARE,
                            effects: vec![
                                Effect::AddBuff(buffs::CONFUSED, Fixed(1), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: TAIL_WHIP,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(8, 10, 10), Target::TargetEnemy),
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), Target::TargetEnemy),
                                Effect::If(Condition::Asc(17), vec![
                                    Effect::AddBuff(buffs::WEAK, Fixed(2), Target::TargetEnemy)
                                ], vec![])
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: BITE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(15, 18, 18), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(PERPLEXING_GLARE),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (60, BITE, 2),
                                (40, TAIL_WHIP, 0)
                            ])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            SNEAKY_GREMLIN => {
                Self {
                    hp_range: (10, 14),
                    hp_range_asc: (11, 15),
                    moveset: vec![
                        MonsterMove {
                            name: PUNCTURE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(9, 10, 10), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(PUNCTURE),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            SPHERIC_GUARDIAN => {
                Self {
                    hp_range: (20, 20),
                    hp_range_asc: (20, 20),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::BARRICADE, Fixed(1), Target::_Self)),
                        (Event::CombatStart, Effect::Block(Fixed(40), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: SLAM,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: ACTIVATE,
                            effects: vec![
                                Effect::Block(ByAsc(25, 25, 35), Target::_Self),
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: HARDEN,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                                Effect::Block(ByAsc(15, 15, 15), Target::_Self),
                            ],
                            intent: Intent::AttackDefend,
                        },
                        MonsterMove {
                            name: ATTACK_DEBUFF,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                                Effect::AddBuff(buffs::FRAIL, Fixed(5), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(ACTIVATE),
                        Move::InOrder(ATTACK_DEBUFF),
                        Move::Loop(vec![
                            Move::InOrder(SLAM),
                            Move::InOrder(HARDEN),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            SPIRE_GROWTH => {
                Self {
                    hp_range: (170, 170),
                    hp_range_asc: (190, 190),
                    moveset: vec![
                        MonsterMove {
                            name: QUICK_TACKLE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SMASH,
                            effects: vec![
                                Effect::Block(ByAsc(25, 25, 35), Target::_Self),
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: CONSTRICT,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(10, 11, 11), Target::TargetEnemy),
                                Effect::Block(ByAsc(15, 15, 15), Target::_Self),
                            ],
                            intent: Intent::AttackDefend,
                        },
                    ],
                    move_order: vec![
                        Move::If(Condition::Asc(17), vec![
                            Move::InOrder(CONSTRICT),
                            Move::Loop(vec![
                                Move::Probability(vec![
                                    (50, QUICK_TACKLE, 2),
                                    (50, SMASH, 2),
                                ]),
                                Move::If(Condition::Buff(Target::AllEnemies, buffs::CONSTRICTED), vec![], vec![
                                    Move::InOrder(CONSTRICT)
                                ])
                            ])
                        ], vec![
                            Move::Loop(vec![
                                Move::AfterMove(vec![
                                    (CONSTRICT, Move::Probability(vec![
                                        (50, QUICK_TACKLE, 1),
                                        (50, SMASH, 1)
                                    ])),
                                ]),
                                Move::If(Condition::Buff(Target::AllEnemies, buffs::CONSTRICTED), vec![
                                    Move::Probability(vec![
                                        (50, QUICK_TACKLE, 1),
                                        (50, SMASH, 1)
                                    ])
                                ], vec![
                                    Move::Probability(vec![
                                        (50, CONSTRICT, 1),
                                        (50, SMASH, 1)
                                    ])
                                ])
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            SPIKE_SLIME_L => {
                Self {
                    hp_range: (64, 70),
                    hp_range_asc: (67, 73),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::SPLIT, Fixed(1), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: FLAME_TACKLE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(16, 18, 18), Target::TargetEnemy),
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::SLIMED), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: Amount::Fixed(2),
                                    modifier: CardModifier::None
                                }
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: LICK,
                            effects: vec![
                                Effect::AddBuff(buffs::FRAIL, ByAsc(2, 2, 3), Target::TargetEnemy),
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: SPLIT,
                            effects: vec![
                                Effect::Split(SPIKE_SLIME_M, SPIKE_SLIME_M)
                            ],
                            intent: Intent::Unknown,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::Asc(17), vec![
                                Move::Probability(vec![
                                    (70, LICK, 1),
                                    (30, FLAME_TACKLE, 2),
                                ])
                            ], vec![
                                Move::Probability(vec![
                                    (70, LICK, 2),
                                    (30, FLAME_TACKLE, 2),
                                ])
                            ])
                        ]),
                        Move::Event(Event::HalfHp(Target::_Self)),
                        Move::InOrder(SPLIT)
                    ],
                    ..BaseMonster::default()
                }
            },
            SPIKE_SLIME_M => {
                Self {
                    hp_range: (28, 32),
                    hp_range_asc: (29, 34),
                    moveset: vec![
                        MonsterMove {
                            name: FLAME_TACKLE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(8, 10, 10), Target::TargetEnemy),
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::SLIMED), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: Amount::Fixed(1),
                                    modifier: CardModifier::None
                                }
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: LICK,
                            effects: vec![
                                Effect::AddBuff(buffs::FRAIL, Fixed(1), Target::TargetEnemy),
                            ],
                            intent: Intent::Debuff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::Asc(17), vec![
                                Move::Probability(vec![
                                    (70, LICK, 1),
                                    (30, FLAME_TACKLE, 2),
                                ])
                            ], vec![
                                Move::Probability(vec![
                                    (70, LICK, 2),
                                    (30, FLAME_TACKLE, 2),
                                ])
                            ])
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            SPIKE_SLIME_S => {
                Self {
                    hp_range: (10, 14),
                    hp_range_asc: (11, 15),
                    moveset: vec![
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(5, 6, 6), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(TACKLE)
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            SPIKER => {
                Self {
                    hp_range: (42, 56),
                    hp_range_asc: (44, 60),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::THORNS, ByAsc(3, 4, 7), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: CUT,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(7, 9, 9), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SPIKE,
                            effects: vec![
                                Effect::AddBuff(buffs::THORNS, Fixed(2), Target::_Self),
                            ],
                            intent: Intent::Buff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::If(Condition::BuffX(Target::_Self, buffs::THORNS, ByAsc(15, 16, 19)), vec![
                                Move::InOrder(CUT)
                            ], vec![
                                Move::Probability(vec![
                                    (50, CUT, 1),
                                    (50, SPIKE, 0),
                                ])
                            ])
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            SPIRE_SHIELD => {
                Self {
                    hp_range: (110, 110),
                    hp_range_asc: (125, 125),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::ARTIFACT, ByAsc(1, 1, 2), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: BASH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(12, 14, 14), Target::TargetEnemy),
                                Effect::If(Condition::HasOrbSlot, vec![
                                    Effect::AddBuff(buffs::FOCUS, Fixed(-1), Target::TargetEnemy)
                                ], vec![
                                    Effect::AddBuff(buffs::STRENGTH, Fixed(-1), Target::TargetEnemy)
                                ])
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: FORTIFY,
                            effects: vec![
                                Effect::Block(Fixed(30), Target::AnyFriendly),
                            ],
                            intent: Intent::Buff,
                        },
                        MonsterMove {
                            name: SMASH,
                            effects: vec![
                                Effect::AttackDamageIfUnblocked(ByAsc(34, 38, 38), Target::TargetEnemy, vec![
                                    Effect::If(Condition::Asc(18), vec![
                                        Effect::Block(Fixed(99), Target::_Self)
                                    ], vec![
                                        Effect::Block(N, Target::_Self)
                                    ])
                                ])
                            ],
                            intent: Intent::Buff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (50, BASH, 1),
                                (50, FORTIFY, 1),
                            ]),
                            Move::Probability(vec![
                                (50, BASH, 1),
                                (50, FORTIFY, 1),
                            ]),
                            Move::InOrder(SMASH)                            
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            SPIRE_SPEAR => {
                Self {
                    hp_range: (160, 160),
                    hp_range_asc: (180, 180),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::ARTIFACT, ByAsc(1, 1, 2), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: BURN_STRIKE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(5, 6, 6), Target::TargetEnemy),
                                Effect::If(Condition::Asc(18), vec![
                                    Effect::AddCard{
                                        card: CardReference::ByName(cards::BURN), 
                                        destination: CardLocation::DrawPile(RelativePosition::Top), 
                                        copies: Fixed(2),
                                        modifier: CardModifier::None
                                    }
                                ], vec![
                                    Effect::AddCard{
                                        card: CardReference::ByName(cards::BURN), 
                                        destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                        copies: Fixed(2),
                                        modifier: CardModifier::None
                                    }
                                ])
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: SKEWER,
                            effects: vec![
                                Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                                Effect::If(Condition::Asc(3), vec![
                                    Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                                ], vec![])
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: PIERCER,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, Fixed(2), Target::AnyFriendly),
                            ],
                            intent: Intent::Buff,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(BURN_STRIKE),
                        Move::Loop(vec![
                            Move::InOrder(SKEWER),
                            Move::Probability(vec![
                                (50, BURN_STRIKE, 1),
                                (50, PIERCER, 1),
                            ]),
                            Move::Probability(vec![
                                (50, BURN_STRIKE, 1),
                                (50, PIERCER, 1),
                            ]),                            
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            TASKMASTER => {
                Self {
                    hp_range: (54, 60),
                    hp_range_asc: (57, 64),
                    moveset: vec![
                        MonsterMove {
                            name: SCOURING_WHIP,
                            effects: vec![
                                Effect::AttackDamage(Fixed(7), Target::TargetEnemy),
                                Effect::If(Condition::Asc(18), vec![ 
                                    Effect::AddBuff(buffs::STRENGTH, Fixed(1), Target::_Self)
                                ], vec![]),
                                Effect::AddCard{
                                    card: CardReference::ByName(cards::WOUND), 
                                    destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                    copies: ByAsc(1, 2, 3),
                                    modifier: CardModifier::None
                                }
                            ],
                            intent: Intent::AttackDebuff,
                        }
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(TASKMASTER),                  
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            THE_CHAMP => {
                Self {
                    hp_range: (420, 420),
                    hp_range_asc: (440, 440),
                    moveset: vec![
                        MonsterMove {
                            name: DEFENSIVE_STANCE,
                            effects: vec![
                                Effect::If(Condition::Asc(9), vec![
                                    Effect::Block(ByAsc(15, 18, 20), Target::_Self),
                                    Effect::AddBuff(buffs::METALLICIZE, ByAsc(5, 6, 7), Target::_Self),   
                                ], vec![
                                    Effect::Block(ByAsc(15, 15, 20), Target::_Self),
                                    Effect::AddBuff(buffs::METALLICIZE, ByAsc(5, 5, 7), Target::_Self),   
                                ])                         
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: FACE_SLAP,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(12, 14, 14), Target::TargetEnemy),
                                Effect::AddBuff(buffs::FRAIL, Fixed(2), Target::TargetEnemy), 
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), Target::TargetEnemy),  
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: TAUNT,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(2), Target::TargetEnemy), 
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), Target::TargetEnemy),
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: HEAVY_SLASH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(16, 18, 18), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: GLOAT,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(2, 3, 4), Target::_Self),
                            ],
                            intent: Intent::Buff,
                        },
                        MonsterMove {
                            name: EXECUTE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(10), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: ANGER,
                            effects: vec![
                                Effect::RemoveDebuffs(Target::_Self),
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(6, 9, 12), Target::_Self),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::If(Condition::Asc(19), vec![
                            Move::Probability(vec![
                                (30, DEFENSIVE_STANCE, 1),
                                (25, FACE_SLAP, 1),
                                (45, HEAVY_SLASH, 1),
                            ]),
                        ], vec![
                            Move::Probability(vec![
                                (15, DEFENSIVE_STANCE, 1),
                                (15, GLOAT, 1),
                                (25, FACE_SLAP, 1),
                                (45, HEAVY_SLASH, 1),
                            ]),
                        ]),
                        Move::Loop(vec![
                            Move::If(Condition::HalfHp(Target::_Self), vec![
                                Move::InOrder(ANGER),
                                Move::Loop(vec![
                                    Move::InOrder(EXECUTE),
                                    Move::If(Condition::BuffX(Target::_Self, buffs::METALLICIZE, Fixed(10)), vec![
                                        Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ]),
                                        Move::AfterMove(vec![
                                            (GLOAT, Move::Probability(vec![
                                                (55, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (FACE_SLAP, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (70, HEAVY_SLASH, 1),
                                            ])),
                                            (HEAVY_SLASH, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (70, FACE_SLAP, 1),
                                            ])),
                                        ]),
                                    ], vec![
                                        Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (15, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ]),
                                        Move::AfterMove(vec![
                                            (DEFENSIVE_STANCE, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (25, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (GLOAT, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (40, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (FACE_SLAP, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (15, GLOAT, 1),
                                                (70, HEAVY_SLASH, 1),
                                            ])),
                                            (HEAVY_SLASH, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (15, GLOAT, 1),
                                                (70, FACE_SLAP, 1),
                                            ])),
                                        ]),
                                    ]),
                                ]),
                            ], vec![
                                Move::If(Condition::BuffX(Target::_Self, buffs::METALLICIZE, Fixed(10)), vec![
                                    Move::AfterMove(vec![
                                        (DEFENSIVE_STANCE, Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ])),
                                        (GLOAT, Move::Probability(vec![
                                            (55, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ])),
                                        (FACE_SLAP, Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (70, HEAVY_SLASH, 1),
                                        ])),
                                        (HEAVY_SLASH, Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (70, FACE_SLAP, 1),
                                        ])),
                                    ]),
                                ], vec![
                                    Move::AfterMove(vec![
                                        (DEFENSIVE_STANCE, Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ])),
                                        (GLOAT, Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (40, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ])),
                                        (FACE_SLAP, Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (15, GLOAT, 1),
                                            (70, HEAVY_SLASH, 1),
                                        ])),
                                        (HEAVY_SLASH, Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (15, GLOAT, 1),
                                            (70, FACE_SLAP, 1),
                                        ])),
                                    ]),
                                ]),
                            ]),
                            Move::If(Condition::HalfHp(Target::_Self), vec![
                                Move::InOrder(ANGER),
                                Move::Loop(vec![
                                    Move::InOrder(EXECUTE),
                                    Move::If(Condition::BuffX(Target::_Self, buffs::METALLICIZE, Fixed(10)), vec![
                                        Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ]),
                                        Move::AfterMove(vec![
                                            (GLOAT, Move::Probability(vec![
                                                (55, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (FACE_SLAP, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (70, HEAVY_SLASH, 1),
                                            ])),
                                            (HEAVY_SLASH, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (70, FACE_SLAP, 1),
                                            ])),
                                        ]),
                                    ], vec![
                                        Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (15, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ]),
                                        Move::AfterMove(vec![
                                            (DEFENSIVE_STANCE, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (25, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (GLOAT, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (40, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (FACE_SLAP, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (15, GLOAT, 1),
                                                (70, HEAVY_SLASH, 1),
                                            ])),
                                            (HEAVY_SLASH, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (15, GLOAT, 1),
                                                (70, FACE_SLAP, 1),
                                            ])),
                                        ]),
                                    ]),
                                ]),
                            ], vec![
                                Move::If(Condition::BuffX(Target::_Self, buffs::METALLICIZE, Fixed(10)), vec![
                                    Move::AfterMove(vec![
                                        (DEFENSIVE_STANCE, Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ])),
                                        (GLOAT, Move::Probability(vec![
                                            (55, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ])),
                                        (FACE_SLAP, Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (70, HEAVY_SLASH, 1),
                                        ])),
                                        (HEAVY_SLASH, Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (70, FACE_SLAP, 1),
                                        ])),
                                    ]),
                                ], vec![
                                    Move::AfterMove(vec![
                                        (DEFENSIVE_STANCE, Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ])),
                                        (GLOAT, Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (40, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ])),
                                        (FACE_SLAP, Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (15, GLOAT, 1),
                                            (70, HEAVY_SLASH, 1),
                                        ])),
                                        (HEAVY_SLASH, Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (15, GLOAT, 1),
                                            (70, FACE_SLAP, 1),
                                        ])),
                                    ]),
                                ]),
                            ]),
                            Move::If(Condition::HalfHp(Target::_Self), vec![
                                Move::InOrder(ANGER),
                                Move::Loop(vec![
                                    Move::InOrder(EXECUTE),
                                    Move::If(Condition::BuffX(Target::_Self, buffs::METALLICIZE, Fixed(10)), vec![
                                        Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ]),
                                        Move::AfterMove(vec![
                                            (GLOAT, Move::Probability(vec![
                                                (55, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (FACE_SLAP, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (70, HEAVY_SLASH, 1),
                                            ])),
                                            (HEAVY_SLASH, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (70, FACE_SLAP, 1),
                                            ])),
                                        ]),
                                    ], vec![
                                        Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (15, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ]),
                                        Move::AfterMove(vec![
                                            (DEFENSIVE_STANCE, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (25, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (GLOAT, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (40, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (FACE_SLAP, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (15, GLOAT, 1),
                                                (70, HEAVY_SLASH, 1),
                                            ])),
                                            (HEAVY_SLASH, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (15, GLOAT, 1),
                                                (70, FACE_SLAP, 1),
                                            ])),
                                        ]),
                                    ]),
                                ]),
                            ], vec![
                                Move::InOrder(TAUNT),
                            ]),
                            Move::If(Condition::HalfHp(Target::_Self), vec![
                                Move::InOrder(ANGER),
                                Move::Loop(vec![
                                    Move::InOrder(EXECUTE),
                                    Move::If(Condition::BuffX(Target::_Self, buffs::METALLICIZE, Fixed(10)), vec![
                                        Move::Probability(vec![
                                            (30, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ]),
                                        Move::AfterMove(vec![
                                            (GLOAT, Move::Probability(vec![
                                                (55, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (FACE_SLAP, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (70, HEAVY_SLASH, 1),
                                            ])),
                                            (HEAVY_SLASH, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (70, FACE_SLAP, 1),
                                            ])),
                                        ]),
                                    ], vec![
                                        Move::Probability(vec![
                                            (15, DEFENSIVE_STANCE, 1),
                                            (15, GLOAT, 1),
                                            (25, FACE_SLAP, 1),
                                            (45, HEAVY_SLASH, 1),
                                        ]),
                                        Move::AfterMove(vec![
                                            (DEFENSIVE_STANCE, Move::Probability(vec![
                                                (30, GLOAT, 1),
                                                (25, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (GLOAT, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (40, FACE_SLAP, 1),
                                                (45, HEAVY_SLASH, 1),
                                            ])),
                                            (FACE_SLAP, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (15, GLOAT, 1),
                                                (70, HEAVY_SLASH, 1),
                                            ])),
                                            (HEAVY_SLASH, Move::Probability(vec![
                                                (15, DEFENSIVE_STANCE, 1),
                                                (15, GLOAT, 1),
                                                (70, FACE_SLAP, 1),
                                            ])),
                                        ]),
                                    ]),
                                ]),
                            ], vec![
                                Move::If(Condition::BuffX(Target::_Self, buffs::METALLICIZE, Fixed(10)), vec![
                                    Move::Probability(vec![
                                        (30, GLOAT, 1),
                                        (25, FACE_SLAP, 1),
                                        (45, HEAVY_SLASH, 1),
                                    ]),
                                ], vec![
                                    Move::Probability(vec![
                                        (15, GLOAT, 1),
                                        (15, DEFENSIVE_STANCE, 1),
                                        (25, FACE_SLAP, 1),
                                        (45, HEAVY_SLASH, 1),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            THE_COLLECTOR => {
                Self {
                    hp_range: (282, 282),
                    hp_range_asc: (300, 300),
                    moveset: vec![
                        MonsterMove {
                            name: BUFF,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 4, 5), Target::AnyFriendly),
                                Effect::Block(ByAsc(15, 18, 23), Target::_Self),
                            ],
                            intent: Intent::DefendBuff,
                        },
                        MonsterMove {
                            name: FIREBALL,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(18, 21, 21), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: MEGA_DEBUFF,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, ByAsc(3, 3, 5), Target::TargetEnemy),
                                Effect::AddBuff(buffs::VULNERABLE, ByAsc(3, 3, 5), Target::TargetEnemy),
                                Effect::AddBuff(buffs::FRAIL, ByAsc(3, 3, 5), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: SPAWN,
                            effects: vec![
                                Effect::If(Condition::HasFriendlies(1), vec![
                                    Effect::Spawn {
                                        choices: vec![TORCH_HEAD],
                                        count: Fixed(1),
                                    },
                                ], vec![
                                    Effect::Spawn {
                                        choices: vec![TORCH_HEAD],
                                        count: Fixed(2),
                                    },
                                ])
                            ],
                            intent: Intent::Unknown,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(SPAWN),
                            Move::Probability(vec![
                                (70, FIREBALL, 1),
                                (30, BUFF, 1)
                            ]),
                            Move::If(Condition::HasFriendlies(2), vec![
                                Move::Probability(vec![
                                    (70, FIREBALL, 0),
                                    (30, BUFF, 0)
                                ]), 
                            ], vec![
                                Move::Probability(vec![
                                    (25, SPAWN, 0),
                                    (45, FIREBALL, 2),
                                    (30, BUFF, 1)
                                ]),
                            ]),
                            Move::InOrder(MEGA_DEBUFF),
                            Move::Loop(vec![
                                Move::If(Condition::HasFriendlies(2), vec![
                                    Move::Probability(vec![
                                        (70, FIREBALL, 2),
                                        (30, BUFF, 1)
                                    ]), 
                                ], vec![
                                    Move::Probability(vec![
                                        (25, SPAWN, 0),
                                        (45, FIREBALL, 2),
                                        (30, BUFF, 1)
                                    ]),
                                ]),
                            ])
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            THE_GUARDIAN => {
                Self {
                    hp_range: (240, 240),
                    hp_range_asc: (250, 250),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::MODE_SHIFT, ByAsc(30, 35, 40), Target::_Self))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: CHARGING_UP,
                            effects: vec![
                                Effect::Block(Fixed(9), Target::_Self),
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: FIERCE_BASH,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(32, 36, 36), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: VENT_STEAM,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(2), Target::TargetEnemy),
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: WHIRLWIND,
                            effects: vec![
                                Effect::AttackDamage(Fixed(5), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(5), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(5), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(5), Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: DEFENSIVE_MODE,
                            effects: vec![
                                Effect::AddBuff(buffs::SHARP_HIDE, ByAsc(3, 3, 4), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: ROLL_ATTACK,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(9, 10, 10), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: TWIN_SLAM,
                            effects: vec![
                                Effect::AttackDamage(Fixed(8), Target::TargetEnemy),
                                Effect::AttackDamage(Fixed(8), Target::TargetEnemy),
                                Effect::Debuff(buffs::SHARP_HIDE, Target::_Self),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(CHARGING_UP),
                            Move::InOrder(FIERCE_BASH),
                            Move::InOrder(VENT_STEAM),
                            Move::InOrder(WHIRLWIND),
                        ]),
                        Move::Event(Event::UnBuff(buffs::MODE_SHIFT, Target::_Self)),
                        Move::InOrder(DEFENSIVE_MODE),
                        Move::InOrder(ROLL_ATTACK),
                        Move::InOrder(TWIN_SLAM),
                        Move::Loop(vec![
                            Move::InOrder(CHARGING_UP),
                            Move::InOrder(FIERCE_BASH),
                            Move::InOrder(VENT_STEAM),
                            Move::InOrder(WHIRLWIND),
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            THE_MAW => {
                Self {
                    hp_range: (300, 300),
                    hp_range_asc: (300, 300),
                    moveset: vec![
                        MonsterMove {
                            name: ROAR,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, ByAsc(3, 3, 5), Target::TargetEnemy),
                                Effect::AddBuff(buffs::FRAIL, ByAsc(3, 3, 5), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: DROOL,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 3, 5), Target::_Self),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SLAM,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(25, 30, 30), Target::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                        MonsterMove {
                            name: NOM,
                            effects: vec![
                                Effect::AttackDamage(Custom, Target::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::InOrder(ROAR),
                        Move::Loop(vec![
                            Move::AfterMove(vec![
                                (NOM, Move::InOrder(DROOL))
                            ]),
                            Move::Probability(vec![
                                (33, DROOL, 1),
                                (33, SLAM, 1),
                                (33, NOM, 1),
                            ]),
                        ])
                    ],
                    ..BaseMonster::default()
                }
            },
            TIME_EATER => {
                Self {
                    hp_range: (456, 456),
                    hp_range_asc: (480, 480),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::TIME_WARP, Fixed(12), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: REVERBERATE,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                                Effect::AttackDamage(ByAsc(7, 8, 8), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: HEAD_SLAM,
                            effects: vec![
                                Effect::AttackDamage(ByAsc(26, 32, 32), Target::TargetEnemy),
                                Effect::AddBuff(buffs::DRAW_REDUCTION, Fixed(2), Target::TargetEnemy),
                                Effect::If(Condition::Asc(19), vec![
                                    Effect::AddCard {
                                        card: CardReference::ByName(cards::SLIMED), 
                                        destination: CardLocation::DiscardPile(RelativePosition::Bottom), 
                                        copies: Fixed(2),
                                        modifier: CardModifier::None,
                                    }
                                ], vec![])    
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: RIPPLE,
                            effects: vec![
                                Effect::Block(Fixed(20), Target::_Self),
                                Effect::AddBuff(buffs::WEAK, Fixed(1), Target::TargetEnemy),
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(1), Target::TargetEnemy),
                                Effect::If(Condition::Asc(19), vec![
                                    Effect::AddBuff(buffs::FRAIL, Fixed(1), Target::TargetEnemy),
                                ], vec![])
                            ],
                            intent: Intent::Defend,
                        },
                        MonsterMove {
                            name: HASTE,
                            effects: vec![
                                Effect::Heal(Amount::Custom, Target::_Self),
                                Effect::If(Condition::Asc(19), vec![
                                    Effect::Block(Fixed(32), Target::_Self),
                                ], vec![]),
                                Effect::RemoveDebuffs(Target::_Self),
                            ],
                            intent: Intent::Buff,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (20, RIPPLE, 1),
                                (45, REVERBERATE, 2),
                                (35, HEAD_SLAM, 1),
                            ]),
                            Move::If(Condition::HalfHp(Target::_Self), vec![
                                Move::InOrder(HASTE),
                                Move::Loop(vec![
                                    Move::Probability(vec![
                                        (20, RIPPLE, 1),
                                        (45, REVERBERATE, 2),
                                        (35, HEAD_SLAM, 1),
                                    ]),                                    
                                ])
                            ], vec![])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            TORCH_HEAD => {
                Self {
                    hp_range: (38, 40),
                    hp_range_asc: (40, 45),
                    moveset: vec![
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![
                                Effect::AttackDamage(Fixed(7), Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(TACKLE)
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            TRANSIENT => {
                Self {
                    hp_range: (999, 999),
                    hp_range_asc: (999, 999),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::FADING, ByAsc(5, 5, 6), Target::_Self)),
                        (Event::CombatStart, Effect::AddBuff(buffs::SHIFTING, Fixed(1), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: ATTACK,
                            effects: vec![
                                Effect::AttackDamage(Custom, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::InOrder(ATTACK)
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },
            WRITHING_MASS => {
                Self {
                    hp_range: (160, 160),
                    hp_range_asc: (175, 175),
                    effects: vec![
                        (Event::CombatStart, Effect::AddBuff(buffs::MALLEABLE, Fixed(3), Target::_Self)),
                        (Event::CombatStart, Effect::AddBuff(buffs::REACTIVE, Fixed(1), Target::_Self)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: IMPLANT,
                            effects: vec![
                                Effect::AddCard {
                                    card: CardReference::ByName(cards::PARASITE), 
                                    destination: CardLocation::DeckPile(RelativePosition::Bottom), 
                                    copies: Fixed(1),
                                    modifier: CardModifier::None
                                }
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: FLAIL,
                            effects: vec![
                                Effect::AttackDamage(Custom, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: WITHER,
                            effects: vec![
                                Effect::AttackDamage(Custom, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: MULTISTRIKE,
                            effects: vec![
                                Effect::AttackDamage(Custom, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: STRONG_STRIKE,
                            effects: vec![
                                Effect::AttackDamage(Custom, Target::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (10, IMPLANT, 1),
                                (30, FLAIL, 1),
                                (20, WITHER, 1),
                                (30, MULTISTRIKE, 1),
                                (10, STRONG_STRIKE, 1),
                            ]),
                            Move::AfterMove(vec![
                                (IMPLANT, Move::Loop(vec![
                                    Move::Probability(vec![
                                        (30, FLAIL, 1),
                                        (20, WITHER, 1),
                                        (30, MULTISTRIKE, 1),
                                        (10, STRONG_STRIKE, 1),
                                    ])
                                ]))
                            ])
                        ]),
                    ],
                    ..BaseMonster::default()
                }
            },            
            _ => panic!("Unrecognized monster")
        }
    }
}

pub const ACID_SLIME_L: &str = "Acid Slime (L)";
pub const ACID_SLIME_M: &str = "Acid Slime (M)";
pub const ACID_SLIME_S: &str = "Acid Slime (S)";
pub const AWAKENED_ONE: &str = "Awakened One";
pub const BEAR: &str = "Bear";
pub const BOOK_OF_STABBING: &str = "Book of Stabbing";
pub const BLUE_SLAVER: &str = "Blue Slaver";
pub const BRONZE_AUTOMATON: &str = "Bronze Automaton";
pub const BRONZE_ORB: &str = "Bronze Orb";
pub const BYRD: &str = "Byrd";
pub const CENTURION: &str = "Centurion";
pub const CHOSEN: &str = "Chosen";
pub const CORRUPT_HEART: &str = "Corrupt Heart";
pub const CULTIST: &str = "Cultist";
pub const DARKLING: &str = "Darkling";
pub const DAGGER: &str = "Dagger";
pub const DECA: &str = "Deca";
pub const DONU: &str = "Donu";
pub const EXPLODER: &str = "Exploder";
pub const FAT_GREMLIN: &str = "Fat Gremlin";
pub const FUNGI_BEAST: &str = "Fungi Beast";
pub const GIANT_HEAD: &str = "Giant Head";
pub const GREEN_LOUSE: &str = "Green Louse";
pub const GREMLIN_LEADER: &str = "Gremlin Leader";
pub const GREMLIN_NOB: &str = "Gremlin Nob";
pub const GREMLIN_WIZARD: &str = "Gremlin Wizard";
pub const HEXAGHOST: &str = "Hexaghost";
pub const JAW_WORM: &str = "Jaw Worm";
pub const LAGAVULIN: &str = "Lagavulin";
pub const LOOTER: &str = "Looter";
pub const MAD_GREMLIN: &str = "Mad Gremlin";
pub const MUGGER: &str = "Mugger";
pub const MYSTIC: &str = "Mystic";
pub const NEMESIS: &str = "Nemesis";
pub const ORB_WALKER: &str = "Orb Walker";
pub const POINTY: &str = "Pointy";
pub const RED_LOUSE: &str = "Red Louse";
pub const RED_SLAVER: &str = "Red Slaver";
pub const REPTOMANCER: &str = "Reptomancer";
pub const REPULSOR: &str = "Repulsor";
pub const ROMEO: &str = "Romeo";
pub const SENTRY: &str = "Sentry";
pub const SHELLED_PARASITE: &str = "Shelled Parasite";
pub const SHIELD_GREMLIN: &str = "Shield Gremlin";
pub const SLIME_BOSS: &str = "Slime Boss";
pub const SNAKE_PLANT: &str = "Snake Plant";
pub const SNECKO: &str = "Snecko";
pub const SNEAKY_GREMLIN: &str = "Sneaky Gremlin";
pub const SPHERIC_GUARDIAN: &str = "Spheric Guardian";
pub const SPIRE_GROWTH: &str = "Spire Growth";
pub const SPIKE_SLIME_L: &str = "Spike Slime (L)";
pub const SPIKE_SLIME_M: &str = "Spike Slime (M)";
pub const SPIKE_SLIME_S: &str = "Spike Slime (S)";
pub const SPIKER: &str = "Spiker";
pub const SPIRE_SHIELD: &str = "Spire Shield";
pub const SPIRE_SPEAR: &str = "Spire Spear";
pub const TASKMASTER: &str = "Taskmaster";
pub const THE_CHAMP: &str = "The Champ";
pub const THE_COLLECTOR: &str = "The Collector";
pub const THE_GUARDIAN: &str = "The Guardian";
pub const THE_MAW: &str = "The Maw";
pub const TIME_EATER: &str = "Time Eater";
pub const TORCH_HEAD: &str = "Torch Head";
pub const TRANSIENT: &str = "Transient";
pub const WRITHING_MASS: &str = "Writhing Mass";



pub const SLASH: &str = "Slash";
pub const SOUL_STRIKE: &str = "Soul Strike";
pub const REBIRTH: &str = "Rebirth";
pub const DARK_ECHO: &str = "Dark Echo";
pub const SLUDGE: &str = "Sludge";
pub const TACKLE: &str = "Tackle";
pub const CORROSIVE_SPIT: &str = "Corrosive Spit";
pub const LICK: &str = "Lick";
pub const SPLIT: &str = "Split";
pub const BEAR_HUG: &str = "Bear Hug";
pub const LUNGE: &str = "Lunge";
pub const MAUL: &str = "Maul";
pub const MULTI_STAB: &str = "Multi-Stab";
pub const SINGLE_STAB: &str = "Single Stab";
pub const STAB: &str = "Stab";
pub const RAKE: &str = "Rake";
pub const FLAIL: &str = "Flail";
pub const BOOST: &str = "Boost";
pub const HYPERBEAM: &str = "HYPER BEAM";
pub const STUNNED: &str = "Stunned";
pub const SPAWN_ORBS: &str = "Spawn Orbs";
pub const STASIS: &str = "Stasis";
pub const BEAM: &str = "Beam";
pub const SUPPORT_BEAM: &str = "Support Beam";
pub const CAW: &str = "Caw";
pub const PECK: &str = "Peck";
pub const SWOOP: &str = "Swoop";
pub const FLY: &str = "Fly";
pub const HEADBUTT: &str = "Headbutt";
pub const FURY: &str = "Fury";
pub const DEFEND: &str = "Defend";
pub const POKE: &str = "Poke";
pub const ZAP: &str = "Zap";
pub const DEBILITATE: &str = "Debilitate";
pub const DRAIN: &str = "Drain";
pub const HEX: &str = "Hex";
pub const BLOOD_SHOTS: &str = "Blood Shots";
pub const ECHO: &str = "Echo";
pub const BUFF: &str = "Buff";
pub const INCANTATION: &str = "Incantation";
pub const DARK_STRIKE: &str = "Dark Strike";
pub const NIP: &str = "Nip";
pub const CHOMP: &str = "Chomp";
pub const HARDEN: &str = "Harden";
pub const REINCARNATE: &str = "Reincarnate";
pub const REGROW: &str = "Regrow";
pub const CIRCLE_OF_POWER: &str = "Circle of Power";
pub const SQUARE_OF_PROTECTION: &str = "Square of Protection";
pub const EXPLODE: &str = "Explode";
pub const SLAM: &str = "Slam";
pub const SMASH: &str = "Smash";
pub const BITE: &str = "Bite";
pub const GROW: &str = "Grow";
pub const COUNT: &str = "Count";
pub const GLARE: &str = "Glare";
pub const IT_IS_TIME: &str = "It Is Time";
pub const SPIT_WEB: &str = "Spit Web";
pub const ENCOURAGE: &str = "Encourage";
pub const RALLY: &str = "Rally!";
pub const BELLOW: &str = "Bellow";
pub const RUSH: &str = "Rush";
pub const SKULL_BASH: &str = "Skull Bash";
pub const CHARGING: &str = "Charging";
pub const ULTIMATE_BLAST: &str = "Ultimate Blast";
pub const ACTIVATE: &str = "Activate";
pub const DIVIDER: &str = "Divider";
pub const INFERNO: &str = "Inferno";
pub const SEAR: &str = "Sear";
pub const INFLAME: &str = "Inflame";
pub const THRASH: &str = "Thrash";
pub const ATTACK: &str = "Attack";
pub const SIPHON_SOUL: &str = "Siphon Soul";
pub const SLEEP: &str = "Sleep";
pub const MUG: &str = "Mug";
pub const SMOKE_BOMB: &str = "Smoke Bomb";
pub const ESCAPE: &str = "Escape";
pub const SCRATCH: &str = "Scratch";
pub const HEAL: &str = "Heal";
pub const ATTACK_DEBUFF: &str = "Attack/Debuff";
pub const DEBUFF: &str = "Debuff";
pub const SCYTHE: &str = "Scythe";
pub const LASER: &str = "Laser";
pub const CLAW: &str = "Claw";
pub const SCRAPE: &str = "Scrape";
pub const ENTANGLE: &str = "Entangle";
pub const SUMMON: &str = "Summon";
pub const SNAKE_STRIKE: &str = "Snake Strike";
pub const BIG_BITE: &str = "Big Bite";
pub const BASH: &str = "Bash";
pub const REPULSE: &str = "Repulse";
pub const MOCK: &str = "Mock";
pub const AGONIZING_SLASH: &str = "Agonizing Slash";
pub const CROSS_SLASH: &str = "Cross Slash";
pub const BOLT: &str = "Bolt";
pub const DOUBLE_STRIKE: &str = "Double Strike";
pub const SUCK: &str = "Suck";
pub const FELL: &str = "Fell";
pub const PROTECT: &str = "Protect";
pub const SHIELD_BASH: &str = "Shield Bash";
pub const GOOP_SPRAY: &str = "Goop Spray";
pub const PREPARING: &str = "Preparing";
pub const ENFEEBLING_SPORES: &str = "Enfeebling Spores";
pub const PERPLEXING_GLARE: &str = "Perplexing Glare";
pub const TAIL_WHIP: &str = "Tail Whip";
pub const PUNCTURE: &str = "Puncture";
pub const QUICK_TACKLE: &str = "Quick Tackle";
pub const CONSTRICT: &str = "Constrict";
pub const FLAME_TACKLE: &str = "Flame Tackle";
pub const CUT: &str = "Cut";
pub const SPIKE: &str = "Spike";
pub const FORTIFY: &str = "Fortify";
pub const BURN_STRIKE: &str = "Burn Strike";
pub const PIERCER: &str = "Piercer";
pub const SKEWER: &str = "Skewer";
pub const SCOURING_WHIP: &str = "Scouring Whip";
pub const DEFENSIVE_STANCE: &str = "Defensive Stance";
pub const FACE_SLAP: &str = "Face Slap";
pub const TAUNT: &str = "Taunt";
pub const HEAVY_SLASH: &str = "Heavy Slash";
pub const GLOAT: &str = "Gloat";
pub const EXECUTE: &str = "Execute";
pub const ANGER: &str = "Anger";
pub const FIREBALL: &str = "Fireball";
pub const MEGA_DEBUFF: &str = "Mega Debuff";
pub const SPAWN: &str = "Spawn";
pub const CHARGING_UP: &str = "Charging Up";
pub const FIERCE_BASH: &str = "Firece Bash";
pub const VENT_STEAM: &str = "Vent Steam";
pub const WHIRLWIND: &str = "Whirlwind";
pub const DEFENSIVE_MODE: &str = "Defensive Mode";
pub const ROLL_ATTACK: &str = "Roll Attack";
pub const TWIN_SLAM: &str = "Twin Slam";
pub const ROAR: &str = "Roar";
pub const DROOL: &str = "Drool";
pub const NOM: &str = "Nom";
pub const REVERBERATE: &str = "Reverberate";
pub const HEAD_SLAM: &str = "Head Slam";
pub const RIPPLE: &str = "Ripple";
pub const HASTE: &str = "Haste";
pub const IMPLANT: &str = "Implant";
pub const WITHER: &str = "Wither";
pub const MULTISTRIKE: &str = "Multi-Strike";
pub const STRONG_STRIKE: &str = "Strong Strike";
