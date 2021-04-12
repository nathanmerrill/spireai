use crate::models;
use crate::models::state::*;
use models::state::GameState;
use itertools::Itertools;

pub mod evaluator;

pub struct SpireAi {
    expected_state: Option<GamePossibilitySet>,
    // Neural net nodes
}

pub enum Choice {
    Start {
        player_class: models::core::Class,
        ascension: Option<u8>,
    },
    DrinkPotion {
        slot: u8,
        target_index: Option<u8>,
    },
    DiscardPotion {
        slot: u8,
    },
    PlayCard {
        card_index: u8,
        target_index: Option<u8>,
    },
    EventChoice(&'static str),
    NavigateToNode(i8),
    TakeReward(u8),
    SelectCard(&'static str),
    BuyCard(String),
    BuyRelic(&'static str),
    BuyPotion(&'static str),
    BuyRemoveCard(String),
    DeckRemove(Vec<String>),
    DeckTransform(Vec<String>, bool), //And upgrade if true
    DeckUpgrade(Vec<String>),
    OpenChest,
    Rest,
    Smith,
    Lift,
    Dig,
    ScryDiscard(Vec<String>),
    Recall,
    Toke(String),
    EnterShop,
    End,
    Proceed,
    Return,
    Skip,
    SingingBowl,
    State,
}

impl SpireAi {
    pub fn new() -> SpireAi {
        SpireAi {
            expected_state: None,
        }
    }

    pub fn choose(&mut self, new_state: &GameState) -> Choice {
        let state: &GameState = match &self.expected_state {
            Some(expected) => verify_state(new_state, &expected),
            None => new_state,
        };

        let choice = make_choice(state);
        self.expected_state = predict_outcome(state, &choice);

        return choice;
    }

    /*
    fn handle_event(&mut self, event: &Event) -> Choice{
        match event.options.len() {
            0 => {
                error!("Unable to handle event with no choices.");
                panic!();
            }
            1 => return Choice::Choose {choice_index: 0},
            _ => {}
        }

        return match event.event_id.as_str() {
            "Neow Event" => {
                Choice::Choose {choice_index: 0}
            }
            _ => {
                error!("Unhandled event: {}", event.event_id);
                panic!();
            }
        }
    }
    */
}

fn verify_state<'a>(outcome: &GameState, prediction: &'a GamePossibilitySet) -> &'a GameState {
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
}

fn make_choice(state: &GameState) -> Choice {
    let mut max_val = f64::MIN;
    let mut best_choice = Choice::State;

    for choice in all_choices(&state) {
        match predict_outcome(state, &choice) {
            Some(outcome) => {
                let rating = rate_possibility_set(outcome);
                if rating > max_val {
                    max_val = rating;
                    best_choice = choice
                }
            }
            None => {
                if max_val == f64::MIN {
                    best_choice = choice
                }
            }
        }
    }

    best_choice
}

fn all_choices(state: &GameState) -> Vec<Choice> {
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
                            if card_targeted(card, state) {
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
                                if potion_targeted(potion, state) {
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
                choices.push(Choice::Rest)
            }
            if !state.relic_names.contains(models::relics::FUSION_HAMMER) {
                if !state.deck.iter().all(|card| evaluator::card_upgradable(card)) {
                    choices.push(Choice::Smith)
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

fn potion_targeted(potion: &Potion, state: &GameState) -> bool {
    evaluator::eval_condition(&potion.base.targeted, state, &evaluator::Binding::Potion(potion), &None)
}

fn card_targeted(card: &Card, state: &GameState) -> bool {
    evaluator::eval_condition(
        &card.base.targeted, 
        state, 
        &evaluator::Binding::Card(card),
        &None,
    )
}

fn predict_outcome(state: &GameState, choice: &Choice) -> Option<GamePossibilitySet> {
    None
}

fn rate_possibility_set(set: GamePossibilitySet) -> f64 {
    panic!("Not implemented")
}

fn rate_possibility(possibility: GamePossibility) -> f64 {
    panic!("Not implemented")
}
