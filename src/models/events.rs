use ::std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, path::Path};

use ron::de::from_reader;

use super::core::{Condition, _true, is_default, is_true, Effect};

#[derive(Eq, Clone, Serialize, Deserialize)]
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
impl Hash for BaseEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
impl PartialEq for BaseEvent {
    fn eq(&self, other: &BaseEvent) -> bool {
        self.name == other.name
    }
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
    pub static ref EVENTS: HashMap<String, BaseEvent> = {
        let mut m = HashMap::new();

        for event in all_events().unwrap() {
            m.insert((&event.name).to_string(), event);
        }

        m
    };
}

fn all_events() -> Result<Vec<BaseEvent>, Box<dyn Error>> {
    let filepath = Path::new("data").join("events.ron");
    let file = File::open(filepath)?;
    let u = from_reader(file)?;
    Ok(u)
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_parse() -> Result<(), String> {
        match super::all_events() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
