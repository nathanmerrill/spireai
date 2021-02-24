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
            moveset: vec![],
            move_order: vec![],
            buffs: vec![],
        }
    }

    pub fn by_name(name: &str) -> BaseMonster {
        match name {
            ACID_SLIME_L => {
                Self {
                    hp_range: (65, 68),
                    hp_range_asc: (69, 72),
                    moveset: vec![
                        MonsterMove {
                            name: CORROSIVE_SPIT,
                            effects: vec![
                                Effect::Damage(ByAsc(11, 12, 12), EffectTarget::TargetEnemy),
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
                                Effect::AddBuff(buffs::WEAK, Fixed(2), EffectTarget::TargetEnemy)
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![Effect::Damage(ByAsc(16, 18, 18), EffectTarget::TargetEnemy)],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SPLIT,
                            effects: vec![
                                Effect::Split(ACID_SLIME_M)
                            ],
                            intent: Intent::Unknown,
                        },
                    ],
                    move_order: vec![
                        Move::IfAsc(17, 
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
                        Move::Event(Event::HalfHp(EffectTarget::_Self), true),
                        Move::InOrder(SPLIT),
                    ],
                    ..BaseMonster::default()
                }
            },
            ACID_SLIME_M => {
                Self {
                    hp_range: (28, 32),
                    hp_range_asc: (29, 34),
                    moveset: vec![
                        MonsterMove {
                            name: CORROSIVE_SPIT,
                            effects: vec![
                                Effect::Damage(ByAsc(7, 8, 8), EffectTarget::TargetEnemy),
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
                                Effect::AddBuff(buffs::WEAK, Fixed(1), EffectTarget::TargetEnemy)
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![Effect::Damage(ByAsc(10, 12, 12), EffectTarget::TargetEnemy)],
                            intent: Intent::Attack,
                        }
                    ],
                    move_order: vec![
                        Move::IfAsc(17, 
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
                    moveset: vec![
                        MonsterMove {
                            name: LICK,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(1), EffectTarget::TargetEnemy)
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: TACKLE,
                            effects: vec![Effect::Damage(ByAsc(3, 4, 4), EffectTarget::TargetEnemy)],
                            intent: Intent::Attack,
                        }
                    ],
                    move_order: vec![
                        Move::IfAsc(17, vec![
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
                    buffs: vec![
                        (buffs::CURIOSITY, ByAsc(1, 1, 2)),
                        (buffs::STRENGTH, ByAsc(0, 2, 2)),
                        (buffs::REGENERATE, ByAsc(10, 10, 15)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: SLASH,
                            effects: vec![
                                Effect::Damage(Fixed(20), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SOUL_STRIKE,
                            effects: vec![
                                Effect::Damage(Fixed(6), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(6), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(6), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(6), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: REBIRTH,
                            effects: vec![Effect::RemoveDebuffs(EffectTarget::_Self)],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: DARK_ECHO,
                            effects: vec![
                                Effect::Damage(Fixed(40), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SLUDGE,
                            effects: vec![
                                Effect::Damage(Fixed(18), EffectTarget::TargetEnemy),
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
                                Effect::Damage(Fixed(10), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(10), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(10), EffectTarget::TargetEnemy),
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
                        Move::Event(Event::Die(EffectTarget::_Self), true),
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
                            effects: vec![Effect::AddBuff(buffs::DEXTERITY, ByAsc(-2, -2, -4), EffectTarget::TargetEnemy)],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: LUNGE,
                            effects: vec![
                                Effect::Damage(Fixed(9), EffectTarget::TargetEnemy),
                                Effect::Block(ByAsc(9, 10, 10), EffectTarget::_Self),
                            ],
                            intent: Intent::AttackDefend,
                        },
                        MonsterMove {
                            name: MAUL,
                            effects: vec![
                                Effect::Damage(ByAsc(18, 20, 20), EffectTarget::TargetEnemy),
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
                    buffs: vec![
                        (buffs::PAINFUL_STABS, Fixed(1))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: MULTI_STAB,
                            effects: vec![Effect::Custom],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SINGLE_STAB,
                            effects: vec![Effect::Damage(ByAsc(21, 24, 24), EffectTarget::TargetEnemy)],
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
                            effects: vec![Effect::Damage(ByAsc(12, 13, 13), EffectTarget::TargetEnemy)],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: RAKE,
                            effects: vec![
                                Effect::Damage(ByAsc(7, 8, 8), EffectTarget::TargetEnemy),
                                Effect::AddBuff(buffs::WEAK, ByAsc(1, 1, 2), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        }
                    ],
                    move_order: vec![
                        Move::IfAsc(17, 
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
                    buffs: vec![
                        (buffs::ARTIFACT, Fixed(3))
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: SPAWN_ORBS,
                            effects: vec![
                                Effect::Spawn(BRONZE_ORB),
                                Effect::Spawn(BRONZE_ORB),
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: FLAIL,
                            effects: vec![
                                Effect::Damage(ByAsc(7, 8, 8), EffectTarget::TargetEnemy),
                                Effect::Damage(ByAsc(7, 8, 8), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: BOOST,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, ByAsc(3, 4, 4), EffectTarget::_Self),
                                Effect::Block(ByAsc(9, 12, 12), EffectTarget::_Self),
                            ],
                            intent: Intent::DefendBuff,
                        },
                        MonsterMove {
                            name: HYPERBEAM,
                            effects: vec![
                                Effect::Damage(ByAsc(45, 50, 50), EffectTarget::TargetEnemy),
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
                            Move::IfAsc(19, vec![
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
                                Effect::Damage(Fixed(8), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SUPPORT_BEAM,
                            effects: vec![
                                Effect::Block(Fixed(12), EffectTarget::Friendly(BRONZE_AUTOMATON)),
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
                        Move::Event(Event::Buff(buffs::STASIS, EffectTarget::_Self), true),
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
                    buffs: vec![(buffs::FLYING, ByAsc(3, 3, 4))],
                    moveset: vec![
                        MonsterMove {
                            name: CAW,
                            effects: vec![
                                Effect::AddBuff(buffs::STRENGTH, Fixed(1), EffectTarget::_Self),
                            ],
                            intent: Intent::Buff
                        },
                        MonsterMove {
                            name: PECK,
                            effects: vec![
                                Effect::Damage(Fixed(1), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(1), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(1), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(1), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(1), EffectTarget::TargetEnemy),
                                Effect::IfAsc(2, vec![
                                    Effect::Damage(Fixed(1), EffectTarget::TargetEnemy)
                                ])
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: SWOOP,
                            effects: vec![
                                Effect::Damage(ByAsc(12, 14, 14), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: FLY,
                            effects: vec![
                                Effect::AddBuff(buffs::FLYING, ByAsc(3, 3, 4), EffectTarget::_Self),
                            ],
                            intent: Intent::Unknown,
                        },
                        MonsterMove {
                            name: HEADBUTT,
                            effects: vec![
                                Effect::Damage(Fixed(3), EffectTarget::TargetEnemy),
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
                            Move::Event(Event::UnBuff(buffs::FLYING, EffectTarget::_Self), true),
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
                                Effect::Damage(ByAsc(12, 14, 14), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: FURY,
                            effects: vec![
                                Effect::Damage(ByAsc(6, 7, 7), EffectTarget::TargetEnemy),
                                Effect::Damage(ByAsc(6, 7, 7), EffectTarget::TargetEnemy),
                                Effect::Damage(ByAsc(6, 7, 7), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: DEFEND,
                            effects: vec![
                                Effect::Block(ByAsc(15, 15, 20), EffectTarget::Friendly(MYSTIC)),
                            ],
                            intent: Intent::Defend,
                        },
                    ],
                    move_order: vec![
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (65, DEFEND, 0),
                                (35, SLASH, 0),
                            ])
                        ]),
                        Move::Event(Event::Die(EffectTarget::Friendly(MYSTIC)), false),
                        Move::Loop(vec![
                            Move::Probability(vec![
                                (65, FURY, 0),
                                (35, SLASH, 0),
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
                                Effect::Damage(ByAsc(5, 6, 6), EffectTarget::TargetEnemy),
                                Effect::Damage(ByAsc(5, 6, 6), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack
                        },
                        MonsterMove {
                            name: ZAP,
                            effects: vec![
                                Effect::Damage(ByAsc(18, 21, 21), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: DEBILITATE,
                            effects: vec![
                                Effect::Damage(ByAsc(10, 12, 12), EffectTarget::TargetEnemy),
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::AttackDebuff,
                        },
                        MonsterMove {
                            name: DRAIN,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(3), EffectTarget::TargetEnemy),
                                Effect::AddBuff(buffs::STRENGTH, Fixed(3), EffectTarget::_Self),
                            ],
                            intent: Intent::Debuff,
                        },
                        MonsterMove {
                            name: HEX,
                            effects: vec![
                                Effect::AddBuff(buffs::HEX, Fixed(1), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::IfAsc(17, vec![], vec![
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
                    hp_range: (76, 80),
                    hp_range_asc: (78, 83),
                    buffs: vec![
                        (buffs::BEAT_OF_DEATH, ByAsc(1, 1, 2)),
                        (buffs::INVINCIBLE, ByAsc(300, 300, 200)),
                    ],
                    moveset: vec![
                        MonsterMove {
                            name: DEBILITATE,
                            effects: vec![
                                Effect::AddBuff(buffs::VULNERABLE, Fixed(2), EffectTarget::TargetEnemy),
                                Effect::AddBuff(buffs::WEAK, Fixed(2), EffectTarget::TargetEnemy),
                                Effect::AddBuff(buffs::FRAIL, Fixed(2), EffectTarget::TargetEnemy),
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
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                Effect::IfAsc(4, vec![
                                    Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                    Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                    Effect::Damage(Fixed(2), EffectTarget::TargetEnemy),
                                ]),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: ECHO,
                            effects: vec![
                                Effect::Damage(ByAsc(40, 45, 45), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::Attack,
                        },
                        MonsterMove {
                            name: DRAIN,
                            effects: vec![
                                Effect::AddBuff(buffs::WEAK, Fixed(3), EffectTarget::TargetEnemy),
                                Effect::AddBuff(buffs::STRENGTH, Fixed(3), EffectTarget::_Self),
                            ],
                            intent: Intent::Buff,
                        },
                        MonsterMove {
                            name: HEX,
                            effects: vec![
                                Effect::AddBuff(buffs::HEX, Fixed(1), EffectTarget::TargetEnemy),
                            ],
                            intent: Intent::StrongDebuff,
                        },
                    ],
                    move_order: vec![
                        Move::IfAsc(17, vec![], vec![
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
pub const LICK: &str = "Lick";
pub const LICK: &str = "Lick";