use crate::models::cards;
use crate::models::relics;
use crate::models::monsters;
use crate::models::core::*;
use std::collections::HashMap;

pub fn by_name(name: &str) -> &'static BaseEvent {
    EVENTS.get(name).expect(format!("Unrecognized event: {}", name).as_str())
}

lazy_static! {
    static ref EVENTS: HashMap<&'static str, BaseEvent> = {
        let mut m = HashMap::new();

        for event in all_events() {
            m.insert(event.name, event);
        }

        m
    };
}

impl BaseEventChoice {
    fn new() -> Self {
        Self {
            name: &"",
            effects: vec![],
            condition: Condition::Always,
            repeats: false,
        }
    }
}


fn all_events() -> Vec<BaseEvent> {
    vec![
        BaseEvent {
            name: NEOW,
            shrine: false,  
            variants: vec![
                NEOW_FIRST_RUN,
                NEOW_SUCCESS_RUN
            ],
            choices: vec![
                BaseEventChoice {
                    name: "Talk",
                    condition: Condition::Custom,
                    effects: vec![
                        Effect::ShowChoices(vec![
                            BaseEventChoice {
                                name: "Enemies in your next three combats have 1 HP",
                                effects: vec![
                                    Effect::AddRelic(relics::NEOWS_LAMENT),
                                ],
                                ..BaseEventChoice::new()
                            },
                            BaseEventChoice {
                                name: "Max HP +6",
                                effects: vec![
                                    Effect::AddMaxHp(Amount::Fixed(6)),
                                ],
                                condition: Condition::Class(Class::Silent),
                                ..BaseEventChoice::new()
                            },
                            BaseEventChoice {
                                name: "Max HP +7",
                                effects: vec![
                                    Effect::AddMaxHp(Amount::Fixed(7)),
                                ],
                                condition: Condition::MultipleOr(vec![
                                    Condition::Class(Class::Defect),
                                    Condition::Class(Class::Watcher)
                                ]),
                                ..BaseEventChoice::new()
                            },
                            BaseEventChoice {
                                name: "Max HP +8",
                                effects: vec![
                                    Effect::AddMaxHp(Amount::Fixed(8)),
                                ],
                                condition: Condition::Class(Class::Ironclad),
                                ..BaseEventChoice::new()
                            },
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
            ],       
        },
        BaseEvent {
            name: ANCIENT_WRITING,
            shrine: false,    
            variants: vec![], 
            choices: vec![
                BaseEventChoice {
                    name: "Elegance",
                    effects: vec![
                        Effect::RemoveCard(1),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Simplicity",
                    effects: vec![
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::All))
                    ],
                    ..BaseEventChoice::new()
                },
            ],    
        },
        BaseEvent {
            name: AUGMENTER,
            shrine: false,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Test J.A.X.",
                    effects: vec![
                        Effect::AddCard {
                            card: CardReference::ByName(cards::JAX),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        }
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Become Test Subject",
                    effects: vec![
                        Effect::TransformCard(2),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Ingest Mutagens",
                    effects: vec![
                        Effect::AddRelic(relics::MUTAGENIC_STRENGTH),
                    ],
                    ..BaseEventChoice::new()
                },
            ],         
        },
        BaseEvent {
            name: BIG_FISH,
            shrine: false,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Banana",
                    effects: vec![
                        Effect::HealPercentage(Amount::Fixed(33), Target::_Self)
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Donut",
                    effects: vec![
                        Effect::AddMaxHp(Amount::Fixed(5))
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Box",
                    effects: vec![
                        Effect::AddCard {
                            card: CardReference::ByName(cards::REGRET),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                        Effect::ShowReward(vec![
                            RewardType::RandomRelic
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
            ],        
        },
        BaseEvent {
            name: BONFIRE_SPIRITS,
            shrine: true,  
            variants: vec![], 
            choices: vec![
                BaseEventChoice {
                    name: "Offer",
                    effects: vec![
                        Effect::Custom
                    ],
                    ..BaseEventChoice::new()
                },
            ],      
        },
        BaseEvent {
            name: THE_CLERIC,
            shrine: false,  
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Heal",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-35)),
                        Effect::HealPercentage(Amount::Fixed(25), Target::_Self),
                    ],
                    condition: Condition::HasGold(Amount::Fixed(35)),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Purify",
                    effects: vec![
                        Effect::AddGold(Amount::ByAsc(-50, -75, -75)),
                        Effect::RemoveCard(1),
                    ],
                    condition: Condition::HasGold(Amount::ByAsc(50, 75, 75)),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave",
                    ..BaseEventChoice::new()
                },
            ],       
        },
        BaseEvent {
            name: THE_COLOSSEUM,
            shrine: false,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Continue",
                    effects: vec![
                        Effect::Fight(vec![monsters::BLUE_SLAVER, monsters::RED_SLAVER], false),
                        Effect::ShowChoices(vec![
                            BaseEventChoice {
                                name: "COWARDICE",
                                ..BaseEventChoice::new()
                            },
                            BaseEventChoice {
                                name: "VICTORY",
                                effects: vec![
                                    Effect::Fight(vec![monsters::TASKMASTER, monsters::GREMLIN_NOB], true),
                                    Effect::ShowReward(vec![
                                        RewardType::Gold(100, 100),
                                        RewardType::Relic(Rarity::Rare),
                                        RewardType::Relic(Rarity::Uncommon),
                                        RewardType::StandardCard,
                                    ]),
                                ],
                                ..BaseEventChoice::new()
                            },
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
            ],         
        },
        BaseEvent {
            name: COUNCIL_OF_GHOSTS,
            shrine: false,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Accept",
                    effects: vec![
                        Effect::ReduceMaxHpPercentage(Amount::Fixed(50)),
                        Effect::AddCard {
                            card: CardReference::ByName(cards::APPARITION),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::ByAsc(5, 3, 3),
                            modifier: CardModifier::None,
                        },
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Refuse",
                    ..BaseEventChoice::new()
                },                
                
            ]
        },
        BaseEvent {
            name: CURSED_TOME,
            shrine: false,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Read",
                    effects: vec![
                        Effect::ShowChoices(vec![
                            BaseEventChoice {
                                name: "Continue",
                                effects: vec![
                                    Effect::LoseHp(Amount::Fixed(1), Target::_Self),
                                ],
                                ..BaseEventChoice::new()
                            }
                        ]),
                        Effect::ShowChoices(vec![
                            BaseEventChoice {
                                name: "Continue",
                                effects: vec![
                                    Effect::LoseHp(Amount::Fixed(2), Target::_Self),
                                ],
                                ..BaseEventChoice::new()
                            }
                        ]),
                        Effect::ShowChoices(vec![
                            BaseEventChoice {
                                name: "Continue",
                                effects: vec![
                                    Effect::LoseHp(Amount::Fixed(3), Target::_Self),
                                ],
                                ..BaseEventChoice::new()
                            }
                        ]),
                        Effect::ShowChoices(vec![
                            BaseEventChoice {
                                name: "Take",
                                effects: vec![
                                    Effect::LoseHp(Amount::ByAsc(10, 15, 15), Target::_Self),
                                    Effect::ShowReward(vec![RewardType::RandomBook]),
                                ],
                                ..BaseEventChoice::new()
                            },
                            BaseEventChoice {
                                name: "Stop",
                                effects: vec![
                                    Effect::LoseHp(Amount::Fixed(3), Target::_Self),
                                ],
                                ..BaseEventChoice::new()
                            },
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave",
                    ..BaseEventChoice::new()
                },
            ]
        },
        BaseEvent {
            name: DEAD_ADVENTURER,
            shrine: false,
            variants: vec![
                DEAD_ADVENTURER_LAGAVULIN,
                DEAD_ADVENTURER_SENTRY,
                DEAD_ADVENTURER_GREMLIN_NOB,
            ],

            choices: vec![
                BaseEventChoice {
                    name: "Leave",
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Search",
                    effects: vec![
                        Effect::RandomChance(vec![
                            (Amount::ByAsc(25, 35, 35), Effect::Multiple(vec![
                                Effect::If(Condition::IsVariant(DEAD_ADVENTURER_GREMLIN_NOB), vec![
                                    Effect::Fight(vec![monsters::GREMLIN_NOB], true)
                                ], vec![
                                    Effect::If(Condition::IsVariant(DEAD_ADVENTURER_LAGAVULIN), vec![
                                        Effect::Fight(vec![monsters::LAGAVULIN], true)
                                    ], vec![
                                        Effect::Fight(vec![monsters::SENTRY, monsters::SENTRY, monsters::SENTRY], true)
                                    ])
                                ]),
                                Effect::ShowReward(vec![
                                    RewardType::RandomRelic,
                                    RewardType::Gold(55, 65),
                                    RewardType::EliteCard,
                                ])
                            ])),
                            (Amount::ByAsc(75, 65, 65), Effect::Multiple(vec![
                                Effect::RandomChance(vec![
                                    (Amount::Fixed(33), Effect::Multiple(vec![
                                        Effect::AddGold(Amount::Fixed(30)),
                                        Effect::AddN(Amount::Fixed(1))
                                    ])),
                                    (Amount::Fixed(33), Effect::Multiple(vec![
                                        Effect::RandomRelic,
                                        Effect::AddN(Amount::Fixed(-1))
                                    ])),
                                    (Amount::Fixed(33), Effect::None),
                                ]),
                                Effect::ShowChoices(vec![
                                    BaseEventChoice {
                                        name: "Leave",
                                        ..BaseEventChoice::new()
                                    },
                                    BaseEventChoice {
                                        name: "Search",
                                        effects: vec![
                                            Effect::RandomChance(vec![
                                                (Amount::ByAsc(50, 60, 60), Effect::Multiple(vec![
                                                    Effect::If(Condition::IsVariant(DEAD_ADVENTURER_GREMLIN_NOB), vec![
                                                        Effect::Fight(vec![monsters::GREMLIN_NOB], true)
                                                    ], vec![
                                                        Effect::If(Condition::IsVariant(DEAD_ADVENTURER_LAGAVULIN), vec![
                                                            Effect::Fight(vec![monsters::LAGAVULIN], true)
                                                        ], vec![
                                                            Effect::Fight(vec![monsters::SENTRY, monsters::SENTRY, monsters::SENTRY], true)
                                                        ])
                                                    ]),     
                                                    Effect::If(Condition::LessThan(Amount::N, Amount::Fixed(0)), vec![
                                                        Effect::ShowReward(vec![
                                                            RewardType::Gold(55, 65),
                                                            RewardType::EliteCard,
                                                        ])
                                                    ], vec![
                                                        Effect::If(Condition::LessThan(Amount::Fixed(0), Amount::N), vec![
                                                            Effect::ShowReward(vec![
                                                                RewardType::RandomRelic,
                                                                RewardType::Gold(25, 35),
                                                                RewardType::EliteCard,
                                                            ])
                                                        ], vec![
                                                            Effect::ShowReward(vec![
                                                                RewardType::RandomRelic,
                                                                RewardType::Gold(55, 65),
                                                                RewardType::EliteCard,
                                                            ])
                                                        ])
                                                    ])
                                                ])),
                                                (Amount::ByAsc(50, 40, 40), Effect::Multiple(vec![
                                                    Effect::If(Condition::LessThan(Amount::N, Amount::Fixed(0)), vec![
                                                        Effect::RandomChance(vec![
                                                            (Amount::Fixed(50), Effect::Multiple(vec![
                                                                Effect::AddGold(Amount::Fixed(30)),
                                                                Effect::AddN(Amount::Fixed(1))
                                                            ])),
                                                            (Amount::Fixed(50), Effect::None),
                                                        ]),
                                                    ], vec![
                                                        Effect::If(Condition::LessThan(Amount::Fixed(0), Amount::N), vec![
                                                            Effect::RandomChance(vec![
                                                                (Amount::Fixed(50), Effect::Multiple(vec![
                                                                    Effect::RandomRelic,
                                                                    Effect::AddN(Amount::Fixed(-1))
                                                                ])),
                                                                (Amount::Fixed(50), Effect::None),
                                                            ]),
                                                        ], vec![
                                                            Effect::RandomChance(vec![
                                                                (Amount::Fixed(50), Effect::Multiple(vec![
                                                                    Effect::RandomRelic,
                                                                    Effect::AddN(Amount::Fixed(-1))
                                                                ])),
                                                                (Amount::Fixed(50), Effect::Multiple(vec![
                                                                    Effect::AddGold(Amount::Fixed(30)),
                                                                    Effect::AddN(Amount::Fixed(1))
                                                                ])),
                                                            ]),
                                                        ])
                                                    ]),
                                                    Effect::ShowChoices(vec![
                                                        BaseEventChoice {
                                                            name: "Leave",
                                                            ..BaseEventChoice::new()
                                                        },
                                                        BaseEventChoice {
                                                            name: "Search",
                                                            effects: vec![
                                                                Effect::RandomChance(vec![
                                                                    (Amount::ByAsc(75, 85, 85), Effect::Multiple(vec![
                                                                        Effect::If(Condition::IsVariant(DEAD_ADVENTURER_GREMLIN_NOB), vec![
                                                                            Effect::Fight(vec![monsters::GREMLIN_NOB], true)
                                                                        ], vec![
                                                                            Effect::If(Condition::IsVariant(DEAD_ADVENTURER_LAGAVULIN), vec![
                                                                                Effect::Fight(vec![monsters::LAGAVULIN], true)
                                                                            ], vec![
                                                                                Effect::Fight(vec![monsters::SENTRY, monsters::SENTRY, monsters::SENTRY], true)
                                                                            ])
                                                                        ]),     
                                                                        Effect::If(Condition::LessThan(Amount::N, Amount::Fixed(0)), vec![
                                                                            Effect::ShowReward(vec![
                                                                                RewardType::Gold(55, 65),
                                                                                RewardType::EliteCard,
                                                                            ])
                                                                        ], vec![
                                                                            Effect::If(Condition::LessThan(Amount::Fixed(0), Amount::N), vec![
                                                                                Effect::ShowReward(vec![
                                                                                    RewardType::RandomRelic,
                                                                                    RewardType::Gold(25, 35),
                                                                                    RewardType::EliteCard,
                                                                                ])
                                                                            ], vec![
                                                                                Effect::ShowReward(vec![
                                                                                    RewardType::Gold(25, 35),
                                                                                    RewardType::EliteCard,
                                                                                ])                                                                
                                                                            ])
                                                                        ]),
                                                                    ])),
                                                                    (Amount::ByAsc(25, 15, 15), Effect::Multiple(vec![
                                                                        
                                                                        Effect::If(Condition::LessThan(Amount::N, Amount::Fixed(0)), vec![
                                                                            Effect::AddGold(Amount::Fixed(30))
                                                                        ], vec![
                                                                            Effect::If(Condition::LessThan(Amount::Fixed(0), Amount::N), vec![
                                                                                Effect::RandomRelic,
                                                                            ], vec![])
                                                                        ]), 
                                                                        Effect::ShowChoices(vec![
                                                                            BaseEventChoice {
                                                                                name: "Leave",
                                                                                ..BaseEventChoice::new()
                                                                            },
                                                                        ])
                                                                    ]))
                                                                ])
                                                            ],
                                                            ..BaseEventChoice::new()
                                                        }
                                                    ])
                                                ]))
                                            ])
                                        ],
                                        ..BaseEventChoice::new()
                                    }
                                ])
                            ]))
                        ])
                    ],
                    ..BaseEventChoice::new()
                },

            ]
        },
        BaseEvent {
            name: DESIGNER_INSPIRE,
            variants: vec![
                DESIGNER_INSPIRE_BASE,
                DESIGNER_INSPIRE_UP2,
                DESIGNER_INSPIRE_TRANS2,
                DESIGNER_INSPIRE_UP2_TRANS2,
            ],
            shrine: true,
            choices: vec![
                BaseEventChoice {
                    name: "Adjustments",
                    effects: vec![
                        Effect::AddGold(Amount::ByAsc(-40, -50, -50)),
                        Effect::If(Condition::MultipleOr(vec![
                            Condition::IsVariant(DESIGNER_INSPIRE_BASE),
                            Condition::IsVariant(DESIGNER_INSPIRE_TRANS2)
                        ]), vec![
                            Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::PlayerChoice(Amount::Fixed(1))))
                        ], vec![
                            Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::Random)),
                            Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::Random)),
                        ])
                    ],
                    condition: Condition::MultipleAnd(vec![
                        Condition::HasUpgradableCard,
                        Condition::HasGold(Amount::ByAsc(40, 50, 50)),
                    ]),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Clean Up",
                    effects: vec![
                        Effect::AddGold(Amount::ByAsc(-60, -75, -75)),
                        Effect::If(Condition::MultipleOr(vec![
                            Condition::IsVariant(DESIGNER_INSPIRE_BASE),
                            Condition::IsVariant(DESIGNER_INSPIRE_UP2)
                        ]), vec![
                            Effect::RemoveCard(1),
                        ], vec![
                            Effect::TransformRandomCard(2),
                        ])
                    ],
                    condition: Condition::MultipleAnd(vec![
                        Condition::HasGold(Amount::ByAsc(60, 75, 75)),
                        Condition::MultipleOr(vec![
                            Condition::MultipleAnd(vec![
                                Condition::MultipleOr(vec![
                                    Condition::IsVariant(DESIGNER_INSPIRE_BASE),
                                    Condition::IsVariant(DESIGNER_INSPIRE_UP2)
                                ]),
                                Condition::HasRemoveableCards(1, CardType::All),
                            ]),
                            Condition::MultipleAnd(vec![
                                Condition::MultipleOr(vec![
                                    Condition::IsVariant(DESIGNER_INSPIRE_TRANS2),
                                    Condition::IsVariant(DESIGNER_INSPIRE_UP2_TRANS2)
                                ]),
                                Condition::HasRemoveableCards(2, CardType::All),
                            ]),
                        ]),
                    ]),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Full Service",
                    effects: vec![
                        Effect::AddGold(Amount::ByAsc(-90, -110, -110)),
                        Effect::RemoveCard(1),
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::Random)),
                    ],
                    condition: Condition::MultipleAnd(vec![
                        Condition::HasRemoveableCards(1, CardType::All),
                        Condition::HasGold(Amount::ByAsc(90, 110, 110)),
                    ]),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Punch",
                    effects: vec![
                        Effect::LoseHp(Amount::ByAsc(3, 5, 5), Target::_Self),
                    ],
                    ..BaseEventChoice::new()
                },
            ]
        },
        BaseEvent {
            name: THE_DIVINE_FOUNTAIN,
            shrine: true,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Drink",
                    effects: vec![
                        Effect::Custom
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave",
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: DUPLICATOR,
            shrine: true,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Pray",
                    effects: vec![
                        Effect::DuplicateCard,
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave",
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: FACE_TRADER,
            shrine: true,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Touch",
                    effects: vec![
                        Effect::DamagePercentage(Amount::Fixed(10)),
                        Effect::If(Condition::Asc(15), vec![
                            Effect::AddGold(Amount::Fixed(50))
                        ], vec![
                            Effect::AddGold(Amount::Fixed(75))
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Trade",
                    effects: vec![
                        Effect::RandomChance(vec![
                            (Amount::Fixed(20), Effect::AddRelic(relics::CULTIST_HEADPIECE)),
                            (Amount::Fixed(20), Effect::AddRelic(relics::FACE_OF_CLERIC)),
                            (Amount::Fixed(20), Effect::AddRelic(relics::GREMLIN_VISAGE)),
                            (Amount::Fixed(20), Effect::AddRelic(relics::NLOTHS_HUNGRY_FACE)),
                            (Amount::Fixed(20), Effect::AddRelic(relics::SSSERPENT_HEAD)),
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave",
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: FALLING,
            shrine: false,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Land",
                    effects: vec![
                        Effect::Custom,
                    ],
                    condition: Condition::HasRemoveableCards(1, CardType::Skill),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Channel",
                    effects: vec![
                        Effect::Custom,
                    ],
                    condition: Condition::HasRemoveableCards(1, CardType::Power),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Attack",
                    effects: vec![
                        Effect::Custom,
                    ],
                    condition: Condition::HasRemoveableCards(1, CardType::Attack),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Land gracefully",
                    condition: Condition::MultipleAnd(vec![
                        Condition::Not(Box::new(Condition::HasRemoveableCards(1, CardType::Skill))),
                        Condition::Not(Box::new(Condition::HasRemoveableCards(1, CardType::Attack))),
                        Condition::Not(Box::new(Condition::HasRemoveableCards(1, CardType::Power))),
                    ]),
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: FORGOTTEN_ALTAR,
            shrine: false,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Offer",
                    effects: vec![
                        Effect::RemoveRelic(relics::GOLDEN_IDOL),
                        Effect::AddRelic(relics::BLOODY_IDOL),
                    ],
                    condition: Condition::HasRelic(relics::GOLDEN_IDOL),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Sacrifice",
                    effects: vec![
                        Effect::DamagePercentage(Amount::ByAsc(25, 35, 35)),
                        Effect::AddMaxHp(Amount::Fixed(5)),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Desecrate",
                    effects: vec![
                        Effect::AddCard{
                            card: CardReference::ByName(cards::DECAY),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: GOLDEN_IDOL,
            shrine: false,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Take",
                    effects: vec![
                        Effect::AddRelic(relics::GOLDEN_IDOL),
                        Effect::ShowChoices(vec![
                            BaseEventChoice {
                                name: "Outrun",
                                effects: vec![
                                    Effect::AddCard{
                                        card: CardReference::ByName(cards::INJURY),
                                        destination: CardLocation::DeckPile(RelativePosition::Bottom),
                                        copies: Amount::Fixed(1),
                                        modifier: CardModifier::None,
                                    },
                                ],
                                ..BaseEventChoice::new()
                            },
                            BaseEventChoice {
                                name: "Smash",
                                effects: vec![
                                    Effect::DamagePercentage(Amount::ByAsc(25, 35, 35)),
                                ],
                                ..BaseEventChoice::new()
                            },
                            BaseEventChoice {
                                name: "Hide",
                                effects: vec![
                                    Effect::ReduceMaxHpPercentage(Amount::ByAsc(8, 10, 10)),
                                ],
                                ..BaseEventChoice::new()
                            },
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave",
                    effects: vec![
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: GOLDEN_SHRINE,
            shrine: true,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Pray",
                    effects: vec![
                        Effect::AddGold(Amount::ByAsc(100, 50, 50)),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Desecrate",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(275)),
                        Effect::AddCard{
                            card: CardReference::ByName(cards::REGRET),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave",
                    effects: vec![],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: THE_JOUST,
            shrine: true,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Bet on Murderer",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-50)),
                        Effect::RandomChance(vec![
                            (Amount::Fixed(70), Effect::AddGold(Amount::Fixed(100)))
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Bet on Owner",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-50)),
                        Effect::RandomChance(vec![
                            (Amount::Fixed(30), Effect::AddGold(Amount::Fixed(250)))
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: KNOWING_SKULL,
            shrine: true,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Success?",
                    effects: vec![
                        Effect::LoseHp(Amount::Sum(vec![
                            Amount::Fixed(6),
                            Amount::N
                        ]), Target::_Self),
                        Effect::AddCard {
                            card: CardReference::RandomClass(Class::None),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                        Effect::AddN(Amount::Fixed(1)),
                    ],
                    repeats: true,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Success?",
                    effects: vec![
                        Effect::LoseHp(Amount::Sum(vec![
                            Amount::Fixed(6),
                            Amount::N
                        ]), Target::_Self),
                        Effect::AddCard {
                            card: CardReference::RandomClass(Class::None),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                        Effect::AddN(Amount::Fixed(1)),
                    ],
                    repeats: true,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "A Pick Me Up?",
                    effects: vec![
                        Effect::LoseHp(Amount::Sum(vec![
                            Amount::Fixed(6),
                            Amount::N
                        ]), Target::_Self),
                        Effect::RandomPotion,
                        Effect::AddN(Amount::Fixed(1)),
                    ],
                    repeats: true,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Riches?",
                    effects: vec![
                        Effect::LoseHp(Amount::Sum(vec![
                            Amount::Fixed(6),
                            Amount::N
                        ]), Target::_Self),
                        Effect::AddGold(Amount::Fixed(90)),
                        Effect::AddN(Amount::Fixed(1)),
                    ],
                    repeats: true,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "How do I leave?",
                    effects: vec![
                        Effect::LoseHp(Amount::Fixed(6), Target::_Self),
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: LAB,
            shrine: true,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Find Some Potions!",
                    effects: vec![
                        Effect::If(Condition::Asc(15), vec![
                            Effect::ShowReward(vec![
                                RewardType::RandomPotion,
                                RewardType::RandomPotion,
                            ])
                        ], vec![
                            Effect::ShowReward(vec![
                                RewardType::RandomPotion,
                                RewardType::RandomPotion,
                                RewardType::RandomPotion,
                            ])
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: THE_LIBRARY,
            variants: vec![],
            shrine: false,
            choices: vec![
                BaseEventChoice {
                    name: "Read",
                    effects: vec![
                        Effect::AddCard {
                            card: CardReference::RandomType(CardType::All, Amount::Fixed(20)),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        }
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Sleep",
                    effects: vec![
                        Effect::HealPercentage(Amount::ByAsc(33, 20, 20), Target::_Self),
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: LIVING_WALL,
            variants: vec![],
            shrine: false,
            choices: vec![
                BaseEventChoice {
                    name: "Forget",
                    effects: vec![
                        Effect::RemoveCard(1),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Change",
                    effects: vec![
                        Effect::TransformCard(1),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Grow",
                    effects: vec![
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::PlayerChoice(Amount::Fixed(1)))),
                    ],
                    condition: Condition::HasUpgradableCard,
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: MASKED_BANDITS,
            variants: vec![],
            shrine: false,
            choices: vec![
                BaseEventChoice {
                    name: "Pay",
                    effects: vec![
                        Effect::LoseAllGold,
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Fight",
                    effects: vec![
                        Effect::Fight(vec![
                            monsters::POINTY,
                            monsters::ROMEO,
                            monsters::BEAR
                        ], false),
                        Effect::ShowReward(vec![
                            RewardType::RelicName(relics::RED_MASK),
                            RewardType::Gold(25, 35),
                            RewardType::StandardCard,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },

    ]
}

pub const NEOW: &str = "Neow";
pub const ANCIENT_WRITING: &str = "Ancient Writing";
pub const AUGMENTER: &str = "Augmenter";
pub const BIG_FISH: &str = "Big Fish";
pub const BONFIRE_SPIRITS: &str = "Bonfire Spirits";
pub const THE_CLERIC: &str = "The Cleric";
pub const THE_COLOSSEUM: &str = "The Colosseum";
pub const COUNCIL_OF_GHOSTS: &str = "Council of Ghosts";
pub const CURSED_TOME: &str = "Cursed Tome";
pub const DEAD_ADVENTURER: &str = "Dead Adventurer";
pub const DESIGNER_INSPIRE: &str = "Designer In-Spire";
pub const THE_DIVINE_FOUNTAIN: &str = "The Divine Fountain";
pub const DUPLICATOR: &str = "Duplicator";
pub const FACE_TRADER: &str = "Face Trader";
pub const FALLING: &str = "Falling";
pub const FORGOTTEN_ALTAR: &str = "Forgotten Altar";
pub const GOLDEN_IDOL: &str = "Golden Idol";
pub const GOLDEN_SHRINE: &str = "Golden Shrine";
pub const THE_JOUST: &str = "The Joust";
pub const KNOWING_SKULL: &str = "Knowing Skull";
pub const LAB: &str = "Lab";
pub const THE_LIBRARY: &str = "The Library";
pub const LIVING_WALL: &str = "Living Wall";
pub const MASKED_BANDITS: &str = "Masked Bandits";
pub const MATCH_AND_KEEP: &str = "Match and Keep";
pub const THE_MAUSOLEUM: &str = "The Mausoleum";
pub const MIND_BLOOM: &str = "Mind Bloom";
pub const THE_MOAI_HEAD: &str = "The Moai Head";
pub const MUSHROOMS: &str = "Mushrooms";
pub const MYSTERIOUS_SPHERE: &str = "Mysterious Sphere";
pub const THE_NEST: &str = "The Nest";
pub const A_NOTE_FOR_YOURSELF: &str = "A Note For Yourself";
pub const NLOTH: &str = "N'loth";
pub const OLD_BEGGAR: &str = "Old Beggar";
pub const OMINOUS_FORGE: &str = "Ominous Forge";
pub const PLEADING_VAGRANT: &str = "Pleading Vagrant";
pub const PURIFIER: &str = "Purifier";
pub const SCRAP_OOZE: &str = "Scrap Ooze";
pub const SECRET_PORTAL: &str = "Secret Portal";
pub const SENSORY_STONE: &str = "Sensory Stone";
pub const SHINING_LIGHT: &str = "Shining Light";
pub const THE_SSSSSERPENT: &str = "The Ssssserpent";
pub const TOMB_OF_LORD_RED_MASK: &str = "Tomb of Lord Red Mask";
pub const TRANSMOGRIFIER: &str = "Transmogrifier";
pub const UPGRADE_SHRINE: &str = "Upgrade Shrine";
pub const VAMPIRES: &str = "Vampires(?)";
pub const WE_MEET_AGAIN: &str = "We Meet Again";
pub const WHEEL_OF_CHANGE: &str = "Wheel of Change";
pub const WINDING_HALLS: &str = "Winding Halls";
pub const WING_STATUE: &str = "Wing Statue";
pub const THE_WOMAN_IN_BLUE: &str = "The Woman In Blue";
pub const WORLD_OF_GOOP: &str = "World of Goop";


pub const DEAD_ADVENTURER_LAGAVULIN: &str = "Dead Adventurer Lagavulin";
pub const DEAD_ADVENTURER_SENTRY: &str = "Dead Adventurer Sentry";
pub const DEAD_ADVENTURER_GREMLIN_NOB: &str = "Dead Adventurer Nob";

pub const NEOW_FIRST_RUN: &str = "Neow First Run";
pub const NEOW_SUCCESS_RUN: &str = "Neow Success Run";

pub const DESIGNER_INSPIRE_BASE: &str = "Designer In-Spire Base";
pub const DESIGNER_INSPIRE_UP2: &str = "Designer In-Spire Upgrade 2";
pub const DESIGNER_INSPIRE_TRANS2: &str = "Designer In-Spire Transform 2";
pub const DESIGNER_INSPIRE_UP2_TRANS2: &str = "Designer In-Spire Transform and Upgrade 2";