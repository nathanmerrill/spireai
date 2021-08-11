use im::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MapState {
    pub nodes: HashMap<(i8, i8), MapNode>,
    pub floor: i8,
    pub x: i8,
}

impl MapState {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            floor: 0,
            x: 0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MapNode {
    pub floor: i8,
    pub x: i8,
    pub next: Vec<i8>,
    pub icon: MapNodeIcon,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum MapNodeIcon {
    Question,
    Elite,
    BurningElite,
    Campfire,
    Boss,
    Monster,
    Shop,
    Chest,
}
