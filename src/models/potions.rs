use crate::models::buffs;
use crate::models::cards;
use crate::models::core::*;
use std::collections::HashMap;
use Amount::*;

pub fn by_name(name: &str) -> &'static BasePotion {
    POTIONS.get(name).unwrap()
}

lazy_static! {
    static ref POTIONS: HashMap<&'static str, BasePotion> = {
        let mut m = HashMap::new();

        for potion in all_potions() {
            m.insert(potion.name, potion);
        }

        m
    };
}

fn all_potions() -> Vec<BasePotion> {
    vec![
        BasePotion {
            name: AMBROSIA,
            _class: Class::Watcher,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::SetStance(Stance::Divinity)],
        },
        BasePotion {
            name: ANCIENT_POTION,
            _class: Class::None,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::AddBuff(
                buffs::ARTIFACT,
                Upgradable(1, 2),
                Target::_Self,
            )],
        },
        BasePotion {
            name: ATTACK_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddCard {
                card: CardReference::RandomType(CardType::Attack, Fixed(3)),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: Upgradable(1, 2),
                modifier: CardModifier::SetZeroTurnCost,
            }],
        },
        BasePotion {
            name: BLESSING_OF_THE_FORGE,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::UpgradeCard(CardLocation::PlayerHand(
                RelativePosition::All,
            ))],
        },
        BasePotion {
            name: BLOCK_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::Block(Upgradable(12, 24), Target::_Self)],
        },
        BasePotion {
            name: BLOOD_POTION,
            _class: Class::Ironclad,
            rarity: Rarity::Common,
            on_drink: vec![Effect::HealPercentage(Upgradable(20, 40), Target::_Self)],
        },
        BasePotion {
            name: BOTTLED_MIRACLE,
            _class: Class::Watcher,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddCard {
                card: CardReference::ByName(cards::MIRACLE),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: Upgradable(2, 4),
                modifier: CardModifier::None,
            }],
        },
        BasePotion {
            name: COLORLESS_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddCard {
                card: CardReference::RandomClass(Class::None),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: Upgradable(1, 2),
                modifier: CardModifier::SetZeroTurnCost,
            }],
        },
        BasePotion {
            name: CULTIST_POTION,
            _class: Class::None,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::AddBuff(
                buffs::RITUAL,
                Upgradable(1, 2),
                Target::_Self,
            )],
        },
        BasePotion {
            name: CUNNING_POTION,
            _class: Class::Silent,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::AddCard {
                card: CardReference::ByName(cards::SHIV),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: Upgradable(3, 6),
                modifier: CardModifier::None,
            }],
        },
        BasePotion {
            name: DEXTERITY_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddBuff(
                buffs::RITUAL,
                Upgradable(2, 4),
                Target::_Self,
            )],
        },
        BasePotion {
            name: DISTILLED_CHAOS,
            _class: Class::None,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::Repeat(
                Upgradable(3, 6),
                Box::new(Effect::AutoPlayCard(CardLocation::DrawPile(
                    RelativePosition::Top,
                ))),
            )],
        },
        BasePotion {
            name: DUPLICATION_POTION,
            _class: Class::None,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::AddBuff(
                buffs::DUPLICATION,
                Upgradable(1, 2),
                Target::_Self,
            )],
        },
        BasePotion {
            name: ELIXIR,
            _class: Class::Ironclad,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::ExhaustCard(CardLocation::PlayerHand(
                RelativePosition::PlayerChoice(Amount::Any),
            ))],
        },
        BasePotion {
            name: ENERGY_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddEnergy(Upgradable(2, 4))],
        },
        BasePotion {
            name: ENTROPIC_BREW,
            _class: Class::None,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::Custom],
        },
        BasePotion {
            name: ESSENCE_OF_DARKNESS,
            _class: Class::Defect,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::Repeat(
                Upgradable(1, 2),
                Box::new(Effect::ChannelOrb(Orb::Dark))
            )],
        },
        BasePotion {
            name: ESSENCE_OF_STEEL,
            _class: Class::None,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::AddBuff(
                buffs::PLATED_ARMOR,
                Upgradable(4, 8),
                Target::_Self,
            )],
        },
        BasePotion {
            name: EXPLOSIVE_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::Damage(Upgradable(10, 20), Target::AllEnemies)],
        },
        BasePotion {
            name: FAIRY_IN_A_BOTTLE,
            _class: Class::None,
            rarity: Rarity::Rare,
            on_drink: vec![],
        },
        BasePotion {
            name: FEAR_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddBuff(
                buffs::VULNERABLE,
                Upgradable(3, 6),
                Target::TargetEnemy,
            )],
        },
        BasePotion {
            name: FIRE_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::Damage(Upgradable(20, 40), Target::TargetEnemy)],
        },
        BasePotion {
            name: FOCUS_POTION,
            _class: Class::Defect,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddBuff(
                buffs::FOCUS,
                Upgradable(2, 4),
                Target::_Self,
            )],
        },
        BasePotion {
            name: FRUIT_JUICE,
            _class: Class::None,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::AddMaxHp(Upgradable(5, 10))],
        },
        BasePotion {
            name: GAMBLERS_BREW,
            _class: Class::None,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::Custom],
        },
        BasePotion {
            name: GHOST_IN_A_JAR,
            _class: Class::Silent,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::AddBuff(
                buffs::INTANGIBLE,
                Upgradable(1, 2),
                Target::_Self,
            )],
        },
        BasePotion {
            name: HEART_OF_IRON,
            _class: Class::Ironclad,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::AddBuff(
                buffs::METALLICIZE,
                Upgradable(6, 12),
                Target::_Self,
            )],
        },
        BasePotion {
            name: LIQUID_BRONZE,
            _class: Class::None,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::AddBuff(
                buffs::THORNS,
                Upgradable(3, 6),
                Target::_Self,
            )],
        },
        BasePotion {
            name: LIQUID_MEMORIES,
            _class: Class::None,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::MoveCard(
                CardLocation::DiscardPile(RelativePosition::PlayerChoice(Fixed(2))),
                CardLocation::PlayerHand(RelativePosition::Bottom),
                CardModifier::SetZeroTurnCost,
            )],
        },
        BasePotion {
            name: POISON_POTION,
            _class: Class::Silent,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddBuff(
                buffs::POISON,
                Upgradable(6, 12),
                Target::TargetEnemy,
            )],
        },
        BasePotion {
            name: POTION_OF_CAPACITY,
            _class: Class::Defect,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::AddOrbSlot(Upgradable(2, 4))],
        },
        BasePotion {
            name: POWER_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddCard {
                card: CardReference::RandomType(CardType::Power, Fixed(3)),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: Upgradable(1, 2),
                modifier: CardModifier::SetZeroTurnCost,
            }],
        },
        BasePotion {
            name: REGEN_POTION,
            _class: Class::None,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::AddBuff(
                buffs::REGENERATION,
                Upgradable(5, 10),
                Target::_Self,
            )],
        },
        BasePotion {
            name: SKILL_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddCard {
                card: CardReference::RandomType(CardType::Skill, Fixed(3)),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: Upgradable(1, 2),
                modifier: CardModifier::SetZeroTurnCost,
            }],
        },
        BasePotion {
            name: SMOKE_BOMB,
            _class: Class::None,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::Custom],
        },
        BasePotion {
            name: SNECKO_OIL,
            _class: Class::None,
            rarity: Rarity::Rare,
            on_drink: vec![Effect::Draw(Upgradable(5, 10)), Effect::Custom],
        },
        BasePotion {
            name: SPEED_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![
                Effect::AddBuff(buffs::DEXTERITY, Upgradable(5, 10), Target::_Self),
                Effect::AddBuff(buffs::DEXTERITY_DOWN, Upgradable(5, 10), Target::_Self),
            ],
        },
        BasePotion {
            name: STANCE_POTION,
            _class: Class::Watcher,
            rarity: Rarity::Uncommon,
            on_drink: vec![Effect::Custom],
        },
        BasePotion {
            name: FLEX_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![
                Effect::AddBuff(buffs::STRENGTH, Upgradable(5, 10), Target::_Self),
                Effect::AddBuff(buffs::STRENGTH_DOWN, Upgradable(5, 10), Target::_Self),
            ],
        },
        BasePotion {
            name: STRENGTH_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddBuff(
                buffs::STRENGTH,
                Upgradable(2, 4),
                Target::_Self,
            )],
        },
        BasePotion {
            name: SWIFT_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::Draw(Upgradable(3, 6))],
        },
        BasePotion {
            name: WEAK_POTION,
            _class: Class::None,
            rarity: Rarity::Common,
            on_drink: vec![Effect::AddBuff(
                buffs::WEAK,
                Upgradable(3, 6),
                Target::TargetEnemy,
            )],
        },
    ]
}

