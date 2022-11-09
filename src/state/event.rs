use crate::{
    models::{
        self,
        core::{DeckOperation, Effect, Amount},
        events::BaseEvent,
    },
    spireai::references::{CardReference, RelicReference},
};

use super::{
    core::{RewardState, Vars},
    game::GameState, probability::Probability,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum EventScreenState {
    Rewards(RewardState),
    DeckChoose(DeckOperation, usize),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct EventState {
    pub base: &'static BaseEvent,
    pub game_state: GameState,
    pub vars: Vars,
    pub variant: Option<String>,
    pub variant_cards: Vec<CardReference>,
    pub variant_relics: Vec<RelicReference>,
    pub variant_amount: Option<u16>,
    pub available_choices: Vec<String>,
    pub screen_state: Option<EventScreenState>,
}

impl EventState {
    pub fn by_name(name: &str, game_state: GameState) -> Self {
        Self::new(models::events::by_name(name), game_state)
    }

    pub fn new(base: &'static BaseEvent, game_state: GameState) -> Self {
        Self {
            base,
            vars: Vars::new(),
            variant: None,
            variant_cards: vec![],
            variant_relics: vec![],
            variant_amount: None,
            available_choices: base
                .choices
                .iter()
                .filter(|c| c.initial)
                .map(|c| c.name.to_string())
                .collect(),
            game_state,
            screen_state: None,
        }
    }
    
    pub fn eval_effects(&mut self, effects: &[Effect], probability: &mut Probability) {
        for effect in effects {
            self.eval_effect(effect, probability);
        }
    }

    pub fn eval_effect(&mut self, effect: &Effect, probability: &mut Probability) {
        match effect {
            Effect::ShowChoices(choices) => {
                self.available_choices = choices.to_vec();
            }
            Effect::SetN(n) => {
                let amount = self.eval_amount(n);
                self.vars.n = amount;
                self.vars.n_reset = amount;
            }
            Effect::AddN(n) => {
                self.vars.n += self.eval_amount(n)
            }
            Effect::SetX(x) => {
                self.vars.x = self.eval_amount(x)
            }
            Effect::AddX(x) => {
                self.vars.x += self.eval_amount(x)
            }
            Effect::ResetN => {
                self.vars.n = self.vars.n_reset
            }
            _ => self.game_state.eval_effect(effect, probability)
        }
    }

    pub fn eval_amount(&self, amount: &Amount) -> i16 {
        match amount {
            Amount::N => self.vars.n,
            Amount::X => self.vars.x,
            Amount::NegX => -self.vars.x,
            Amount::Mult(amount_mult) => {
                let mut product = 1;
                for amount in amount_mult {
                    product *= self.eval_amount(amount);
                }
                product
            }
            Amount::Sum(amount_sum) => {
                let mut sum = 0;
                for amount in amount_sum {
                    sum += self.eval_amount(amount);
                }
                sum
            }
            _ => self.game_state.eval_amount(amount)
        }
    }
}
