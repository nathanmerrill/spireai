use crate::models;
use crate::models::core::CardDestination;
use crate::spireai::*;
use crate::state::core::Card;
use crate::state::game::FloorState;
use models::choices::Choice;

use super::evaluator::GamePossibility;

pub fn predict_outcome(choice: &Choice, possibility: &mut GamePossibility) {
    match choice {
        Choice::BuyCard(name) => {
            let cost = if let FloorState::Shop { ref mut cards, .. } = possibility.state.floor_state
            {
                let position = cards
                    .iter()
                    .position(|(card, _)| &card.base.name == name)
                    .expect("Unable to find card that matches in shop");
                let (_, cost) = cards.remove(position);
                cost
            } else {
                panic!("Expected store floor state")
            };

            possibility.spend_money(cost, true);
            possibility.add_card(Card::by_name(name), CardDestination::DeckPile);
        }
        Choice::BuyPotion(name) => {
            let cost = if let FloorState::Shop {
                ref mut potions, ..
            } = possibility.state.floor_state
            {
                let position = potions
                    .iter()
                    .position(|(potion, _)| potion == name)
                    .expect("Unable to find potion that matches in shop");
                let (_, cost) = potions.remove(position);
                cost
            } else {
                panic!("Expected store floor state");
            };

            possibility.spend_money(cost, true);
            possibility.state.add_potion(name);
        }
        Choice::BuyRelic(name) => {
            let cost =
                if let FloorState::Shop { ref mut relics, .. } = possibility.state.floor_state {
                    let position = relics
                        .iter()
                        .position(|(relic, _)| relic == name)
                        .expect("Unable to find relic that matches in shop");
                    let (_, cost) = relics.remove(position);
                    cost
                } else {
                    panic!("Expected store floor state");
                };

            possibility.spend_money(cost, true);
            possibility.add_relic(name);
        }
        Choice::BuyRemoveCard(card) => {
            let cost = if let FloorState::Shop {
                ref mut purge_cost, ..
            } = possibility.state.floor_state
            {
                let cost_ret = *purge_cost;
                *purge_cost = 0;
                cost_ret
            } else {
                panic!("Expected store floor state");
            };

            possibility.spend_money(cost, true);
            possibility.state.remove_card(*card);
        }
        Choice::DeckRemove(cards) => {
            for card in cards {
                possibility.state.remove_card(*card);
            }
        }
        Choice::DeckTransform(cards, should_upgrade) => {
            let sets: Vec<Vec<&&'static models::cards::BaseCard>> = cards
                .iter()
                .map(|p| {
                    let (class, name) = {
                        let card = &possibility.state.deck[&p.uuid];
                        (card.base._class, card.base.name.to_string())
                    };

                    let available_cards: Vec<&&'static models::cards::BaseCard> =
                        models::cards::available_cards_by_class(class)
                            .iter()
                            .filter(move |c| c.name != name)
                            .collect();

                    available_cards
                })
                .collect();

            for card in cards {
                possibility.state.remove_card(*card);
            }

            for set in sets {
                let base = possibility.probability.choose(set).unwrap();
                if let Some(card_ref) =
                    possibility.add_card(Card::new(base), CardDestination::DeckPile)
                {
                    if *should_upgrade {
                        possibility.state.get_mut(card_ref).upgrade()
                    }
                }
            }
        }
        Choice::DeckUpgrade(cards) => {
            for card in cards {
                possibility.state.get_mut(*card).upgrade();
            }
        }
        Choice::Dig => {
            let relic = possibility.random_relic(None);
            possibility.add_relic(relic);
        }
        Choice::DiscardPotion { slot } => {
            possibility.state.potions[*slot] = None;
        }
        Choice::DrinkPotion { slot, target } => possibility.drink_potion(
            possibility.state.potion_at(*slot).unwrap(),
            target.map(|m| m.creature_ref()),
        ),
        Choice::End => possibility.end_turn(),
        _ => unimplemented!(),
    }
}

#[allow(unused_variables)]
pub fn verify_prediction<'a>(outcome: &'a GameState, choice: &'a GamePossibility) -> &'a GameState {
    unimplemented!()
    /*
    let matches: Vec<&GameState> = prediction
        .states
        .iter()
        .map(|a| &a.state)
        .filter(|a| a == &outcome)
        .collect();
    match matches.len() {
        0 => panic!("New state did not match any of the predicted states.\n New state: {:?}.\n\n Expected states: {:?}", outcome, prediction.states),
        1 => matches.get(0).unwrap(),
        _ => panic!("New state matched multiple predicted states.\n New state: {:?}.\n\n Expected states: {:?}", outcome, prediction.states),
    }
    */
}
