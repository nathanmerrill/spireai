use crate::models;
use crate::models::core::DeckOperation;
use crate::state::core::{CardOffer, RewardState, Reward};
use crate::state::event::EventScreenState;
use crate::state::floor::{FloorState, RestScreenState};
use crate::state::game::{GameState, DeckCard};
use crate::state::shop::ShopScreenState;
use im::Vector;
use itertools::Itertools;
use models::choices::Choice;

pub fn all_choices(state: &FloorState) -> Vec<Choice> {
    let mut choices: Vec<Choice> = Vec::new();
    choices.extend(
        state.game_state()
            .potions
            .iter()
            .enumerate()
            .filter(|(_, potion)| potion.is_some())
            .map(|(position, _)| Choice::DiscardPotion { slot: position }),
    );

    match state {
        FloorState::Battle(battle_state) => {
            match &battle_state.card_choose {
                Some(card_choice_state) => choices.extend(
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
                None => {
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

                    for potion in battle_state.game_state.potions() {
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
            }
        }
        FloorState::BattleOver(state) => {
            choices.extend(get_reward_choices(&state.rewards, &state.game_state))
        }
        FloorState::Chest(state) => {
            match state.rewards {
                Some(rewards) => choices.extend(get_reward_choices(&rewards, &state.game_state)),
                None => {
                    choices.push(Choice::OpenChest);
                    choices.push(Choice::Proceed);
                }
            }
        }
        FloorState::Event(event) => {
            match &event.screen_state {
                Some(EventScreenState::Rewards(rewards)) => choices.extend(get_reward_choices(rewards, &event.game_state)),
                Some(EventScreenState::DeckChoose(operation, count)) => {
                    let cards: Vec<DeckCard> = match operation {
                        DeckOperation::Duplicate => event.game_state.deck().collect(),
                        DeckOperation::Remove
                        | DeckOperation::Transform
                        | DeckOperation::TransformUpgrade => event.game_state.removable_cards().collect(),
                        DeckOperation::Upgrade => event.game_state.upgradable_cards().collect(),
                    };
        
                    let mut real_count = *count as usize;
                    if cards.len() > real_count {
                        real_count = cards.len()
                    }
        
                    choices.extend(
                        cards
                            .into_iter()
                            .combinations(real_count)
                            .map(|cards| Choice::DeckSelect(cards, *operation)),
                    )
                }
                None => {
                    for available_choice in &event.available_choices {
                        choices.push(Choice::Event(available_choice.to_string()))
                    }
                }
            }
        }
        FloorState::GameOver(_) => {
            choices = vec![Choice::Proceed];
        }
        FloorState::Map(state) => {
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
        FloorState::Rest(state) => {
            match &state.screen_state {
                Some(RestScreenState::DreamCatch(offer)) => {
                    choices.extend(get_offer_choices(offer, &state.game_state))
                }
                Some(RestScreenState::Smith) => {
                    for card in state.game_state.upgradable_cards() {
                        choices.push(Choice::DeckSelect(vec![card], DeckOperation::Upgrade))
                    }
                }
                Some(RestScreenState::Toke) => {
                    for card in state.game_state.removable_cards() {
                        choices.push(Choice::DeckSelect(vec![card], DeckOperation::Remove))
                    }
                }
                None => {
                    if !state.game_state.relics.contains("Coffee Dripper") {
                        choices.push(Choice::Rest)
                    }
                    if !state.game_state.relics.contains("Fusion Hammer") {
                        choices.extend(
                            state.game_state
                                .upgradable_cards()
                                .map(Choice::Smith),
                        );
                    }
                    if let Some(relic) = state.game_state.relics.find("Girya") {
                        if relic.vars.x < 3 {
                            choices.push(Choice::Lift)
                        }
                    }
                    if state.game_state.relics.contains("Shovel") {
                        choices.push(Choice::Dig)
                    }
                    if state.game_state.relics.contains("Peace Pipe") {
                        choices.extend(
                            state.game_state
                                .removable_cards()
                                .map(Choice::Toke),
                        );
                    }
                    match &state.game_state.keys {
                        Some(keys) => {
                            if !keys.ruby {
                                choices.push(Choice::Recall)
                            }
                        }
                        None => {}
                    }

                }
            }
        }
        FloorState::Shop(shop) => {
            match &shop.screen_state {
                ShopScreenState::Dolly => {
                    for card in shop.game_state.deck() {
                        choices.push(Choice::DeckSelect(vec![card], DeckOperation::Duplicate))
                    }
                }
                ShopScreenState::Entrance => {
                    choices.push(Choice::EnterShop);
                    choices.push(Choice::Proceed);
                }
                ShopScreenState::Reward(reward) => {
                    choices.extend(get_reward_choices(reward, &shop.game_state))
                }
                ShopScreenState::InShop => {
                    for (index, (_, cost)) in shop.cards.iter().enumerate() {
                        if *cost <= shop.game_state.gold {
                            choices.push(Choice::BuyCard(index))
                        }
                    }
        
                    for (index, (_, cost)) in shop.relics.iter().enumerate() {
                        if *cost <= shop.game_state.gold {
                            choices.push(Choice::BuyRelic(index))
                        }
                    }
        
                    if shop.game_state.potions.iter().any(|a| a.is_none()) {
                        for (index, (_, cost)) in shop.potions.iter().enumerate() {
                            if *cost <= shop.game_state.gold {
                                choices.push(Choice::BuyPotion(index))
                            }
                        }
                    }
        
                    if shop.can_purge && shop.purge_cost() <= shop.game_state.gold {
                        choices.extend(
                            shop.game_state
                                .removable_cards()
                                .map(Choice::BuyRemoveCard),
                        );
                    }
                }
            }
        }
    }

    choices
}

fn get_reward_choices(rewards: &RewardState, state: &GameState) -> Vec<Choice> {
    match rewards.viewing_reward {
        Some(index) => {
            match &rewards.rewards[index] {
                Reward::CardChoice(offers, _ , _) => {
                    get_offer_choices(offers, state)
                }
                _ => panic!("Viewing a reward that is not a card choice!")
            }
        }
        None => {
            let mut choices = Vec::new();
            choices.push(Choice::Proceed);
            for index in 0..rewards.rewards.len() {
                choices.push(Choice::TakeReward(index))
            }
            choices
        }
    }
}

fn get_offer_choices(offers: &Vector<CardOffer>, state: &GameState) -> Vec<Choice> {
    let mut choices = Vec::new();
    choices.push(Choice::Skip);
    for card in offers {
        choices.push(Choice::AddCardToDeck(card.base.name.to_string()))
    }

    if state.relics.contains("Singing Bowl") {
        choices.push(Choice::SingingBowl)
    }

    choices.push(Choice::Proceed);
    choices
}

