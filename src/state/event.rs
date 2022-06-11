use crate::{
    models::{
        self,
        core::{DeckOperation, EventEffect},
        events::BaseEvent,
    },
    spireai::references::{CardReference, RelicReference},
};

use super::{
    core::{RewardState, Vars},
    game::GameState,
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

    pub fn eval_effects(&mut self, _effects: &[EventEffect]) {
        unimplemented!()
    }
}
