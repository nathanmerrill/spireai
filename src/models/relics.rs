use ::std::hash::{Hash, Hasher};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, path::Path, ptr};

use ron::de::from_reader;

use super::core::{is_default, Class, Effect, Rarity, When};

#[derive(Clone, Serialize, Deserialize)]
pub struct BaseRelic {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rarity: Rarity,
    #[serde(default, skip_serializing_if = "is_default")]
    pub activation: Activation,
    #[serde(default, skip_serializing_if = "is_default")]
    pub effect: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub disable_at: When,
    #[serde(default, skip_serializing_if = "is_default")]
    pub class: Class,
    #[serde(default, skip_serializing_if = "is_default")]
    pub energy_relic: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub replaces_starter: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub starting_x: i16,
    #[serde(default, skip_serializing_if = "is_default")]
    pub max_floor: u8,
    #[serde(default, skip_serializing_if = "is_default")]
    pub shop_relic: bool,
}

impl<'de> Deserialize<'de> for &'static BaseRelic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(BaseRelicVisitor)
    }
}

struct BaseRelicVisitor;

impl<'de> serde::de::Visitor<'de> for BaseRelicVisitor {
    type Value = &'static BaseRelic;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        RELICS
            .get(v)
            .ok_or(E::custom(format!("Unable to find {} as a relic", v)))
    }
}

impl std::fmt::Debug for BaseRelic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BaseRelic")
            .field("name", &self.name)
            .finish()
    }
}

impl Hash for &'static BaseRelic {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state)
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Deserialize, Serialize)]
pub enum Activation {
    Immediate,
    When(When),
    Counter {
        increment: When,
        reset: When,
        auto_reset: bool,
        target: u16,
    },
    Uses {
        use_when: When,
        uses: u16,
    },
    WhenEnabled {
        //Activation is triggered before any enable/disable checks
        activated_at: When,
        enabled_at: When,
        disabled_at: When,
    },
    Custom,
}

impl Default for Activation {
    fn default() -> Self {
        Activation::Custom
    }
}

pub fn by_name(name: &str) -> &'static BaseRelic {
    RELICS.get(name).unwrap_or_else(|| {
        panic!(
            "Unrecognized relic: {}, Available relics: {:?}",
            name,
            RELICS.keys().cloned().collect_vec()
        )
    })
}

lazy_static! {
    pub static ref RELICS: HashMap<String, BaseRelic> = {
        let mut m = HashMap::new();

        for relic in all_relics().unwrap() {
            m.insert((&relic.name).to_string(), relic);
        }

        m
    };
    pub static ref BAD_RELIC: BaseRelic = BaseRelic {
        name: String::from("ERROR"),
        rarity: Rarity::Special,
        activation: Activation::Custom,
        effect: vec![],
        disable_at: When::Never,
        class: Class::None,
        energy_relic: false,
        replaces_starter: false,
        shop_relic: false,
        starting_x: 0,
        max_floor: 0
    };
}

impl PartialEq for &'static BaseRelic {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for &'static BaseRelic {}

pub static ANCIENT_TEA_SET: &'static BaseRelic =
    RELICS.get("Ancient Tea Set").unwrap_or(&BAD_RELIC);
pub static BAG_OF_PREPARATION: &'static BaseRelic =
    RELICS.get("Bag of Preparation").unwrap_or(&BAD_RELIC);
pub static BLOODY_IDOL: &'static BaseRelic = RELICS.get("Bloody Idol").unwrap_or(&BAD_RELIC);
pub static BOTTLED_FLAME: &'static BaseRelic = RELICS.get("Bottled Flame").unwrap_or(&BAD_RELIC);
pub static BOTTLED_LIGHTNING: &'static BaseRelic =
    RELICS.get("Bottled Lightning").unwrap_or(&BAD_RELIC);
pub static BOTTLED_TORNADO: &'static BaseRelic =
    RELICS.get("Bottled Tornado").unwrap_or(&BAD_RELIC);
