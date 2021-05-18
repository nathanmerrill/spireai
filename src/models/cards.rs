use ron::de::from_reader;

use std::{collections::HashMap, fs::File, path::Path};

use super::core::{BaseCard, Class, Rarity, CardType};

pub fn by_name(name: &String) -> &'static BaseCard {
    ALL_CARDS
        .get(name)
        .unwrap_or_else(|| panic!("Unrecognized card: {}", name))
}

pub fn available_cards_by_class(class: Class) -> &'static Vec<String> {
    match class {
        Class::All => &ANY_CLASS_CARDS,
        Class::Curse => &CURSES,
        Class::Defect => &DEFECT_CARDS,
        Class::Ironclad => &IRONCLAD_CARDS,
        Class::None => &COLORLESS_CARDS,
        Class::Silent => &SILENT_CARDS,
        Class::Watcher => &WATCHER_CARDS,
    }
}

lazy_static! {
    pub static ref ALL_CARDS: HashMap<String, BaseCard> = {
        let mut m = HashMap::new();

        for card in all_cards() {
            m.insert((&card.name).to_string(), card);
        }

        m
    };

    // Sets of cards available for transformations and shop inventory
    static ref ANY_CLASS_CARDS: Vec<String> =
        ALL_CARDS.values()
        .filter(|a| a._class != Class::Curse && a.rarity != Rarity::Starter && a.rarity != Rarity::Special)
        .map(|a| a.name.to_string())
        .collect();
    static ref IRONCLAD_CARDS: Vec<String> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::Ironclad && a.rarity != Rarity::Starter)
        .map(|a| a.name.to_string())
        .collect();
    static ref SILENT_CARDS: Vec<String> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::Silent)
        .map(|a| a.name.to_string())
        .collect();
    static ref DEFECT_CARDS: Vec<String> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::Defect)
        .map(|a| a.name.to_string())
        .collect();
    static ref WATCHER_CARDS: Vec<String> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::Watcher)
        .map(|a| a.name.to_string())
        .collect();

    static ref CURSES: Vec<String> =
        ALL_CARDS.values()
        .filter(|a| a._type == CardType::Curse)
        .map(|a| a.name.to_string())
        .collect();

    static ref COLORLESS_CARDS: Vec<String> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::None && a.rarity != Rarity::Special && a._type != CardType::Curse)
        .map(|a| a.name.to_string())
        .collect();
}

pub fn all_cards() -> Vec<BaseCard> {
    let filepath = Path::new("data").join("cards.ron");
    let file = File::open(filepath).unwrap();
    from_reader(file).unwrap()
}

