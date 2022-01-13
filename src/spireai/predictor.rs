use crate::models;
use crate::models::acts::MonsterSet;
use crate::models::core::{CardDestination, CardType, Rarity, FightType, ChestType};
use crate::spireai::*;
use crate::spireai::references::{Binding, EventReference};
use crate::state::core::{Card, Event, Relic};
use crate::state::game::{FloorState, Reward};
use crate::state::map::MapNodeIcon;
use im::vector;
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
            if let Some(relic) = possibility.state.find_relic("Maw Bank") {
                if relic.enabled {
                    possibility.state.gold += 12;
                }
            }
            let node = possibility.state.map.nodes[&(possibility.state.map.floor, *node)].clone();
            let act = &models::acts::ACTS[possibility.state.act as usize];
            match node.icon {
                MapNodeIcon::Boss(name) => {
                    let boss = act.bosses.iter().find(|b| b.name == name).unwrap();
                    let monsters = eval_monster_set(&boss.monsters, possibility);
                    possibility.fight(&monsters, FightType::Boss)
                }
                MapNodeIcon::BurningElite | MapNodeIcon::Elite => {
                    let options = 
                    if let Some(last) = possibility.state.last_elite {
                        let mut vec = (0 .. last).collect_vec();
                        vec.extend((last+1) .. act.elites.len());
                        vec
                    } else {
                        (0..act.elites.len()).collect_vec()
                    };

                    let choice = possibility.probability.choose(options).unwrap();
                    let monsters = eval_monster_set(&act.elites[choice], possibility);
                    possibility.fight(&monsters, FightType::Elite {burning: node.icon == MapNodeIcon::BurningElite});
                }
                MapNodeIcon::Campfire => {
                    possibility.state.floor_state = FloorState::Rest;
                }
                MapNodeIcon::Chest => {
                    let chests = &vec![
                        (ChestType::Small, 3),
                        (ChestType::Medium, 2),
                        (ChestType::Large, 1),
                    ];
                    let chest_type = possibility.probability.choose_weighted(chests).unwrap();

                    possibility.state.floor_state = FloorState::Chest(*chest_type);
                }
                MapNodeIcon::Monster => {
                    normal_fight(possibility);
                }
                MapNodeIcon::Question => {
                    if possibility.state.has_relic("Ssserpent Head") {
                        possibility.state.gold = 50;
                    }

                    let mut normal_probability = (possibility.state.unknown_normal_count + 1) * 10;
                    let mut shop_probability = (possibility.state.unknown_shop_count + 1) * 3;
                    let mut treasure_probability = (possibility.state.unknown_treasure_count + 1) * 2;

                    match possibility.state.floor_state {
                         FloorState::Shop {..} | FloorState::ShopEntrance => {
                            shop_probability = 0
                        }
                        _ => {}
                    }

                    if let Some(relic) = possibility.state.find_relic_mut("Tiny Chest") {
                        relic.vars.x += 1;
                        if relic.vars.x == 4 {
                            relic.vars.x = 0;
                            shop_probability = 0;
                            treasure_probability = 100;
                            normal_probability = 0;
                        }
                    }

                    if possibility.state.has_relic("Juzu Bracelet") {
                        normal_probability = 0;
                    }

                    let mut total_probability = normal_probability + shop_probability + treasure_probability;
                    if total_probability > 100 {
                        let reduction = (total_probability - 100).min(treasure_probability);
                        treasure_probability -= reduction;
                        total_probability -= reduction;
                    }
                    if total_probability > 100 {
                        let reduction = (total_probability - 100).min(shop_probability);
                        shop_probability -= reduction;
                        total_probability -= reduction;
                    }
                    let choices = vec![
                        (UnknownRoom::Fight, normal_probability),
                        (UnknownRoom::Shop, shop_probability),
                        (UnknownRoom::Treasure, treasure_probability),
                        (UnknownRoom::Event, 100 - total_probability)
                    ];

                    let choice = *possibility.probability.choose_weighted(&choices).unwrap();

                    match choice {
                        UnknownRoom::Fight => {
                            possibility.state.unknown_normal_count = 0;
                            possibility.state.unknown_shop_count += 1;
                            possibility.state.unknown_treasure_count += 1;
                            normal_fight(possibility)
                        }
                        UnknownRoom::Shop => {
                            possibility.state.unknown_normal_count += 1;
                            possibility.state.unknown_shop_count = 0;
                            possibility.state.unknown_treasure_count += 1;
                            shop(possibility);
                        }
                        UnknownRoom::Treasure => {
                            possibility.state.unknown_normal_count += 1;
                            possibility.state.unknown_shop_count += 1;
                            possibility.state.unknown_treasure_count = 0;
                            treasure(possibility);
                        }
                        UnknownRoom::Event => {
                            possibility.state.unknown_normal_count += 1;
                            possibility.state.unknown_shop_count += 1;
                            possibility.state.unknown_treasure_count += 1;
                            event(possibility);
                        }
                    }
                }
                MapNodeIcon::Shop => {
                    shop(possibility)
                }
            }
        }
        Choice::OpenChest => {
            if let FloorState::Chest(chest) = possibility.state.floor_state {
                let relic = possibility.random_relic(Some(chest), None, None, false);
                let (gold_chance, gold_min, gold_max) = match chest {
                    ChestType::Small => (50, 23, 27),
                    ChestType::Medium => (35, 45, 55),
                    ChestType::Large => (50, 68, 82),
                    ChestType::Boss => (0, 0, 0)
                };
                let gets_gold = *possibility.probability.choose_weighted(&vec![(true, gold_chance), (false, 100-gold_chance)]).unwrap();
                let mut rewards = vector![
                    Reward::Relic(Relic::new(relic))
                ];
                
                if gets_gold {
                    let gold_amount = (possibility.probability.range(gold_max - gold_min) + gold_min) as u16;
                    rewards.push_back(Reward::Gold(gold_amount));
                };

                possibility.state.floor_state = FloorState::Rewards(rewards)
            } else {
                panic!("Floor state is not a chest!")
            }
        }        
        _ => unimplemented!()
    }
}


