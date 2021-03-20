use crate::models::cards;
use crate::models::core::*;
use std::collections::HashMap;
use Amount::*;

impl BaseEvent {
    fn default() -> Self {
        Self {
            name: &"",
            choices: vec![],
            shrine: false,
        }
    }
}

pub fn by_name(name: &str) -> &'static BaseEvent {
    EVENTS.get(name).unwrap()
}

lazy_static! {
    static ref EVENTS: HashMap<&'static str, BaseEvent> = {
        let mut m = HashMap::new();

        for event in all_events() {
            m.insert(event.name, event);
        }

        m
    };
}

fn all_events() -> Vec<BaseEvent> {
    vec![
        BaseEvent {
            name: NEOW,
            choices: vec![
                BaseEventChoice {
                    name: TALK,
                    effects: vec![],
                }
            ],
            shrine: false,         
        },
    ]
}

pub const NEOW: &str = "Neow Event";

pub const TALK: &str = "Talk";