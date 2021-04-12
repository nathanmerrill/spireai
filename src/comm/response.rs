use crate::comm::request::GameState;
use crate::spireai::Choice;

pub fn serialize_response(response: &Response, state: &Option<GameState>) -> String {
    match response {
        Response::Simple(literal) => literal.to_string(),
        Response::Choose(name) => {
            let selection = state.as_ref().unwrap().choice_list.iter()
                .position(|a| a.as_str().to_ascii_lowercase() == name.to_ascii_lowercase())
                .expect(format!("Could not find option with name {}", name).as_str());

            format!("CHOOSE {}", selection)
        },
        Response::Card(id) => {
            match &state.as_ref().unwrap().screen_state {
                crate::comm::request::ScreenState::Grid(grid) => {
                    pick_by_id(id, &grid.cards)
                },
                crate::comm::request::ScreenState::CardReward(reward) => {
                    pick_by_id(id, &reward.cards)
                },
                _ => {
                    panic!("Unexpected Card choice")
                }
            }
        }
    }
}

fn pick_by_id(id: &String, cards: &Vec<crate::comm::request::Card>) -> String {
    let selection = cards.iter()
        .position(|a| a.id.as_str() == id)
        .expect(format!("Could not find card with id {}", id).as_str());

    format!("CHOOSE {}", selection)
}


pub enum Response {
    Simple(String),
    Choose(String),
    Card(String),
}

fn fmt_opt_i(i: Option<u8>) -> String {
    i.map(|a| a.to_string()).unwrap_or(String::default())
}

pub fn decompose_choice(choice: Choice) -> Vec<Response> {
    match choice {
        Choice::Start {player_class, ascension} => vec![
            Response::Simple(format!("START {} {}", player_class, fmt_opt_i(ascension)))
        ],
        Choice::DiscardPotion {slot} => vec![
            Response::Simple(format!("POTION Discard {}", slot))
        ],
        Choice::DrinkPotion {slot, target_index} => vec![
            Response::Simple(format!("POTION Use {} {}", slot, fmt_opt_i(target_index)))
        ],
        Choice::PlayCard {card_index, target_index} => vec![
            Response::Simple(format!("PLAY {} {}", (card_index+1)%10, fmt_opt_i(target_index)))
        ],
        Choice::End => vec![Response::Simple(String::from("END"))],
        Choice::EnterShop => vec![Response::Simple(String::from("CHOOSE 0"))],
        Choice::EventChoice(name) => vec![Response::Choose(String::from(name))],
        Choice::Proceed => vec![Response::Simple(String::from("PROCEED"))],
        Choice::Return => vec![Response::Simple(String::from("RETURN"))],
        Choice::State => vec![Response::Simple(String::from("STATE"))],
        Choice::Skip => vec![Response::Simple(String::from("SKIP"))],
        Choice::SingingBowl => vec![Response::Simple(String::from("SINGING_BOWL"))],
        Choice::BuyCard(card) => vec![Response::Choose(String::from(card))],
        Choice::BuyPotion(potion) => vec![Response::Choose(String::from(potion))],
        Choice::BuyRelic(relic) => vec![Response::Choose(String::from(relic))],
        Choice::BuyRemoveCard(card) => vec![
            Response::Choose(String::from("purge")),
            Response::Card(String::from(card))
        ],
        Choice::TakeReward(idx) => vec![Response::Simple(format!("CHOOSE {}", idx))],
        Choice::NavigateToNode(idx) => vec![Response::Choose(format!("x={}",idx))],
        Choice::SelectCard(card) => vec![Response::Choose(String::from(card))],
        Choice::Rest => vec![
            Response::Choose(String::from("rest")),
            Response::Simple(String::from("PROCEED"))
        ],
        Choice::Smith => vec![
            Response::Choose(String::from("smith")),
            Response::Simple(String::from("PROCEED"))
        ],
        Choice::Dig => vec![
            Response::Choose(String::from("dig")),
            Response::Simple(String::from("PROCEED"))
        ],
        Choice::Lift => vec![
            Response::Choose(String::from("lift")),
            Response::Simple(String::from("PROCEED"))
        ],
        Choice::Recall => vec![
            Response::Choose(String::from("recall")),
            Response::Simple(String::from("PROCEED"))
        ],
        Choice::Toke(card) => vec![
            Response::Choose(String::from("toke")),
            Response::Card(String::from(card))
        ],
        Choice::ScryDiscard(cards) => grid_choice(cards, true),
        Choice::DeckUpgrade(cards) => grid_choice(cards, true),
        Choice::DeckTransform(cards, _) => grid_choice(cards, true),
        Choice::DeckRemove(cards) => grid_choice(cards, true),
        Choice::OpenChest => vec![Response::Simple(String::from("CHOOSE 0"))],
    }
}
    
fn grid_choice(cards: Vec<String>, add_confirm: bool) -> Vec<Response> {
    let mut response: Vec<Response> = Vec::new();
    for card in cards {
        response.push(Response::Card(String::from(card)));
    }
    if add_confirm {
        response.push(Response::Simple(String::from("Confirm")));
    }
    response
}