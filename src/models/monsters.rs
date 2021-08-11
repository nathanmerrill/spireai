use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, path::Path};

use ::std::hash::{Hash, Hasher};

use super::core::{is_default, Amount, Condition, Effect, FightType, When, WhenEffect};

#[derive(Eq, Clone, Serialize, Deserialize)]
pub struct BaseMonster {
    pub name: String,
    pub hp_range: SimpleRange,
    pub hp_range_asc: SimpleRange,
    pub fight_type: FightType,
    pub moveset: Vec<MonsterMove>,
    pub phases: Vec<Phase>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub n_range: Option<Range>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub x_range: Option<Range>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<WhenEffect>,
}
impl std::fmt::Debug for BaseMonster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseMonster")
            .field("name", &self.name)
            .finish()
    }
}

impl Hash for BaseMonster {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
impl PartialEq for BaseMonster {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Phase {
    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub next: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub moves: Vec<Move>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub when: When,
    #[serde(
        default = "Condition::always",
        skip_serializing_if = "Condition::is_always"
    )]
    pub when_condition: Condition,
}

#[derive(Eq, Clone, Debug, Deserialize, Serialize)]
pub struct MonsterMove {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<Effect>,
    pub intent: Intent,
}

impl Hash for MonsterMove {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
impl PartialEq for MonsterMove {
    fn eq(&self, other: &MonsterMove) -> bool {
        self.name == other.name
    }
}

impl Default for MonsterMove {
    fn default() -> Self {
        MonsterMove {
            name: String::default(),
            effects: vec![],
            intent: Intent::None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Intent {
    Attack,
    AttackBuff,
    AttackDebuff,
    AttackDefend,
    Buff,
    Debuff,
    StrongDebuff,
    Defend,
    DefendDebuff,
    DefendBuff,
    Escape,
    None,
    Sleep,
    Stun,
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Move {
    If {
        condition: Condition,
        #[serde(default, skip_serializing_if = "is_default")]
        then_phase: String,
        #[serde(default, skip_serializing_if = "is_default")]
        else_phase: String,
    },
    Fixed(String),
    Probability(Vec<ProbabilisticMove>), // Weight, name, repeats
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct ProbabilisticMove {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub weight: Amount,
    #[serde(default, skip_serializing_if = "is_default")]
    pub max_repeats: Amount,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Range {
    pub min: Amount,
    pub max: Amount,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SimpleRange {
    pub min: u16,
    pub max: u16,
}

pub fn by_name(name: &str) -> &'static BaseMonster {
    MONSTERS
        .get(name)
        .unwrap_or_else(|| panic!("Unexpected monster: {}", name))
}

lazy_static! {
    pub static ref MONSTERS: HashMap<String, BaseMonster> = {
        let mut m = HashMap::new();

        for monster in all_monsters().unwrap() {
            m.insert((&monster.name).to_string(), monster);
        }

        m
    };
}

fn all_monsters() -> Result<Vec<BaseMonster>, Box<dyn Error>> {
    let filepath = Path::new("data").join("monsters.ron");
    let file = File::open(filepath)?;
    let u = from_reader(file)?;
    Ok(u)
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_parse() -> Result<(), String> {
        match super::all_monsters() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
