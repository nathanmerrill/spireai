use ron::de::from_reader;
use std::{collections::HashMap, fs::File, path::Path};
use serde::{Deserialize, Serialize};

use super::core::{Amount, Condition, Effect, When, WhenEffect, is_default, one, is_one};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BaseMonster {
    pub name: String,
    pub hp_range: SimpleRange,
    pub hp_range_asc: SimpleRange,
    pub moveset: Vec<MonsterMove>,
    pub move_order: Vec<Move>,
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

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct MonsterMove {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effects: Vec<Effect>,
    pub intent: Intent,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Deserialize, Serialize)]
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
        then: Vec<Move>,
        #[serde(default, skip_serializing_if = "is_default")]
        _else: Vec<Move>,
    },
    Loop(Vec<Move>),
    InOrder(String),
    Probability(Vec<ProbabilisticMove>), // Weight, name, repeats
    When(When),
    AfterMove(Vec<(String, Move)>),
    WhenMove(String),
    WhenLoseBuff(String),
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub struct ProbabilisticMove {
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub weight: u8,
    pub name: String,
    #[serde(default = "one", skip_serializing_if = "is_one")]
    pub max_repeats: u8,
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
    static ref MONSTERS: HashMap<String, BaseMonster> = {
        let mut m = HashMap::new();

        for monster in all_monsters() {
            m.insert((&monster.name).to_string(), monster);
        }

        m
    };
}

fn all_monsters() -> Vec<BaseMonster> {
    let filepath = Path::new("data").join("monsters.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
