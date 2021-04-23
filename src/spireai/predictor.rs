use crate::spireai::*;
use crate::models;
use models::relics::*;
use models::state::*;
use models::core::*;
use models::choices::Choice;
use rand::seq::SliceRandom;

pub fn predict_outcome(state: &GameState, choice: &Choice) -> GamePossibilitySet {
    let mut new_state = state.clone();
    let mut rng = rand::thread_rng();
    match choice {
        Choice::BuyCard(name) => {
            let cost = if let FloorState::Shop{ref mut cards, ..} = new_state.floor_state {
                let position = cards.iter().position(|(card, _)| card == name).expect("Unable to find card that matches in shop");
                let (_, cost) = cards.remove(position);
                cost
            } else { 
                panic!("Expected store floor state") 
            };

            spend_money(cost, true, &mut new_state);
            add_card_to_deck(name, false, &mut new_state);
            (new_state, 1.0)
        },

        Choice::BuyPotion(name) => {
            let cost = if let FloorState::Shop{ref mut potions, ..} = new_state.floor_state {
                let position = potions.iter().position(|(potion, _)| potion == name).expect("Unable to find potion that matches in shop");
                let (_, cost) = potions.remove(position);
                cost
            } else {
                panic!("Expected store floor state");
            };

            spend_money(cost, true, &mut new_state);
            add_potion(name, &mut new_state);
            (new_state, 1.0)
        },
        Choice::BuyRelic(name) => {
            let cost = if let FloorState::Shop{ref mut relics, ..} = new_state.floor_state {
                let position = relics.iter().position(|(relic, _)| relic == name).expect("Unable to find relic that matches in shop");
                let (_, cost) = relics.remove(position);
                cost
            } else {
                panic!("Expected store floor state");
            };

            spend_money(cost, true, &mut new_state);
            add_relic(name, &mut new_state);
            (new_state, 1.0)
        },
        Choice::BuyRemoveCard(position) => {
            let cost = if let FloorState::Shop{ref mut purge_cost, ..} = new_state.floor_state {
                let cost_ret = *purge_cost;
                *purge_cost = 0;
                cost_ret
            } else {
                panic!("Expected store floor state");
            };

            spend_money(cost, true, &mut new_state);
            remove_card_from_deck(*position, &mut new_state);
            (new_state, 1.0)
        },
        Choice::DeckRemove(positions) => {
            for position in positions {
                remove_card_from_deck(*position, &mut new_state);
            }
            (new_state, 1.0)
        },
        Choice::DeckTransform(positions, should_upgrade) => {    
            let mut state_space: f64 = 1.0;        
            let sets = positions.iter().map(|p| {
                let card = &state.deck[*p];
                let available_cards: Vec<&'static str> = 
                    models::cards::available_cards_by_class(card.base._class)
                    .iter().copied().filter(move |c| *c != card.base.name).collect();
                    
                state_space *= available_cards.len() as f64;
                available_cards
            });
            
            for position in positions {
                remove_card_from_deck(*position, &mut new_state);
            }


            for set in sets {
                let card = set.choose(&mut rng).expect("No available cards to be chosen!");
                add_card_to_deck(card, *should_upgrade, &mut new_state);
            }

            (new_state, 1.0 / state_space)
        },
        Choice::DeckUpgrade(positions) => {
            let mut possibility_set = (new_state, 1.0);

            for position in positions {
                upgrade_card_in_deck(*position, &mut possibility_set.0);
            }

            possibility_set
        },
        Choice::Dig => {
            let mut possibility_set = (new_state, 1.0);

            add_random_relic((1, 2, 3), &mut possibility_set, &mut rng);

            possibility_set
        },
        Choice::DiscardPotion{ slot } => {
            new_state.potions[*slot] = None;
            (new_state, 1.0)
        },
        Choice::DrinkPotion { slot, target_index } => {
            let potion = state.potions[*slot].as_ref().expect("Potion does not exist in slot!");
            let mut possibility_set = (new_state, 1.0);
            evaluator::eval_effects(&potion.base.on_drink, &mut possibility_set, &evaluator::Binding::Potion(potion), &Some(GameAction {
                creature: &state.player,
                is_attack: false,
                target: *target_index
            }), &mut rng);
            possibility_set
        }
        _ => unimplemented!()        
    }
}

