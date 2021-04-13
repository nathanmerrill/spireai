use crate::spireai::{evaluator};
use crate::models;
use models::choices::Choice;
use models::state::{BattleState, GameState, FloorState, CardChoiceType};
use itertools::Itertools;

pub fn all_choices(state: &GameState) -> Vec<Choice> {
    let mut choices: Vec<Choice> = Vec::new();
    match &state.floor_state {
        FloorState::Battle => {
            let battle_state: &BattleState = state.battle_state.as_ref().expect("Battle state not set when floor state is battle");

            match battle_state.card_choice_type {
                CardChoiceType::Scry => {
                    for combination in battle_state.card_choices.iter().map(|a| a.id.to_string()).powerset() {
                        choices.push(Choice::ScryDiscard(combination))
                    }
                }

                CardChoiceType::None => {
                    choices.push(Choice::End);
                    for (card_index, card) in battle_state.hand.iter().enumerate() {
                        if evaluator::card_playable(card, battle_state, state) {
                            if evaluator::card_targeted(card, state) {
                                for monster in &battle_state.monsters {
                                    if monster.targetable {
                                        choices.push(Choice::PlayCard {
                                            card_index: card_index as u8,
                                            target_index: Some(monster.creature.position),
                                        });
                                    }
                                }
                            } else {
                                choices.push(Choice::PlayCard {
                                    card_index: card_index as u8,
                                    target_index: None,
                                })
                            }
                        }
                    }
        
                    for (potion_index, potion_slot) in state.potions.iter().enumerate() {
                        match potion_slot {
                            Some(potion) => {
                                if evaluator::potion_targeted(potion, state) {
                                    for monster in &battle_state.monsters {
                                        if monster.targetable {
                                            choices.push(Choice::DrinkPotion {
                                                slot: potion_index as u8,
                                                target_index: Some(monster.creature.position),
                                            });
                                        }
                                    }
                                } else {
                                    choices.push(Choice::DrinkPotion {
                                        slot: potion_index as u8,
                                        target_index: None,
                                    });
                                }
        
                                choices.push(Choice::DiscardPotion {
                                    slot: potion_index as u8
                                });
                            },
                            None => {}
                        }
                    }
                }
            }
        },
        FloorState::Event(event_state) => {
            for available_choice in &event_state.available_choices {
                choices.push(Choice::EventChoice(available_choice))
            }
        },
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
        },
        FloorState::GameOver => {
            choices.push(Choice::Proceed)
        },
        FloorState::Rewards(rewards) => {
            choices.push(Choice::Proceed);
            for index in 0..rewards.len() {
                choices.push(Choice::TakeReward(index as u8))
            }
        },
        FloorState::CardReward(card_choices) => {
            choices.push(Choice::Skip);
            for card in card_choices {
                choices.push(Choice::SelectCard(card.base.name))
            }
            if state.relic_names.contains(models::relics::SINGING_BOWL) {
                choices.push(Choice::SingingBowl)
            }
        },
        FloorState::ShopEntrance => {
            choices.push(Choice::EnterShop);
            choices.push(Choice::Proceed);
        },
        FloorState::Shop(cards, relics, potions, remove) => {
            for (card, cost) in cards {
                if *cost <= state.gold {
                    choices.push(Choice::BuyCard(card.id.to_string()))
                }
            }

            for (relic, cost) in relics {
                if *cost <= state.gold {
                    choices.push(Choice::BuyRelic(relic.base.name))
                }
            }

            for (potion, cost) in potions {
                if *cost <= state.gold {
                    choices.push(Choice::BuyPotion(potion.base.name))
                }
            }
            
            if *remove != 0 && *remove <= state.gold {
                for card in &state.deck {
                    if evaluator::card_removable(&card) {
                        choices.push(Choice::BuyRemoveCard(card.id.to_string()))
                    }
                }
            }
        },
        FloorState::Rest => {
            if !state.relic_names.contains(models::relics::COFFEE_DRIPPER) {
                if state.relic_names.contains(models::relics::DREAM_CATCHER) {
                    choices.push(Choice::RestDreamCatcher)
                } else {
                    choices.push(Choice::Rest) 
                }
            }
            if !state.relic_names.contains(models::relics::FUSION_HAMMER) {
                for card in &state.deck {
                    if evaluator::card_upgradable(&card) {
                        choices.push(Choice::Smith(card.id.to_string()))
                    }
                }
            }
            if state.relic_names.contains(models::relics::GIRYA) {
                let relic = state.relics.iter().find(|relic| relic.base.name == models::relics::GIRYA).unwrap();
                if relic.vars.x < 3 {
                    choices.push(Choice::Lift)
                }
            }
            if state.relic_names.contains(models::relics::SHOVEL) {
                choices.push(Choice::Dig)
            }
            match &state.keys {
                Some(keys) => {
                    if !keys.ruby {
                        choices.push(Choice::Recall)
                    }
                },
                None => {}
            }
        },
        FloorState::EventRemove(count) => {
            for combination in state.deck.iter()
                                .filter(|card| evaluator::card_removable(card))
                                .map(|card| card.id.to_string())
                                .combinations((*count).into()) {
                choices.push(Choice::DeckRemove(combination))
            }
        },
        FloorState::EventTransform(count, upgrade) => {
            for combination in state.deck.iter()
                                .filter(|card| evaluator::card_removable(card))
                                .map(|card| card.id.to_string())
                                .combinations((*count).into()) {
                choices.push(Choice::DeckTransform(combination, *upgrade))
            }
        },
        FloorState::EventUpgrade(count) => {
            for combination in state.deck.iter()
                                .filter(|card| evaluator::card_upgradable(card))
                                .map(|card| card.id.to_string())
                                .combinations((*count).into()) {
                choices.push(Choice::DeckUpgrade(combination))
            }
        },
        FloorState::Chest(_) => {
            choices.push(Choice::Proceed);
            choices.push(Choice::OpenChest);
        }
    }

    choices
}