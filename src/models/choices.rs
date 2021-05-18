use crate::models::core::*;

pub enum Choice {
    Start {
        player_class: Class,
        ascension: Option<u8>,
    },
    DrinkPotion {
        slot: usize,
        target_index: Option<usize>,
    },
    DiscardPotion {
        slot: usize,
    },
    PlayCard {
        card_index: usize,
        target_index: Option<usize>,
    },
    EventChoice(String),
    NavigateToNode(i8),
    TakeReward(usize),
    SelectCard(String),
    BuyCard(String),
    BuyRelic(String),
    BuyPotion(String),
    BuyRemoveCard(usize),
    DeckRemove(Vec<usize>),
    DeckTransform(Vec<usize>, bool), //And upgrade if true
    DeckUpgrade(Vec<usize>),
    OpenChest,
    Rest,
    RestDreamCatcher,
    Smith(usize),
    Lift,
    Dig,
    ScryDiscard(Vec<usize>),
    Recall,
    Toke(usize),
    EnterShop,
    End,
    Proceed,
    Return,
    Skip,
    SingingBowl,
    State,
}
