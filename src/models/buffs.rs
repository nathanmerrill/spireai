use ron::de::from_reader;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fs::File, path::Path};

use super::core::{EffectGroup, Event, EventEffect, is_default};

#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct BaseBuff {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub repeats: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub singular: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub debuff: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_add: EffectGroup,
    #[serde(default, skip_serializing_if = "is_default")]
    pub reduce_at: Event,
    #[serde(default, skip_serializing_if = "is_default")]
    pub expire_at: Event,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<EventEffect>,
}
impl std::fmt::Debug for BaseBuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseBuff")
            .field("name", &self.name)
            .finish()
    }
}

pub fn by_name(name: &str) -> &'static BaseBuff {
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

fn all_buffs() -> Vec<BaseBuff> {
    let filepath = Path::new("data").join("buffs.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
