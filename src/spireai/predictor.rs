use crate::models;
use crate::models::acts::MonsterSet;
use crate::models::core::{CardDestination, ChestType, DeckOperation, FightType, When};
use crate::spireai::*;
use crate::state::battle::BattleState;
use crate::state::core::{Card, Reward, RewardState};
use crate::state::event::{EventScreenState, EventState};
use crate::state::floor::{ChestState, FloorState, RestScreenState, RestState};
use crate::state::map::MapNodeIcon;
use crate::state::shop::{ShopScreenState, ShopState};
use im::vector;
use models::choices::Choice;

pub fn predict_outcome(choice: Choice, possibility: &mut GamePossibility) {
    match choice {
        Choice::BuyCard(index) => {
            if let FloorState::Shop(shop) = &mut possibility.state {
                shop.buy_card(index, &mut possibility.probability);
            } else {
                panic!("Expected a Shop in BuyCard");
            }
        }
        Choice::BuyPotion(index) => {
            if let FloorState::Shop(shop) = &mut possibility.state {
                shop.buy_potion(index, &mut possibility.probability);
            } else {
                panic!("Expected a Shop in BuyPotion");
            }
        }
        Choice::BuyRelic(index) => {
            if let FloorState::Shop(shop) = &mut possibility.state {
                shop.buy_relic(index, &mut possibility.probability)
            } else {
                panic!("Expected a Shop in BuyPotion");
            }
        }
        Choice::BuyRemoveCard(card) => {
            if let FloorState::Shop(shop) = &mut possibility.state {
                shop.purge(card);
            } else {
                panic!("Expected a Shop in BuyPotion");
            }
        }
        Choice::DeckSelect(cards, operation) => {
            match operation {
                DeckOperation::Duplicate => {
                    let state = possibility.state.game_state_mut();
                    for card in cards {
                        let mut new_card = state.deck[&card.uuid].clone();
                        new_card.uuid = Uuid::new_v4();
                        state.deck.insert(new_card.uuid, new_card);
                    }
                }
                DeckOperation::Remove => {
                    let state = possibility.state.game_state_mut();
                    for card in cards {
                        state.remove_card(card.uuid);
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

                    let state = possibility.state.game_state_mut();
                    for card in cards {
                        state.remove_card(card.uuid);
                    }

                    for set in sets {
                        let base = possibility.probability.choose(set).unwrap();
                        let mut card = Card::new(base);
                        if operation == DeckOperation::TransformUpgrade {
                            card.upgrade()
                        }
                        state.add_card(card);
                    }
                }
                DeckOperation::Upgrade => {
                    let state = possibility.state.game_state_mut();
                    for card in cards {
                        state.deck[&card.uuid].upgrade();
                    }
                }
                DeckOperation::BottleFlame
                | DeckOperation::BottleLightning
                | DeckOperation::BottleTornado => {
                    let state = possibility.state.game_state_mut();
                    for card in cards {
                        state.deck[&card.uuid].bottled = true;
                    }
                }
            }

            match &mut possibility.state {
                FloorState::Rest(rest) => {
                    rest.screen_state = RestScreenState::Proceed;
                }
                FloorState::Event(event) => event.screen_state = None,
                FloorState::Shop(shop) => {
                    shop.screen_state = ShopScreenState::InShop;
                }
                _ => panic!("Unexpected floor state when performing a deck operation!"),
            }
        }
        Choice::Dig => {
            if let FloorState::Rest(rest) = &mut possibility.state {
                let relic =
                    rest.game_state
                        .random_relic(None, None, false, &mut possibility.probability);
                rest.screen_state = RestScreenState::Dig(RewardState {
                    rewards: vector![Reward::Relic(relic)],
                    viewing_reward: None,
                    deck_operation: None,
                });
            } else {
                panic!("Expected a Rest in Dig");
            }
        }
        Choice::DiscardPotion { slot } => {
            let state = possibility.state.game_state_mut();
            state.potions[slot] = None;
        }
        Choice::DrinkPotion { slot, target } => {
            if let FloorState::Battle(battle) = &mut possibility.state {
                battle.drink_potion(
                    battle.game_state.potion_at(slot).unwrap(),
                    target,
                    &mut possibility.probability,
                )
            } else {
                let state = possibility.state.game_state_mut();
                state.drink_potion(
                    state.potion_at(slot).unwrap(),
                    true,
                    &mut possibility.probability,
                )
            }
        }
        Choice::End => {
            if let FloorState::Battle(battle) = &mut possibility.state {
                battle.end_turn(&mut possibility.probability)
            } else {
                panic!("Expected a battle in Choice::End")
            }
        }
        Choice::EnterShop => {
            if let FloorState::Shop(shop) = &mut possibility.state {
                shop.generate(&mut possibility.probability);
                shop.screen_state = ShopScreenState::InShop
            } else {
                panic!("Expected a Shop in EnterShop");
            }
        }
        Choice::Event(name) => {
            if let FloorState::Event(event) = &mut possibility.state {
                let choice = event
                    .base
                    .choices
                    .iter()
                    .find(|base| base.name == name)
                    .unwrap();
                if choice.effects.is_empty() {
                    let state = std::mem::take(&mut event.game_state);
                    possibility.state = FloorState::Map(state)
                } else {
                    event.eval_effects(&choice.effects);
                }
            } else {
                panic!("Expected an Event in Event");
            }
        }
        Choice::Lift => {
            if let FloorState::Rest(rest) = &mut possibility.state {
                rest.game_state.relics.find_mut("Girya").unwrap().vars.x += 1;
                rest.screen_state = RestScreenState::Proceed
            } else {
                panic!("Expected a Shop in Lift");
            }
        }
        Choice::NavigateToNode(node) => {
            let floor: FloorState = if let FloorState::Map(state) = &mut possibility.state {
                let mut state = std::mem::take(state);
                state.map.floor += 1;
                if let Some(relic) = state.relics.find("Maw Bank") {
                    if relic.enabled {
                        state.gold += 12;
                    }
                }

                let icon = state.map.nodes[&(state.map.floor, node)].icon.clone();
                let last_shop = state.map.history.last_shop;
                state.map.history.last_shop = false;
                let act = &models::acts::ACTS[state.act as usize];
                match icon {
                    MapNodeIcon::Boss(name) => {
                        let boss = act.bosses.iter().find(|b| b.name == name).unwrap();
                        let monsters =
                            eval_monster_set(&boss.monsters, &mut possibility.probability);
                        FloorState::Battle(BattleState::new(
                            state,
                            &monsters,
                            FightType::Boss,
                            &mut possibility.probability,
                        ))
                    }
                    MapNodeIcon::BurningElite | MapNodeIcon::Elite => {
                        let options = if let Some(last) = state.map.history.last_elite {
                            let mut vec = (0..last).collect_vec();
                            vec.extend((last + 1)..act.elites.len());
                            vec
                        } else {
                            (0..act.elites.len()).collect_vec()
                        };

                        let choice = possibility.probability.choose(options).unwrap();
                        let monsters =
                            eval_monster_set(&act.elites[choice], &mut possibility.probability);

                        FloorState::Battle(BattleState::new(
                            state,
                            &monsters,
                            FightType::Elite {
                                burning: icon == MapNodeIcon::BurningElite,
                            },
                            &mut possibility.probability,
                        ))
                    }
                    MapNodeIcon::Campfire => {
                        if let Some(relic) = state.relics.find_mut("Ancient Tea Set") {
                            relic.enabled = true;
                        }
                        if state.relics.contains("Eternal Feather") {
                            state.heal((state.deck.len() / 5 * 3) as f64)
                        }
                        FloorState::Rest(RestState {
                            screen_state: RestScreenState::IShouldRest,
                            game_state: state,
                        })
                    }
                    MapNodeIcon::Chest => treasure(state, &mut possibility.probability),
                    MapNodeIcon::Monster => normal_fight(state, &mut possibility.probability),
                    MapNodeIcon::Question => {
                        if state.relics.contains("Ssserpent Head") {
                            state.gold += 50;
                        }

                        let mut normal_probability =
                            (state.map.history.unknown_normal_count + 1) * 10;
                        let mut shop_probability = (state.map.history.unknown_shop_count + 1) * 3;
                        let mut treasure_probability =
                            (state.map.history.unknown_treasure_count + 1) * 2;

                        if last_shop {
                            shop_probability = 0;
                        }

                        if let Some(relic) = state.relics.find_mut("Tiny Chest") {
                            relic.vars.x += 1;
                            if relic.vars.x == 4 {
                                relic.vars.x = 0;
                                shop_probability = 0;
                                treasure_probability = 100;
                                normal_probability = 0;
                            }
                        }

                        if state.relics.contains("Juzu Bracelet") {
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
                                state.map.history.unknown_normal_count = 0;
                                state.map.history.unknown_shop_count += 1;
                                state.map.history.unknown_treasure_count += 1;
                                normal_fight(state, &mut possibility.probability)
                            }
                            UnknownRoom::Shop => {
                                state.map.history.unknown_normal_count += 1;
                                state.map.history.unknown_shop_count = 0;
                                state.map.history.unknown_treasure_count += 1;
                                shop(state)
                            }
                            UnknownRoom::Treasure => {
                                state.map.history.unknown_normal_count += 1;
                                state.map.history.unknown_shop_count += 1;
                                state.map.history.unknown_treasure_count = 0;
                                treasure(state, &mut possibility.probability)
                            }
                            UnknownRoom::Event => {
                                state.map.history.unknown_normal_count += 1;
                                state.map.history.unknown_shop_count += 1;
                                state.map.history.unknown_treasure_count += 1;
                                event(state, &mut possibility.probability)
                            }
                        }
                    }
                    MapNodeIcon::Shop => shop(state),
                }
            } else {
                panic!("Unexpected floor state!")
            };

            possibility.state = floor;
        }
        Choice::OpenChest => {
            if let FloorState::Chest(chest) = &mut possibility.state {
                let relic = chest.game_state.random_relic(
                    Some(chest.chest),
                    None,
                    false,
                    &mut possibility.probability,
                );
                let (gold_chance, gold_min, gold_max) = match chest.chest {
                    ChestType::Small => (50, 23, 27),
                    ChestType::Medium => (35, 45, 55),
                    ChestType::Large => (50, 68, 82),
                    ChestType::Boss => (0, 0, 0),
                };
                let gets_gold = *possibility
                    .probability
                    .choose_weighted(&[(true, gold_chance), (false, 100 - gold_chance)])
                    .unwrap();
                let mut rewards = if !chest.game_state.keys.map(|k| k.sapphire).unwrap_or(true) {
                    vector![Reward::SapphireKey, Reward::SapphireLinkedRelic(relic)]
                } else {
                    vector![Reward::Relic(relic)]
                };

                let extra_relic =
                    if let Some(relic) = chest.game_state.relics.find_mut("Matryoshka") {
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
                    let relic = chest.game_state.random_relic(
                        Some(chest.chest),
                        None,
                        false,
                        &mut possibility.probability,
                    );
                    rewards.insert(0, Reward::Relic(relic))
                }

                if gets_gold {
                    let gold_amount =
                        (possibility.probability.range(gold_max - gold_min) + gold_min) as u16;
                    rewards.push_back(Reward::Gold(gold_amount));
                };

                chest.rewards = Some(RewardState {
                    viewing_reward: None,
                    rewards,
                    deck_operation: None,
                })
            } else {
                panic!("Floor state is not a chest!")
            }
        }
        Choice::PlayCard { card, target } => {
            if let FloorState::Battle(battle) = &mut possibility.state {
                battle.play_card(card, target, &mut possibility.probability)
            } else {
                panic!("Expected a battle in Choice::PlayCard")
            }
        }
        Choice::Proceed => {
            let state = std::mem::take(possibility.state.game_state_mut());
            possibility.state = FloorState::Map(state)
        }
        Choice::Recall => {
            if let FloorState::Rest(rest) = &mut possibility.state {
                rest.game_state.keys.as_mut().unwrap().ruby = true;
                rest.screen_state = RestScreenState::Proceed;
            } else {
                panic!("Expected a rest in Choice::Recall")
            }
        }
        Choice::Rest => {
            if let FloorState::Rest(rest) = &mut possibility.state {
                let mut amount = rest.game_state.hp.max as f64 * 0.3;
                if rest.game_state.relics.contains("Regal Pillow") {
                    amount += 15.0;
                }
                rest.game_state.heal(amount);
                if rest.game_state.relics.contains("Dream Catcher") {
                    let offers = rest.game_state.generate_card_rewards(
                        None,
                        false,
                        &mut possibility.probability,
                    );
                    rest.screen_state = RestScreenState::DreamCatch(offers)
                } else {
                    rest.screen_state = RestScreenState::Proceed
                }
            } else {
                panic!("Expected a rest in Choice::Rest")
            }
        }
        Choice::Scry(cards) => {
            if let FloorState::Battle(battle) = &mut possibility.state {
                for card in cards {
                    battle.move_card(
                        CardDestination::DiscardPile,
                        card,
                        &mut possibility.probability,
                    );
                }

                battle.eval_when(When::Scry, &mut possibility.probability);
            } else {
                panic!("Expected a battle in Choice::PlayCard")
            }
        }
        Choice::AddCardToDeck(card) => {
            let card = Card::by_name(&card);
            match &mut possibility.state {
                FloorState::Battle(_) => panic!("Unexpected battle state when adding card to deck"),
                FloorState::Rest(rest) => {
                    // Dreamcatching
                    rest.game_state.add_card(card);
                    rest.screen_state = RestScreenState::Proceed
                }
                FloorState::BattleOver(state) => {
                    state.game_state.add_card(card);
                    remove_card_reward(&mut state.rewards);
                }
                FloorState::Chest(chest) => {
                    chest.game_state.add_card(card);
                    if let Some(rewards) = &mut chest.rewards {
                        remove_card_reward(rewards);
                    } else {
                        panic!("Expected a rewards screen when adding a card to deck")
                    }
                }
                FloorState::Event(event) => {
                    event.game_state.add_card(card);
                    if let Some(screen_state) = &mut event.screen_state {
                        match screen_state {
                            EventScreenState::Rewards(reward) => remove_card_reward(reward),
                            _ => panic!("Expected a rewards screen when adding a card to deck"),
                        }
                    }
                }
                FloorState::GameOver(_) => {
                    panic!("Unexpected game over state when adding card to deck")
                }
                FloorState::Map(_) => panic!("Unexpected map state when adding card to deck"),
                FloorState::Menu => panic!("Unexpected menu state when adding card to deck"),
                FloorState::Shop(shop) => {
                    shop.game_state.add_card(card);
                    if let ShopScreenState::Reward(reward) = &mut shop.screen_state {
                        remove_card_reward(reward)
                    } else {
                        panic!("Expected a rewards screen when adding card to deck")
                    }
                }
            }
        }
        Choice::SelectCards(cards) => {
            if let FloorState::Battle(battle) = &mut possibility.state {
                if let Some(choice) = battle.card_choose.clone() {
                    let then = choice.then.into_iter().collect_vec();
                    for card in cards {
                        battle.eval_card_effects(&then, card, &mut possibility.probability)
                    }
                }
            } else {
                panic!("Expected Battle state during SelectCards choice")
            }
        }
        Choice::SingingBowl => {
            match &mut possibility.state {
                FloorState::Battle(_) => panic!("Unexpected battle state during singing bowl"),
                FloorState::Rest(rest) => {
                    // Dreamcatching
                    rest.game_state.add_max_hp(2);
                    rest.screen_state = RestScreenState::Proceed
                }
                FloorState::BattleOver(state) => {
                    state.game_state.add_max_hp(2);
                    remove_card_reward(&mut state.rewards);
                }
                FloorState::Chest(chest) => {
                    chest.game_state.add_max_hp(2);
                    if let Some(rewards) = &mut chest.rewards {
                        remove_card_reward(rewards);
                    } else {
                        panic!("Expected a rewards screen during singing bowl")
                    }
                }
                FloorState::Event(event) => {
                    event.game_state.add_max_hp(2);
                    if let Some(screen_state) = &mut event.screen_state {
                        match screen_state {
                            EventScreenState::Rewards(reward) => remove_card_reward(reward),
                            _ => panic!("Expected a rewards screen during singing bowl"),
                        }
                    }
                }
                FloorState::GameOver(_) => panic!("Unexpected GameOver state during singing bowl"),
                FloorState::Map(_) => panic!("Unexpected map state during singing bowl"),
                FloorState::Menu => panic!("Unexpected menu state during singing bowl"),
                FloorState::Shop(shop) => {
                    shop.game_state.add_max_hp(2);
                    if let ShopScreenState::Reward(reward) = &mut shop.screen_state {
                        remove_card_reward(reward)
                    } else {
                        panic!("Expected a rewards screen during singing bowl")
                    }
                }
            }
        }
        Choice::Skip => {
            match &mut possibility.state {
                FloorState::Battle(_) => panic!("Unexpected battle state during skip"),
                FloorState::Rest(rest) => {
                    // Dreamcatching
                    rest.screen_state = RestScreenState::Proceed
                }
                FloorState::BattleOver(state) => {
                    state.rewards.viewing_reward = None;
                }
                FloorState::Chest(chest) => {
                    if let Some(rewards) = &mut chest.rewards {
                        rewards.viewing_reward = None;
                    } else {
                        panic!("Expected a rewards screen during skip")
                    }
                }
                FloorState::Event(event) => {
                    if let Some(screen_state) = &mut event.screen_state {
                        match screen_state {
                            EventScreenState::Rewards(reward) => {
                                reward.viewing_reward = None;
                            }
                            _ => panic!("Expected a rewards screen during skip"),
                        }
                    }
                }
                FloorState::GameOver(_) => panic!("Unexpected GameOver state during skip"),
                FloorState::Map(_) => panic!("Unexpected map state during skip"),
                FloorState::Menu => panic!("Unexpected menu state during skip"),
                FloorState::Shop(shop) => {
                    if let ShopScreenState::Reward(reward) = &mut shop.screen_state {
                        reward.viewing_reward = None
                    } else {
                        panic!("Expected a rewards screen during singing bowl")
                    }
                }
            }
        }
        Choice::Smith => {
            if let FloorState::Rest(rest) = &mut possibility.state {
                rest.screen_state = RestScreenState::Smith
            }
        }
        Choice::Start {
            player_class,
            ascension,
        } => {
            *possibility = GamePossibility {
                state: FloorState::Event(EventState::by_name(
                    "Neow",
                    GameState::new(player_class, ascension.unwrap_or(0)),
                )),
                probability: Probability::new(),
            };
        }
        Choice::State => {}
        Choice::TakeReward(reward_index) => {
            match get_rewards_mut(&mut possibility.state).rewards[reward_index].clone() {
                Reward::CardChoice(offer, fight_type, colorless) => {
                    let new_offer = if offer.is_empty() {
                        possibility.state.game_state_mut().generate_card_rewards(
                            fight_type,
                            colorless,
                            &mut possibility.probability,
                        )
                    } else {
                        vector![]
                    };

                    let reward_state = get_rewards_mut(&mut possibility.state);
                    reward_state.viewing_reward = Some(reward_index);
                    if let Reward::CardChoice(choices, _, _) =
                        &mut reward_state.rewards[reward_index]
                    {
                        choices.extend(new_offer);
                    }
                }
                Reward::EmeraldKey => {
                    if let Some(keys) = &mut possibility.state.game_state_mut().keys {
                        keys.emerald = true
                    }

                    get_rewards_mut(&mut possibility.state)
                        .rewards
                        .remove(reward_index);
                }
                Reward::Gold(amount) => {
                    possibility.state.game_state_mut().add_gold(amount);

                    get_rewards_mut(&mut possibility.state)
                        .rewards
                        .remove(reward_index);
                }
                Reward::Potion(potion) => {
                    if possibility.state.game_state_mut().add_potion(potion) {
                        get_rewards_mut(&mut possibility.state)
                            .rewards
                            .remove(reward_index);
                    }
                }
                Reward::Relic(relic) => {
                    possibility
                        .state
                        .game_state_mut()
                        .add_relic(relic, &mut possibility.probability);

                    get_rewards_mut(&mut possibility.state)
                        .rewards
                        .remove(reward_index);
                }
                Reward::SapphireKey => {
                    if let Some(keys) = &mut possibility.state.game_state_mut().keys {
                        keys.sapphire = true;
                    }
                    let rewards = &mut get_rewards_mut(&mut possibility.state).rewards;
                    rewards.remove(reward_index);
                    rewards.remove(reward_index); //Remove linked relic
                }
                Reward::SapphireLinkedRelic(relic) => {
                    possibility
                        .state
                        .game_state_mut()
                        .add_relic(relic, &mut possibility.probability);
                    let rewards = &mut get_rewards_mut(&mut possibility.state).rewards;
                    rewards.remove(reward_index - 1); //Remove linked key
                    rewards.remove(reward_index - 1);
                }
            }
        }
        Choice::Toke => {
            if let FloorState::Rest(rest_state) = &mut possibility.state {
                rest_state.screen_state = RestScreenState::Toke
            } else {
                panic!("Expected rest state when toking!")
            }
        }
    }
}

