use ron::de::from_reader;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, error::Error, fs::File, path::Path};

use ::std::hash::{Hash, Hasher};

use super::core::{is_default, Effect, When, WhenEffect};

#[derive(Clone, Eq, Deserialize, Serialize)]
pub struct BaseBuff {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub repeats: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub singular: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub debuff: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_add: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub reduce_at: When,
    #[serde(default, skip_serializing_if = "is_default")]
    pub expire_at: When,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<WhenEffect>,
}
impl std::fmt::Debug for BaseBuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseBuff")
            .field("name", &self.name)
            .finish()
    }
}

impl Hash for BaseBuff {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl PartialEq for BaseBuff {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn by_name(name: &str) -> &'static BaseBuff {
    BUFFS
        .get(name)
        .unwrap_or_else(|| panic!("Unrecognized buff: {}", name))
}

lazy_static! {
    pub static ref BUFFS: HashMap<String, BaseBuff> = {
        let mut m = HashMap::new();

        for buff in all_buffs().unwrap() {
            m.insert((&buff.name).to_string(), buff);
        }

        m
    };
}

fn all_buffs() -> Result<Vec<BaseBuff>, Box<dyn Error>> {
    let filepath = Path::new("data").join("buffs.ron");
    let file = File::open(filepath)?;
    let u = from_reader(file)?;
    Ok(u)
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_parse() -> Result<(), String> {
        match super::all_buffs() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
