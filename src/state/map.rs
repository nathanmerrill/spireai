use im::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MapState {
    pub nodes: HashMap<(i8, i8), MapNode>,
    pub floor: i8,
    pub x: i8,
    pub history: MapHistory,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MapHistory {
    pub last_elite: Option<usize>,
    pub last_normal: Option<usize>,
    pub easy_fight_count: u8,
    pub unknown_normal_count: u8,
    pub unknown_shop_count: u8,
    pub unknown_treasure_count: u8,
    pub event_history: HashSet<String>,
    pub last_shop: bool,
}

impl MapHistory {
    pub fn new() -> Self {
        Self {
            last_elite: None,
            last_normal: None,
            easy_fight_count: 0,
            unknown_normal_count: 0,
            unknown_shop_count: 0,
            unknown_treasure_count: 0,
            event_history: HashSet::new(),
            last_shop: false,
        }
    }
}

impl MapState {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            floor: 0,
            x: 0,
            history: MapHistory::new(),
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
    Boss(String),
    Monster,
    Shop,
    Chest,
}
