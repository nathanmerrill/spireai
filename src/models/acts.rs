use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Act {
    pub num: u8,
    pub easy_count: u8,
    pub easy_fights: Vec<ProbabilisticFight>,
    pub normal_fights: Vec<ProbabilisticFight>,
    pub elites: Vec<MonsterSet>,
    pub bosses: Vec<MonsterSet>,
    pub events: Vec<String>,
}

#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct ProbabilisticFight {
    pub probability: u8,
    pub set: MonsterSet,
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum MonsterSet {
    Fixed(Vec<String>),
    ChooseN { n: u8, choices: Vec<String> },
    RandomSet(Vec<Vec<String>>),
}

impl std::fmt::Debug for Act {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Act").field("act", &self.num).finish()
    }
}

fn all_acts() -> Vec<Act> {
    let filepath = Path::new("data").join("acts.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
