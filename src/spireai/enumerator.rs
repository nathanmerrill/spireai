use crate::models;
use crate::spireai::evaluator;
use itertools::Itertools;
use models::choices::Choice;
use models::state::*;

pub fn all_choices(state: &GameState) -> Vec<Choice> {
    let mut choices: Vec<Choice> = Vec::new();
    choices.extend(
        state
            .potions
            .iter()
            .enumerate()
            .filter(|(_, potion)| potion.is_some())
            .map(|(position, _)| Choice::DiscardPotion { slot: position }),
    );

    match &state.floor_state {
        FloorState::Battle => {
            let battle_state: &BattleState = state
                .battle_state
                .as_ref()
                .expect("Battle state not set when floor state is battle");

            match battle_state.card_choice_type {
                CardChoiceType::Scry => {
                    (0..(battle_state.card_choices.len()))
                        .powerset()
                        .for_each(|combination| choices.push(Choice::ScryDiscard(combination)));
                }

                CardChoiceType::None => {
                    choices.push(Choice::End);
                    for card_index in 0 .. battle_state.hand.len() {
                        let card_ref = evaluator::CardReference::Hand(card_index);
                        if evaluator::card_playable(card_ref, battle_state, state) {
                            if evaluator::card_targeted(card_ref, state) {
                                choices.extend(
                                    battle_state
                                        .monsters
                                        .iter()
                                        .filter(|monster| monster.targetable)
                                        .map(|monster| Choice::PlayCard {
                                            card_index,
                                            target_index: Some(monster.creature.position),
                                        }),
                                );
                            } else {
                                choices.push(Choice::PlayCard {
                                    card_index,
                                    target_index: None,
                                })
                            }
                        }
                    }

                    for (potion_index, potion_slot) in state.potions.iter().enumerate() {
                        match potion_slot {
                            Some(_) => {
                                if evaluator::potion_targeted(
                                    evaluator::PotionReference {
                                        potion: potion_index,
                                    },
                                    state,
                                ) {
                                    choices.extend(
                                        battle_state
                                            .monsters
                                            .iter()
                                            .enumerate()
                                            .filter(|(_, monster)| monster.targetable)
                                            .map(|(position, _)| Choice::DrinkPotion {
                                                slot: potion_index,
                                                target_index: Some(position),
                                            }),
                                    );
                                } else {
                                    choices.push(Choice::DrinkPotion {
                                        slot: potion_index,
                                        target_index: None,
                                    });
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }
        FloorState::Event => {
            unimplemented!()
            // for available_choice in &event_state.available_choices {
            //     choices.push(Choice::EventChoice(available_choice))
            // }
        }
        FloorState::Map => {
            let map = &state.map;
            match &map.floor {
                -1 => {
                    for (y, x) in map.nodes.keys() {
                        if *y == 0 {
                            choices.push(Choice::NavigateToNode(*x))
                        }
                    }
                }
                _ => {
                    let node = &map.nodes[&(map.floor, map.x)];

                    for next in &node.next {
                        choices.push(Choice::NavigateToNode(*next))
                    }
                }
            }
        }
        FloorState::GameOver => choices.push(Choice::Proceed),
        FloorState::Rewards(rewards) => {
            choices.push(Choice::Proceed);
            for index in 0..rewards.len() {
                choices.push(Choice::TakeReward(index))
            }
        }
        FloorState::CardReward(card_choices) => {
            choices.push(Choice::Skip);
            for (card, _) in card_choices {
                choices.push(Choice::SelectCard(card.to_string()))
            }

            if state.relic_names.contains(&String::from("Singing Bowl")) {
                choices.push(Choice::SingingBowl)
            }
        }
        FloorState::ShopEntrance => {
            choices.push(Choice::EnterShop);
            choices.push(Choice::Proceed);
        }
        FloorState::Shop {
            cards,
            relics,
            potions,
            purge_cost,
        } => {
            for (card, cost) in cards {
                if *cost <= state.gold {
                    choices.push(Choice::BuyCard(card.to_string()))
                }
            }

            for (relic, cost) in relics {
                if *cost <= state.gold {
                    choices.push(Choice::BuyRelic(relic.to_string()))
                }
            }

            if state.potions.iter().any(|a| a.is_none()) {
                for (potion, cost) in potions {
                    if *cost <= state.gold {
                        choices.push(Choice::BuyPotion(potion.to_string()))
                    }
                }
            }

            if *purge_cost != 0 && *purge_cost <= state.gold {
                choices.extend(
                    state
                        .deck
                        .iter()
                        .enumerate()
                        .filter(|(_, card)| evaluator::card_removable(&card))
                        .map(|(position, _)| Choice::BuyRemoveCard(position)),
                );
            }
        }
        FloorState::Rest => {
            if !state.relic_names.contains(&String::from("Coffee Dripper")) {
                if state.relic_names.contains(&String::from("Dream Catcher")) {
                    choices.push(Choice::RestDreamCatcher)
                } else {
                    choices.push(Choice::Rest)
                }
            }
            if !state.relic_names.contains(&String::from("Fusion Hammer")) {
                choices.extend(
                    state
                        .deck
                        .iter()
                        .enumerate()
                        .filter(|(_, card)| evaluator::card_upgradable(card))
                        .map(|(position, _)| Choice::Smith(position)),
                );
            }
            if state.relic_names.contains(&String::from("Girya")) {
                let relic = state
                    .relics
                    .iter()
                    .find(|relic| relic.base.name == "Girya")
                    .unwrap();
                if relic.vars.x < 3 {
                    choices.push(Choice::Lift)
                }
            }
            if state.relic_names.contains(&String::from("Shovel")) {
                choices.push(Choice::Dig)
            }
            if state.relic_names.contains(&String::from("Peace Pipe")) {
                choices.extend(
                    state
                        .deck
                        .iter()
                        .enumerate()
                        .filter(|(_, card)| evaluator::card_removable(card))
                        .map(|(position, _)| Choice::Toke(position)),
                );
            }
            match &state.keys {
                Some(keys) => {
                    if !keys.ruby {
                        choices.push(Choice::Recall)
                    }
                }
                None => {}
            }
        }
        FloorState::EventRemove(count) => choices.extend(
            state
                .deck
                .iter()
                .enumerate()
                .filter(|(_, card)| evaluator::card_removable(card))
                .map(|(position, _)| position)
                .combinations((*count).into())
                .map(Choice::DeckRemove),
        ),
        FloorState::EventTransform(count, upgrade) => {
            choices.extend(
                state
                    .deck
                    .iter()
                    .enumerate()
                    .filter(|(_, card)| evaluator::card_removable(card))
                    .map(|(position, _)| position)
                    .combinations((*count).into())
                    .map(|combination| Choice::DeckTransform(combination, *upgrade)),
            );
        }
        FloorState::EventUpgrade(count) => choices.extend(
            state
                .deck
                .iter()
                .enumerate()
                .filter(|(_, card)| evaluator::card_upgradable(card))
                .map(|(position, _)| position)
                .combinations((*count).into())
                .map(Choice::DeckUpgrade),
        ),
        FloorState::Chest(_) => {
            choices.push(Choice::Proceed);
            choices.push(Choice::OpenChest);
        }
    }

    choices
}
