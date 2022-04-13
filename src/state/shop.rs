use im::{Vector, vector};
use itertools::Itertools;

use crate::{models::{potions::BasePotion, relics::BaseRelic, core::{CardType, Rarity, Class, DeckOperation}, self}};

use super::{core::{CardOffer, Card, RewardState, Reward}, game::{GameState, DeckCard}, probability::Probability};


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ShopState {
    pub generated: bool,
    pub cards: Vector<StoreCard>,
    pub potions: Vector<StorePotion>,
    pub relics: Vector<StoreRelic>,
    pub can_purge: bool,
    pub game_state: GameState,
    pub screen_state: ShopScreenState,
}

type StoreCard = (CardOffer, u16);
type StorePotion = (&'static BasePotion, u16);
type StoreRelic = (&'static BaseRelic, u16);

impl ShopState {
    pub fn spend_gold(&mut self, amount: u16) {
        self.game_state.gold -= amount;

        if let Some(relic) = self.game_state.relics.find_mut("Maw Bank") {
            relic.enabled = false;
        }
    }

    pub fn buy_potion(&mut self, index: usize, probability: &mut Probability) {
        let (potion, cost) = self.potions.remove(index);

        if self.game_state.relics.contains("The Courier") {

            let new_potion = self.generate_potion(probability);

            self.potions.insert(index, new_potion)
        }
        

        self.spend_gold(cost);
        self.game_state.add_potion(potion);
    }

    pub fn buy_card(&mut self, index: usize, probability: &mut Probability) {
        let (offer, cost) = self.cards.remove(index);

        if self.game_state.relics.contains("The Courier") {
            let available = models::cards::available_cards_by_class(offer.base._class);

            if offer.base._class != Class::None {
                available = &available.iter().copied().filter(|c| c._type == offer.base._type).collect_vec()
            }

            let new_offer = self.generate_card_offer(offer.base._type, false, None, probability);

            self.cards.insert(index, new_offer)
        }
        
        self.spend_gold(cost);
        self.game_state.add_card(Card::new(offer.base));
    }

    pub fn purge(&mut self, card: DeckCard) 
    {
        let cost = self.purge_cost();
        self.spend_gold(cost);
        self.game_state.remove_card(card.uuid);
        self.game_state.purge_count += 1;
        self.can_purge = false;
    }
    
    pub fn purge_cost(&self) -> u16 {
        if self.game_state.relics.contains("Smiling Mask") {
            50
        } else {
            let discount = if self.game_state.relics.contains("Membership Card") {
                if self.game_state.relics.contains("The Courier") {
                    0.6
                } else {
                    0.5
                }
            } else if self.game_state.relics.contains("The Courier") {
                0.8
            } else {
                1.0
            };

            ((self.game_state.purge_count * 25 + 75) as f32 * discount).ceil() as u16
        }
    }

    pub fn buy_relic(&mut self, index: usize, probability: &mut Probability) {
        let (relic, cost) = self.relics.remove(index);

        if self.game_state.relics.contains("The Courier") {

            let new_relic = self.generate_relic(false, probability);

            self.relics.insert(index, new_relic)
        }
        

        self.spend_gold(cost);
        self.game_state.add_relic(relic, probability);
        match relic.name.as_str() {
            "Cauldron" => {
                self.screen_state = ShopScreenState::Reward(RewardState {
                    rewards: (0..5).map(|_|
                        Reward::Potion(super::game::random_potion(false, probability))                        
                    ).collect(),
                    viewing_reward: None
                })
            }
            "Orrery" => {
                self.screen_state = ShopScreenState::Reward(RewardState {
                    rewards: (0..5).map(|_|
                        Reward::CardChoice(vector![], None, false)
                    ).collect(),
                    viewing_reward: None
                })
            }
            "Dollys Mirror" => {
                self.screen_state = ShopScreenState::DeckChoose(DeckOperation::Duplicate)
            }
            "Bottled Flame" => {
                self.screen_state = ShopScreenState::DeckChoose(DeckOperation::BottleFlame)
            }
            "Bottled Lightning" => {
                self.screen_state = ShopScreenState::DeckChoose(DeckOperation::BottleLightning)
            }
            "Bottled Tornado" => {
                self.screen_state = ShopScreenState::DeckChoose(DeckOperation::BottleTornado)
            }
            _ => {}
        }
    }

    pub fn generate(&mut self, probability: &mut Probability) {
        if !self.generated {
            let on_sale = probability.range(5);
            let attack1 = self.generate_card_offer( CardType::Attack, on_sale == 0, None, probability);
            let attack2 = self.generate_card_offer(CardType::Attack, on_sale == 1, Some(attack1), probability);
            let skill1 = self.generate_card_offer( CardType::Skill, on_sale == 2, None, probability);
            let skill2 = self.generate_card_offer(CardType::Skill, on_sale == 3, Some(skill1), probability);
            let power = self.generate_card_offer( CardType::Power, on_sale == 4, None, probability);
            let colorless1 = self.generate_colorless_offer(false, probability);
            let colorless2 = self.generate_colorless_offer(true, probability);

            self.cards = vector![
                attack1, attack2, skill1, skill2, power, colorless1, colorless2,
            ];


            let relic1 = self.game_state.random_relic(None, None,  true, probability);
            let relic2 = self.game_state.random_relic(None, None, true, probability);
            let relic3 = self.game_state.random_relic(None, Some(Rarity::Shop),  true, probability);

            self.relics = vector![
                self.generate_relic(false, probability),
                self.generate_relic(false, probability),
                self.generate_relic(true, probability),
            ];

            self.potions = (0..3).map(|_|self.generate_potion(probability)).collect();
            self.generated = true;
        }
    }

    fn generate_relic(&self, is_shop: bool, probability: &mut Probability) -> (&'static BaseRelic, u16) {
        let rarity = if is_shop { Some(Rarity::Shop) } else { None };

        let relic = self.game_state.random_relic(None, rarity,  true, probability);

        let (mut min, mut max) = match relic.rarity {
            Rarity::Shop | Rarity::Common => (143, 157),
            Rarity::Uncommon => (238, 262),
            Rarity::Rare => (285, 315),
            _ => panic!("Unexpected rarity"),
        };
        
        let discount = self.get_discount();

        min = (min as f64 * discount).ceil() as usize;
        max = (max as f64 * discount).ceil() as usize;

        (relic, (probability.range(max - min) + min) as u16)
        
    }

    fn get_discount(&self) -> f64 {
        let mut discount = 1.0;
        if self.game_state.relics.contains("The Courier") {
            discount = 0.8;
        }
        if self.game_state.relics.contains("Membership Card") {
            discount /= 2.0;
        }
        discount
    }

    fn generate_potion(&self, probability: &mut Probability) -> StorePotion {
        let potion = super::game::random_potion(false, probability);
        let (mut min, mut max) = match potion.rarity {
            Rarity::Common => (48, 52),
            Rarity::Uncommon => (72, 78),
            Rarity::Rare => (95, 105),
            _ => panic!("Unexpected rarity"),
        };

        let discount = self.get_discount();

        min = (min as f64 * discount).ceil() as usize;
        max = (max as f64 * discount).ceil() as usize;

        (
            potion,
            (probability.range(max - min) + min) as u16,
        )
    }

    fn generate_card_offer(&self, _type: CardType, on_sale: bool, exclude: Option<StoreCard>, probability: &mut Probability) -> StoreCard {
        let cards = models::cards::available_cards_by_class(self.game_state.class);
        let available = cards.iter().copied().filter(|card| {
            card._type == _type && exclude.map(|a| a.0.base.name != card.name).unwrap_or(true)
        }).collect_vec();

        let card = self.game_state.generate_card_offer(None, &available, probability);
        
        let cost = self.calculate_card_cost(card, on_sale, probability);

        (card, cost)
    }

    fn generate_colorless_offer(&self, rare: bool, probability: &mut Probability) -> StoreCard {
        let rarity = if rare { models::core::Rarity::Rare } else { models::core::Rarity::Uncommon };
        let available = models::cards::available_cards_by_class(models::core::Class::None)
                    .iter()
                    .copied()
                    .filter(|c| c.rarity == models::core::Rarity::Uncommon)
                    .collect_vec();

        let card = self.game_state.generate_card_offer(None, &available, probability);

        let cost = self.calculate_card_cost(card, false, probability);

        (card, cost)
    }
    
    fn calculate_card_cost(&self, card: CardOffer, on_sale: bool, probability: &mut Probability) -> u16 {
        let (min, max) = match card.base._class {
            models::core::Class::None => match card.base.rarity {
                models::core::Rarity::Uncommon => (81, 91),
                models::core::Rarity::Rare => (162, 198),
                _ => panic!("Unexpected rarity"),
            },
            _ => match card.base.rarity {
                models::core::Rarity::Common => (45, 55),
                models::core::Rarity::Uncommon => (68, 82),
                models::core::Rarity::Rare => (135, 165),
                _ => panic!("Unexpected rarity"),
            },
        };

        let discount = self.get_discount();
        if on_sale {
            discount *= 0.5;
        }

        let min = (min as f64 * discount).ceil() as usize;
        let max = (max as f64 * discount).ceil() as usize;

        (probability.range(max - min) + min) as u16
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ShopScreenState {
    Entrance,
    DeckChoose(DeckOperation),
    Reward(RewardState), // After this, state goes to the entrance
    InShop,
}