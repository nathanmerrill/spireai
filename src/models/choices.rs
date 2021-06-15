use crate::{models::core::*, spireai::evaluator::{CardReference, CreatureReference}};

pub enum Choice {
    Start {
        player_class: Class,
        ascension: Option<u8>,
    },
    DrinkPotion {
        slot: usize,
        target: Option<CreatureReference>,
    },
    DiscardPotion {
        slot: usize,
    },
    PlayCard {
        card: CardReference,
        target: Option<CreatureReference>,
    },
    EventChoice(String),
    NavigateToNode(i8),
    TakeReward(usize),
    SelectCard(String),
    BuyCard(String),
    BuyRelic(String),
    BuyPotion(String),
    BuyRemoveCard(CardReference),
    DeckRemove(Vec<CardReference>),
    DeckTransform(Vec<CardReference>, bool), //And upgrade if true
    DeckUpgrade(Vec<CardReference>),
    OpenChest,
    Rest,
    RestDreamCatcher,
    Smith(CardReference),
    Lift,
    Dig,
    ScryDiscard(Vec<CardReference>),
    Recall,
    Toke(CardReference),
    EnterShop,
    End,
    Proceed,
    Return,
    Skip,
    SingingBowl,
    State,
}
