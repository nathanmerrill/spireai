use std::{collections::HashMap, fs::File, path::Path};

use ron::de::from_reader;

use super::core::BaseRelic;

pub fn by_name(name: &str) -> &'static BaseRelic {
    RELICS
        .get(name)
        .unwrap_or_else(|| panic!("Unrecognized relic: {}", name))
}

lazy_static! {
    pub static ref RELICS: HashMap<String, BaseRelic> = {
        let mut m = HashMap::new();

        for relic in all_relics() {
            m.insert((&relic.name).to_string(), relic);
        }

        m
    };
}

pub fn all_relics() -> Vec<BaseRelic> {
    let filepath = Path::new("data").join("potions.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
