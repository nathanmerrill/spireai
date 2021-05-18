use std::{collections::HashMap, fs::File, path::Path};

use ron::de::from_reader;

use super::core::BaseEvent;

pub fn by_name(name: &str) -> &'static BaseEvent {
    EVENTS
        .get(name)
        .unwrap_or_else(|| panic!("Unrecognized event: {}", name))
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