pub const AMBROSIA: &str = "Ambrosia";
pub const ANCIENT_POTION: &str = "Ancient Potion";
pub const ATTACK_POTION: &str = "Attack Potion";
pub const BLESSING_OF_THE_FORGE: &str = "Blessing of the Forge";
pub const BLOCK_POTION: &str = "Block Potion";
pub const BLOOD_POTION: &str = "Blood Potion";
pub const BOTTLED_MIRACLE: &str = "Bottled Miracle";
pub const COLORLESS_POTION: &str = "Colorless Potion";
pub const CULTIST_POTION: &str = "Cultist Potion";
pub const CUNNING_POTION: &str = "Cunning Potion";
pub const DEXTERITY_POTION: &str = "Dexterity Potion";
pub const DISTILLED_CHAOS: &str = "Distilled Chaos";
pub const DUPLICATION_POTION: &str = "Duplication Potion";
pub const ELIXIR: &str = "Elixir";
pub const ENERGY_POTION: &str = "Energy Potion";
pub const ENTROPIC_BREW: &str = "Entropic Brew";
pub const ESSENCE_OF_DARKNESS: &str = "Essence of Darkness";
pub const ESSENCE_OF_STEEL: &str = "Essence of Steel";
pub const EXPLOSIVE_POTION: &str = "Explosive Potion";
pub const FAIRY_IN_A_BOTTLE: &str = "Fairy in a Bottle";
pub const FEAR_POTION: &str = "Fear Potion";
pub const FIRE_POTION: &str = "Fire Potion";
pub const FOCUS_POTION: &str = "Focus Potion";
pub const FRUIT_JUICE: &str = "Fruit Juice";
pub const GAMBLERS_BREW: &str = "Gambler's Brew";
pub const GHOST_IN_A_JAR: &str = "Ghost In A Jar";
pub const HEART_OF_IRON: &str = "Heart of Iron";
pub const LIQUID_BRONZE: &str = "Liquid Bronze";
pub const LIQUID_MEMORIES: &str = "Liquid Memories";
pub const POISON_POTION: &str = "Poison Potion";
pub const POTION_OF_CAPACITY: &str = "Potion of Capacity";
pub const POWER_POTION: &str = "Power Potion";
pub const REGEN_POTION: &str = "Regen Potion";
pub const SKILL_POTION: &str = "Skill Potion";
pub const SMOKE_BOMB: &str = "Smoke Bomb";
pub const SNECKO_OIL: &str = "Snecko Oil";
pub const SPEED_POTION: &str = "Speed Potion";
pub const STANCE_POTION: &str = "Stance Potion";
pub const FLEX_POTION: &str = "Flex Potion";
pub const STRENGTH_POTION: &str = "Strength Potion";
pub const SWIFT_POTION: &str = "Swift Potion";
pub const WEAK_POTION: &str = "Weak Potion";
