use crate::models;
use crate::spireai::*;
use models::choices::Choice;
use models::core::*;
use models::state::*;
use rand::seq::SliceRandom;

pub fn predict_outcome(state: &GameState, choice: &Choice) -> GamePossibilitySet {
    let mut new_state = state.clone();
    let mut rng = rand::thread_rng();
    match choice {
        Choice::BuyCard(name) => {
            let cost = if let FloorState::Shop { ref mut cards, .. } = new_state.floor_state {
                let position = cards
                    .iter()
                    .position(|(card, _)| card == name)
                    .expect("Unable to find card that matches in shop");
                let (_, cost) = cards.remove(position);
                cost
            } else {
                panic!("Expected store floor state")
            };

            spend_money(cost, true, &mut new_state);
            evaluator::add_card_to_deck(name, false, &mut new_state);
            (new_state, 1.0)
        }

        Choice::BuyPotion(name) => {
            let cost = if let FloorState::Shop {
                ref mut potions, ..
            } = new_state.floor_state
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

            spend_money(cost, true, &mut new_state);
            add_potion(name, &mut new_state);
            (new_state, 1.0)
        }
        Choice::BuyRelic(name) => {
            let cost = if let FloorState::Shop { ref mut relics, .. } = new_state.floor_state {
                let position = relics
                    .iter()
                    .position(|(relic, _)| relic == name)
                    .expect("Unable to find relic that matches in shop");
                let (_, cost) = relics.remove(position);
                cost
            } else {
                panic!("Expected store floor state");
            };

            spend_money(cost, true, &mut new_state);
            add_relic(name, &mut new_state);
            (new_state, 1.0)
        }
        Choice::BuyRemoveCard(position) => {
            let cost = if let FloorState::Shop {
                ref mut purge_cost, ..
            } = new_state.floor_state
            {
                let cost_ret = *purge_cost;
                *purge_cost = 0;
                cost_ret
            } else {
                panic!("Expected store floor state");
            };

            spend_money(cost, true, &mut new_state);
            remove_card_from_deck(*position, &mut new_state);
            (new_state, 1.0)
        }
        Choice::DeckRemove(positions) => {
            for position in positions {
                remove_card_from_deck(*position, &mut new_state);
            }
            (new_state, 1.0)
        }
        Choice::DeckTransform(positions, should_upgrade) => {
            let mut state_space: f64 = 1.0;
            let sets = positions.iter().map(|p| {
                let card = &state.deck[*p];
                let available_cards: Vec<&String> =
                    models::cards::available_cards_by_class(card.base._class)
                        .iter()
                        .filter(move |c| **c != card.base.name)
                        .collect();

                state_space *= available_cards.len() as f64;
                available_cards
            });

            for position in positions {
                remove_card_from_deck(*position, &mut new_state);
            }

            for set in sets {
                let card = set
                    .choose(&mut rng)
                    .expect("No available cards to be chosen!");
                evaluator::add_card_to_deck(card, *should_upgrade, &mut new_state);
            }

            (new_state, 1.0 / state_space)
        }
        Choice::DeckUpgrade(positions) => {
            let mut possibility_set = (new_state, 1.0);

            for position in positions {
                upgrade_card_in_deck(*position, &mut possibility_set.0);
            }

            possibility_set
        }
        Choice::Dig => {
            let mut possibility_set = (new_state, 1.0);

            add_random_relic((1, 2, 3), &mut possibility_set, &mut rng);

            possibility_set
        }
        Choice::DiscardPotion { slot } => {
            new_state.potions[*slot] = None;
            (new_state, 1.0)
        }
        Choice::DrinkPotion { slot, target_index } => {
            let potion = state.potions[*slot]
                .as_ref()
                .expect("Potion does not exist in slot!");
            let mut possibility_set = (new_state, 1.0);
            evaluator::eval_effects(
                &potion.base.on_drink,
                &mut possibility_set,
                evaluator::Binding::Potion(evaluator::PotionReference{ potion: *slot }),
                &Some(GameAction {
                    creature: &state.player,
                    is_attack: false,
                    target: *target_index,
                }),
                &mut rng,
            );
            possibility_set
        }
        _ => unimplemented!(),
    }
}

fn add_random_relic<R>(weights: (u8, u8, u8), state: &mut GamePossibilitySet, rng: &mut R)
where
    R: Rng + ?Sized,
{
    let sum = weights.0 + weights.1 + weights.2;

    let mut sample_space: f64 = sum as f64;
    let rarities = [
        (Rarity::Common, weights.0),
        (Rarity::Uncommon, weights.1),
        (Rarity::Rare, weights.2),
    ];

    let (rarity, weight) = rarities.choose_weighted(rng, |item| item.1).unwrap();

    let samples = *weight as f64;

    let available_relics: Vec<&String> = models::relics::RELICS
        .values()
        .filter(|relic| {
            relic.rarity == *rarity
                && (relic.class == state.0.class || relic.class == Class::All)
                && !state.0.relic_names.contains(&relic.name)
        })
        .map(|relic| &relic.name)
        .collect();

    let relic = *available_relics
        .choose(rng)
        .expect("No available relics to be chosen!");
    sample_space *= available_relics.len() as f64;

    if relic == "War Paint" || relic == "Whetstone" {
        let card_type = if relic == "War Paint" {
            CardType::Skill
        } else {
            CardType::Attack
        };
        let available_cards: Vec<usize> = state
            .0
            .deck
            .iter()
            .enumerate()
            .filter(|(_, card)| card.base._type == card_type && evaluator::card_upgradable(card))
            .map(|(p, _)| p)
            .collect();

        let cards = available_cards.choose_multiple(rng, 2);

        let mut remaining_card_count = available_cards.len();

        for card in cards {
            sample_space *= remaining_card_count as f64;
            remaining_card_count -= 1;
            upgrade_card_in_deck(*card, &mut state.0);
        }
    }

    state.1 *= samples / sample_space;
}

fn upgrade_card_in_deck(position: usize, state: &mut GameState) {
    state.deck[position].upgrades += 1;
}

fn remove_card_from_deck(position: usize, state: &mut GameState) {
    state.deck.remove(position);
}

fn spend_money(amount: u16, at_shop: bool, state: &mut GameState) {
    state.gold -= amount;

    if at_shop{
        evaluator::find_relic(&String::from("Maw Bank"), state).map(|relic| relic.enabled = false);
    }
}

fn add_relic(name: &String, state: &mut GameState) {
    let relic = evaluator::create_relic(name);
    state.relic_names.insert(relic.base.name.to_string());
    state.relics.push(relic);
}

fn add_potion(name: &String, state: &mut GameState) {
    let potion = evaluator::create_potion(name);
    *state
        .potions
        .iter_mut()
        .find(|a| a.is_none())
        .expect("Expected potion in potions") = Some(potion);
}

pub fn verify_prediction<'a>(
    outcome: &'a GameState,
    prediction: &'a GamePossibilitySet,
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
