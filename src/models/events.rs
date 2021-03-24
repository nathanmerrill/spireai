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
                    condition: Condition::HasGold(35),
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Purify",
                    effects: vec![
                        Effect::If(Condition::Asc(15), vec![
                            Effect::AddGold(Amount::Fixed(-50))
                        ], vec![
                            Effect::AddGold(Amount::Fixed(-75))
                        ]),
                        Effect::RemoveCard(1),
                    ],
                    condition: Condition::MultipleOr(vec![
                        Condition::MultipleAnd(vec![
                            Condition::Asc(15),
                            Condition::HasGold(75),
                        ]),
                        Condition::MultipleAnd(vec![
                            Condition::Not(Box::new(Condition::Asc(15))),
                            Condition::HasGold(50),
                        ])
                    ]),
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
                        Effect::ReduceMaxHpPercentage(50),
                        Effect::If(Condition::Asc(15), vec![
                            Effect::AddCard {
                                card: CardReference::ByName(cards::APPARITION),
                                destination: CardLocation::DeckPile(RelativePosition::Bottom),
                                copies: Amount::Fixed(3),
                                modifier: CardModifier::None,
                            },
                        ], vec![
                            Effect::AddCard {
                                card: CardReference::ByName(cards::APPARITION),
                                destination: CardLocation::DeckPile(RelativePosition::Bottom),
                                copies: Amount::Fixed(5),
                                modifier: CardModifier::None,
                            },
                        ])
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
                                    Effect::If(Condition::Asc(15), vec![
                                        Effect::LoseHp(Amount::Fixed(15), Target::_Self),
                                    ], vec![
                                        Effect::LoseHp(Amount::Fixed(10), Target::_Self),
                                    ]),
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
                        Effect::If(Condition::Asc(15), vec![
                            Effect::AddX(Amount::Fixed(35))
                        ], vec![
                            Effect::AddX(Amount::Fixed(25))
                        ]),
                        Effect::RandomChance(Amount::X, vec![
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
                        ], vec![
                            Effect::RandomChance(Amount::Fixed(33), vec![
                                Effect::AddGold(Amount::Fixed(30)),
                                Effect::AddN(Amount::Fixed(1))
                            ], vec![
                                Effect::RandomChance(Amount::Fixed(50), vec![
                                    Effect::RandomRelic,
                                    Effect::AddN(Amount::Fixed(-1))
                                ], vec![])
                            ]),
                            Effect::AddX(Amount::Fixed(10)),
                            Effect::ShowChoices(vec![
                                BaseEventChoice {
                                    name: "Leave",
                                    ..BaseEventChoice::new()
                                },
                                BaseEventChoice {
                                    name: "Search",
                                    effects: vec![
                                        Effect::RandomChance(Amount::X, vec![
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
                                        ], vec![
                                            Effect::If(Condition::LessThan(Amount::N, Amount::Fixed(0)), vec![
                                                Effect::RandomChance(Amount::Fixed(50), vec![
                                                    Effect::AddGold(Amount::Fixed(30)),
                                                    Effect::AddN(Amount::Fixed(1))
                                                ], vec![])
                                            ], vec![
                                                Effect::If(Condition::LessThan(Amount::Fixed(0), Amount::N), vec![
                                                    Effect::RandomChance(Amount::Fixed(50), vec![
                                                        Effect::RandomRelic,
                                                        Effect::AddN(Amount::Fixed(-1))
                                                    ], vec![])
                                                ], vec![
                                                    Effect::RandomChance(Amount::Fixed(50), vec![
                                                        Effect::RandomRelic,
                                                        Effect::AddN(Amount::Fixed(-1))
                                                    ], vec![
                                                        Effect::AddGold(Amount::Fixed(30)),
                                                        Effect::AddN(Amount::Fixed(1))
                                                    ])
                                                ])
                                            ]),
                                            Effect::AddX(Amount::Fixed(10)),
                                            Effect::ShowChoices(vec![
                                                BaseEventChoice {
                                                    name: "Leave",
                                                    ..BaseEventChoice::new()
                                                },
                                                BaseEventChoice {
                                                    name: "Search",
                                                    effects: vec![
                                                        Effect::RandomChance(Amount::X, vec![
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
                                                        ], vec![
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
                                                        ])
                                                    ],
                                                    ..BaseEventChoice::new()
                                                }
                                            ])
                                        ])
                                    ],
                                    ..BaseEventChoice::new()
                                }
                            ])
                        ])
                    ],
                    ..BaseEventChoice::new()
                },

            ]
        },
        BaseEvent {
            name: DESIGNER_INSPIRE,
            variants: vec![],
            shrine: true,
            choices: vec![
                BaseEventChoice {
                    name: "Adjustments",
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Clean Up",
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Full Service",
                    ..BaseEventChoice::new()
                },
                BaseEventChoice {
                    name: "Punch",
                    ..BaseEventChoice::new()
                },
            ]
        }

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