use ron::de::from_reader;

use super::core::BaseBuff;
use std::{collections::HashMap, fs::File, path::Path};

pub fn by_name(name: &String) -> &'static BaseBuff {
    BUFFS
        .get(name)
        .unwrap_or_else(|| panic!("Unrecognized buff: {}", name))
}

lazy_static! {
    static ref BUFFS: HashMap<String, BaseBuff> = {
        let mut m = HashMap::new();

        for buff in all_buffs() {
            m.insert((&buff.name).to_string(), buff);
        }

        m
    };
}

pub fn all_buffs() -> Vec<BaseBuff> {
    let filepath = Path::new("data").join("buffs.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
