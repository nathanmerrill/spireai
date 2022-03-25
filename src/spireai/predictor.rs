use crate::models;
use crate::models::acts::MonsterSet;
use crate::models::core::{CardType, ChestType, DeckOperation, FightType, Rarity, RoomType, When, CardDestination};
use crate::spireai::references::Binding;
use crate::spireai::*;
use crate::state::core::{Card, Event};
use crate::state::game::{FloorState, Reward, ScreenState, ShopState};
use crate::state::map::MapNodeIcon;
use im::vector;
use models::choices::Choice;

use super::evaluator::GamePossibility;
use super::references::CreatureReference;

pub fn predict_outcome(choice: Choice, possibility: &mut GamePossibility) {
    match choice {
        Choice::BuyCard(name) => {
            let cards = &mut possibility.state.floor_state.shop_mut().cards;

            let position = cards
                .iter()
                .position(|(card, _)| card.base.name == name)
                .expect("Unable to find card that matches in shop");

            let (_, cost) = cards.remove(position);

            possibility.state.spend_gold(cost);
            possibility.state.add_card(Card::by_name(&name));
        }
        Choice::BuyPotion(name) => {
            let potions = &mut possibility.state.floor_state.shop_mut().potions;
            let position = potions
                .iter()
                .position(|(potion, _)| potion.name == name)
                .expect("Unable to find potion that matches in shop");

            let (_, cost) = potions.remove(position);

            possibility.state.spend_gold(cost);
            possibility.state.add_potion(models::potions::by_name(&name));
        }
        Choice::BuyRelic(name) => {
            let relics = &mut possibility.state.floor_state.shop_mut().relics;

            let position = relics
                .iter()
                .position(|(relic, _)| relic.name == name)
                .expect("Unable to find relic that matches in shop");
            let (_, cost) = relics.remove(position);

            possibility.state.spend_gold(cost);
            possibility.add_relic(models::relics::by_name(&name));
        }
        Choice::BuyRemoveCard(card) => {
            let cost = possibility.state.purge_cost();
            possibility.state.spend_gold(cost);
            possibility.state.remove_card(card.uuid);
            possibility.state.purge_count += 1;
            possibility.state.floor_state.shop_mut().can_purge = false;
        }
        Choice::DeckSelect(cards) => if let ScreenState::DeckChoose(_, operation) = possibility.state.screen_state {
            match operation {
                DeckOperation::Duplicate => {
                    for card in cards {
                        let mut new_card = possibility.state.deck[&card.uuid].clone();
                        new_card.uuid = Uuid::new_v4();
                        possibility.state.deck.insert(new_card.uuid, new_card);
                    }
                }
                DeckOperation::Remove => {
                    for card in cards {
                        possibility.state.remove_card(card.uuid);
                    }
                }
                DeckOperation::Transform | DeckOperation::TransformUpgrade => {
                    let sets: Vec<Vec<&&'static models::cards::BaseCard>> = cards
                        .iter()
                        .map(|p| {
                            let available_cards: Vec<&&'static models::cards::BaseCard> =
                                models::cards::available_cards_by_class(p.base._class)
                                    .iter()
                                    .filter(move |c| c.name != p.base.name)
                                    .collect();

                            available_cards
                        })
                        .collect();

                    for card in cards {
                        possibility.state.remove_card(card.uuid);
                    }

                    for set in sets {
                        let base = possibility.probability.choose(set).unwrap();
                        let mut card = Card::new(base);
                        if operation == DeckOperation::TransformUpgrade {
                            card.upgrade()
                        }
                        possibility.state.add_card(card);
                    }
                }
                DeckOperation::Upgrade => {
                    for card in cards {
                        possibility.state.deck[&card.uuid].upgrade();
                    }
                }
            }
        } else {
            panic!("DeckSelect choice not in deck choose screen");
        },
        Choice::Dig => {
            let relic = possibility.random_relic(None, None, None, false);
            possibility.add_relic(relic);
        }
        Choice::DiscardPotion { slot } => {
            possibility.state.potions[slot] = None;
        }
        Choice::DrinkPotion { slot, target } => possibility.drink_potion(
            possibility.state.potion_at(slot).unwrap(),
            target.map(|m| m.creature_ref()),
        ),
        Choice::End => possibility.end_turn(),
        Choice::EnterShop => {
            if !possibility.state.floor_state.shop().generated {
                let mut discount = 1.0;
                if possibility.state.relics.contains("The Courier") {
                    discount = 0.8;
                }
                if possibility.state.relics.contains("Membership Card") {
                    discount /= 2.0;
                }

                let available_cards =
                    models::cards::available_cards_by_class(possibility.state.class);
                let available_attacks = available_cards
                    .iter()
                    .copied()
                    .filter(|c| c._type == CardType::Attack)
                    .collect_vec();
                let attack1 = possibility.generate_card_offer(None, &available_attacks);
                let attack2 = possibility.generate_card_offer(
                    None,
                    &available_attacks
                        .into_iter()
                        .filter(|c| c.name != attack1.base.name)
                        .collect_vec(),
                );

                let available_skills = available_cards
                    .iter()
                    .copied()
                    .filter(|c| c._type == CardType::Skill)
                    .collect_vec();
                let skill1 = possibility.generate_card_offer(None, &available_skills);
                let skill2 = possibility.generate_card_offer(
                    None,
                    &available_skills
                        .into_iter()
                        .filter(|c| c.name != skill1.base.name)
                        .collect_vec(),
                );

                let available_powers = available_cards
                    .iter()
                    .copied()
                    .filter(|c| c._type == CardType::Power)
                    .collect_vec();
                let power1 = possibility.generate_card_offer(None, &available_powers);

                let available_colorless =
                    models::cards::available_cards_by_class(models::core::Class::None);
                let available_colorless_uncommon = available_colorless
                    .iter()
                    .copied()
                    .filter(|c| c.rarity == models::core::Rarity::Uncommon)
                    .collect_vec();
                let available_colorless_rare = available_colorless
                    .iter()
                    .copied()
                    .filter(|c| c.rarity == models::core::Rarity::Rare)
                    .collect_vec();

                let colorless1 =
                    possibility.generate_card_offer(None, &available_colorless_uncommon);
                let colorless2 = possibility.generate_card_offer(None, &available_colorless_rare);

                let cards = vec![
                    attack1, attack2, skill1, skill2, power1, colorless1, colorless2,
                ];
                let on_sale = possibility.probability.range(5);

                let cards = cards
                    .into_iter()
                    .enumerate()
                    .map(|(p, c)| {
                        let (min, max) = match c.base._class {
                            models::core::Class::None => match c.base.rarity {
                                models::core::Rarity::Uncommon => (81, 91),
                                models::core::Rarity::Rare => (162, 198),
                                _ => panic!("Unexpected rarity"),
                            },
                            _ => match c.base.rarity {
                                models::core::Rarity::Common => (45, 55),
                                models::core::Rarity::Uncommon => (68, 82),
                                models::core::Rarity::Rare => (135, 165),
                                _ => panic!("Unexpected rarity"),
                            },
                        };

                        let final_discount = if p == on_sale {
                            discount / 2.0
                        } else {
                            discount
                        };

                        let min = (min as f64 * final_discount).ceil() as usize;
                        let max = (max as f64 * final_discount).ceil() as usize;

                        (c, (possibility.probability.range(max - min) + min) as u16)
                    })
                    .collect();

                let relic1 = possibility.random_relic(None, None, None, true);
                let relic2 = possibility.random_relic(None, None, Some(relic1), true);
                let relic3 = possibility.random_relic(None, Some(Rarity::Shop), None, true);

                let relics = vec![relic1, relic2, relic3];
                let relics_prices = relics
                    .into_iter()
                    .map(|r| {
                        let (min, max) = match r.rarity {
                            Rarity::Shop | Rarity::Common => (143, 157),
                            Rarity::Uncommon => (238, 262),
                            Rarity::Rare => (285, 315),
                            _ => panic!("Unexpected rarity"),
                        };

                        (r, (possibility.probability.range(max - min) + min) as u16)
                    })
                    .collect();

                let potions = (0..3)
                    .map(|_| {
                        let potion = possibility.random_potion(false);
                        let (min, max) = match potion.rarity {
                            Rarity::Common => (48, 52),
                            Rarity::Uncommon => (72, 78),
                            Rarity::Rare => (95, 105),
                            _ => panic!("Unexpected rarity"),
                        };

                        (
                            potion,
                            (possibility.probability.range(max - min) + min) as u16,
                        )
                    })
                    .collect();

                possibility.state.floor_state = FloorState::Shop(ShopState {
                    generated: true,
                    can_purge: true,
                    cards,
                    relics: relics_prices,
                    potions,
                });
            }

            possibility.state.screen_state = ScreenState::InShop
        }
        Choice::Event(name) => {
            let event = possibility.state.floor_state.event().base;
            let choice = event
                .choices
                .iter()
                .find(|base| base.name == name)
                .unwrap();
            if choice.effects.is_empty() {
                possibility.state.screen_state = ScreenState::Map;
            } else {
                possibility.eval_effects(&choice.effects, Binding::CurrentEvent, None);
            }
        }
        Choice::Lift => {
            possibility.state.relics.find_mut("Girya").unwrap().vars.x += 1;
        }
        Choice::NavigateToNode(node) => {
            possibility.state.map.floor += 1;
            if let Some(relic) = possibility.state.relics.find("Maw Bank") {
                if relic.enabled {
                    possibility.state.gold += 12;
                }
            }
            let node = possibility.state.map.nodes[&(possibility.state.map.floor, node)].clone();
            let act = &models::acts::ACTS[possibility.state.act as usize];
            match node.icon {
                MapNodeIcon::Boss(name) => {
                    let boss = act.bosses.iter().find(|b| b.name == name).unwrap();
                    let monsters = eval_monster_set(&boss.monsters, possibility);
                    possibility.fight(&monsters, FightType::Boss)
                }
                MapNodeIcon::BurningElite | MapNodeIcon::Elite => {
                    let options = if let Some(last) = possibility.state.map.history.last_elite {
                        let mut vec = (0..last).collect_vec();
                        vec.extend((last + 1)..act.elites.len());
                        vec
                    } else {
                        (0..act.elites.len()).collect_vec()
                    };

                    let choice = possibility.probability.choose(options).unwrap();
                    let monsters = eval_monster_set(&act.elites[choice], possibility);
                    possibility.fight(
                        &monsters,
                        FightType::Elite {
                            burning: node.icon == MapNodeIcon::BurningElite,
                        },
                    );
                }
                MapNodeIcon::Campfire => {
                    possibility.state.floor_state = FloorState::Rest;
                    possibility.eval_when(When::RoomEnter(RoomType::Rest))
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
                    if possibility.state.relics.contains("Ssserpent Head") {
                        possibility.state.gold = 50;
                    }

                    let mut normal_probability =
                        (possibility.state.map.history.unknown_normal_count + 1) * 10;
                    let mut shop_probability =
                        (possibility.state.map.history.unknown_shop_count + 1) * 3;
                    let mut treasure_probability =
                        (possibility.state.map.history.unknown_treasure_count + 1) * 2;

                    if let FloorState::Shop(_) = possibility.state.floor_state {
                        shop_probability = 0
                    }

                    if let Some(relic) = possibility.state.relics.find_mut("Tiny Chest") {
                        relic.vars.x += 1;
                        if relic.vars.x == 4 {
                            relic.vars.x = 0;
                            shop_probability = 0;
                            treasure_probability = 100;
                            normal_probability = 0;
                        }
                    }

                    if possibility.state.relics.contains("Juzu Bracelet") {
                        normal_probability = 0;
                    }

                    let mut total_probability =
                        normal_probability + shop_probability + treasure_probability;
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
                        (UnknownRoom::Event, 100 - total_probability),
                    ];

                    let choice = *possibility.probability.choose_weighted(&choices).unwrap();

                    match choice {
                        UnknownRoom::Fight => {
                            possibility.state.map.history.unknown_normal_count = 0;
                            possibility.state.map.history.unknown_shop_count += 1;
                            possibility.state.map.history.unknown_treasure_count += 1;
                            normal_fight(possibility)
                        }
                        UnknownRoom::Shop => {
                            possibility.state.map.history.unknown_normal_count += 1;
                            possibility.state.map.history.unknown_shop_count = 0;
                            possibility.state.map.history.unknown_treasure_count += 1;
                            shop(possibility);
                        }
                        UnknownRoom::Treasure => {
                            possibility.state.map.history.unknown_normal_count += 1;
                            possibility.state.map.history.unknown_shop_count += 1;
                            possibility.state.map.history.unknown_treasure_count = 0;
                            treasure(possibility);
                        }
                        UnknownRoom::Event => {
                            possibility.state.map.history.unknown_normal_count += 1;
                            possibility.state.map.history.unknown_shop_count += 1;
                            possibility.state.map.history.unknown_treasure_count += 1;
                            event(possibility);
                        }
                    }
                }
                MapNodeIcon::Shop => shop(possibility),
            }
        }
        Choice::OpenChest => {
            if let FloorState::Chest(chest) = possibility.state.floor_state {
                let relic = possibility.random_relic(Some(chest), None, None, false);
                let (gold_chance, gold_min, gold_max) = match chest {
                    ChestType::Small => (50, 23, 27),
                    ChestType::Medium => (35, 45, 55),
                    ChestType::Large => (50, 68, 82),
                    ChestType::Boss => (0, 0, 0),
                };
                let gets_gold = *possibility
                    .probability
                    .choose_weighted(&[(true, gold_chance), (false, 100 - gold_chance)])
                    .unwrap();
                let mut rewards = 
                if !possibility.state.keys.map(|k| k.sapphire).unwrap_or(true) {
                    vector![
                        Reward::SapphireKey,
                        Reward::SapphireLinkedRelic(relic)
                    ]
                } else {
                    vector![Reward::Relic(relic)]
                };

                let extra_relic = if let Some(relic) = possibility.state.relics.find_mut("Matryoshka") {
                    if relic.vars.x < 2 {
                        relic.vars.x += 1;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };

                if extra_relic {
                    let relic = possibility.random_relic(Some(chest), None, Some(relic), false);
                    rewards.insert(0, Reward::Relic(relic))
                }

                if gets_gold {
                    let gold_amount =
                        (possibility.probability.range(gold_max - gold_min) + gold_min) as u16;
                    rewards.push_back(Reward::Gold(gold_amount));
                };

                possibility.state.floor_state = FloorState::Rewards(rewards)
            } else {
                panic!("Floor state is not a chest!")
            }
        }
        Choice::PlayCard { card, target } => {
            possibility.play_card(card, target.map(|t| t.creature_ref()))
        }
        Choice::Proceed => possibility.state.screen_state = ScreenState::Map,
        Choice::Recall => {
            possibility.state.keys.as_mut().unwrap().ruby = true;
            possibility.state.screen_state = ScreenState::Proceed;
        }
        Choice::Rest => {
            possibility.heal(
                possibility.state.player.creature.max_hp as f64 * 0.3,
                CreatureReference::Player,
            );
            possibility.eval_when(When::Rest);
            possibility.state.screen_state = ScreenState::Proceed;
        }
        Choice::Scry(cards) => {
            let battle = possibility.state.floor_state.battle_mut();
            
            for card in cards {
                battle.move_card(CardDestination::DiscardPile, card, &mut possibility.probability);
            }

            possibility.eval_when(When::Scry)
        }
        Choice::AddCardToDeck(card) => {
            let card = Card::by_name(&card);
            possibility.state.add_card(card);
            remove_card_reward(possibility);
        }
        Choice::SelectCards(cards) => {
            let effects = 
            if let ScreenState::CardChoose(choice_state) = &possibility.state.screen_state {
                choice_state.then.iter().cloned().collect_vec()
            } else {
                panic!("Screen state is not card choose!")
            };
            
            for card in cards {
                possibility.eval_card_effects(&effects, card)
            }
        }
        Choice::SingingBowl => {
            possibility.state.player.add_max_hp(2, &possibility.state.relics);
            remove_card_reward(possibility);
        }
        Choice::Skip => {
            possibility.state.screen_state = ScreenState::Normal;
        }
        Choice::Smith(card) => {
            possibility.state.deck[&card.uuid].upgrade();
            possibility.state.screen_state = ScreenState::Proceed;
        }
        Choice::Start {player_class, ascension} => {
            *possibility = GamePossibility {
                state: GameState::new(player_class, ascension.unwrap_or(0)),
                probability: Probability::new()
            };
        }
        Choice::State => {}
        Choice::TakeReward(reward_index) => {
            let (card_reward, gold_reward) = if let FloorState::Rewards(rewards) = &possibility.state.floor_state {
                let reward = &rewards[reward_index];
                match reward {
                    Reward::CardChoice(_, fight_type, colorless) => {
                        (Some((*fight_type, *colorless)), None)
                    }
                    Reward::Gold(amount) => {
                        (None, Some(*amount))
                    }
                    _ => (None, None)
                }
            } else {
                panic!("Floor state is not rewards!")
            };

            let card_offers = card_reward.map(|(fight_type, colorless)| {
                possibility.generate_card_rewards(fight_type, colorless)
            });

            if let Some(gold) = gold_reward {
                possibility.state.add_gold(gold);
            }


            if let FloorState::Rewards(rewards) = &mut possibility.state.floor_state {
                let reward = &mut rewards[reward_index];
                match reward {
                    Reward::CardChoice(offer, _, _) => {
                        if offer.is_empty() {
                            offer.extend(card_offers.unwrap());
                        }
                    }
                    Reward::EmeraldKey => {
                        possibility.state.keys.as_mut().unwrap().emerald = true;
                        rewards.remove(reward_index);
                    }
                    Reward::Gold(_) => {
                        rewards.remove(reward_index);
                    }
                    Reward::Potion(potion) => {
                        if let Some(empty) = possibility.state.potions.iter_mut().find(|f| f.is_none()) {
                            *empty = Some(*potion);
                            rewards.remove(reward_index);
                        }
                    }
                    Reward::Relic(relic) => {
                        possibility.state.relics.add(relic);
                        rewards.remove(reward_index);
                    }
                    Reward::SapphireKey => {
                        possibility.state.keys.as_mut().unwrap().sapphire = true;
                        rewards.remove(reward_index);
                        rewards.remove(reward_index);//Remove linked relic
                    }
                    Reward::SapphireLinkedRelic(relic) => {
                        possibility.state.relics.add(relic);
                        rewards.remove(reward_index-1); //Remove linked key
                        rewards.remove(reward_index-1);
                    }
                    
                }
            } else {
                panic!("Floor state is not rewards!")
            }
        }
        Choice::Toke(card) => {
            possibility.state.deck.remove(&card.uuid);
            possibility.state.screen_state = ScreenState::Proceed;
        }
    }
}

