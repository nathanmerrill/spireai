use crate::{models, spireai::evaluator::CreatureReference};
use crate::spireai::*;
use models::choices::Choice;
use models::core::*;
use models::state::*;
use crate::spireai::evaluator::*;

pub fn predict_outcome(choice: &Choice, state: &mut GamePossibility) {
    match choice {
        Choice::BuyCard(name) => {
            let cost = if let FloorState::Shop { ref mut cards, .. } = state.state.floor_state {
                let position = cards
                    .iter()
                    .position(|(card, _)| card == name)
                    .expect("Unable to find card that matches in shop");
                let (_, cost) = cards.remove(position);
                cost
            } else {
                panic!("Expected store floor state")
            };

            spend_money(cost, true, state.into());
            evaluator::add_card_to_deck(name, state.into());
        }
        Choice::BuyPotion(name) => {
            let cost = if let FloorState::Shop {
                ref mut potions, ..
            } = state.state.floor_state
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

            spend_money(cost, true, state.into());
            add_potion(name, state.into());
        }
        Choice::BuyRelic(name) => {
            let cost = if let FloorState::Shop { ref mut relics, .. } = state.state.floor_state {
                let position = relics
                    .iter()
                    .position(|(relic, _)| relic == name)
                    .expect("Unable to find relic that matches in shop");
                let (_, cost) = relics.remove(position);
                cost
            } else {
                panic!("Expected store floor state");
            };

            spend_money(cost, true, state.into());
            evaluator::add_relic(name, state.into());
        }
        Choice::BuyRemoveCard(card) => {
            let cost = if let FloorState::Shop {
                ref mut purge_cost, ..
            } = state.state.floor_state
            {
                let cost_ret = *purge_cost;
                *purge_cost = 0;
                cost_ret
            } else {
                panic!("Expected store floor state");
            };

            spend_money(cost, true, state.into());
            evaluator::remove_card_from_location(*card, &mut state.state);
        }
        Choice::DeckRemove(cards) => {
            for card in cards {
                evaluator::remove_card_from_location(*card, &mut state.state);
            }
        }
        Choice::DeckTransform(cards, should_upgrade) => {
            let sets: Vec<Vec<&&'static models::cards::BaseCard>> = cards.iter().map(|p| {
                let (class, name) = {
                    let card = &state.state.deck[&p.uuid];
                    (card.base._class, card.base.name.to_string())
                };

                let available_cards: Vec<&&'static models::cards::BaseCard> =
                    models::cards::available_cards_by_class(class)
                        .into_iter()
                        .filter(move |c| c.name != name)
                        .collect();

                available_cards
            }).collect();

            for card in cards {
                evaluator::remove_card_from_location(*card, &mut state.state);
            }

            for set in sets {
                let base = state.choose(set).unwrap();
                if let Some(card_ref) = evaluator::add_card_to_deck(&base.name, state) {
                    if *should_upgrade {
                        card_ref.get_mut(&mut state.state).upgrades = 1;
                    }
                }
            }
        }
        Choice::DeckUpgrade(cards) => {
            for card in cards {
                evaluator::upgrade_card(*card, &mut state.state);
            }
        }
        Choice::Dig => {
            add_random_relic((1, 2, 3), state);
        }
        Choice::DiscardPotion { slot } => {
            state.state.potions[*slot] = None;
        }
        Choice::DrinkPotion { slot, target } => {
            let potion = state.state.potions[*slot]
                .as_ref()
                .expect("Potion does not exist in slot!");
            evaluator::eval_effects(
                &potion.base.on_drink,
                state,
                evaluator::Binding::Potion(evaluator::PotionReference { potion: *slot }),
                Some(GameAction {
                    creature: CreatureReference::Player,
                    is_attack: false,
                    target: *target,
                }),
            );
        }
        Choice::End => {
            evaluator::end_turn(state)
        }
        _ => unimplemented!(),
    }
}

fn add_random_relic(weights: (u8, u8, u8), state: &mut GamePossibility) {
    let choices = vec![
        (Rarity::Common, weights.0),
        (Rarity::Uncommon, weights.1),
        (Rarity::Rare, weights.1),
    ];

    let rarity = state.choose_weighted(&choices).unwrap();

    let available_relics: Vec<&String> = models::relics::RELICS
        .values()
        .filter(|relic| {
            relic.rarity == *rarity
                && (relic.class == state.state.class || relic.class == Class::All)
                && !state.state.relic_names.contains_key(&relic.name)
        })
        .map(|relic| &relic.name)
        .collect();

    let relic: &String = state.choose(available_relics)
        .expect("No available relics to be chosen!");

    evaluator::add_relic(relic, state);
}

fn spend_money(amount: u16, at_shop: bool, state: &mut GameState) -> &Option<Relic> {
    state.gold -= amount;

    if at_shop {
        if let Some(relic) = evaluator::find_relic_mut(&String::from("Maw Bank"), state) {
            relic.enabled = false;
        }
    }
    &None
}

fn add_potion(name: &str, state: &mut GameState) {
    let potion = evaluator::create_potion(name);
    *state.potions.iter_mut().find(|a| a.is_none()).unwrap() = Some(potion);
}

pub fn verify_prediction<'a>(
    outcome: &'a GameState,
    choice: &'a GamePossibility,
) -> &'a GameState {
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
