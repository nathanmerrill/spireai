use crate::comm::request::*;
use crate::models::choices::Choice;
use std::iter::once;

pub fn serialize_response(response: &Response, state: &Option<GameState>) -> String {
    match response {
        Response::Simple(literal) => literal.to_string(),
        Response::Choose(name) => {
            let selection = state
                .as_ref()
                .unwrap()
                .choice_list
                .iter()
                .position(|a| a.as_str().to_ascii_lowercase() == name.to_ascii_lowercase())
                .unwrap_or_else(|| panic!("Could not find option with name {}", name));

            format!("CHOOSE {}", selection)
        }
        Response::CardInGrid(position) => format!("CHOOSE {}", position),
        Response::CardInDeck(position) => {
            let game_state = state.as_ref().unwrap();
            let ref_card = &game_state.deck[*position];
            if let ScreenState::Grid(grid) = &game_state.screen_state {
                let position = grid
                    .cards
                    .iter()
                    .position(|card| card.id == ref_card.id)
                    .unwrap_or_else(|| panic!("Did not find card {} in the grid", ref_card.name));

                format!("CHOOSE {}", position)
            } else {
                panic!("Expected a Grid in CardInDeck")
            }
        }
    }
}

pub enum Response {
    Simple(String),
    Choose(String),
    CardInGrid(usize),
    CardInDeck(usize),
}

fn fmt_opt_i<T: std::fmt::Display>(i: Option<T>) -> String {
    i.map(|a| a.to_string()).unwrap_or_default()
}

pub fn decompose_choice(choice: Choice) -> Vec<Response> {
    match choice {
        Choice::Start {
            player_class,
            ascension,
        } => vec![Response::Simple(format!(
            "START {} {}",
            player_class,
            fmt_opt_i(ascension)
        ))],
        Choice::DiscardPotion { slot } => {
            vec![Response::Simple(format!("POTION Discard {}", slot))]
        }
        Choice::DrinkPotion { slot, target_index } => vec![Response::Simple(format!(
            "POTION Use {} {}",
            slot,
            fmt_opt_i(target_index)
        ))],
        Choice::PlayCard {
            card_index,
            target_index,
        } => vec![Response::Simple(format!(
            "PLAY {} {}",
            (card_index + 1) % 10,
            fmt_opt_i(target_index)
        ))],
        Choice::End => vec![Response::Simple(String::from("END"))],
        Choice::EnterShop => vec![Response::Simple(String::from("CHOOSE 0"))],
        Choice::EventChoice(name) => vec![Response::Choose(name)],
        Choice::Proceed => vec![Response::Simple(String::from("PROCEED"))],
        Choice::Return => vec![Response::Simple(String::from("RETURN"))],
        Choice::State => vec![Response::Simple(String::from("STATE"))],
        Choice::Skip => vec![Response::Simple(String::from("SKIP"))],
        Choice::SingingBowl => vec![Response::Simple(String::from("SINGING_BOWL"))],
        Choice::BuyCard(card) => vec![Response::Choose(card)],
        Choice::BuyPotion(potion) => vec![Response::Choose(potion)],
        Choice::BuyRelic(relic) => vec![Response::Choose(relic)],
        Choice::BuyRemoveCard(card) => vec![
            Response::Choose(String::from("purge")),
            Response::CardInDeck(card),
        ],
        Choice::TakeReward(idx) => vec![Response::Simple(format!("CHOOSE {}", idx))],
        Choice::NavigateToNode(idx) => vec![Response::Choose(format!("x={}", idx))],
        Choice::SelectCard(card) => vec![Response::Choose(card)],
        Choice::RestDreamCatcher => vec![Response::Choose(String::from("rest"))],
        Choice::Rest => vec![
            Response::Choose(String::from("rest")),
            Response::Simple(String::from("PROCEED")),
        ],
        Choice::Smith(card) => vec![
            Response::Choose(String::from("smith")),
            Response::CardInDeck(card),
            Response::Simple(String::from("CONFIRM")),
            Response::Simple(String::from("PROCEED")),
        ],
        Choice::Dig => vec![
            Response::Choose(String::from("dig")),
            Response::Simple(String::from("PROCEED")),
        ],
        Choice::Lift => vec![
            Response::Choose(String::from("lift")),
            Response::Simple(String::from("PROCEED")),
        ],
        Choice::Recall => vec![
            Response::Choose(String::from("recall")),
            Response::Simple(String::from("PROCEED")),
        ],
        Choice::Toke(card) => vec![
            Response::Choose(String::from("toke")),
            Response::CardInDeck(card),
        ],
        Choice::ScryDiscard(cards) => cards
            .into_iter()
            .map(Response::CardInGrid)
            .chain(once(Response::Simple(String::from("CONFIRM"))))
            .collect(),

        Choice::DeckUpgrade(cards)
        | Choice::DeckTransform(cards, _)
        | Choice::DeckRemove(cards) => cards
            .into_iter()
            .map(Response::CardInDeck)
            .chain(once(Response::Simple(String::from("CONFIRM"))))
            .collect(),

        Choice::OpenChest => vec![Response::Simple(String::from("CHOOSE 0"))],
    }
}
