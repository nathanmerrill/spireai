use std::{collections::HashMap, fs::File, path::Path};
use serde::{Deserialize, Serialize};

use ron::de::from_reader;

use super::core::{Class, Effect, When, Rarity, is_default};



#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct BaseRelic {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rarity: Rarity,
    #[serde(default, skip_serializing_if = "is_default")]
    pub activation: Activation,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effect: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub disable_at: When,
    #[serde(default, skip_serializing_if = "is_default")]
    pub class: Class,
    #[serde(default, skip_serializing_if = "is_default")]
    pub energy_relic: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub replaces_starter: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub starting_x: i16,
}
impl std::fmt::Debug for BaseRelic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseRelic")
            .field("name", &self.name)
            .finish()
    }
}
impl PartialEq for BaseRelic {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Activation {
    Immediate,
    When(When),
    Counter {
        increment: When,
        reset: When,
        auto_reset: bool,
        target: u16,
    },
    Uses {
        use_when: When,
        uses: u16,
    },
    WhenEnabled {
        //Activation is triggered before any enable/disable checks
        activated_at: When,
        enabled_at: When,
        disabled_at: When,
    },
    Custom,
}

impl Default for Activation {
    fn default() -> Self {
        Activation::Custom
    }
}

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

fn all_relics() -> Vec<BaseRelic> {
    let filepath = Path::new("data").join("potions.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
