use ::std::hash::{Hash, Hasher};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, path::Path};

use super::core::{is_default, Effect, Class, Rarity};

#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct BasePotion {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub class: Class,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rarity: Rarity,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_drink: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub targeted: bool,
}
impl std::fmt::Debug for BasePotion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BasePotion")
            .field("name", &self.name)
            .finish()
    }
}

impl Hash for BasePotion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
impl PartialEq for BasePotion {
    fn eq(&self, other: &BasePotion) -> bool {
        self.name == other.name
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

        for potion in all_potions().unwrap() {
            m.insert((&potion.name).to_string(), potion);
        }

        m
    };
}

fn all_potions() -> Result<Vec<BasePotion>, Box<dyn Error>> {
    let filepath = Path::new("data").join("potions.ron");
    let file = File::open(filepath)?;
    let u = from_reader(file)?;
    Ok(u)
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_parse() -> Result<(), String> {
        match super::all_potions() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
