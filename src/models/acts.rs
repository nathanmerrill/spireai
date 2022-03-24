use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, path::Path};

#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Act {
    // Upgrade chance is 0, 25, 50, then (25, 50) if ascension (>12, <11)
    pub num: u8,
    pub easy_count: u8,
    pub easy_fights: Vec<ProbabilisticFight>,
    pub normal_fights: Vec<ProbabilisticFight>,
    pub elites: Vec<MonsterSet>,
    pub bosses: Vec<Boss>,
    pub events: Vec<String>,
}

#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Boss {
    pub name: String,
    pub monsters: MonsterSet,
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

pub fn all_acts() -> Result<Vec<Act>, Box<dyn Error>> {
    let filepath = Path::new("data").join("acts.ron");
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let u = ron::de::from_reader(reader)?;
    Ok(u)
}

lazy_static! {
    pub static ref ACTS: Vec<Act> = all_acts().unwrap();
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_parse() -> Result<(), String> {
        match super::all_acts() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
