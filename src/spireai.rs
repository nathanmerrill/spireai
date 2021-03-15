use crate::models;
use crate::models::core::*;
use crate::models::state::*;
use models::state::GameState;

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
    Potion {
        should_use: bool,
        slot: u8,
        target_index: Option<u8>,
    },
    Play {
        card_index: u8,
        target_index: Option<u8>,
    },
    End,
    Choose(u8),
    Proceed,
    Return,
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
            _ => {}
        }
    }

    best_choice
}

fn all_choices(state: &GameState) -> Vec<Choice> {
    let mut choices: Vec<Choice> = Vec::new();
    match &state.screen {
        ScreenState::Battle(battle_state) => {
            for (card_index, card) in battle_state.hand.iter().enumerate() {
                if card_playable(card, battle_state, state) {
                    if card_targeted(card) {
                        for monster in &battle_state.monsters {
                            if monster.targetable {
                                choices.push(Choice::Play {
                                    card_index: card_index as u8,
                                    target_index: Some(monster.creature.position),
                                });
                            }
                        }
                    } else {
                        choices.push(Choice::Play {
                            card_index: card_index as u8,
                            target_index: None,
                        })
                    }
                }
            }

            for (potion_index, potion) in state.potions.iter().enumerate() {
                if potion_targeted(potion) {
                    for monster in &battle_state.monsters {
                        if monster.targetable {
                            choices.push(Choice::Potion {
                                should_use: true,
                                slot: potion_index as u8,
                                target_index: Some(monster.creature.position),
                            });
                        }
                    }
                } else {
                    choices.push(Choice::Potion {
                        should_use: true,
                        slot: potion_index as u8,
                        target_index: None,
                    });
                }

                choices.push(Choice::Potion {
                    should_use: false,
                    slot: potion_index as u8,
                    target_index: None,
                });
            }

            choices.push(Choice::End);
        }
        _ => {
            panic!("Unrecognized screen state")
        }
    }

    choices
}

fn potion_targeted(potion: &Potion) -> bool {
    match potion.base.targeted {
        StaticCondition::True => true,
        _ => false,
    }
}

fn card_targeted(card: &Card) -> bool {
    match card.base.targeted {
        StaticCondition::True => true,
        StaticCondition::False => false,
        StaticCondition::WhenUpgraded => card.upgrades > 0,
        StaticCondition::WhenUnupgraded => card.upgrades == 0,
    }
}

fn card_playable(card: &Card, battle_state: &BattleState, game_state: &GameState) -> bool {
    card.cost <= battle_state.energy
        && evaluator::eval_condition(
            &card.base.playable_if,
            battle_state,
            game_state,
            &evaluator::Binding::Card(card),
            &None,
        )
}

fn predict_outcome(state: &GameState, choice: &Choice) -> Option<GamePossibilitySet> {
    panic!("Not implemented")
}

fn rate_possibility_set(set: GamePossibilitySet) -> f64 {
    panic!("Not implemented")
}

fn rate_possibility(possibility: GamePossibility) -> f64 {
    panic!("Not implemented")
}
