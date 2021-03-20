use crate::models::cards;
use crate::models::relics;
use crate::models::core::*;
use std::collections::HashMap;
use Amount::*;

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

fn all_events() -> Vec<BaseEvent> {
    vec![
        BaseEvent {
            name: NEOW,
            choices: vec![
                BaseEventChoice {
                    name: "Talk",
                    effects: vec![],
                    allowed: StaticCondition::True,
                }
            ],
            shrine: false,         
        },
        BaseEvent {
            name: ANCIENT_WRITING,
            choices: vec![
                BaseEventChoice {
                    name: "Elegance",
                    effects: vec![
                        Effect::RemoveCard(1),
                    ],
                    allowed: StaticCondition::True,
                },
                BaseEventChoice {
                    name: "Simplicity",
                    effects: vec![
                        Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::All))
                    ],
                    allowed: StaticCondition::True,
                },
            ],
            shrine: false,         
        },
        BaseEvent {
            name: AUGMENTER,
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
                    allowed: StaticCondition::True,
                },
                BaseEventChoice {
                    name: "Become Test Subject",
                    effects: vec![
                        Effect::TransformCard(2),
                    ],
                    allowed: StaticCondition::DeckSize(2),
                },
                BaseEventChoice {
                    name: "Ingest Mutagens",
                    effects: vec![
                        Effect::AddRelic(relics::MUTAGENIC_STRENGTH),
                    ],
                    allowed: StaticCondition::True,
                },
            ],
            shrine: false,         
        },
        BaseEvent {
            name: BIG_FISH,
            choices: vec![
                BaseEventChoice {
                    name: "Banana",
                    effects: vec![
                        Effect::AddCard {
                            card: CardReference::ByName(cards::JAX),
                            destination: CardLocation::DeckPile(RelativePosition::Bottom),
                            copies: Amount::Fixed(1),
                            modifier: CardModifier::None,
                        }
                    ],
                    allowed: StaticCondition::True,
                },
                BaseEventChoice {
                    name: "Donut",
                    effects: vec![
                        Effect::TransformCard(2),
                    ],
                    allowed: StaticCondition::DeckSize(2),
                },
                BaseEventChoice {
                    name: "Box",
                    effects: vec![
                        Effect::AddRelic(relics::MUTAGENIC_STRENGTH),
                    ],
                    allowed: StaticCondition::True,
                },
            ],
            shrine: false,         
        },

    ]
}

pub const NEOW: &str = "Neow";
pub const ANCIENT_WRITING: &str = "Ancient Writing";
pub const AUGMENTER: &str = "Augmenter";
pub const BIG_FISH: &str = "Big Fish";
