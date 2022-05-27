use im::HashMap;
use uuid::Uuid;

use crate::comm::request::*;
use crate::models::choices::Choice;
use crate::spireai::references::{CardReference, MonsterReference};
use crate::state::game::DeckCard;
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
        Response::ChooseIdx(idx) => {
            format!("CHOOSE {}", idx)
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
    ChooseIdx(usize),
    CardInGrid(usize),
    CardInDeck(usize),
}

fn fmt_opt_i<T: std::fmt::Display>(i: Option<T>) -> String {
    i.map(|a| a.to_string()).unwrap_or_default()
}

pub fn decompose_choice(
    choice: Choice,
    request: &Request,
    uuid_map: &HashMap<String, Uuid>,
) -> Vec<Response> {
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
        Choice::DrinkPotion { slot, target } => {
            let monster = target.map(|c| get_monster_position(c, request, uuid_map));
            vec![Response::Simple(format!(
                "POTION Use {} {}",
                slot,
                fmt_opt_i(monster)
            ))]
        }
        Choice::PlayCard { card, target } => {
            let position = get_card_position(card, request, uuid_map);
            let monster = target.map(|c| get_monster_position(c, request, uuid_map));
            vec![Response::Simple(format!(
                "PLAY {} {}",
                (position + 1) % 10, // "0" selects the 10th card
                fmt_opt_i(monster)
            ))]
        }
        Choice::End => vec![Response::Simple(String::from("END"))],
        Choice::EnterShop => vec![Response::Simple(String::from("CHOOSE 0"))],
        Choice::Event(name) => vec![Response::Choose(name)],
        Choice::Proceed => vec![Response::Simple(String::from("PROCEED"))],
        Choice::State => vec![Response::Simple(String::from("STATE"))],
        Choice::Skip => vec![Response::Simple(String::from("SKIP"))],
        Choice::SingingBowl => vec![Response::Simple(String::from("SINGING_BOWL"))],
        Choice::BuyCard(card) => vec![Response::ChooseIdx(card)],
        Choice::BuyPotion(potion) => vec![Response::ChooseIdx(potion)],
        Choice::BuyRelic(relic) => vec![Response::ChooseIdx(relic)],
        Choice::BuyRemoveCard(card) => vec![
            Response::Choose(String::from("purge")),
            Response::CardInDeck(get_card_position_in_deck(card, request, uuid_map)),
        ],
        Choice::TakeReward(idx) => vec![Response::Simple(format!("CHOOSE {}", idx))],
        Choice::NavigateToNode(idx) => vec![Response::Choose(format!("x={}", idx))],
        Choice::AddCardToDeck(card) => vec![Response::Choose(card)],
        Choice::Rest => vec![
            Response::Choose(String::from("rest")),
            Response::Simple(String::from("PROCEED")),
        ],
        Choice::Smith => vec![
            Response::Choose(String::from("smith")),
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
        Choice::Toke => vec![
            Response::Choose(String::from("toke")),
        ],
        Choice::SelectCards(cards) | Choice::Scry(cards) => {
            let available_cards = &request
                .game_state
                .as_ref()
                .unwrap()
                .combat_state
                .as_ref()
                .unwrap()
                .limbo;

            cards
                .into_iter()
                .map(|card| {
                    let (card_id, _) = uuid_map.iter().find(|(_, id)| **id == card.uuid).unwrap();
                    let (position, _) = available_cards
                        .iter()
                        .enumerate()
                        .find(|(_, card)| &card.id == card_id)
                        .unwrap();
                    Response::CardInGrid(position)
                })
                .chain(once(Response::Simple(String::from("CONFIRM"))))
                .collect()
        }

        Choice::DeckSelect(cards, _) => cards
            .into_iter()
            .map(|card| Response::CardInDeck(get_card_position_in_deck(card, request, uuid_map)))
            .chain(once(Response::Simple(String::from("CONFIRM"))))
            .collect(),

        Choice::OpenChest => vec![Response::Simple(String::from("CHOOSE 0"))],
    }
}

fn uuid_to_id(uuid: Uuid, uuid_map: &HashMap<String, Uuid>) -> &String {
    uuid_map.iter().find(|(_, id)| **id == uuid).unwrap().0
}

fn get_monster_position(
    monster: MonsterReference,
    request: &Request,
    uuid_map: &HashMap<String, Uuid>,
) -> usize {
    let id = uuid_to_id(monster.uuid, uuid_map);
    let combat_state = request
        .game_state
        .as_ref()
        .unwrap()
        .combat_state
        .as_ref()
        .unwrap();
    combat_state
        .monsters
        .iter()
        .position(|card| &card.id == id)
        .unwrap()
}

fn get_card_position_in_deck(
    card: DeckCard,
    request: &Request,
    uuid_map: &HashMap<String, Uuid>,
) -> usize {
    let id = uuid_to_id(card.uuid, uuid_map);
    request
        .game_state
        .as_ref()
        .unwrap()
        .deck
        .iter()
        .position(|card| &card.id == id)
        .unwrap()
}

fn get_card_position(
    card: CardReference,
    request: &Request,
    uuid_map: &HashMap<String, Uuid>,
) -> usize {
    let id = uuid_to_id(card.uuid, uuid_map);
    let combat_state = request
        .game_state
        .as_ref()
        .unwrap()
        .combat_state
        .as_ref()
        .unwrap();
    combat_state
        .limbo
        .iter()
        .position(|card| &card.id == id)
        .or_else(|| combat_state.hand.iter().position(|card| &card.id == id))
        .or_else(|| {
            combat_state
                .draw_pile
                .iter()
                .position(|card| &card.id == id)
        })
        .or_else(|| {
            combat_state
                .discard_pile
                .iter()
                .position(|card| &card.id == id)
        })
        .or_else(|| {
            combat_state
                .exhaust_pile
                .iter()
                .position(|card| &card.id == id)
        })
        .unwrap()
}
