use im::{HashMap, HashSet};
use itertools::Itertools;

use super::probability::Probability;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct MapState {
    pub nodes: [Option<MapNode>; 105],
    pub boss: String,
    pub floor: i8,
    pub index: Option<usize>,
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
            nodes: [None; 105],
            floor: -1,
            index: None,
            boss: String::new(),
            history: MapHistory::new(),
        }
    }

    pub fn parents(&self, node: MapNode) -> (Option<MapNode>, Option<MapNode>, Option<MapNode>)
    {
        let index = node.index();
        if index < 7 {
            (None, None, None)
        } else {
            (
                if node.x > 0 { self.nodes[index-8].and_then(|f| if f.right { Some(f) } else { None})} else {None},
                if node.x > 0 { self.nodes[index-7].and_then(|f| if f.up { Some(f) } else { None})} else {None},
                if node.x > 0 { self.nodes[index-6].and_then(|f| if f.left { Some(f) } else { None})} else {None}
            )
        }
    }

    pub fn generate(&mut self, more_elites: bool, burning_elite: bool, probability: &mut Probability) {
        let mut grid: [Option<MapNode>; 105] = [None; 105];
        let mut first_x = 0;
        let mut last_direction = Direction::Up;
        for path_num in 0 .. 6 {// Create 6 paths to the top

            let mut next_x = if path_num != 1 { // Ensure that the second path does not start on the same node
                let next_x = probability.range(7) as u8;
                if path_num == 0 {
                    first_x = next_x;
                }
                next_x
            } else {
                let next_x = probability.range(6) as u8;
                if next_x >= first_x {
                    next_x + 1
                } else {
                    next_x
                }
            };

            for y in 0 .. 15 {
                let index = (next_x + (y * 7)) as usize;
                
                let can_left = next_x != 0 && grid[index-1].map_or(true, |node| !node.right);
                let can_right = next_x != 6 && grid[index+1].map_or(true, |node| !node.left);

                let node = grid[index].get_or_insert(MapNode {
                    x: next_x,
                    y,
                    left: false,
                    up: false,
                    right: false,
                    icon: MapNodeIcon::BurningElite // Using burning elite to indicate "unselected"
                });


                let mut directions = vec![Direction::Up];
                if can_left {
                    directions.push(Direction::Left);
                }
                if can_right {
                    directions.push(Direction::Right);
                }

                last_direction = probability.choose(directions).unwrap();
    
                next_x = match last_direction {
                    Direction::Left => {
                        node.left = true;
                        next_x - 1
                    }
                    Direction::Right => {
                        node.right = true;
                        next_x + 1
                    }
                    Direction::Up => {
                        node.up = true;
                        next_x
                    }
                };
            }
        }
        let mut count: u8 = 0;
        for node in grid.iter_mut() {
            if let Some(node) = node {
                match node.y {
                    14 => node.icon = MapNodeIcon::Campfire,
                    8 => node.icon = MapNodeIcon::Chest,
                    0 => node.icon = MapNodeIcon::Monster,
                    _ => count += 1,
                }
            }
        }

        let shops = ((count as f64) * 0.05).round() as u8;
        let rests = ((count as f64) * 0.12).round() as u8;
        let events = ((count as f64) * 0.22).round() as u8;
        let elites = ((count as f64) * (if more_elites {0.128} else {0.08})).round() as u8;
        let monsters = count - shops - rests - events - elites;
        
        let mut all_options: HashMap<MapNodeIcon, u8> = HashMap::new();
        all_options.insert(MapNodeIcon::Shop, shops);
        all_options.insert(MapNodeIcon::Campfire, rests);
        all_options.insert(MapNodeIcon::Question, events);
        all_options.insert(MapNodeIcon::Elite, elites);
        all_options.insert(MapNodeIcon::Monster, monsters);

        for index in 0..105 {
            if let Some(node) = grid[index] {
                let mut options = all_options.clone();
                match node.y {
                    0|8|14 => continue,
                    _ => {
                        let (left, down, right) = self.parents(node);

                        if let Some(left_parent) = left {
                            if left_parent.left {
                                options.remove(&grid[index - 2].unwrap().icon);
                            }
                            if left_parent.up {
                                options.remove(&grid[index - 1].unwrap().icon);
                            }
                            match left_parent.icon {
                                MapNodeIcon::Question | MapNodeIcon::Monster => {}
                                _ => {
                                    options.remove(&left_parent.icon);
                                }
                            }
                        }

                        if let Some(down_parent) = down {
                            if down_parent.left {
                                options.remove(&grid[index - 1].unwrap().icon);
                            }

                            // We don't check to the right, because that node isn't generated yet
                            
                            match down_parent.icon {
                                MapNodeIcon::Question | MapNodeIcon::Monster => {}
                                _ => {
                                    options.remove(&down_parent.icon);
                                }
                            }
                        }

                        if let Some(right_parent) = right {
                            
                            match right_parent.icon {
                                MapNodeIcon::Question | MapNodeIcon::Monster => {}
                                _ => {
                                    options.remove(&right_parent.icon);
                                }
                            }
                        }
                        if node.y < 5 {
                            options.remove(&MapNodeIcon::Elite);
                            options.remove(&MapNodeIcon::Campfire);
                        }
                        if node.y == 13 {
                            options.remove(&MapNodeIcon::Campfire);
                        }
                    }
                }

                let options = options.into_iter().collect_vec();

                let icon = *probability.choose_weighted(&options).unwrap();

                grid[index].as_mut().unwrap().icon = icon;
                all_options[&icon] -= 1;

            }
        }

        for node in grid.iter_mut() {
            if let Some(node) = node {
                if node.icon == MapNodeIcon::BurningElite {
                    node.icon = MapNodeIcon::Monster
                }
            }
        }

        if burning_elite {
            let choices = grid.iter().flatten().filter(|a| a.icon == MapNodeIcon::Elite).collect();
            let choice = probability.choose(choices).unwrap();
            grid[choice.index()].unwrap().icon = MapNodeIcon::BurningElite;
        }

        self.nodes = grid;
        
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MapNode {
    pub y: u8,
    pub x: u8,
    pub left: bool,
    pub up: bool,
    pub right: bool,
    pub icon: MapNodeIcon,
}

impl MapNode {
    pub fn index(&self) -> usize {
        (self.x + self.y * 7) as usize
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MapNodeIcon {
    Question,
    Elite,
    BurningElite,
    Campfire,
    Monster,
    Shop,
    Chest,
}


