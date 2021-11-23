use crate::models;
use crate::models::core::{CardDestination, CardType, Rarity};
use crate::spireai::*;
use crate::state::core::{Card, Event};
use crate::state::game::FloorState;
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
            let mut event = Event::by_name(name);
            event.variant = possibility.probability.choose(event.base.variants.clone());
            match event.base.name.as_str() {
                "Falling" => {
                    let choices = [CardType::Attack, CardType::Skill, CardType::Power].iter().map(|t| {
                        possibility.probability.choose(possibility.state.deck().filter(|c| c.base._type == *t).collect())
                    }).flatten().collect();
                   
                    event.variant_cards = choices;
                }
                "Nloth" => {
                    let choices = possibility.probability.choose_multiple(possibility.state.relics().collect(), 2);
                    event.variant_relics = choices;
                }
                "We Meet Again" => {
                    if possibility.state.gold >= 50 {
                        event.variant_amount = Some(possibility.probability.range(possibility.state.gold as usize - 50) as u16 + 50)
                    }
                }
                _ => {}
            }
        }
        _ => unimplemented!()
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