fn add_random_relic<R>(weights: (u8, u8, u8), state: &mut GamePossibilitySet, rng: &mut R)
where R: Rng + ?Sized {

    let sum = weights.0 + weights.1 + weights.2;

    let mut sample_space: f64 = sum as f64; 
    let rarities = [
        (Rarity::Common, weights.0), 
        (Rarity::Uncommon, weights.1), 
        (Rarity::Rare, weights.2)
    ];
    

    let (rarity, weight) = rarities.choose_weighted(rng, |item| item.1).unwrap();

    let samples = *weight as f64;

    let available_relics: Vec<&'static str> = models::relics::RELICS.values()
        .filter(|relic| {
            relic.rarity == *rarity && 
            (relic.class == state.0.class || relic.class == Class::All) &&
            !state.0.relic_names.contains(relic.name)
        }).map(|relic| relic.name).collect();

    let relic = *available_relics.choose(rng).expect("No available relics to be chosen!");
    sample_space *= available_relics.len() as f64;


    if relic == models::relics::WAR_PAINT || relic == models::relics::WHETSTONE {
        let card_type = if relic == models::relics::WAR_PAINT { CardType::Skill } else { CardType::Attack };
        let available_cards: Vec<usize> = state.0.deck.iter()
            .enumerate()
            .filter(|(_, card)| {
                card.base._type == card_type && evaluator::card_upgradable(card)
            })
            .map(|(p, _)| p).collect();
        
        let cards = available_cards.choose_multiple(rng, 2);

        let mut remaining_card_count = available_cards.len();

        for card in cards {
            sample_space *= remaining_card_count as f64;
            remaining_card_count -= 1;
            upgrade_card_in_deck(*card, &mut state.0);
        }
    }

    state.1 *= samples/sample_space;
}

fn upgrade_card_in_deck(position: usize, state: &mut GameState) {
    state.deck[position].upgrades += 1;
}

fn remove_card_from_deck(position: usize, state: &mut GameState) {
    state.deck.remove(position);
}

fn spend_money(amount: u16, at_shop: bool, state: &mut GameState) {
    state.gold -= amount;

    if at_shop && state.relic_names.contains(MAW_BANK) {
        state.relics.iter_mut()
            .find(|relic| relic.base.name == MAW_BANK)
            .expect("Expected Maw Bank in relics")
            .enabled = false;
    }
}

fn add_relic(name: &str, state: &mut GameState) {
    let relic = evaluator::create_relic(name);
    state.relic_names.insert(relic.base.name);
    state.relics.push(relic);
}

fn add_potion(name: &str, state: &mut GameState) {
    let potion = evaluator::create_potion(name);
    *state.potions.iter_mut().find(|a| a.is_none()).expect("Expected potion in potions") = Some(potion);
}

fn find_relic<'a>(name: &str, state: &'a mut GameState) -> Option<&'a mut Relic> {
    if state.relic_names.contains(MAW_BANK) {
        match state.relics.iter_mut()
            .find(|relic| relic.base.name == MAW_BANK) {
            Some(relic) => Some(relic),
            None => panic!("Expected to find {} in relics", name)
        }
    } else {
        None
    }
}

fn add_card_to_deck(name: &str, upgraded: bool, state: &mut GameState) {
    let mut card = evaluator::create_card(name);
    if card.base._type == CardType::Curse {
        if let Some(relic) = find_relic(OMAMORI, state) {
            if relic.vars.x > 0 {
                relic.vars.x -= 1;
                return;
            }
        }

        if state.relic_names.contains(DARKSTONE_PERIAPT) {
            add_max_hp(6, state);
        }
    }

    let is_upgraded = upgraded || match card.base._type {
        CardType::Attack => state.relic_names.contains(MOLTEN_EGG),
        CardType::Skill => state.relic_names.contains(TOXIC_EGG),
        CardType::Power => state.relic_names.contains(FROZEN_EGG),
        CardType::Curse => false,
        CardType::Status => false,
        CardType::All => panic!("Unexpected card type of All"),
        CardType::ByName(_) => panic!("Unexpected card type of ByName"),
    };

    if is_upgraded {
        card.upgrades = 1;
    }

    if state.relic_names.contains(CERAMIC_FISH) {
        add_gold(9, state);
    }

    state.deck.push(card);
}

fn add_gold(amount: u16, state: &mut GameState) {
    if state.relic_names.contains(ECTOPLASM) {
        return;
    }

    if state.relic_names.contains(BLOODY_IDOL) {
        heal(5, state);
    }

    state.gold += amount;
}

fn add_max_hp(amount: u16, state: &mut GameState) {
    state.player.max_hp += amount;
    heal(amount, state)
}
 
fn heal(mut amount: u16, state: &mut GameState){
    if state.relic_names.contains(MARK_OF_THE_BLOOM) {
        return;
    }

    if state.battle_state.is_some() && state.relic_names.contains(MAGIC_FLOWER) {
        amount += div_up(amount, 2)
    }

    state.player.hp += amount;

    if state.player.hp > state.player.max_hp {
        state.player.hp = state.player.max_hp;
    }
}

fn div_up(a: u16, b: u16) -> u16 {
    (a + (b - 1))/b
}

pub fn verify_prediction<'a>(outcome: &'a GameState, prediction: &'a GamePossibilitySet) -> &'a GameState {
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