fn remove_card_reward(possibility: &mut GamePossibility) {
    if let ScreenState::CardReward(_, index) = possibility.state.screen_state {    
        if let Some(index) = index {
            if let FloorState::Rewards(rewards) = &mut possibility.state.floor_state {
                rewards.remove(index);
            } else {
                panic!("Floor state is not a rewards!")
            }
        }
    } else {
        panic!("Not in card reward screen!")
    }

    possibility.state.screen_state = ScreenState::Normal;
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum UnknownRoom {
    Event,
    Fight,
    Shop,
    Treasure,
}

fn event(possibility: &mut GamePossibility) {
    let act = &models::acts::ACTS[possibility.state.act as usize];
    let events = act
        .events
        .iter()
        .filter(|f| possibility.state.map.history.event_history.contains(*f))
        .map(|n| models::events::by_name(n.as_str()))
        .filter(|e| possibility.eval_condition(&e.condition, Binding::CurrentEvent, None))
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

    let event_set = if is_shrine { shrines } else { nonshrines };

    let base_event = possibility.probability.choose(event_set).unwrap();

    possibility.state.floor_state = FloorState::Event(Event::new(base_event))
}

fn shop(possibility: &mut GamePossibility) {
    possibility.state.floor_state = FloorState::Shop(ShopState {
        generated: false,
        cards: vector![],
        potions: vector![],
        relics: vector![],
        can_purge: true,
    });

    possibility.state.screen_state = ScreenState::Normal
}

fn treasure(possibility: &mut GamePossibility) {
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
    if possibility.state.map.history.easy_fight_count == act.easy_count {
        possibility.state.map.history.last_normal = None
    }

    possibility.state.map.history.easy_fight_count += 1;

    let fights = if possibility.state.map.history.easy_fight_count <= act.easy_count {
        &act.easy_fights
    } else {
        &act.normal_fights
    };

    let options = if let Some(last) = possibility.state.map.history.last_normal {
        fights[0..last]
            .iter()
            .chain(fights[last + 1..fights.len()].iter())
            .collect_vec()
    } else {
        fights.iter().collect_vec()
    };

    let probabilities = options
        .iter()
        .map(|f| (&f.set, f.probability))
        .collect_vec();
    let fight = possibility
        .probability
        .choose_weighted(&probabilities)
        .unwrap();
    let monsters = eval_monster_set(fight, possibility);
    possibility.fight(&monsters, FightType::Common);
}

fn eval_monster_set(set: &MonsterSet, possibility: &mut GamePossibility) -> Vec<String> {
    match set {
        MonsterSet::ChooseN { n, choices } => possibility
            .probability
            .choose_multiple(choices.to_vec(), *n as usize),
        MonsterSet::Fixed(monsters) => monsters.to_vec(),
        MonsterSet::RandomSet(sets) => possibility.probability.choose(sets.to_vec()).unwrap(),
    }
}
