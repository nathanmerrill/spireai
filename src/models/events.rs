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
            initial: true,
        }
    }
}


fn leave(initial: bool) -> BaseEventChoice {
    BaseEventChoice {
        name: LEAVE,
        effects: vec![],
        condition: Condition::Always,
        initial: initial
    }
}


fn all_events() -> Vec<BaseEvent> {
    vec![
        BaseEvent {
            name: NEOW,
            shrine: false,
            condition: Condition::Always,
            variants: vec![
                NEOW_FIRST_RUN,
                NEOW_SUCCESS_RUN
            ],
            choices: vec![
                BaseEventChoice {
                    name: "Talk",
                    effects: vec![
                        Effect::ShowChoices(vec![
                            NEOW_ONE_HP,
                            NEOW_SIX_HP,
                            NEOW_SEVEN_HP,
                            NEOW_EIGHT_HP
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: NEOW_ONE_HP,
                    initial: false,
                    effects: vec![
                        Effect::AddRelic(relics::NEOWS_LAMENT),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: NEOW_SIX_HP,
                    initial: false,
                    effects: vec![
                        Effect::AddMaxHp(Amount::Fixed(6)),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    condition: Condition::Class(Class::Silent),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: NEOW_SEVEN_HP,
                    initial: false,
                    effects: vec![
                        Effect::AddMaxHp(Amount::Fixed(7)),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    condition: Condition::MultipleOr(vec![
                        Condition::Class(Class::Defect),
                        Condition::Class(Class::Watcher)
                    ]),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: NEOW_EIGHT_HP,
                    initial: false,
                    effects: vec![
                        Effect::AddMaxHp(Amount::Fixed(8)),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    condition: Condition::Class(Class::Ironclad),
                    ..BaseEventChoice::new()
                },
                leave(false)
            ],       
        },
        BaseEvent {
            name: ANCIENT_WRITING,
            shrine: false,
            condition: Condition::Always, 
            variants: vec![], 
            choices: vec![
                BaseEventChoice {
                    name: "Elegance",
                    effects: vec![
                        Effect::RemoveCard(1),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Simplicity",
                    effects: vec![
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::All)),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                leave(false)
            ],    
        },
        BaseEvent {
            name: AUGMENTER,
            shrine: false,
            condition: Condition::Always,
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
                        },
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Become Test Subject",
                    effects: vec![
                        Effect::TransformCard(2),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Ingest Mutagens",
                    effects: vec![
                        Effect::AddRelic(relics::MUTAGENIC_STRENGTH),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                leave(false)
            ],         
        },
        BaseEvent {
            name: BIG_FISH,
            shrine: false,
            condition: Condition::Always,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Banana",
                    effects: vec![
                        Effect::HealPercentage(Amount::Fixed(33), Target::_Self),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Donut",
                    effects: vec![
                        Effect::AddMaxHp(Amount::Fixed(5)),
                        Effect::ShowChoices(vec![LEAVE]),
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
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                leave(false),
            ],        
        },
        BaseEvent {
            name: BONFIRE_SPIRITS,
            shrine: true,
            condition: Condition::Always,
            variants: vec![], 
            choices: vec![
                BaseEventChoice {
                    name: "Offer",
                    effects: vec![
                        Effect::Custom,
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                leave(false),
            ],      
        },
        BaseEvent {
            name: THE_CLERIC,
            shrine: false,
            condition: Condition::HasGold(Amount::Fixed(35)),  
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Heal",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-35)),
                        Effect::HealPercentage(Amount::Fixed(25), Target::_Self),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    condition: Condition::HasGold(Amount::Fixed(35)),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Purify",
                    effects: vec![
                        Effect::AddGold(Amount::ByAsc(-50, -75, -75)),
                        Effect::RemoveCard(1),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    condition: Condition::HasGold(Amount::ByAsc(50, 75, 75)),
                    ..BaseEventChoice::new()
                },
                leave(true),
            ], 
        },
        BaseEvent {
            name: THE_COLOSSEUM,
            shrine: false,
            variants: vec![],
            condition: Condition::OnFloor(26),
            choices: vec![
                BaseEventChoice {
                    name: "Continue",
                    effects: vec![
                        Effect::Fight(vec![monsters::BLUE_SLAVER, monsters::RED_SLAVER], RoomType::HallwayFight),
                        Effect::ShowChoices(vec![
                            THE_COLOSSEUM_COWARDACE,
                            THE_COLOSSEUM_VICTORY,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: THE_COLOSSEUM_COWARDACE,
                    initial: false,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: THE_COLOSSEUM_VICTORY,
                    initial: false,
                    effects: vec![
                        Effect::Fight(vec![monsters::TASKMASTER, monsters::GREMLIN_NOB], RoomType::Elite),
                        Effect::ShowReward(vec![
                            RewardType::Gold(100, 100),
                            RewardType::Relic(Rarity::Rare),
                            RewardType::Relic(Rarity::Uncommon),
                            RewardType::StandardCard,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
            ],         
        },
        BaseEvent {
            name: COUNCIL_OF_GHOSTS,
            shrine: false,
            condition: Condition::Always,
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
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Refuse",
                    ..BaseEventChoice::new()
                },                
                leave(false),
            ]
        },
        BaseEvent {
            name: CURSED_TOME,
            shrine: false,
            condition: Condition::Always,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Read",
                    effects: vec![
                        Effect::ShowChoices(vec![CURSED_TOME_CONTINUE]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: CURSED_TOME_CONTINUE,
                    effects: vec![
                        Effect::AddN(Amount::Fixed(1)),
                        Effect::LoseHp(Amount::N, Target::_Self),
                        Effect::If(Condition::Equals(Amount::N, Amount::Fixed(3)), vec![
                            Effect::ShowChoices(vec![CURSED_TOME_CONTINUE])
                        ], vec![
                            Effect::ShowChoices(vec![CURSED_TOME_TAKE, CURSED_TOME_STOP])
                        ])
                    ],
                    initial: false,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: CURSED_TOME_TAKE,
                    effects: vec![
                        Effect::LoseHp(Amount::ByAsc(10, 15, 15), Target::_Self),
                        Effect::ShowReward(vec![RewardType::RandomBook]),
                    ],
                    initial: false,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: CURSED_TOME_STOP,
                    effects: vec![
                        Effect::LoseHp(Amount::Fixed(3), Target::_Self),
                    ],
                    initial: false,
                    ..BaseEventChoice::new()
                },
                leave(true),
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
            condition: Condition::OnFloor(7),  
            choices: vec![
                BaseEventChoice {
                    name: DEAD_ADVENTURER_SEARCH,
                    effects: vec![
                        // These values are compared against 78 to get the right chances
                        Effect::If(Condition::Equals(Amount::X, Amount::Fixed(0)), vec![
                            Effect::SetX(Amount::ByAsc(26, 42, 42)) // 25%, 35%
                        ], vec![
                            Effect::If(Condition::LessThan(Amount::X, Amount::Fixed(60)), vec![
                                Effect::SetX(Amount::ByAsc(78, 117, 117)) // 50%, 60%
                            ], vec![
                                Effect::SetX(Amount::ByAsc(243, 442, 442)) // 75%, 85%
                            ])
                        ]),
                        Effect::RandomChance(vec![
                            (Amount::X, Effect::Multiple(vec![
                                Effect::If(Condition::IsVariant(DEAD_ADVENTURER_GREMLIN_NOB), vec![
                                    Effect::Fight(vec![monsters::GREMLIN_NOB], RoomType::Elite)
                                ], vec![
                                    Effect::If(Condition::IsVariant(DEAD_ADVENTURER_LAGAVULIN), vec![
                                        Effect::Fight(vec![monsters::LAGAVULIN], RoomType::Elite)
                                    ], vec![
                                        Effect::Fight(vec![monsters::SENTRY, monsters::SENTRY, monsters::SENTRY], RoomType::Elite)
                                    ])
                                ]),
                                //+4 If relic has already been given
                                //+2 If gold has already been given
                                //+1 If nothing has already been given
                                Effect::If(Condition::LessThan(Amount::N, Amount::Fixed(2)), vec![
                                    Effect::ShowReward(vec![
                                        RewardType::RandomRelic,
                                        RewardType::Gold(55, 65),
                                        RewardType::EliteCard,
                                    ])
                                ], vec![
                                    Effect::If(Condition::LessThan(Amount::N, Amount::Fixed(4)), vec![
                                        Effect::ShowReward(vec![
                                            RewardType::RandomRelic,
                                            RewardType::EliteCard,
                                        ])
                                    ], vec![
                                        Effect::If(Condition::LessThan(Amount::N, Amount::Fixed(6)), vec![
                                            Effect::ShowReward(vec![
                                                RewardType::Gold(55, 65),
                                                RewardType::EliteCard,
                                            ])
                                        ], vec![
                                            Effect::ShowReward(vec![
                                                RewardType::EliteCard,
                                            ])
                                        ]),
                                    ]),
                                ]),
                            ])),
                            (Amount::Fixed(78), Effect::Multiple(vec![
                                Effect::If(Condition::Equals(Amount::N, Amount::Fixed(0)), vec![
                                    Effect::RandomChance(vec![
                                        (Amount::Fixed(33), Effect::AddN(Amount::Fixed(1))),
                                        (Amount::Fixed(33), Effect::Multiple(vec![
                                            Effect::AddGold(Amount::Fixed(30)),
                                            Effect::AddN(Amount::Fixed(2))
                                        ])),
                                        (Amount::Fixed(33), Effect::Multiple(vec![
                                            Effect::RandomRelic,
                                            Effect::AddN(Amount::Fixed(4))
                                        ])),
                                    ]),
                                ], vec![
                                    Effect::If(Condition::Equals(Amount::N, Amount::Fixed(1)), vec![
                                        Effect::RandomChance(vec![
                                            (Amount::Fixed(33), Effect::Multiple(vec![
                                                Effect::AddGold(Amount::Fixed(30)),
                                                Effect::AddN(Amount::Fixed(2))
                                            ])),
                                            (Amount::Fixed(33), Effect::Multiple(vec![
                                                Effect::RandomRelic,
                                                Effect::AddN(Amount::Fixed(4))
                                            ])),
                                        ]),
                                    ], vec![
                                        Effect::If(Condition::Equals(Amount::N, Amount::Fixed(2)), vec![
                                            Effect::RandomChance(vec![
                                                (Amount::Fixed(33), Effect::AddN(Amount::Fixed(1))),
                                                (Amount::Fixed(33), Effect::Multiple(vec![
                                                    Effect::RandomRelic,
                                                    Effect::AddN(Amount::Fixed(4))
                                                ])),
                                            ]),
                                        ], vec![
                                            Effect::If(Condition::Equals(Amount::N, Amount::Fixed(3)), vec![
                                                Effect::Multiple(vec![
                                                    Effect::RandomRelic,
                                                    Effect::AddN(Amount::Fixed(4))
                                                ]),
                                            ], vec![
                                                Effect::If(Condition::Equals(Amount::N, Amount::Fixed(4)), vec![
                                                    Effect::RandomChance(vec![
                                                        (Amount::Fixed(33), Effect::AddN(Amount::Fixed(1))),
                                                        (Amount::Fixed(33), Effect::Multiple(vec![
                                                            Effect::AddGold(Amount::Fixed(30)),
                                                            Effect::AddN(Amount::Fixed(2))
                                                        ])),
                                                    ]),
                                                ], vec![
                                                    Effect::If(Condition::Equals(Amount::N, Amount::Fixed(5)), vec![
                                                        Effect::Multiple(vec![
                                                            Effect::AddGold(Amount::Fixed(30)),
                                                            Effect::AddN(Amount::Fixed(2))
                                                        ]),
                                                    ], vec![
                                                        Effect::AddN(Amount::Fixed(1))
                                                    ]),    
                                                ]),
                                            ]),
                                        ]),
                                    ]),
                                ]),
                                Effect::If(Condition::Equals(Amount::N, Amount::Fixed(7)), vec![], vec![
                                    Effect::ShowChoices(vec![
                                        LEAVE,
                                        DEAD_ADVENTURER_SEARCH
                                    ])
                                ]),
                            ])),
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
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
            condition: Condition::HasGold(Amount::Fixed(75)),
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
            condition: Condition::HasCard(CardLocation::DeckPile(RelativePosition::All), CardType::Curse),
            choices: vec![
                BaseEventChoice {
                    name: "Drink",
                    effects: vec![
                        Effect::Custom
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: DUPLICATOR,
            shrine: true,
            condition: Condition::Always,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Pray",
                    effects: vec![
                        Effect::DuplicateCard,
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: FACE_TRADER,
            shrine: true,
            condition: Condition::Always,
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
                leave(true),
            ],
        },
        BaseEvent {
            name: FALLING,
            shrine: false,
            condition: Condition::Always,
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
            condition: Condition::Always,
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
            condition: Condition::Always,
            variants: vec![],
            choices: vec![
                BaseEventChoice {
                    name: "Take",
                    effects: vec![
                        Effect::AddRelic(relics::GOLDEN_IDOL),
                        Effect::ShowChoices(vec![
                            GOLDEN_IDOL_HIDE, 
                            GOLDEN_IDOL_OUTRUN,
                            GOLDEN_IDOL_SMASH
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: GOLDEN_IDOL_OUTRUN,
                    effects: vec![
                        Effect::AddCard{
                            card: CardReference::ByName(cards::INJURY),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                    ],
                    initial: false,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: GOLDEN_IDOL_SMASH,
                    effects: vec![
                        Effect::DamagePercentage(Amount::ByAsc(25, 35, 35)),
                    ],
                    initial: false,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: GOLDEN_IDOL_HIDE,
                    effects: vec![
                        Effect::ReduceMaxHpPercentage(Amount::ByAsc(8, 10, 10)),
                    ],
                    initial: false,
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: GOLDEN_SHRINE,
            shrine: true,
            condition: Condition::Always,
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
                leave(true),
            ],
        },
        BaseEvent {
            name: THE_JOUST,
            shrine: true,
            variants: vec![],
            condition: Condition::HasGold(Amount::Fixed(50)),
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
            condition: Condition::RemainingHp(Amount::Fixed(12), Target::_Self),
            choices: vec![
                BaseEventChoice {
                    name: KNOWING_SKULL_SUCCESS,
                    effects: vec![
                        Effect::LoseHp(Amount::Sum(vec![
                            Amount::Fixed(6),
                            Amount::Custom,
                        ]), Target::_Self),
                        Effect::AddCard {
                            card: CardReference::RandomClass(Class::None),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                        Effect::ShowChoices(vec![
                            KNOWING_SKULL_SUCCESS,
                            KNOWING_SKULL_PICK_ME_UP,
                            KNOWING_SKULL_RICHES,
                            KNOWING_SKULL_LEAVE
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: KNOWING_SKULL_PICK_ME_UP,
                    effects: vec![
                        Effect::LoseHp(Amount::Sum(vec![
                            Amount::Fixed(6),
                            Amount::Custom,
                        ]), Target::_Self),
                        Effect::RandomPotion,
                        Effect::ShowChoices(vec![
                            KNOWING_SKULL_SUCCESS,
                            KNOWING_SKULL_PICK_ME_UP,
                            KNOWING_SKULL_RICHES,
                            KNOWING_SKULL_LEAVE
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: KNOWING_SKULL_RICHES,
                    effects: vec![
                        Effect::LoseHp(Amount::Sum(vec![
                            Amount::Fixed(6),
                            Amount::Custom
                        ]), Target::_Self),
                        Effect::AddGold(Amount::Fixed(90)),
                        Effect::ShowChoices(vec![
                            KNOWING_SKULL_SUCCESS,
                            KNOWING_SKULL_PICK_ME_UP,
                            KNOWING_SKULL_RICHES,
                            KNOWING_SKULL_LEAVE
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: KNOWING_SKULL_LEAVE,
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
            condition: Condition::Always,
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
            condition: Condition::Always,
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
            condition: Condition::Always,
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
            condition: Condition::Always,
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
                        ], RoomType::HallwayFight),
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
        BaseEvent {
            name: MATCH_AND_KEEP,
            variants: vec![],
            shrine: true,
            condition: Condition::Always,
            choices: vec![],
        },
        BaseEvent {
            name: THE_MAUSOLEUM,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Open Coffin",
                    effects: vec![
                        Effect::RandomChance(vec![
                            (Amount::ByAsc(50, 100, 100), 
                            Effect::AddCard {
                                card: CardReference::ByName(cards::WRITHE),
                                destination: CardLocation::DeckPile(RelativePosition::Bottom),
                                copies: Amount::Fixed(1),
                                modifier: CardModifier::None,
                            })
                        ]),
                        Effect::RandomRelic,                        
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: MIND_BLOOM,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "I am War",
                    effects: vec![
                        Effect::RandomChance(vec![
                            (Amount::Fixed(33), Effect::Fight(vec![monsters::SLIME_BOSS], RoomType::Boss)),
                            (Amount::Fixed(33), Effect::Fight(vec![monsters::THE_GUARDIAN], RoomType::Boss)),
                            (Amount::Fixed(33), Effect::Fight(vec![monsters::HEXAGHOST], RoomType::Boss)),
                        ]),
                        Effect::If(Condition::Asc(15), vec![
                            Effect::ShowReward(vec![
                                RewardType::Relic(Rarity::Rare),
                                RewardType::Gold(25, 25),
                                RewardType::StandardCard
                            ])
                        ], vec![
                            Effect::ShowReward(vec![
                                RewardType::Relic(Rarity::Rare),
                                RewardType::Gold(50, 50),
                                RewardType::StandardCard
                            ])

                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "I am Awake",
                    effects: vec![
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::All)),
                        Effect::AddRelic(relics::MARK_OF_THE_BLOOM),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "I am Awake",
                    effects: vec![
                        Effect::If(Condition::OnFloor(41), vec![
                            Effect::HealPercentage(Amount::Fixed(100), Target::_Self),
                            Effect::AddCard {
                                card: CardReference::ByName(cards::DOUBT),
                                destination: CardLocation::DeckPile(RelativePosition::Bottom),
                                copies: Amount::Fixed(1),
                                modifier: CardModifier::None,
                            }
                        ], vec![
                            Effect::AddGold(Amount::Fixed(999)),
                            Effect::AddCard {
                                card: CardReference::ByName(cards::NORMALITY),
                                destination: CardLocation::DeckPile(RelativePosition::Bottom),
                                copies: Amount::Fixed(2),
                                modifier: CardModifier::None,
                            }
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: THE_MOAI_HEAD,
            variants: vec![],
            shrine: false,
            condition: Condition::MultipleOr(vec![
                Condition::HasRelic(relics::GOLDEN_IDOL),
                Condition::HalfHp(Target::_Self),
            ]),
            choices: vec![
                BaseEventChoice {
                    name: "Jump Inside",
                    effects: vec![
                        Effect::ReduceMaxHpPercentage(Amount::Custom),
                        Effect::HealPercentage(Amount::Fixed(100), Target::_Self),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Offer",
                    effects: vec![
                        Effect::RemoveRelic(relics::GOLDEN_IDOL),
                        Effect::AddGold(Amount::Fixed(333)),
                    ],
                    condition: Condition::HasRelic(relics::GOLDEN_IDOL),
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: MUSHROOMS,
            variants: vec![],
            shrine: false,
            condition: Condition::OnFloor(7),
            choices: vec![
                BaseEventChoice {
                    name: "Stomp",
                    effects: vec![
                        Effect::Fight(vec![monsters::FUNGI_BEAST, monsters::FUNGI_BEAST, monsters::FUNGI_BEAST], RoomType::HallwayFight),
                        Effect::ShowReward(vec![
                            RewardType::RelicName(relics::ODD_MUSHROOM),
                            RewardType::Gold(20, 30),
                            RewardType::StandardCard,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Eat",
                    effects: vec![
                        Effect::HealPercentage(Amount::Fixed(25), Target::_Self),
                        Effect::AddCard {
                            card: CardReference::ByName(cards::PARASITE),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        }
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: MYSTERIOUS_SPHERE,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Open Sphere",
                    effects: vec![
                        Effect::Fight(vec![monsters::ORB_WALKER, monsters::ORB_WALKER], RoomType::HallwayFight),
                        Effect::ShowReward(vec![
                            RewardType::Relic(Rarity::Rare),
                            RewardType::Gold(45, 55),
                            RewardType::StandardCard,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: THE_NEST,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Smash and Grab",
                    effects: vec![
                        Effect::AddGold(Amount::ByAsc(99, 50, 50)),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Stay in Line",
                    effects: vec![
                        Effect::Damage(Amount::Fixed(6), Target::_Self),
                        Effect::AddCard {
                            card: CardReference::ByName(cards::RITUAL_DAGGER),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        }
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: A_NOTE_FOR_YOURSELF,
            variants: vec![],
            shrine: true,
            condition: Condition::Asc(15),
            choices: vec![
                BaseEventChoice {
                    name: "Continue",
                    effects: vec![
                        Effect::ShowChoices(vec![A_NOTE_FOR_YOURSELF_TAKE, A_NOTE_FOR_YOURSELF_IGNORE])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: A_NOTE_FOR_YOURSELF_TAKE,
                    effects: vec![
                        Effect::Custom,
                    ],
                    initial: false,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: A_NOTE_FOR_YOURSELF_IGNORE,
                    initial: false,
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: NLOTH,
            variants: vec![],
            shrine: true,
            condition: Condition::Custom,
            choices: vec![
                BaseEventChoice {
                    name: "Offer Relic",
                    effects: vec![
                        Effect::Custom,
                        Effect::AddRelic(relics::NLOTHS_GIFT),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Offer Relic",
                    effects: vec![
                        Effect::Custom,
                        Effect::AddRelic(relics::NLOTHS_GIFT),
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: OLD_BEGGAR,
            variants: vec![],
            shrine: false,
            condition: Condition::HasGold(Amount::Fixed(75)),
            choices: vec![
                BaseEventChoice {
                    name: "Offer Gold",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-75)),
                        Effect::RemoveCard(1),
                    ],
                    condition: Condition::MultipleAnd(vec![
                        Condition::HasRemoveableCards(1, CardType::All),
                        Condition::HasGold(Amount::Fixed(75)),
                    ]),
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: OMINOUS_FORGE,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Forge",
                    effects: vec![
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::PlayerChoice(Amount::Fixed(1)))),
                    ],
                    condition: Condition::HasUpgradableCard,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Rummage",
                    effects: vec![
                        Effect::AddRelic(relics::WARPED_TONGS),
                        Effect::AddCard {
                            card: CardReference::ByName(cards::PAIN),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: PLEADING_VAGRANT,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Offer Gold",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-85)),
                        Effect::RandomRelic,
                    ],
                    condition: Condition::HasGold(Amount::Fixed(85)),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Rob",
                    effects: vec![
                        Effect::RandomRelic,
                        Effect::AddCard {
                            card: CardReference::ByName(cards::SHAME),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: PURIFIER,
            variants: vec![],
            shrine: true,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Pray",
                    effects: vec![
                        Effect::RemoveCard(1),
                    ],
                    condition: Condition::HasRemoveableCards(1, CardType::All),
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: SCRAP_OOZE,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: SCRAP_OOZE_REACH_INSIDE,
                    effects: vec![
                        Effect::LoseHp(Amount::Sum(vec![Amount::ByAsc(3, 5, 5), Amount::N]), Target::_Self),
                        Effect::RandomChance(vec![
                            (Amount::Sum(vec![Amount::Fixed(25), Amount::X]), Effect::RandomRelic),
                            (Amount::Sum(vec![Amount::Fixed(75), Amount::NegX]), Effect::Multiple(vec![
                                Effect::AddN(Amount::Fixed(1)),
                                Effect::AddX(Amount::Fixed(10)),
                                Effect::If(Condition::Equals(Amount::X, Amount::Fixed(80)), vec![
                                    Effect::SetX(Amount::Fixed(75))
                                ], vec![]),
                                Effect::ShowChoices(vec![
                                    SCRAP_OOZE_REACH_INSIDE,
                                    LEAVE
                                ])
                            ])),
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: SECRET_PORTAL,
            variants: vec![],
            shrine: true,
            condition: Condition::Custom,
            choices: vec![
                BaseEventChoice {
                    name: "Enter the Portal",
                    effects: vec![
                        Effect::Custom,
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true)
            ],
        },
        BaseEvent {
            name: SENSORY_STONE,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Recall (1 card)",
                    effects: vec![
                        Effect::ShowReward(vec![
                            RewardType::ColorlessCard,
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Recall (2 card)",
                    effects: vec![
                        Effect::LoseHp(Amount::Fixed(5), Target::_Self),
                        Effect::ShowReward(vec![
                            RewardType::ColorlessCard,
                            RewardType::ColorlessCard,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Recall (3 card)",
                    effects: vec![
                        Effect::LoseHp(Amount::Fixed(10), Target::_Self),
                        Effect::ShowReward(vec![
                            RewardType::ColorlessCard,
                            RewardType::ColorlessCard,
                            RewardType::ColorlessCard,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: SHINING_LIGHT,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Enter",
                    effects: vec![
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::Random)),
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::Random)),
                        Effect::DamagePercentage(Amount::ByAsc(20, 30, 30)),
                    ],
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: THE_SSSSSERPENT,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Agree",
                    effects: vec![
                        Effect::AddGold(Amount::ByAsc(175, 150, 150)),
                        Effect::AddCard {
                            card: CardReference::ByName(cards::DOUBT),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Disagree",
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: TOMB_OF_LORD_RED_MASK,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Offer Gold",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-32768)),
                        Effect::AddRelic(relics::RED_MASK),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Don the Mask",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(222)),
                        Effect::RemoveRelic(relics::RED_MASK),
                    ],
                    condition: Condition::HasRelic(relics::RED_MASK),
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: TRANSMOGRIFIER,
            variants: vec![],
            shrine: true,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Pray",
                    effects: vec![
                        Effect::TransformCard(1),
                    ],
                    condition: Condition::HasRemoveableCards(1, CardType::All),
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: UPGRADE_SHRINE,
            variants: vec![],
            shrine: true,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Pray",
                    effects: vec![
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::PlayerChoice(Amount::Fixed(1)))),
                    ],
                    condition: Condition::HasUpgradableCard,
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: VAMPIRES,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Accept",
                    effects: vec![
                        Effect::ReduceMaxHpPercentage(Amount::Fixed(30)),
                        Effect::Custom,
                        Effect::AddCard {
                            card: CardReference::ByName(cards::BITE),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(5),
                            modifier: CardModifier::None,
                        },
                    ],
                    condition: Condition::Not(Box::new(Condition::HasRelic(relics::BLOOD_VIAL))),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Accept",
                    effects: vec![
                        Effect::RemoveRelic(relics::BLOOD_VIAL),
                        Effect::Custom,
                        Effect::AddCard {
                            card: CardReference::ByName(cards::BITE),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(5),
                            modifier: CardModifier::None,
                        },
                    ],
                    condition: Condition::HasRelic(relics::BLOOD_VIAL),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Refuse",
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: WE_MEET_AGAIN,
            variants: vec![],
            shrine: true,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Give Potion",
                    effects: vec![
                        Effect::Custom,
                        Effect::RandomRelic,
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Give Gold",
                    effects: vec![
                        Effect::AddGold(Amount::Custom),
                        Effect::RandomRelic,
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Give Card",
                    effects: vec![
                        Effect::Custom,
                        Effect::RandomRelic,
                    ],
                    condition: Condition::Custom,
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Attack",
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: WHEEL_OF_CHANGE,
            variants: vec![],
            shrine: true,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Spin",
                    effects: vec![
                        Effect::RandomChance(vec![
                            (Amount::Fixed(1), Effect::If(Condition::Act(1), vec![
                                Effect::AddGold(Amount::Fixed(100)),
                            ], vec![
                                Effect::If(Condition::Act(2), vec![
                                    Effect::AddGold(Amount::Fixed(200)),
                                ], vec![
                                    Effect::AddGold(Amount::Fixed(300)),
                                ])
                            ])),
                            (Amount::Fixed(1), Effect::RandomRelic),
                            (Amount::Fixed(1), Effect::HealPercentage(Amount::Fixed(100), Target::_Self)),
                            (Amount::Fixed(1), Effect::AddCard {
                                card: CardReference::ByName(cards::DECAY),
                                destination: CardLocation::DeckPile(RelativePosition::Bottom),
                                copies: Amount::Fixed(1),
                                modifier: CardModifier::None,
                            }),
                            (Amount::Fixed(1), Effect::RemoveCard(1)),
                            (Amount::Fixed(1), Effect::DamagePercentage(Amount::ByAsc(10, 15, 15))),
                        ])
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: WINDING_HALLS,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Embrace Madness",
                    effects: vec![
                        Effect::AddCard {
                            card: CardReference::ByName(cards::MADNESS),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(2),
                            modifier: CardModifier::None,
                        },
                        Effect::DamagePercentage(Amount::Custom),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Focus",
                    effects: vec![
                        Effect::AddCard {
                            card: CardReference::ByName(cards::WRITHE),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        },
                        Effect::HealPercentage(Amount::ByAsc(25, 20, 20), Target::_Self),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Retrace Your Steps",
                    effects: vec![
                        Effect::ReduceMaxHpPercentage(Amount::Fixed(5)),
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: WING_STATUE,
            variants: vec![],
            shrine: false,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Pray",
                    effects: vec![
                        Effect::RemoveCard(1),
                        Effect::LoseHp(Amount::Fixed(7), Target::_Self)
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Destroy",
                    effects: vec![
                        Effect::AddGold(Amount::Custom),
                    ],
                    condition: Condition::Custom,
                    ..BaseEventChoice::new()
                },
                leave(true),
            ],
        },
        BaseEvent {
            name: THE_WOMAN_IN_BLUE,
            variants: vec![],
            shrine: true,
            condition: Condition::HasGold(Amount::Fixed(50)),
            choices: vec![
                BaseEventChoice {
                    name: "Buy 1 Potion",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-20)),
                        Effect::ShowReward(vec![
                            RewardType::RandomPotion,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Buy 2 Potions",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-30)),
                        Effect::ShowReward(vec![
                            RewardType::RandomPotion,
                            RewardType::RandomPotion,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Buy 3 Potions",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(-40)),
                        Effect::ShowReward(vec![
                            RewardType::RandomPotion,
                            RewardType::RandomPotion,
                            RewardType::RandomPotion,
                        ]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave",
                    effects: vec![
                        Effect::If(Condition::Asc(15), vec![
                            Effect::ReduceMaxHpPercentage(Amount::Fixed(5))
                        ], vec![])
                    ],
                    ..BaseEventChoice::new()
                },
            ],
        },
        BaseEvent {
            name: WORLD_OF_GOOP,
            variants: vec![],
            shrine: true,
            condition: Condition::Always,
            choices: vec![
                BaseEventChoice {
                    name: "Gather Gold",
                    effects: vec![
                        Effect::AddGold(Amount::Fixed(75)),
                        Effect::LoseHp(Amount::Fixed(11), Target::_Self),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Leave It",
                    effects: vec![
                        Effect::AddGold(Amount::Custom),
                        Effect::ShowChoices(vec![LEAVE]),
                    ],
                    ..BaseEventChoice::new()
                },
                leave(false),
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
pub const THE_WOMAN_IN_BLUE: &str = "The Woman in Blue";
pub const WORLD_OF_GOOP: &str = "World of Goop";

pub const LEAVE: &str = "Leave";

pub const A_NOTE_FOR_YOURSELF_TAKE: &str = "Take and Give";
pub const A_NOTE_FOR_YOURSELF_IGNORE: &str = "Ignore";

pub const DEAD_ADVENTURER_LAGAVULIN: &str = "Dead Adventurer Lagavulin";
pub const DEAD_ADVENTURER_SENTRY: &str = "Dead Adventurer Sentry";
pub const DEAD_ADVENTURER_GREMLIN_NOB: &str = "Dead Adventurer Nob";
pub const DEAD_ADVENTURER_SEARCH: &str = "Search";


pub const DESIGNER_INSPIRE_BASE: &str = "Designer In-Spire Base";
pub const DESIGNER_INSPIRE_UP2: &str = "Designer In-Spire Upgrade 2";
pub const DESIGNER_INSPIRE_TRANS2: &str = "Designer In-Spire Transform 2";
pub const DESIGNER_INSPIRE_UP2_TRANS2: &str = "Designer In-Spire Transform and Upgrade 2";

pub const NEOW_FIRST_RUN: &str = "Neow First Run";
pub const NEOW_SUCCESS_RUN: &str = "Neow Success Run";
pub const NEOW_ONE_HP: &str = "Enemies in your next three combats have 1 HP";
pub const NEOW_SIX_HP: &str = "Max HP +6";
pub const NEOW_SEVEN_HP: &str = "Max HP +7";
pub const NEOW_EIGHT_HP: &str = "Max HP +8";

pub const THE_COLOSSEUM_COWARDACE: &str = "Cowardace";
pub const THE_COLOSSEUM_VICTORY: &str = "Victory";

pub const CURSED_TOME_CONTINUE: &str = "Continue";
pub const CURSED_TOME_TAKE: &str = "Take";
pub const CURSED_TOME_STOP: &str = "Stop";

pub const GOLDEN_IDOL_SMASH: &str = "Smash";
pub const GOLDEN_IDOL_OUTRUN: &str = "Outrun";
pub const GOLDEN_IDOL_HIDE: &str = "Hide";

pub const KNOWING_SKULL_SUCCESS: &str = "Success?";
pub const KNOWING_SKULL_PICK_ME_UP: &str = "A Pick Me Up?";
pub const KNOWING_SKULL_RICHES: &str = "Riches?";
pub const KNOWING_SKULL_LEAVE: &str = "How do I leave?";

pub const SCRAP_OOZE_REACH_INSIDE: &str = "Reach Inside";