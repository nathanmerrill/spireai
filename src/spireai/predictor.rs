use crate::models;
use crate::models::acts::MonsterSet;
use crate::models::core::{CardDestination, CardType, Rarity, FightType, ChestType};
use crate::spireai::*;
use crate::spireai::references::{Binding, EventReference};
use crate::state::core::{Card};
use crate::state::game::FloorState;
use crate::state::map::MapNodeIcon;
use models::choices::Choice;

use super::evaluator::GamePossibility;

pub fn predict_outcome(choice: &Choice, possibility: &mut GamePossibility) {
    match choice {
        Choice::BuyCard(name) => {
            let cost = if let FloorState::Shop { ref mut cards, .. } = possibility.state.floor_state
            {
                let position = cards
                    .iter()
                    .position(|(card, _)| &card.base.name == name)
                    .expect("Unable to find card that matches in shop");
                let (_, cost) = cards.remove(position);
                cost
            } else {
                panic!("Expected store floor state")
            };

            possibility.spend_money(cost, true);
            possibility.add_card(Card::by_name(name), CardDestination::DeckPile);
        }
        Choice::BuyPotion(name) => {
            let cost = if let FloorState::Shop {
                ref mut potions, ..
            } = possibility.state.floor_state
            {
                let position = potions
                    .iter()
                    .position(|(potion, _)| &potion.name == name)
                    .expect("Unable to find potion that matches in shop");
                let (_, cost) = potions.remove(position);
                cost
            } else {
                panic!("Expected store floor state");
            };

            possibility.spend_money(cost, true);
            possibility.state.add_potion(models::potions::by_name(name));
        }
        Choice::BuyRelic(name) => {
            let cost =
                if let FloorState::Shop { ref mut relics, .. } = possibility.state.floor_state {
                    let position = relics
                        .iter()
                        .position(|(relic, _)| &relic.name == name)
                        .expect("Unable to find relic that matches in shop");
                    let (_, cost) = relics.remove(position);
                    cost
                } else {
                    panic!("Expected store floor state");
                };

            possibility.spend_money(cost, true);
            possibility.add_relic(models::relics::by_name(name));
        }
        Choice::BuyRemoveCard(card) => {
            let cost = possibility.state.purge_cost();
            possibility.spend_money(cost, true);
            possibility.state.remove_card(*card);
            possibility.state.base_purge_cost += 25;
            if let FloorState::Shop { ref mut can_purge, ..} = possibility.state.floor_state {
                *can_purge = true;
            } else {
                panic!("Unexpected floor state!")
            }
        }
        Choice::DeckRemove(cards) => {
            for card in cards {
                possibility.state.remove_card(*card);
            }
        }
        Choice::DeckTransform(cards, should_upgrade) => {
            let sets: Vec<Vec<&&'static models::cards::BaseCard>> = cards
                .iter()
                .map(|p| {
                    let (class, name) = {
                        let card = &possibility.state.deck[&p.uuid];
                        (card.base._class, card.base.name.to_string())
                    };

                    let available_cards: Vec<&&'static models::cards::BaseCard> =
                        models::cards::available_cards_by_class(class)
                            .iter()
                            .filter(move |c| c.name != name)
                            .collect();

                    available_cards
                })
                .collect();

            for card in cards {
                possibility.state.remove_card(*card);
            }

            for set in sets {
                let base = possibility.probability.choose(set).unwrap();
                if let Some(card_ref) =
                    possibility.add_card(Card::new(base), CardDestination::DeckPile)
                {
                    if *should_upgrade {
                        possibility.state.get_mut(card_ref).upgrade()
                    }
                }
            }
        }
        Choice::DeckUpgrade(cards) => {
            for card in cards {
                possibility.state.get_mut(*card).upgrade();
            }
        }
        Choice::Dig => {
            let relic = possibility.random_relic(None, None, None, false);
            possibility.add_relic(relic);
        }
        Choice::DiscardPotion { slot } => {
            possibility.state.potions[*slot] = None;
        }
        Choice::DrinkPotion { slot, target } => possibility.drink_potion(
            possibility.state.potion_at(*slot).unwrap(),
            target.map(|m| m.creature_ref()),
        ),
        Choice::End => possibility.end_turn(),
        Choice::EnterShop => {
            let mut discount = 1.0;
            if possibility.state.has_relic("The Courier") {
                discount = 0.8;
            }
            if possibility.state.has_relic("Membership Card") {
                discount /= 2.0;
            }

            let available_cards = models::cards::available_cards_by_class(possibility.state.class);
            let available_attacks = available_cards.into_iter().map(|a|*a).filter(|c| c._type == CardType::Attack).collect();
            let attack1 = possibility.generate_card_offer(None, &available_attacks);
            let attack2 = possibility.generate_card_offer(None, &available_attacks.into_iter().filter(|c| c.name != attack1.base.name).collect());
            
            let available_skills = available_cards.into_iter().map(|a|*a).filter(|c| c._type == CardType::Skill).collect();
            let skill1 = possibility.generate_card_offer(None, &available_skills);
            let skill2 = possibility.generate_card_offer(None, &available_skills.into_iter().filter(|c| c.name != skill1.base.name).collect());
            
            let available_powers = available_cards.into_iter().map(|a|*a).filter(|c| c._type == CardType::Power).collect();
            let power1 = possibility.generate_card_offer(None, &available_powers);
            
            let available_colorless = models::cards::available_cards_by_class(models::core::Class::None);
            let available_colorless_uncommon = available_colorless.into_iter().map(|a|*a).filter(|c| c.rarity == models::core::Rarity::Uncommon).collect();
            let available_colorless_rare = available_colorless.into_iter().map(|a|*a).filter(|c| c.rarity == models::core::Rarity::Rare).collect();

            let colorless1 = possibility.generate_card_offer(None, &available_colorless_uncommon);
            let colorless2 = possibility.generate_card_offer(None, &available_colorless_rare);

            let cards = vec![attack1, attack2, skill1, skill2, power1, colorless1, colorless2];
            let on_sale = possibility.probability.range(5);
            
            let card_prices = cards.into_iter().enumerate().map(|(p, c)| {
                let (min, max) = match c.base._class {
                    models::core::Class::None => {
                        match c.base.rarity {
                            models::core::Rarity::Uncommon => (81, 91),
                            models::core::Rarity::Rare => (162, 198),
                            _ => panic!("Unexpected rarity"),
                        }
                    }
                    _ => {
                        match c.base.rarity {
                            models::core::Rarity::Common => (45, 55),
                            models::core::Rarity::Uncommon => (68, 82),
                            models::core::Rarity::Rare => (135, 165),
                            _ => panic!("Unexpected rarity"),
                        }
                    }
                };

                let final_discount = if p == on_sale {
                    discount / 2.0
                } else {
                    discount
                };

                let min = (min as f64 * final_discount).ceil() as usize;
                let max = (max as f64 * final_discount).ceil() as usize;

                (c, (possibility.probability.range(max-min)+min) as u16)
            }).collect();

            let relic1 = possibility.random_relic(None, None, None, true);
            let relic2 = possibility.random_relic(None, None, Some(relic1), true);
            let relic3 = possibility.random_relic(None, Some(Rarity::Shop), None, true);

            let relics = vec![relic1, relic2, relic3];
            let relic_prices = relics.into_iter().map(|r| {
                let (min, max) = match r.rarity {
                    Rarity::Shop | Rarity::Common => (143, 157),
                    Rarity::Uncommon => (238, 262),
                    Rarity::Rare => (285, 315), 
                    _ => panic!("Unexpected rarity"),
                };

                (r, (possibility.probability.range(max-min)+min) as u16)
            }).collect();

            let potions = (0 .. 3).map(|_| {
                let potion = possibility.random_potion(false);
                let (min, max) = match potion.rarity {
                    Rarity::Common => (48, 52),
                    Rarity::Uncommon => (72, 78),
                    Rarity::Rare => (95, 105),
                    _ => panic!("Unexpected rarity"),
                };
                
                (potion, (possibility.probability.range(max-min)+min) as u16)
            }).collect();

            possibility.state.floor_state = FloorState::Shop {
                cards: card_prices,
                relics: relic_prices,
                potions: potions,
                can_purge: true,
            }
        }
        Choice::Event(name) => {
            let event = possibility.state.event_state.as_ref().unwrap().base;
            let choice = event.choices.iter().find(|base| &base.name == name).unwrap();
            if choice.effects.is_empty() {
                possibility.state.event_state = None;
                possibility.state.floor_state = FloorState::Map;
            } else {
                possibility.eval_effects(&choice.effects, Binding::Event(EventReference { base: event }), None);
            }            
        }
        Choice::Lift => {
            possibility.state.find_relic_mut("Girya").unwrap().vars.x += 1;
        }
        Choice::NavigateToNode(node) => {
            possibility.state.map.floor += 1;
            let node = possibility.state.map.nodes[&(possibility.state.map.floor, *node)];
            let act = models::acts::ACTS[possibility.state.act as usize];
            match node.icon {
                MapNodeIcon::Boss(name) => {
                    let boss = act.bosses.iter().find(|b| b.name == name).unwrap();
                    let monsters = eval_monster_set(boss.monsters, &mut possibility);
                    possibility.fight(&monsters, FightType::Boss)
                }
                MapNodeIcon::BurningElite | MapNodeIcon::Elite => {
                    let options = 
                    if let Some(last) = possibility.state.last_elite {
                        let vec = (0 .. last).collect_vec();
                        vec.extend((last+1) .. act.elites.len());
                        vec
                    } else {
                        (0..act.elites.len()).collect_vec()
                    };

                    let choice = possibility.probability.choose(options).unwrap();
                    let elite = act.elites[choice];
                    let monsters = eval_monster_set(elite, &mut possibility);
                    possibility.fight(&monsters, FightType::Elite {burning: node.icon == MapNodeIcon::BurningElite});
                }
                MapNodeIcon::Campfire => {
                    possibility.state.floor_state = FloorState::Rest;
                }
                MapNodeIcon::Chest => {
                    let chest_type = possibility.probability.choose_weighted(&vec![
                        (ChestType::Small, 3),
                        (ChestType::Medium, 2),
                        (ChestType::Large, 1),
                    ]).unwrap();

                    possibility.state.floor_state = FloorState::Chest(*chest_type);
                }
                MapNodeIcon::Monster => {
                    eval_normal_fight(possibility);
                }
                MapNodeIcon::Question => {
                    let probabilities = vec![(

                    )];
                }
            }
        }
        
        _ => unimplemented!()
    }
}

