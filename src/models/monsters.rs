use std::{collections::HashMap, fs::File, path::Path};
use ron::de::from_reader;

use super::core::BaseMonster;

pub fn by_name(name: &String) -> &'static BaseMonster {
    MONSTERS
        .get(name)
        .unwrap_or_else(|| panic!("Unexpected monster: {}", name))
}

lazy_static! {
    static ref MONSTERS: HashMap<String, BaseMonster> = {
        let mut m = HashMap::new();

        for monster in all_monsters() {
            m.insert((&monster.name).to_string(), monster);
        }

        m
    };
}

pub fn all_monsters() -> Vec<BaseMonster> {
    let filepath = Path::new("data").join("monsters.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
