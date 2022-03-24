use crate::models;
use crate::models::core::DeckOperation;
use crate::state::game::DeckCard;
use crate::state::game::GameState;
use crate::state::game::ScreenState;
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

    match &state.screen_state {
        ScreenState::CardChoose(card_choice_state) => choices.extend(
            card_choice_state
                .count_range
                .clone()
                .flat_map(|i| card_choice_state.choices.iter().copied().combinations(i))
                .map(Choice::SelectCards),
        ),
        ScreenState::CardReward(offers) => {
            choices.push(Choice::Skip);
            for card in offers {
                choices.push(Choice::SelectCard(card.base.name.to_string()))
            }

            if state.relics.contains("Singing Bowl") {
                choices.push(Choice::SingingBowl)
            }
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
                    .map(|cards| Choice::DeckSelect(cards, *operation)),
            )
        }
        _ => unimplemented!(),
    }

    choices
    /*
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
        FloorState::Event(event) => {
            for available_choice in &event.available_choices {
                 choices.push(Choice::Event(available_choice.to_string()))
            }
        }

        FloorState::Map(maps) => {
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
        FloorState::ShopEntrance => {
            choices.push(Choice::EnterShop);
            choices.push(Choice::Proceed);
        }
        FloorState::Shop(shop_state) => {
            for (card, cost) in &shop_state.cards {
                if *cost <= state.gold {
                    choices.push(Choice::BuyCard(card.base.name.to_string()))
                }
            }

            for (relic, cost) in &shop_state.relics {
                if *cost <= state.gold {
                    choices.push(Choice::BuyRelic(relic.name.to_string()))
                }
            }

            if state.potions.iter().any(|a| a.is_none()) {
                for (potion, cost) in shop_state.potions {
                    if *cost <= state.gold {
                        choices.push(Choice::BuyPotion(potion.name.to_string()))
                    }
                }
            }

            if shop_state.can_purge && state.purge_cost() <= state.gold {
                choices.extend(
                    state
                        .removable_cards()
                        .map(Choice::BuyRemoveCard),
                );
            }
        }
        FloorState::Rest => {
            if !state.relic_names.contains_key("Coffee Dripper") {
                choices.push(Choice::Rest)
            }
            if !state.relic_names.contains_key("Fusion Hammer") {
                choices.extend(
                    state
                        .upgradable_cards()
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
        FloorState::EventRemove(count) => choices.extend(
            state
                .removable_cards()
                .combinations((*count).into())
                .map(Choice::DeckRemove),
        ),
        FloorState::EventTransform(count, upgrade) => {
            choices.extend(
                state
                    .removable_cards()
                    .combinations((*count).into())
                    .map(|combination| Choice::DeckTransform(combination, *upgrade)),
            );
        }
        FloorState::EventUpgrade(count) => choices.extend(
            state
                .upgradable_cards()
                .combinations((*count).into())
                .map(Choice::DeckUpgrade),
        ),
        FloorState::Chest(_) => {
            choices.push(Choice::Proceed);
            choices.push(Choice::OpenChest);
        }
    }

    choices

         */
}
