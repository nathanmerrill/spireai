use crate::models::core::{CardType, ChestType, Class, DeckOperation};
use crate::models::{self, relics};
use crate::state::core::{CardOffer, Reward, RewardState};
use crate::state::event::EventScreenState;
use crate::state::floor::{FloorState, RestScreenState};
use crate::state::game::{DeckCard, GameState};
use crate::state::shop::ShopScreenState;
use im::Vector;
use itertools::Itertools;
use models::choices::Choice;

pub fn all_choices(state: &FloorState) -> Vec<Choice> {
    let mut choices: Vec<Choice> = Vec::new();

    if !matches!(state, FloorState::Menu) {
        for potion in state.game_state().potions() {
            choices.push(Choice::DiscardPotion { slot: potion.index });

            match potion.base.name.as_str() {
                "Blood Potion" | "Entropic Brew" | "Fruit Juice" => {
                    choices.push(Choice::DrinkPotion {
                        slot: potion.index,
                        target: None,
                    });
                }
                _ => {}
            }
        }
    }

    match state {
        FloorState::Battle(battle_state) => {
            if battle_state.battle_over {
                choices.push(Choice::Proceed)
            } else {
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
                        if battle_state.stance_pot {
                            choices.push(Choice::StanceCalm);
                            choices.push(Choice::StanceWrath);
                        } else if battle_state.wish > 0 {
                            choices.push(Choice::WishGold);
                            choices.push(Choice::WishPlated);
                            choices.push(Choice::WishStrength);
                        } else {
                            choices.push(Choice::End);
                            for card_ref in battle_state.hand() {
                                if battle_state.card_playable(card_ref) {
                                    if battle_state.get_card(card_ref).targeted() {
                                        choices.extend(battle_state.available_monsters().map(
                                            |monster| Choice::PlayCard {
                                                card: card_ref,
                                                target: Some(monster),
                                            },
                                        ));
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
                                    choices.extend(battle_state.available_monsters().map(
                                        |monster| Choice::DrinkPotion {
                                            slot: potion.index,
                                            target: Some(monster),
                                        },
                                    ));
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
            }
        }
        FloorState::Menu => {
            choices.push(Choice::Start {
                player_class: Class::Ironclad,
                ascension: None,
            });
            choices.push(Choice::Start {
                player_class: Class::Silent,
                ascension: None,
            });
            choices.push(Choice::Start {
                player_class: Class::Defect,
                ascension: None,
            });
            choices.push(Choice::Start {
                player_class: Class::Watcher,
                ascension: None,
            });
        }
        FloorState::BattleRewards(state) => {
            choices.extend(get_reward_choices(&state.rewards, &state.game_state))
        }
        FloorState::Chest(state) => match &state.rewards {
            Some(rewards) => match state.chest {
                ChestType::Boss => {
                    if rewards.rewards.is_empty() {
                        choices.push(Choice::Proceed)
                    } else {
                        for (index, reward) in rewards.rewards.iter().enumerate() {
                            match reward {
                                Reward::Relic(_) => choices.push(Choice::TakeReward(index)),
                                _ => panic!("Unexpected non-relic reward in boss chest"),
                            }
                        }
                        choices.push(Choice::Skip)
                    }
                }
                _ => {
                    choices.extend(get_reward_choices(rewards, &state.game_state));
                }
            },
            None => {
                choices.push(Choice::OpenChest);
                choices.push(Choice::Proceed);
            }
        },
        FloorState::Event(event) => match &event.screen_state {
            Some(EventScreenState::Rewards(rewards)) => {
                choices.extend(get_reward_choices(rewards, &event.game_state))
            }
            Some(EventScreenState::DeckChoose(operation, count)) => {
                choices.extend(get_operation_choices(*operation, *count, &event.game_state))
            }
            None => {
                for available_choice in &event.available_choices {
                    choices.push(Choice::Event(available_choice.to_string()))
                }
            }
        },
        FloorState::GameOver(..) => {
            choices = vec![Choice::Proceed];
        }
        FloorState::Map(state) => {
            let map = &state.map;
            match &map.index {
                None => {
                    let bottom_nodes = map.nodes.iter().flatten().take_while(|a| a.y == 0);
                    choices.extend(bottom_nodes.map(|a| Choice::NavigateToNode(a.x)))
                }
                Some(node_index) => {
                    let node = &map.nodes[*node_index].unwrap();

                    if node.left {
                        choices.push(Choice::NavigateToNode(node.x - 1))
                    }

                    if node.up {
                        choices.push(Choice::NavigateToNode(node.x))
                    }

                    if node.right {
                        choices.push(Choice::NavigateToNode(node.x + 1))
                    }

                    if node.is_top() {
                        // Boss
                        choices.push(Choice::NavigateToNode(0))
                    }
                }
            }
        }
        FloorState::Rest(state) => match &state.screen_state {
            RestScreenState::DreamCatch(offer) => {
                choices.extend(get_offer_choices(offer, &state.game_state))
            }
            RestScreenState::Smith => {
                for card in state.game_state.upgradable_cards() {
                    choices.push(Choice::DeckSelect(vec![card], DeckOperation::Upgrade))
                }
            }
            RestScreenState::Toke => {
                for card in state.game_state.removable_cards() {
                    choices.push(Choice::DeckSelect(vec![card], DeckOperation::Remove))
                }
            }
            RestScreenState::Dig(rewards) => {
                choices.extend(get_reward_choices(rewards, &state.game_state))
            }
            RestScreenState::DeckSelect(operation) => {
                choices.extend(get_operation_choices(*operation, 1, &state.game_state))
            }
            RestScreenState::Proceed => choices.push(Choice::Proceed),
            RestScreenState::IShouldRest => {
                let mut rest_choices = vec![];
                if !state.game_state.has_relic(relics::COFFEE_DRIPPER) {
                    rest_choices.push(Choice::Rest)
                }
                if !state.game_state.has_relic(relics::FUSION_HAMMER) {
                    rest_choices.push(Choice::Smith)
                }
                if let Some(relic) = state.game_state.get_relic(relics::GIRYA) {
                    if relic.vars.x < 3 {
                        rest_choices.push(Choice::Lift)
                    }
                }
                if state.game_state.has_relic(relics::SHOVEL) {
                    rest_choices.push(Choice::Dig)
                }
                if state.game_state.has_relic(relics::PEACE_PIPE) {
                    rest_choices.push(Choice::Toke)
                }
                match &state.game_state.keys {
                    Some(keys) => {
                        if !keys.ruby {
                            rest_choices.push(Choice::Recall)
                        }
                    }
                    None => {}
                }

                if rest_choices.is_empty() {
                    choices.push(Choice::Proceed)
                } else {
                    choices.extend(rest_choices)
                }
            }
        },
        FloorState::Shop(shop) => match &shop.screen_state {
            ShopScreenState::DeckChoose(operation) => {
                choices.extend(get_operation_choices(*operation, 1, &shop.game_state))
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
                    choices.extend(shop.game_state.removable_cards().map(Choice::BuyRemoveCard));
                }
            }
        },
    }

    choices
}

fn get_operation_choices(
    operation: DeckOperation,
    mut count: usize,
    state: &GameState,
) -> impl Iterator<Item = Choice> + '_ {
    let cards: Vec<DeckCard> = match operation {
        DeckOperation::Duplicate => state.deck().collect(),
        DeckOperation::Remove | DeckOperation::Transform | DeckOperation::TransformUpgrade => {
            state.removable_cards().collect()
        }
        DeckOperation::Upgrade => state.upgradable_cards().collect(),
        DeckOperation::BottleFlame => state
            .deck()
            .filter(|a| a.base._type == CardType::Attack)
            .collect(),
        DeckOperation::BottleLightning => state
            .deck()
            .filter(|a| a.base._type == CardType::Skill)
            .collect(),
        DeckOperation::BottleTornado => state
            .deck()
            .filter(|a| a.base._type == CardType::Power)
            .collect(),
    };

    if cards.len() < count {
        count = cards.len()
    }

    cards
        .into_iter()
        .combinations(count)
        .map(move |cards| Choice::DeckSelect(cards, operation))
}

fn get_reward_choices(rewards: &RewardState, state: &GameState) -> Vec<Choice> {
    match rewards.viewing_reward {
        Some(index) => match &rewards.rewards[index] {
            Reward::CardChoice(offers, _, _) => get_offer_choices(offers, state),
            _ => panic!("Viewing a reward that is not a card choice!"),
        },
        None => {
            let mut choices = vec![Choice::Proceed];
            for index in 0..rewards.rewards.len() {
                choices.push(Choice::TakeReward(index))
            }
            choices
        }
    }
}

fn get_offer_choices(offers: &Vector<CardOffer>, state: &GameState) -> Vec<Choice> {
    let mut choices = vec![Choice::Skip];
    for card in offers {
        choices.push(Choice::AddCardToDeck(card.base.name.to_string()))
    }

    if state.has_relic(relics::SINGING_BOWL) {
        choices.push(Choice::SingingBowl)
    }

    choices
}