pub static BURNING_BLOOD: &'static BaseRelic = RELICS.get("Burning Blood").unwrap_or(&BAD_RELIC);
pub static BUSTED_CROWN: &'static BaseRelic = RELICS.get("Busted Crown").unwrap_or(&BAD_RELIC);
pub static CALIPERS: &'static BaseRelic = RELICS.get("Calipers").unwrap_or(&BAD_RELIC);
pub static CERAMIC_FISH: &'static BaseRelic = RELICS.get("Ceramic Fish").unwrap_or(&BAD_RELIC);
pub static COFFEE_DRIPPER: &'static BaseRelic = RELICS.get("Coffee Dripper").unwrap_or(&BAD_RELIC);
pub static CURSED_KEY: &'static BaseRelic = RELICS.get("Cursed Key").unwrap_or(&BAD_RELIC);
pub static CRACKED_CORE: &'static BaseRelic = RELICS.get("Cracked Core").unwrap_or(&BAD_RELIC);
pub static DARKSTONE_PERIAPT: &'static BaseRelic =
    RELICS.get("Darkstone Periapt").unwrap_or(&BAD_RELIC);
pub static DREAM_CATCHER: &'static BaseRelic = RELICS.get("Dream Catcher").unwrap_or(&BAD_RELIC);
pub static ECTOPLASM: &'static BaseRelic = RELICS.get("Ectoplasm").unwrap_or(&BAD_RELIC);
pub static ETERNAL_FEATHER: &'static BaseRelic =
    RELICS.get("Eternal Feather").unwrap_or(&BAD_RELIC);
pub static FROZEN_EGG: &'static BaseRelic = RELICS.get("Frozen Egg").unwrap_or(&BAD_RELIC);
pub static FROZEN_EYE: &'static BaseRelic = RELICS.get("Frozen Eye").unwrap_or(&BAD_RELIC);
pub static FUSION_HAMMER: &'static BaseRelic = RELICS.get("Fusion Hammer").unwrap_or(&BAD_RELIC);
pub static GIRYA: &'static BaseRelic = RELICS.get("Girya").unwrap_or(&BAD_RELIC);
pub static GOLDEN_IDOL: &'static BaseRelic = RELICS.get("Golden Idol").unwrap_or(&BAD_RELIC);
pub static JUZU_BRACELET: &'static BaseRelic = RELICS.get("Juzu Bracelet").unwrap_or(&BAD_RELIC);
pub static LIZARD_TAIL: &'static BaseRelic = RELICS.get("Lizard Tail").unwrap_or(&BAD_RELIC);
pub static MAGIC_FLOWER: &'static BaseRelic = RELICS.get("Magic Flower").unwrap_or(&BAD_RELIC);
pub static MARK_OF_THE_BLOOM: &'static BaseRelic =
    RELICS.get("Mark of the Bloom").unwrap_or(&BAD_RELIC);
pub static MATRYOSHKA: &'static BaseRelic = RELICS.get("Matryoshka").unwrap_or(&BAD_RELIC);
pub static MAW_BANK: &'static BaseRelic = RELICS.get("Maw Bank").unwrap_or(&BAD_RELIC);
pub static MEAL_TICKET: &'static BaseRelic = RELICS.get("Meal Ticket").unwrap_or(&BAD_RELIC);
pub static MEMBERSHIP_CARD: &'static BaseRelic =
    RELICS.get("Membership Card").unwrap_or(&BAD_RELIC);
pub static MOLTEN_EGG: &'static BaseRelic = RELICS.get("Molten Egg").unwrap_or(&BAD_RELIC);
pub static NLOTHS_GIFT: &'static BaseRelic = RELICS.get("N'loth's Gift").unwrap_or(&BAD_RELIC);
pub static ODD_MUSHROOM: &'static BaseRelic = RELICS.get("Odd Mushroom").unwrap_or(&BAD_RELIC);
pub static OMAMORI: &'static BaseRelic = RELICS.get("Omamori").unwrap_or(&BAD_RELIC);
pub static PANTOGRAPH: &'static BaseRelic = RELICS.get("Pantograph").unwrap_or(&BAD_RELIC);
pub static PAPER_KRANE: &'static BaseRelic = RELICS.get("Paper Krane").unwrap_or(&BAD_RELIC);
pub static PAPER_PHROG: &'static BaseRelic = RELICS.get("Paper Phrog").unwrap_or(&BAD_RELIC);
pub static PEACE_PIPE: &'static BaseRelic = RELICS.get("Peace Pipe").unwrap_or(&BAD_RELIC);
pub static PRAYER_WHEEL: &'static BaseRelic = RELICS.get("Prayer Wheel").unwrap_or(&BAD_RELIC);
pub static PRESERVED_INSECT: &'static BaseRelic =
    RELICS.get("Preserved Insect").unwrap_or(&BAD_RELIC);
