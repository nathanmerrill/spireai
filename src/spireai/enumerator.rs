use crate::models;
use crate::state::battle::BattleState;
use crate::state::game::CardChoiceEffect;
use crate::state::game::FloorState;
use crate::state::game::GameState;
use itertools::Itertools;
use models::choices::Choice;

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
            let battle_state: &BattleState = &state.battle_state;

            match state.card_choices.effect {
                CardChoiceEffect::Scry => {
                    state
                        .card_choices()
                        .powerset()
                        .for_each(|cards| choices.push(Choice::ScryDiscard(cards)));
                }
                CardChoiceEffect::None => {
                    choices.push(Choice::End);
                    for card_ref in battle_state.hand() {
                        if state.card_playable(card_ref) {
                            if state.get(card_ref).targeted() {
                                choices.extend(battle_state.available_monsters().map(|monster| {
                                    Choice::PlayCard {
                                        card: card_ref,
                                        target: Some(monster),
                                    }
                                }));
                            } else {
                                choices.push(Choice::PlayCard {
                                    card: card_ref,
                                    target: None,
                                })
                            }
                        }
                    }

                    for potion in state.potions() {
                        if potion.base.targeted {
                            choices.extend(battle_state.available_monsters().map(|monster| {
                                Choice::DrinkPotion {
                                    slot: potion.index,
                                    target: Some(monster),
                                }
                            }));
                        } else {
                            choices.push(Choice::DrinkPotion {
                                slot: potion.index,
                                target: None,
                            });
                        }
                    }
                }

                _ => unimplemented!(),
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

            if state.relic_names.contains_key("Singing Bowl") {
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
                        .deck()
                        .filter(|card| state.get(*card).removable())
                        .map(Choice::BuyRemoveCard),
                );
            }
        }
        FloorState::Rest => {
            if !state.relic_names.contains_key("Coffee Dripper") {
                if state.relic_names.contains_key("Dream Catcher") {
                    choices.push(Choice::RestDreamCatcher)
                } else {
                    choices.push(Choice::Rest)
                }
            }
            if !state.relic_names.contains_key("Fusion Hammer") {
                choices.extend(
                    state
                        .deck()
                        .filter(|card| state.get(*card).upgradable())
                        .map(Choice::Smith),
                );
            }
            if let Some(uuid) = state.relic_names.get("Girya") {
                if state.relics.get(uuid).unwrap().vars.x < 3 {
                    choices.push(Choice::Lift)
                }
            }
            if state.relic_names.contains_key("Shovel") {
                choices.push(Choice::Dig)
            }
            if state.relic_names.contains_key("Peace Pipe") {
                choices.extend(
                    state
                        .deck()
                        .filter(|card| state.get(*card).removable())
                        .map(Choice::Toke),
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
                .deck()
                .filter(|card| state.get(*card).removable())
                .combinations((*count).into())
                .map(Choice::DeckRemove),
        ),
        FloorState::EventTransform(count, upgrade) => {
            choices.extend(
                state
                    .deck()
                    .filter(|card| state.get(*card).removable())
                    .combinations((*count).into())
                    .map(|combination| Choice::DeckTransform(combination, *upgrade)),
            );
        }
        FloorState::EventUpgrade(count) => choices.extend(
            state
                .deck()
                .filter(|card| state.get(*card).upgradable())
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
