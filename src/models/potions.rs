use ron::de::from_reader;
use std::{collections::HashMap, fs::File, path::Path};
use serde::{Deserialize, Serialize};

use super::core::{Class, Condition, Effect, Rarity, is_default};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BasePotion {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub class: Class,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rarity: Rarity,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_drink: Vec<Effect>,
    #[serde(
        default = "Condition::never",
        skip_serializing_if = "Condition::is_never"
    )]
    pub targeted: Condition,
}
impl std::fmt::Debug for BasePotion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BasePotion")
            .field("name", &self.name)
            .finish()
    }
}

pub fn by_name(name: &str) -> &'static BasePotion {
    POTIONS
        .get(name)
        .unwrap_or_else(|| panic!("Potion {} not found", name))
}

lazy_static! {
    pub static ref POTIONS: HashMap<String, BasePotion> = {
        let mut m = HashMap::new();

        for potion in all_potions() {
            m.insert((&potion.name).to_string(), potion);
        }

        m
    };
}

fn all_potions() -> Vec<BasePotion> {
    let filepath = Path::new("data").join("potions.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