pub static PRISMATIC_SHARD: &'static BaseRelic =
    RELICS.get("Prismatic Shard").unwrap_or(&BAD_RELIC);
pub static PURE_WATER: &'static BaseRelic = RELICS.get("Pure Water").unwrap_or(&BAD_RELIC);
pub static QUESTION_CARD: &'static BaseRelic = RELICS.get("Question Card").unwrap_or(&BAD_RELIC);
pub static REGAL_PILLOW: &'static BaseRelic = RELICS.get("Regal Pillow").unwrap_or(&BAD_RELIC);
pub static RING_OF_THE_SNAKE: &'static BaseRelic =
    RELICS.get("Ring Of The Snake").unwrap_or(&BAD_RELIC);
pub static RUNIC_DOME: &'static BaseRelic = RELICS.get("Runic Dome").unwrap_or(&BAD_RELIC);
pub static RUNIC_PYRAMID: &'static BaseRelic = RELICS.get("Runic Pyramid").unwrap_or(&BAD_RELIC);
pub static SACRED_BARK: &'static BaseRelic = RELICS.get("Sacred Bark").unwrap_or(&BAD_RELIC);
pub static SHOVEL: &'static BaseRelic = RELICS.get("Shovel").unwrap_or(&BAD_RELIC);
pub static SINGING_BOWL: &'static BaseRelic = RELICS.get("Singing Bowl").unwrap_or(&BAD_RELIC);
pub static SLING_OF_COURAGE: &'static BaseRelic =
    RELICS.get("Sling of Courage").unwrap_or(&BAD_RELIC);
pub static SMILING_MASK: &'static BaseRelic = RELICS.get("Smiling Mask").unwrap_or(&BAD_RELIC);
pub static SNECKO_EYE: &'static BaseRelic = RELICS.get("Snecko Eye").unwrap_or(&BAD_RELIC);
pub static SSSERPENT_HEAD: &'static BaseRelic = RELICS.get("Ssserpent Head").unwrap_or(&BAD_RELIC);
pub static STRANGE_SPOON: &'static BaseRelic = RELICS.get("Strange Spoon").unwrap_or(&BAD_RELIC);
pub static THE_BOOT: &'static BaseRelic = RELICS.get("The Boot").unwrap_or(&BAD_RELIC);
pub static THE_COURIER: &'static BaseRelic = RELICS.get("The Courier").unwrap_or(&BAD_RELIC);
pub static TINY_CHEST: &'static BaseRelic = RELICS.get("Tiny Chest").unwrap_or(&BAD_RELIC);
pub static TORII: &'static BaseRelic = RELICS.get("Torii").unwrap_or(&BAD_RELIC);
pub static TOXIC_EGG: &'static BaseRelic = RELICS.get("Toxic Egg").unwrap_or(&BAD_RELIC);
pub static TOY_ORNITHOPTER: &'static BaseRelic =
    RELICS.get("Toy Ornithopter").unwrap_or(&BAD_RELIC);
pub static TUNGSTEN_ROD: &'static BaseRelic = RELICS.get("Tungsten Rod").unwrap_or(&BAD_RELIC);
pub static VIOLET_LOTUS: &'static BaseRelic = RELICS.get("Violet Lotus").unwrap_or(&BAD_RELIC);

fn all_relics() -> Result<Vec<BaseRelic>, Box<dyn Error>> {
    let filepath = Path::new("data").join("relics.ron");
    let file = File::open(filepath)?;
    let u = from_reader(file)?;
    Ok(u)
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_parse() -> Result<(), String> {
        match super::all_relics() {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
