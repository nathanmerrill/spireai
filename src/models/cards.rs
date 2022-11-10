use ron::de::from_reader;
use serde::{Deserialize, Serialize};

use ::std::hash::{Hash, Hasher};
use std::{collections::HashMap, error::Error, fs::File, path::Path};

use super::core::{is_default, Amount, Effect, CardType, Class, Condition, Rarity};

#[derive(Eq, Clone, Deserialize, Serialize)]
pub struct BaseCard {
    pub name: String,
    #[serde(rename = "type")]
    pub _type: CardType,
    #[serde(rename = "class")]
    pub _class: Class,
    #[serde(default, skip_serializing_if = "is_default")]
    pub cost: Amount,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rarity: Rarity,
    #[serde(
        default = "Condition::always",
        skip_serializing_if = "Condition::is_always"
    )]
    pub playable_if: Condition,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_start: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_play: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_discard: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_draw: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_exhaust: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_retain: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub on_turn_end: Vec<Effect>, //Happens if card is in hand, before cards are discarded
    #[serde(
        default = "Condition::never",
        skip_serializing_if = "Condition::is_never"
    )]
    pub innate: Condition,
    #[serde(
        default = "Condition::never",
        skip_serializing_if = "Condition::is_never"
    )]
    pub retain: Condition,
    #[serde(
        default = "Condition::never",
        skip_serializing_if = "Condition::is_never"
    )]
    pub targeted: Condition,
}
impl std::fmt::Debug for BaseCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseCard")
            .field("name", &self.name)
            .finish()
    }
}

impl Hash for BaseCard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
impl PartialEq for BaseCard {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn by_name(name: &str) -> &'static BaseCard {
    ALL_CARDS
        .get(name)
        .unwrap_or_else(|| panic!("Unrecognized card: {}", name))
}

pub fn available_cards_by_class(class: Class) -> &'static Vec<&'static BaseCard> {
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

        for card in all_cards().unwrap() {
            m.insert((&card.name).to_string(), card);
        }

        m
    };

    // Sets of cards available for transformations and shop inventory
    static ref ANY_CLASS_CARDS: Vec<&'static BaseCard> =
        ALL_CARDS.values()
        .filter(|a| a._class != Class::Curse && a.rarity != Rarity::Starter && a.rarity != Rarity::Special)
        //.map(|a| a.name.to_string())
        .collect();
    static ref IRONCLAD_CARDS: Vec<&'static BaseCard> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::Ironclad && a.rarity != Rarity::Starter)
        //.map(|a| a.name.to_string())
        .collect();
    static ref SILENT_CARDS: Vec<&'static BaseCard> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::Silent)
        //.map(|a| a.name.to_string())
        .collect();
    static ref DEFECT_CARDS: Vec<&'static BaseCard> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::Defect)
        //.map(|a| a.name.to_string())
        .collect();
    static ref WATCHER_CARDS: Vec<&'static BaseCard> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::Watcher)
        //.map(|a| a.name.to_string())
        .collect();

    static ref CURSES: Vec<&'static BaseCard> =
        ALL_CARDS.values()
        .filter(|a| a._type == CardType::Curse)
        .filter(|a| a.rarity != Rarity::Special)
        //.map(|a| a.name.to_string())
        .collect();

    static ref COLORLESS_CARDS: Vec<&'static BaseCard> =
        ALL_CARDS.values()
        .filter(|a| a._class == Class::None && a.rarity != Rarity::Special && a._type != CardType::Curse)
        //.map(|a| a.name.to_string())
        .collect();
}

fn all_cards() -> Result<Vec<BaseCard>, Box<dyn Error>> {
    let filepath = Path::new("data").join("cards.ron");
    let file = File::open(filepath)?;
    let u = from_reader(file)?;
    Ok(u)
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_parse() -> Result<(), String> {
        match super::all_cards() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
