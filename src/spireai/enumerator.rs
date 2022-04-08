use crate::models;
use crate::models::core::DeckOperation;
use crate::state::game::DeckCard;
use crate::state::game::FloorState;
use crate::state::game::ScreenState;
use itertools::Itertools;
use models::choices::Choice;

pub fn all_choices(state: &FloorState) -> Vec<Choice> {
    let mut choices: Vec<Choice> = Vec::new();
    choices.extend(
        state
            .potions
            .iter()
            .enumerate()
            .filter(|(_, potion)| potion.is_some())
            .map(|(position, _)| Choice::DiscardPotion { slot: position }),
    );

    match &state.screen_state {
        ScreenState::CardChoose(card_choice_state) => choices.extend(
            card_choice_state
                .count_range
                .clone()
                .flat_map(|i| card_choice_state.choices.iter().copied().combinations(i))
                .map(|a| {
                    if card_choice_state.scry {
                        Choice::Scry(a)
                    } else {
                        Choice::SelectCards(a)
                    }
                }),
        ),
        ScreenState::CardReward(offers, _) => {
            choices.push(Choice::Skip);
            for card in offers {
                choices.push(Choice::AddCardToDeck(card.base.name.to_string()))
            }

            if state.relics.contains("Singing Bowl") {
                choices.push(Choice::SingingBowl)
            }
            choices.push(Choice::Proceed);
        }
        ScreenState::DeckChoose(count, operation) => {
            let cards: Vec<DeckCard> = match operation {
                DeckOperation::Duplicate => state.deck().collect(),
                DeckOperation::Remove
                | DeckOperation::Transform
                | DeckOperation::TransformUpgrade => state.removable_cards().collect(),
                DeckOperation::Upgrade => state.upgradable_cards().collect(),
            };

            let mut real_count = *count as usize;
            if cards.len() > real_count {
                real_count = cards.len()
            }

            choices.extend(
                cards
                    .into_iter()
                    .combinations(real_count)
                    .map(|cards| Choice::DeckSelect(cards)),
            )
        }
        ScreenState::InShop => {
            let shop = state.floor_state.shop();
            for (card, cost) in &shop.cards {
                if *cost <= state.gold {
                    choices.push(Choice::BuyCard(card.base.name.to_string()))
                }
            }

            for (relic, cost) in &shop.relics {
                if *cost <= state.gold {
                    choices.push(Choice::BuyRelic(relic.name.to_string()))
                }
            }

            if state.potions.iter().any(|a| a.is_none()) {
                for (potion, cost) in &shop.potions {
                    if *cost <= state.gold {
                        choices.push(Choice::BuyPotion(potion.name.to_string()))
                    }
                }
            }

            if shop.can_purge && state.purge_cost() <= state.gold {
                choices.extend(
                    state
                        .removable_cards()
                        .map(Choice::BuyRemoveCard),
                );
            }
        }
        ScreenState::Map => {
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
        ScreenState::Proceed => {
            choices.push(Choice::Proceed)
        }
        ScreenState::Normal => {
            match &state.floor_state {
                FloorState::Battle(battle_state) => {
                    choices.push(Choice::End);
                    for card_ref in battle_state.hand() {
                        if battle_state.card_playable(card_ref) {
                            if battle_state.get_card(card_ref).targeted() {
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
                FloorState::Chest(_) => {
                    choices.push(Choice::Proceed);
                    choices.push(Choice::OpenChest);
                }
                FloorState::Event(event) => {
                    for available_choice in &event.available_choices {
                         choices.push(Choice::Event(available_choice.to_string()))
                    }
                }
                FloorState::GameOver => choices.push(Choice::Proceed),
                FloorState::Shop(_) => {
                    choices.push(Choice::EnterShop);
                    choices.push(Choice::Proceed);
                }
                FloorState::Rest => {
                    if !state.relics.contains("Coffee Dripper") {
                        choices.push(Choice::Rest)
                    }
                    if !state.relics.contains("Fusion Hammer") {
                        choices.extend(
                            state
                                .upgradable_cards()
                                .map(Choice::Smith),
                        );
                    }
                    if let Some(relic) = state.relics.find("Girya") {
                        if relic.vars.x < 3 {
                            choices.push(Choice::Lift)
                        }
                    }
                    if state.relics.contains("Shovel") {
                        choices.push(Choice::Dig)
                    }
                    if state.relics.contains("Peace Pipe") {
                        choices.extend(
                            state
                                .removable_cards()
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
                FloorState::Rewards(rewards) => {
                    choices.push(Choice::Proceed);
                    for index in 0..rewards.len() {
                        choices.push(Choice::TakeReward(index))
                    }
                }
            }
        }
    }

    choices
}