fn get_rewards_mut(state: &mut FloorState) -> &mut RewardState {
    match state {
        FloorState::Battle(_) => panic!("No rewards during Battle"),
        FloorState::GameOver(_) => panic!("No rewards during GameOver"),
        FloorState::Map(_) => panic!("No rewards during Map"),
        FloorState::Menu => panic!("No rewards during Menu"),
        FloorState::Rest(rest) => match &mut rest.screen_state {
            RestScreenState::Dig(rewards) => rewards,
            _ => panic!("Rewards only apply when digging"),
        },
        FloorState::BattleOver(state) => &mut state.rewards,
        FloorState::Chest(chest) => chest.rewards.as_mut().expect("Chest is empty!"),
        FloorState::Event(event) => {
            match event
                .screen_state
                .as_mut()
                .expect("No screen state when fetching rewards!")
            {
                EventScreenState::Rewards(rewards) => rewards,
                _ => panic!("Unexpected rewards during FloorState::Event"),
            }
        }
        FloorState::Shop(shop) => match &mut shop.screen_state {
            ShopScreenState::Reward(rewards) => rewards,
            _ => panic!("Unexpected rewards in shop"),
        },
    }
}

fn remove_card_reward(rewards: &mut RewardState) {
    if let Some(reward_index) = rewards.viewing_reward {
        rewards.rewards.remove(reward_index);
        rewards.viewing_reward = None;
    } else {
        panic!("Expected to be viewing a reward in Choice::AddCardToDeck FloorState::BattleOver")
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum UnknownRoom {
    Event,
    Fight,
    Shop,
    Treasure,
}

fn event(state: GameState, probability: &mut Probability) -> FloorState {
    let act = &models::acts::ACTS[state.act as usize];
    let events = act
        .events
        .iter()
        .filter(|f| state.map.history.event_history.contains(*f))
        .map(|n| models::events::by_name(n.as_str()))
        .filter(|e| state.eval_condition(&e.condition))
        .collect_vec();

    let shrines = events.iter().filter(|f| f.shrine).copied().collect_vec();
    let nonshrines = events.into_iter().filter(|f| !f.shrine).collect_vec();

    let is_shrine = if shrines.is_empty() {
        false
    } else if nonshrines.is_empty() {
        true
    } else {
        probability.range(4) == 0
    };

    let event_set = if is_shrine { shrines } else { nonshrines };

    let base_event = probability.choose(event_set).unwrap();

    FloorState::Event(EventState::new(base_event, state))
}

fn shop(mut state: GameState) -> FloorState {
    state.map.history.last_shop = true;

    FloorState::Shop(ShopState {
        game_state: state,
        updated: false,
        generated: false,
        cards: vector![],
        potions: vector![],
        relics: vector![],
        can_purge: true,
        screen_state: ShopScreenState::Entrance,
    })
}

fn treasure(state: GameState, probability: &mut Probability) -> FloorState {
    let types = vec![
        (ChestType::Small, 3),
        (ChestType::Medium, 2),
        (ChestType::Large, 1),
    ];
    let chest_type = probability.choose_weighted(&types).unwrap();

    FloorState::Chest(ChestState {
        chest: *chest_type,
        rewards: None,
        game_state: state,
    })
}

fn normal_fight(mut state: GameState, probability: &mut Probability) -> FloorState {
    let act = &models::acts::ACTS[state.act as usize];
    if state.map.history.easy_fight_count == act.easy_count {
        state.map.history.last_normal = None
    }

    state.map.history.easy_fight_count += 1;

    let fights = if state.map.history.easy_fight_count <= act.easy_count {
        &act.easy_fights
    } else {
        &act.normal_fights
    };

    let options = if let Some(last) = state.map.history.last_normal {
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
    let fight = probability.choose_weighted(&probabilities).unwrap();
    let monsters = eval_monster_set(fight, probability);
    FloorState::Battle(BattleState::new(
        state,
        &monsters,
        FightType::Common,
        probability,
    ))
}

fn eval_monster_set(set: &MonsterSet, probability: &mut Probability) -> Vec<String> {
    match set {
        MonsterSet::ChooseN { n, choices } => {
            probability.choose_multiple(choices.to_vec(), *n as usize)
        }
        MonsterSet::Fixed(monsters) => monsters.to_vec(),
        MonsterSet::RandomSet(sets) => probability.choose(sets.to_vec()).unwrap(),
    }
}