#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum UnknownRoom {
    Event,
    Fight,
    Shop,
    Treasure,
}

fn event(possibility: &mut GamePossibility) {
    possibility.state.floor_state = FloorState::Event;
    let act = &models::acts::ACTS[possibility.state.act as usize];
    let events = 
        act.events.iter()
            .filter(|f| possibility.state.event_history.contains(*f))
            .map(|n| models::events::by_name(n.as_str()))
            .filter(|e| possibility.eval_condition(&e.condition, Binding::Event(EventReference {base: e}), None))
            .collect_vec();    

    let shrines = events.iter().filter(|f| f.shrine).copied().collect_vec();
    let nonshrines = events.into_iter().filter(|f| !f.shrine).collect_vec();

    let is_shrine = if shrines.is_empty() {
        false
    } else if nonshrines.is_empty() {
        true
    } else {
        possibility.probability.range(4) == 0
    };

    let event_set = if is_shrine { shrines } else {nonshrines};
    
    let base_event = possibility.probability.choose(event_set).unwrap();

    possibility.state.event_state = Some(Event::new(base_event))
}

fn treasure(possibility: &mut GamePossibility) {
    possibility.state.floor_state = FloorState::ShopEntrance;
}

fn shop(possibility: &mut GamePossibility) {
    let types = vec![
        (ChestType::Small, 3),
        (ChestType::Medium, 2),
        (ChestType::Large, 1),
    ];
    let chest_type = possibility.probability.choose_weighted(&types).unwrap();

    possibility.state.floor_state = FloorState::Chest(*chest_type);
}


fn normal_fight(possibility: &mut GamePossibility) {
    let act = &models::acts::ACTS[possibility.state.act as usize];
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

    let probabilities = options.iter().map(|f| (&f.set, f.probability)).collect_vec();
    let fight = possibility.probability.choose_weighted(&probabilities).unwrap();
    let monsters = eval_monster_set(fight, possibility);
    possibility.fight(&monsters, FightType::Common);
}

fn eval_monster_set(set: &MonsterSet, possibility: &mut GamePossibility) -> Vec<String> {
    match set {
        MonsterSet::ChooseN{n, choices} => {
            possibility.probability.choose_multiple(choices.to_vec(), *n as usize)
        }
        MonsterSet::Fixed(monsters) => {
            monsters.to_vec()
        }
        MonsterSet::RandomSet(sets) => {
            possibility.probability.choose(sets.to_vec()).unwrap()
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
