use crate::{
    models::core::*,
    spireai::references::{CardReference, MonsterReference},
    state::game::DeckCard,
};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Choice {
    Start {
        player_class: Class,
        ascension: Option<u8>,
    },
    DrinkPotion {
        slot: usize,
        target: Option<MonsterReference>,
    },
    DiscardPotion {
        slot: usize,
    },
    PlayCard {
        card: CardReference,
        target: Option<MonsterReference>,
    },
    Event(String),
    NavigateToNode(i8),
    TakeReward(usize),
    SelectCard(String),
    SelectCards(Vec<CardReference>),
    BuyCard(String),
    BuyRelic(String),
    BuyPotion(String),
    BuyRemoveCard(DeckCard),
    DeckSelect(Vec<DeckCard>, DeckOperation),
    OpenChest,
    Rest,
    Smith(DeckCard),
    Lift,
    Dig,
    Recall,
    Toke(DeckCard),
    EnterShop,
    End,
    Proceed,
    Skip,
    SingingBowl,
    State,
}