enum UnknownRoom {
    Event,
    Fight,
    Rest,
    Shop,
    Treasure,
}

fn eval_normal_fight(possibility: &mut GamePossibility) {
    let act = models::acts::ACTS[possibility.state.act as usize];
    if possibility.state.easy_fight_count == act.easy_count {
        possibility.state.last_normal = None
    }

    possibility.state.easy_fight_count += 1;

    let fights = if possibility.state.easy_fight_count <= act.easy_count {
        &act.easy_fights
    } else {
        &act.normal_fights
    };

    let options = 
    if let Some(last) = possibility.state.last_normal {
        fights[0..last].iter().chain(fights[last+1..fights.len()].iter()).collect_vec()
    } else {
        fights.iter().collect_vec()
    };

    let fight = possibility.probability.choose_weighted(&options.iter().map(|f| (f.set, f.probability)).collect_vec()).unwrap();
    let monsters = eval_monster_set(*fight, &mut possibility);
    possibility.fight(&monsters, FightType::Common);
}

fn eval_monster_set(set: MonsterSet, possibility: &mut GamePossibility) -> Vec<String> {
    match set {
        MonsterSet::ChooseN{n, choices} => {
            possibility.probability.choose_multiple(choices, n as usize)
        }
        MonsterSet::Fixed(monsters) => {
            monsters
        }
        MonsterSet::RandomSet(sets) => {
            possibility.probability.choose(sets).unwrap()
        }
    }
}

#[allow(unused_variables)]
pub fn verify_prediction<'a>(outcome: &'a GameState, choice: &'a GamePossibility) -> &'a GameState {
    unimplemented!()
    /*
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
    */
}
