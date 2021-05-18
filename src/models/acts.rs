use std::{fs::File, path::Path};
use ron::de::from_reader;

use super::core::Act;

pub fn all_acts() -> Vec<Act> {
    let filepath = Path::new("data").join("acts.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}
