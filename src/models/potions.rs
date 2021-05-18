use std::{collections::HashMap, fs::File, path::Path};
use ron::de::from_reader;

use super::core::BasePotion;

pub fn by_name(name: &String) -> &'static BasePotion {
    POTIONS
        .get(name)
        .unwrap_or_else(|| panic!("Potion {} not found", name))
}

lazy_static! {
    static ref POTIONS: HashMap<String, BasePotion> = {
        let mut m = HashMap::new();

        for potion in all_potions() {
            m.insert((&potion.name).to_string(), potion);
        }

        m
    };
}

pub fn all_potions() -> Vec<BasePotion> {
    let filepath = Path::new("data").join("potions.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
