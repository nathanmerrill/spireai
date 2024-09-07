use ron::de::from_reader;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, error::Error, fs::File, hash::Hash, path::Path, ptr};

use super::core::{is_default, Effect, When, WhenEffect};

#[derive(Clone, Deserialize, Serialize)]
pub struct BaseBuff {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub repeats: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub singular: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub zeroable: bool,
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

impl<'de> Deserialize<'de> for &'static BaseBuff {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(BaseBuffVisitor)
    }
}

struct BaseBuffVisitor;

impl<'de> serde::de::Visitor<'de> for BaseBuffVisitor {
    type Value = &'static BaseBuff;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        BUFFS
            .get(v)
            .ok_or(E::custom(format!("Unable to find {} as a buff", v)))
    }
}

impl Hash for &'static BaseBuff {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(self, state)
    }
}

impl std::fmt::Debug for BaseBuff {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseBuff")
            .field("name", &self.name)
            .finish()
    }
}

impl PartialEq for &'static BaseBuff {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}
impl Eq for &'static BaseBuff {}

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
    pub static ref BAD_BUFF: BaseBuff = BaseBuff {
        name: String::from("ERROR"),
        repeats: false,
        singular: false,
        zeroable: false,
        debuff: false,
        on_add: vec![],
        reduce_at: When::Never,
        expire_at: When::Never,
        effects: vec![]
    };
}

pub static BARRICADE: &'static BaseBuff = BUFFS.get("Barricade").unwrap_or(&BAD_BUFF);
pub static BLUR: &'static BaseBuff = BUFFS.get("Blur").unwrap_or(&BAD_BUFF);
pub static DRAW_CARD: &'static BaseBuff = BUFFS.get("Draw Card").unwrap_or(&BAD_BUFF);
pub static ELECTRO: &'static BaseBuff = BUFFS.get("Electro").unwrap_or(&BAD_BUFF);
pub static FOCUS: &'static BaseBuff = BUFFS.get("Focus").unwrap_or(&BAD_BUFF);
pub static FREE_ATTACK_POWER: &'static BaseBuff =
    BUFFS.get("Free Attack Power").unwrap_or(&BAD_BUFF);
pub static INNATE_THEIVERY: &'static BaseBuff = BUFFS.get("Innate Theivery").unwrap_or(&BAD_BUFF);
pub static INTANGIBLE: &'static BaseBuff = BUFFS.get("Intangible").unwrap_or(&BAD_BUFF);
pub static INVINCIBLE: &'static BaseBuff = BUFFS.get("Invincible").unwrap_or(&BAD_BUFF);
pub static LOCK_ON: &'static BaseBuff = BUFFS.get("Lock On").unwrap_or(&BAD_BUFF);
pub static MARK: &'static BaseBuff = BUFFS.get("Mark").unwrap_or(&BAD_BUFF);
pub static MASTER_REALITY: &'static BaseBuff = BUFFS.get("Master Reality").unwrap_or(&BAD_BUFF);
pub static METALLICIZE: &'static BaseBuff = BUFFS.get("Metallicize").unwrap_or(&BAD_BUFF);
pub static MODE_SHIFT: &'static BaseBuff = BUFFS.get("Mode Shift").unwrap_or(&BAD_BUFF);
pub static PLATED_ARMOR: &'static BaseBuff = BUFFS.get("Plated Armor").unwrap_or(&BAD_BUFF);
pub static POISON: &'static BaseBuff = BUFFS.get("Poison").unwrap_or(&BAD_BUFF);
pub static REGENERATE: &'static BaseBuff = BUFFS.get("Regenerate").unwrap_or(&BAD_BUFF);
pub static RUSHDOWN: &'static BaseBuff = BUFFS.get("Rushdown").unwrap_or(&BAD_BUFF);
pub static SLOW: &'static BaseBuff = BUFFS.get("Slow").unwrap_or(&BAD_BUFF);
pub static STASIS: &'static BaseBuff = BUFFS.get("Stasis").unwrap_or(&BAD_BUFF);
pub static STRENGTH: &'static BaseBuff = BUFFS.get("Strength").unwrap_or(&BAD_BUFF);
pub static VIGOR: &'static BaseBuff = BUFFS.get("Vigor").unwrap_or(&BAD_BUFF);
pub static VULNERABLE: &'static BaseBuff = BUFFS.get("Vulnerable").unwrap_or(&BAD_BUFF);
pub static WEAK: &'static BaseBuff = BUFFS.get("Weak").unwrap_or(&BAD_BUFF);

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
