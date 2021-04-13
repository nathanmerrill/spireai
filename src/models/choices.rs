use crate::models::core::Class;

pub enum Choice {
    Start {
        player_class: Class,
        ascension: Option<u8>,
    },
    DrinkPotion {
        slot: u8,
        target_index: Option<u8>,
    },
    DiscardPotion {
        slot: u8,
    },
    PlayCard {
        card_index: u8,
        target_index: Option<u8>,
    },
    EventChoice(&'static str),
    NavigateToNode(i8),
    TakeReward(u8),
    SelectCard(&'static str),
    BuyCard(String),
    BuyRelic(&'static str),
    BuyPotion(&'static str),
    BuyRemoveCard(String),
    DeckRemove(Vec<String>),
    DeckTransform(Vec<String>, bool), //And upgrade if true
    DeckUpgrade(Vec<String>),
    OpenChest,
    Rest,
    RestDreamCatcher,
    Smith(String),
    Lift,
    Dig,
    ScryDiscard(Vec<String>),
    Recall,
    Toke(String),
    EnterShop,
    End,
    Proceed,
    Return,
    Skip,
    SingingBowl,
    State,
}