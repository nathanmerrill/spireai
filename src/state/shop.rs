use im::Vector;

use crate::{models::{potions::BasePotion, relics::BaseRelic}};

use super::{core::CardOffer, game::{GameState, RewardState}};


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ShopState {
    pub generated: bool,
    pub cards: Vector<(CardOffer, u16)>,
    pub potions: Vector<(&'static BasePotion, u16)>,
    pub relics: Vector<(&'static BaseRelic, u16)>,
    pub can_purge: bool,
    pub game_state: GameState,
    pub screen_state: ShopScreenState,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ShopScreenState {
    Entrance,
    Dolly,
    Purge,
    Reward(RewardState), // After this, state goes to the entrance
}