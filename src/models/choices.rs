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
    EventChoice(&'static str),
    NavigateToNode(i8),
    TakeReward(usize),
    SelectCard(&'static str),
    BuyCard(&'static str),
    BuyRelic(&'static str),
    BuyPotion(&'static str),
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