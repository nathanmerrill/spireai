use crate::models::core::*;
use log::error;

use crate::models;
use crate::models::state::*;
use models::state::GameState;

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

    pub fn choose(&mut self, state: &GameState) -> Choice {
        match &self.expected_state {
            Some(expected) => verify_state(state, &expected),
            None => {}
        }
        let choice = make_choice(&state);
        self.expected_state = predict_outcome(&state, &choice);

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

fn verify_state(outcome: &GameState, prediction: &GamePossibilitySet) {
    if prediction.states.iter().all(|a| &a.state != outcome) {
        error!(
            "New state does not match expected state.  New state: {:?}",
            outcome
        );
        panic!()
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
        _ => {panic!("Unrecognized screen state")}
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
    card.cost <= battle_state.energy &&
    eval_condition(&card.base.playable_if, battle_state, game_state, &card.vars, &game_state.player, &None, card.base.name)
}

fn eval_condition<'a>(condition: &Condition, battle_state: &BattleState, game_state: &GameState, vars: &Vars, creature: &Creature, action: &Option<GameAction>, name: &'static str) -> bool {
    match condition {
        Condition::Always => {
            true
        },
        Condition::Never => {
            false
        },
        Condition::Custom => {
            match name {
                _ => panic!("Unhandled custom condition: {}", name)
            }
        },
        Condition::Dead(target) => {
            match eval_target(target, battle_state, creature, action) {
                ResolvedTarget::Player => {
                    game_state.player.hp == 0
                },
                ResolvedTarget::Monster(idx) => {
                    battle_state.monsters[idx as usize].creature.hp == 0
                },
                ResolvedTarget::AllMonsters => {
                    battle_state.monsters.iter().all(|m| m.creature.hp == 0)
                },
                _ => panic!("Unexpected Dead condition: {:?}", condition)
            }
        },
        _ => panic!("Unhandled condition: {:?}", condition)
    }
}

enum ResolvedTarget {
    Player,
    Monster(u8),
    AllMonsters,
    RandomMonster(Vec<u8>),
    None,
}

fn eval_target(target: &Target, battle_state: &BattleState, creature: &Creature, action: &Option<GameAction>) -> ResolvedTarget {
    match target {
        Target::_Self => {
            match creature.is_player {
                true => ResolvedTarget::Player,
                false => ResolvedTarget::Monster(creature.position)
            }
        },
        Target::AllEnemies => {
            match creature.is_player {
                true => ResolvedTarget::AllMonsters,
                false => ResolvedTarget::Player,
            }
        },
        Target::AnyFriendly => {
            match creature.is_player {
                true => ResolvedTarget::Player,
                false => ResolvedTarget::AllMonsters,
            }
        },
        Target::Attacker => {
            match action {
                Some(_action) => {
                    match _action.is_attack {
                        true => {
                            match _action.creature.is_player {
                                true => ResolvedTarget::Player,
                                false => ResolvedTarget::Monster(_action.creature.position),
                            }
                        }
                        false => ResolvedTarget::None,
                    }
                },
                None => ResolvedTarget::None,
            }
        },
        Target::Friendly(name) => {
            match battle_state.monsters.iter().find(|m| &m.base.name == name) {
                Some(monster) => {
                    ResolvedTarget::Monster(monster.creature.position)
                },
                None => ResolvedTarget::None,
            }
        },
        Target::RandomEnemy => {
            match creature.is_player {
                true => ResolvedTarget::RandomMonster((0..battle_state.monsters.len() as u8).collect()),
                false => ResolvedTarget::Player,
            }
        },
        Target::RandomFriendly => {
            match creature.is_player {
                true => ResolvedTarget::Player,
                false => {
                    let mut positions: Vec<u8> = (0..creature.position).collect();
                    positions.extend(creature.position+1 .. battle_state.monsters.len() as u8);
                    ResolvedTarget::RandomMonster(positions)
                },
            }
        },
        Target::TargetEnemy => {
            match action {
                Some(_action) => {
                    match _action.creature.is_player {
                        true => ResolvedTarget::Monster(_action.creature.position),
                        false => ResolvedTarget::Player,
                    }
                },
                None => ResolvedTarget::None,
            }
        }
    }
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
