use std::{collections::HashMap, fs::File, path::Path};
use serde::{Deserialize, Serialize};

use ron::de::from_reader;

use super::core::{Condition, Effect, is_default, is_true, _true};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BaseEvent {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub choices: Vec<BaseEventChoice>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub shrine: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub variants: Vec<String>,
    #[serde(
        default = "Condition::always",
        skip_serializing_if = "Condition::is_always"
    )]
    pub condition: Condition,
}
#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct BaseEventChoice {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<Effect>,
    #[serde(
        default = "Condition::always",
        skip_serializing_if = "Condition::is_always"
    )]
    pub condition: Condition,
    #[serde(default = "_true", skip_serializing_if = "is_true")]
    pub initial: bool,
}
impl std::fmt::Debug for BaseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseEvent")
            .field("name", &self.name)
            .finish()
    }
}

pub fn by_name(name: &str) -> &'static BaseEvent {
    EVENTS
        .get(name)
        .unwrap_or_else(|| panic!("Unrecognized when: {}", name))
}

lazy_static! {
    static ref EVENTS: HashMap<String, BaseEvent> = {
        let mut m = HashMap::new();

        for event in all_events() {
            m.insert((&event.name).to_string(), event);
        }

        m
    };
}

fn all_events() -> Vec<BaseEvent> {
    let filepath = Path::new("data").join("events.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
