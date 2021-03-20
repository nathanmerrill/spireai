use crate::models::buffs;
use crate::models::cards;
use crate::models::core::*;
use std::collections::HashMap;
use Amount::*;

impl BaseRelic {
    fn default() -> BaseRelic {
        BaseRelic {
            name: &"",
            effect: Effect::None,
            activation: Activation::Custom,
            disable_at: Event::Never,
            rarity: Rarity::Common,
            class: Class::All,
            energy_relic: false,
            replaces_starter: false,
        }
    }
}

pub fn by_name(name: &str) -> &'static BaseRelic {
    RELICS.get(name).expect(format!("Unrecognized relic: {}", name).as_str())
}

lazy_static! {
    static ref RELICS: HashMap<&'static str, BaseRelic> = {
        let mut m = HashMap::new();

        for relic in all_relics() {
            m.insert(relic.name, relic);
        }

        m
    };
}

fn all_relics() -> Vec<BaseRelic> {
    vec![
        BaseRelic {
            name: BURNING_BLOOD,
            activation: Activation::Event(Event::CombatEnd),
            effect: Effect::Heal(Fixed(6), Target::_Self),
            rarity: Rarity::Starter,
            class: Class::Ironclad,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: RING_OF_THE_SNAKE,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::Draw(Fixed(2)),
            rarity: Rarity::Starter,
            class: Class::Silent,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CRACKED_CORE,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::ChannelOrb(OrbType::Lightning),
            rarity: Rarity::Starter,
            class: Class::Defect,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PURE_WATER,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddCard {
                card: CardReference::ByName(cards::MIRACLE),
                destination: CardLocation::PlayerHand(RelativePosition::Top),
                copies: Fixed(1),
                modifier: CardModifier::None,
            },
            rarity: Rarity::Starter,
            class: Class::Watcher,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: AKABEKO,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::VIGOR, Fixed(8), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ANCHOR,
            activation: Activation::WhenEnabled {
                activated_at: Event::TurnEnd,
                enabled_at: Event::CombatStart,
                disabled_at: Event::TurnEnd,
            },
            effect: Effect::Block(Fixed(10), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ANCIENT_TEA_SET,
            activation: Activation::WhenEnabled {
                enabled_at: Event::RoomEnter(RoomType::Battle),
                disabled_at: Event::CombatStart,
                activated_at: Event::CombatStart,
            },
            effect: Effect::AddEnergy(Fixed(2)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ART_OF_WAR,
            activation: Activation::WhenEnabled {
                enabled_at: Event::CombatStart,
                disabled_at: Event::PlayCard(CardType::Attack),
                activated_at: Event::CombatStart,
            },
            effect: Effect::AddEnergy(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BAG_OF_MARBLES,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::VULNERABLE, Fixed(1), Target::AllEnemies),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BAG_OF_PREPARATION,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::Draw(Fixed(2)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BLOOD_VIAL,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::Heal(Fixed(2), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BRONZE_SCALES,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::THORNS, Fixed(3), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CENTENNIAL_PUZZLE,
            activation: Activation::Event(Event::HpLoss(Target::_Self)),
            effect: Effect::Draw(Fixed(3)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CERAMIC_FISH,
            activation: Activation::Event(Event::AddToDeck(CardType::All)),
            effect: Effect::AddGold(Fixed(9)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: DREAM_CATCHER,
            activation: Activation::Event(Event::Rest),
            effect: Effect::CardReward,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: HAPPY_FLOWER,
            activation: Activation::Counter {
                increment: Event::BeforeHandDraw,
                reset: Event::Never,
                auto_reset: true,
                target: 3,
            },
            effect: Effect::AddEnergy(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: JUZU_BRACELET,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: LANTERN,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddEnergy(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MAW_BANK,
            activation: Activation::Event(Event::RoomEnter(RoomType::All)),
            effect: Effect::AddGold(Fixed(12)),
            disable_at: Event::SpendGold,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MEAL_TICKET,
            activation: Activation::Event(Event::RoomEnter(RoomType::Shop)),
            effect: Effect::Heal(Fixed(15), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: NUNCHAKU,
            activation: Activation::Counter {
                increment: Event::PlayCard(CardType::Attack),
                reset: Event::Never,
                auto_reset: true,
                target: 10,
            },
            effect: Effect::Heal(Fixed(15), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ODDLY_SMOOTH_STONE,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::DEXTERITY, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: OMAMORI,
            activation: Activation::Uses {
                use_when: Event::AddToDeck(CardType::Curse),
                uses: 2,
            },
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ORICHALCUM,
            activation: Activation::Event(Event::TurnEnd),
            effect: Effect::If(
                Condition::NoBlock(Target::_Self),
                vec![Effect::Block(Fixed(6), Target::_Self)],
                vec![],
            ),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PEN_NIB,
            activation: Activation::Counter {
                increment: Event::PlayCard(CardType::Attack),
                reset: Event::Never,
                auto_reset: true,
                target: 10,
            },
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: POTION_BELT,
            activation: Activation::Immediate,
            effect: Effect::AddPotionSlot(Fixed(2)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PRESERVED_INSECT,
            activation: Activation::Event(Event::RoomEnter(RoomType::Elite)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: REGAL_PILLOW,
            activation: Activation::Event(Event::Rest),
            effect: Effect::Heal(Fixed(15), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SMILING_MASK,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: STRAWBERRY,
            activation: Activation::Immediate,
            effect: Effect::AddMaxHp(Fixed(7)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: THE_BOOT,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TINY_CHEST,
            activation: Activation::Counter {
                increment: Event::RoomEnter(RoomType::Question),
                reset: Event::Never,
                auto_reset: true,
                target: 4,
            },
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TOY_ORNITHOPTER,
            activation: Activation::Event(Event::UsePotion),
            effect: Effect::Heal(Fixed(5), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: VAJRA,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::STRENGTH, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: WAR_PAINT,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: WHETSTONE,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: DAMARU,
            class: Class::Watcher,
            activation: Activation::Event(Event::BeforeHandDraw),
            effect: Effect::AddBuff(buffs::MANTRA, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: DATA_DISK,
            class: Class::Defect,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::FOCUS, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: RED_SKULL,
            class: Class::Ironclad,
            activation: Activation::Event(Event::HpChange(Target::_Self)),
            effect: Effect::AddBuff(buffs::STRENGTH, Amount::Custom, Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SNECKO_SKULL,
            class: Class::Silent,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BLUE_CANDLE,
            rarity: Rarity::Uncommon,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BOTTLED_FLAME,
            activation: Activation::Immediate,
            rarity: Rarity::Uncommon,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BOTTLED_LIGHTNING,
            activation: Activation::Immediate,
            rarity: Rarity::Uncommon,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BOTTLED_TORNADO,
            activation: Activation::Immediate,
            rarity: Rarity::Uncommon,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: DARKSTONE_PERIAPT,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::AddToDeck(CardType::Curse)),
            effect: Effect::AddMaxHp(Fixed(6)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ETERNAL_FEATHER,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::RoomEnter(RoomType::Rest)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: FROZEN_EGG,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::AddToDeck(CardType::Power)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: GREMLIN_HORN,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::Die(Target::AllEnemies)),
            effect: Effect::Multiple(vec![Effect::AddEnergy(Fixed(1)), Effect::Draw(Fixed(1))]),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: HORN_CLEAT,
            rarity: Rarity::Uncommon,
            activation: Activation::Counter {
                increment: Event::TurnEnd,
                reset: Event::CombatStart,
                auto_reset: false,
                target: 2,
            },
            effect: Effect::Block(Fixed(14), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: INK_BOTTLE,
            rarity: Rarity::Uncommon,
            activation: Activation::Counter {
                increment: Event::PlayCard(CardType::All),
                reset: Event::Never,
                auto_reset: true,
                target: 10,
            },
            effect: Effect::Draw(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: KUNAI,
            rarity: Rarity::Uncommon,
            activation: Activation::Counter {
                increment: Event::PlayCard(CardType::Attack),
                reset: Event::BeforeHandDraw,
                auto_reset: true,
                target: 3,
            },
            effect: Effect::AddBuff(buffs::DEXTERITY, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: LETTER_OPENER,
            rarity: Rarity::Uncommon,
            activation: Activation::Counter {
                increment: Event::PlayCard(CardType::Skill),
                reset: Event::BeforeHandDraw,
                auto_reset: true,
                target: 3,
            },
            effect: Effect::Damage(Fixed(5), Target::AllEnemies),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MATRYOSHKA,
            rarity: Rarity::Uncommon,
            activation: Activation::Uses {
                use_when: Event::ChestOpen,
                uses: 2,
            },
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MEAT_ON_THE_BONE,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::CombatEnd),
            effect: Effect::If(
                Condition::HalfHp(Target::_Self),
                vec![Effect::Heal(Fixed(12), Target::_Self)],
                vec![],
            ),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MERCURY_HOURGLASS,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::BeforeHandDraw),
            effect: Effect::Damage(Fixed(3), Target::AllEnemies),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MOLTEN_EGG,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::AddToDeck(CardType::Attack)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MUMMIFIED_HAND,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::PlayCard(CardType::Power)),
            effect: Effect::SetCardModifier(
                CardLocation::PlayerHand(RelativePosition::Random),
                CardModifier::SetZeroTurnCost,
            ),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ORNAMENTAL_FAN,
            rarity: Rarity::Uncommon,
            activation: Activation::Counter {
                increment: Event::PlayCard(CardType::Attack),
                reset: Event::BeforeHandDraw,
                auto_reset: true,
                target: 3,
            },
            effect: Effect::Block(Fixed(4), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PANTOGRAPH,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::RoomEnter(RoomType::Boss)),
            effect: Effect::Heal(Fixed(25), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PEAR,
            rarity: Rarity::Uncommon,
            activation: Activation::Immediate,
            effect: Effect::AddMaxHp(Fixed(10)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: QUESTION_CARD,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::CardReward),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SHURIKEN,
            rarity: Rarity::Uncommon,
            activation: Activation::Counter {
                increment: Event::PlayCard(CardType::Attack),
                reset: Event::BeforeHandDraw,
                auto_reset: true,
                target: 3,
            },
            effect: Effect::AddBuff(buffs::STRENGTH, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SINGING_BOWL,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::CardReward),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: STRIKE_DUMMY,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::PlayCard(CardType::Attack)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SUNDIAL,
            rarity: Rarity::Uncommon,
            activation: Activation::Counter {
                increment: Event::Shuffle,
                reset: Event::Never,
                auto_reset: true,
                target: 3,
            },
            effect: Effect::AddEnergy(Fixed(2)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: THE_COURIER,
            rarity: Rarity::Uncommon,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TOXIC_EGG,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::AddToDeck(CardType::Skill)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: WHITE_BEAST_STATUE,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::CardReward),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: DUALITY,
            class: Class::Watcher,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::PlayCard(CardType::Attack)),
            effect: Effect::Multiple(vec![
                Effect::AddBuff(buffs::DEXTERITY, Fixed(1), Target::_Self),
                Effect::AddBuff(buffs::DEXTERITY_DOWN, Fixed(1), Target::_Self),
            ]),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: GOLD_PLATED_CABLES,
            class: Class::Defect,
            rarity: Rarity::Uncommon,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: NINJA_SCROLL,
            class: Class::Silent,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddCard {
                card: CardReference::ByName(cards::SHIV),
                destination: CardLocation::PlayerHand(RelativePosition::Top),
                copies: Fixed(3),
                modifier: CardModifier::None,
            },
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PAPER_KRANE,
            class: Class::Silent,
            rarity: Rarity::Uncommon,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PAPER_PHROG,
            class: Class::Ironclad,
            rarity: Rarity::Uncommon,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SELF_FORMING_CLAY,
            class: Class::Ironclad,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::HpLoss(Target::_Self)),
            effect: Effect::AddBuff(buffs::NEXT_TURN_BLOCK, Fixed(3), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SYMBIOTIC_VIRUS,
            class: Class::Defect,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::ChannelOrb(OrbType::Dark),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TEARDROP_LOCKET,
            class: Class::Watcher,
            rarity: Rarity::Uncommon,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::SetStance(Stance::Calm),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BIRD_FACED_URN,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::PlayCard(CardType::Power)),
            effect: Effect::Heal(Fixed(2), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CALIPERS,
            rarity: Rarity::Rare,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CAPTAINS_WHEEL,
            rarity: Rarity::Uncommon,
            activation: Activation::Counter {
                increment: Event::TurnEnd,
                reset: Event::CombatStart,
                auto_reset: false,
                target: 3,
            },
            effect: Effect::Block(Fixed(18), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: DEAD_BRANCH,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Exhaust),
            effect: Effect::AddCard {
                card: CardReference::RandomType(CardType::All, Fixed(1)),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: Fixed(1),
                modifier: CardModifier::None,
            },
            ..BaseRelic::default()
        },
        BaseRelic {
            name: DU_VU_DOLL,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::STRENGTH, Amount::Custom, Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: FOSSILIZED_HELIX,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::BUFFER, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: GAMBLING_CHIP,
            rarity: Rarity::Rare,
            activation: Activation::WhenEnabled {
                activated_at: Event::AfterHandDraw,
                enabled_at: Event::CombatStart,
                disabled_at: Event::AfterHandDraw,
            },
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: GINGER,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Buff(buffs::WEAK, Target::_Self)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: GIRYA,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::STRENGTH, X, Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ICE_CREAM,
            rarity: Rarity::Rare,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: INCENSE_BURNER,
            rarity: Rarity::Rare,
            activation: Activation::Counter {
                increment: Event::BeforeHandDraw,
                reset: Event::Never,
                auto_reset: true,
                target: 6,
            },
            effect: Effect::AddBuff(buffs::INTANGIBLE, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: LIZARD_TAIL,
            rarity: Rarity::Rare,
            activation: Activation::Uses {
                use_when: Event::Die(Target::_Self),
                uses: 1,
            },
            effect: Effect::HealPercentage(Fixed(50), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MANGO,
            rarity: Rarity::Rare,
            activation: Activation::Immediate,
            effect: Effect::AddMaxHp(Fixed(14)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: OLD_COIN,
            rarity: Rarity::Rare,
            activation: Activation::Immediate,
            effect: Effect::AddGold(Fixed(300)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PEACE_PIPE,
            rarity: Rarity::Rare,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: POCKETWATCH,
            rarity: Rarity::Rare,
            activation: Activation::Custom,
            effect: Effect::Draw(Fixed(3)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PRAYER_WHEEL,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::CardReward),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SHOVEL,
            rarity: Rarity::Rare,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: STONE_CALENDAR,
            rarity: Rarity::Rare,
            activation: Activation::Counter {
                increment: Event::BeforeEnemyMove,
                reset: Event::CombatStart,
                auto_reset: false,
                target: 7,
            },
            effect: Effect::Damage(Fixed(50), Target::AllEnemies),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: THREAD_AND_NEEDLE,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::PLATED_ARMOR, Fixed(4), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TORII,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::UnblockedDamage(Target::_Self)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TUNGSTEN_ROD,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::HpLoss(Target::_Self)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TURNIP,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Buff(buffs::FRAIL, Target::_Self)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: UNCEASING_TOP,
            rarity: Rarity::Rare,
            activation: Activation::Custom,
            effect: Effect::Draw(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: WING_BOOTS,
            rarity: Rarity::Rare,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CHAMPION_BELT,
            class: Class::Ironclad,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Buff(buffs::VULNERABLE, Target::AllEnemies)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CHARONS_ASHES,
            class: Class::Ironclad,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Exhaust),
            effect: Effect::Damage(Fixed(3), Target::AllEnemies),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: EMOTION_CHIP,
            class: Class::Defect,
            rarity: Rarity::Rare,
            activation: Activation::WhenEnabled {
                enabled_at: Event::HpLoss(Target::_Self),
                disabled_at: Event::BeforeHandDraw,
                activated_at: Event::BeforeHandDraw,
            },
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CLOAK_CLASP,
            class: Class::Watcher,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::TurnEnd),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: GOLDEN_EYE,
            class: Class::Watcher,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Scry),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MAGIC_FLOWER,
            class: Class::Ironclad,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Heal(Target::_Self)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: THE_SPECIMEN,
            class: Class::Silent,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Die(Target::AllEnemies)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TINGSHA,
            class: Class::Silent,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Discard),
            effect: Effect::Damage(Fixed(3), Target::RandomEnemy),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TOUGH_BANDAGES,
            class: Class::Silent,
            rarity: Rarity::Rare,
            activation: Activation::Event(Event::Discard),
            effect: Effect::Block(Fixed(3), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CAULDRON,
            rarity: Rarity::Shop,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CHEMICAL_X,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::PlayCard(CardType::All)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CLOCKWORK_SOUVENIR,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::ARTIFACT, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: DOLLYS_MIRROR,
            rarity: Rarity::Shop,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: FROZEN_EYE,
            rarity: Rarity::Shop,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: HAND_DRILL,
            rarity: Rarity::Shop,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: LEES_WAFFLE,
            rarity: Rarity::Shop,
            activation: Activation::Immediate,
            effect: Effect::Multiple(vec![
                Effect::HealPercentage(Fixed(100), Target::_Self),
                Effect::AddMaxHp(Fixed(7)),
            ]),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MEDICAL_KIT,
            rarity: Rarity::Shop,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MEMBERSHIP_CARD,
            rarity: Rarity::Shop,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ORANGE_PELLETS,
            rarity: Rarity::Shop,
            activation: Activation::Custom,
            effect: Effect::RemoveDebuffs(Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ORRERY,
            rarity: Rarity::Shop,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PRISMATIC_SHARD,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::CardReward),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SLING_OF_COURAGE,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::RoomEnter(RoomType::Elite)),
            effect: Effect::AddBuff(buffs::STRENGTH, Fixed(2), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: STRANGE_SPOON,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::Exhaust),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: THE_ABACUS,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::Shuffle),
            effect: Effect::Block(Fixed(6), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TOOLBOX,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BRIMSTONE,
            class: Class::Ironclad,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::BeforeHandDraw),
            effect: Effect::Multiple(vec![
                Effect::AddBuff(buffs::STRENGTH, Fixed(2), Target::_Self),
                Effect::AddBuff(buffs::STRENGTH, Fixed(1), Target::AllEnemies),
            ]),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MELANGE,
            class: Class::Watcher,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::Shuffle),
            effect: Effect::Scry(Fixed(3)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: RUNIC_CAPACITOR,
            class: Class::Defect,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddOrbSlot(Fixed(3)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TWISTED_FUNNEL,
            class: Class::Defect,
            rarity: Rarity::Shop,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::POISON, Fixed(4), Target::AllEnemies),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ASTROLABE,
            rarity: Rarity::Boss,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BLACK_STAR,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::CardReward),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BUSTED_CROWN,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::CardReward),
            effect: Effect::Custom,
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CALLING_BELL,
            rarity: Rarity::Boss,
            activation: Activation::Immediate,
            effect: Effect::Multiple(vec![
                Effect::AddCard {
                    card: CardReference::ByName(cards::CURSE_OF_THE_BELL),
                    destination: CardLocation::DeckPile(RelativePosition::Bottom),
                    copies: Fixed(1),
                    modifier: CardModifier::None,
                },
                Effect::ShowReward {
                    cards: 0,
                    potions: 0,
                    relics: 3,
                    gold: 0,
                },
            ]),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: COFFEE_DRIPPER,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::RoomEnter(RoomType::Rest)),
            effect: Effect::Custom,
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CURSED_KEY,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::ChestOpen),
            effect: Effect::AddCard {
                card: CardReference::RandomType(CardType::Curse, Fixed(1)),
                destination: CardLocation::DeckPile(RelativePosition::Bottom),
                copies: Fixed(1),
                modifier: CardModifier::None,
            },
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ECTOPLASM,
            rarity: Rarity::Boss,
            activation: Activation::Custom,
            effect: Effect::Custom,
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: EMPTY_CAGE,
            rarity: Rarity::Boss,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: FUSION_HAMMER,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::RoomEnter(RoomType::Rest)),
            effect: Effect::Custom,
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PANDORAS_BOX,
            rarity: Rarity::Boss,
            activation: Activation::Immediate,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: PHILOSOPHERS_STONE,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::STRENGTH, Fixed(1), Target::AllEnemies),
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: RUNIC_DOME,
            rarity: Rarity::Boss,
            activation: Activation::Custom,
            effect: Effect::Custom,
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: RUNIC_PYRAMID,
            rarity: Rarity::Boss,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SACRED_BARK,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::UsePotion),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SLAVERS_COLLAR,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::Multiple(vec![
                Event::RoomEnter(RoomType::Boss),
                Event::RoomEnter(RoomType::Elite),
            ])),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SNECKO_EYE,
            rarity: Rarity::Boss,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SOZU,
            rarity: Rarity::Boss,
            activation: Activation::Custom,
            effect: Effect::Custom,
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: TINY_HOUSE,
            rarity: Rarity::Boss,
            activation: Activation::Immediate,
            effect: Effect::Multiple(vec![
                Effect::AddMaxHp(Fixed(6)),
                Effect::UpgradeCard(CardLocation::DeckPile(RelativePosition::Random)),
                Effect::ShowReward {
                    potions: 1,
                    cards: 1,
                    gold: 50,
                    relics: 0,
                },
            ]),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: VELVET_CHOKER,
            rarity: Rarity::Boss,
            activation: Activation::Counter {
                increment: Event::PlayCard(CardType::All),
                reset: Event::BeforeHandDraw,
                auto_reset: false,
                target: 6,
            },
            effect: Effect::Custom,
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BLACK_BLOOD,
            class: Class::Ironclad,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::CombatEnd),
            effect: Effect::Heal(Fixed(12), Target::_Self),
            replaces_starter: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: RING_OF_THE_SERPENT,
            class: Class::Silent,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::BeforeHandDraw),
            effect: Effect::Draw(Fixed(1)),
            replaces_starter: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: FROZEN_CORE,
            class: Class::Defect,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::BeforeEnemyMove),
            effect: Effect::Custom,
            replaces_starter: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: HOLY_WATER,
            class: Class::Watcher,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddCard {
                card: CardReference::ByName(cards::MIRACLE),
                destination: CardLocation::PlayerHand(RelativePosition::Top),
                copies: Fixed(3),
                modifier: CardModifier::None,
            },
            replaces_starter: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MARK_OF_PAIN,
            class: Class::Ironclad,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddCard {
                card: CardReference::ByName(cards::WOUND),
                destination: CardLocation::DeckPile(RelativePosition::Random),
                copies: Fixed(2),
                modifier: CardModifier::None,
            },
            energy_relic: true,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: RUNIC_CUBE,
            class: Class::Ironclad,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::HpLoss(Target::_Self)),
            effect: Effect::Draw(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: WRIST_BLADE,
            class: Class::Silent,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::PlayCard(CardType::Attack)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: HOVERING_KITE,
            class: Class::Silent,
            rarity: Rarity::Boss,
            activation: Activation::WhenEnabled {
                activated_at: Event::Discard,
                enabled_at: Event::BeforeHandDraw,
                disabled_at: Event::Discard,
            },
            effect: Effect::AddEnergy(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: INSERTER,
            class: Class::Defect,
            rarity: Rarity::Boss,
            activation: Activation::Counter {
                increment: Event::BeforeHandDraw,
                reset: Event::Never,
                auto_reset: true,
                target: 2,
            },
            effect: Effect::AddOrbSlot(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: NUCLEAR_BATTERY,
            class: Class::Defect,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::ChannelOrb(OrbType::Plasma),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: VIOLET_LOTUS,
            class: Class::Defect,
            rarity: Rarity::Boss,
            activation: Activation::Event(Event::StanceChange(Stance::Calm, Stance::All)),
            effect: Effect::AddEnergy(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: BLOODY_IDOL,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::GainGold),
            effect: Effect::Heal(Fixed(5), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CULTIST_HEADPIECE,
            rarity: Rarity::Event,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ENCHIRIDION,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddCard {
                card: CardReference::RandomType(CardType::Power, Fixed(1)),
                destination: CardLocation::PlayerHand(RelativePosition::Bottom),
                copies: Fixed(1),
                modifier: CardModifier::SetZeroTurnCost,
            },
            ..BaseRelic::default()
        },
        BaseRelic {
            name: FACE_OF_CLERIC,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::CombatEnd),
            effect: Effect::AddMaxHp(Fixed(1)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: GOLDEN_IDOL,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::GainGold),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: GREMLIN_VISAGE,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::WEAK, Fixed(1), Target::_Self),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MARK_OF_THE_BLOOM,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::Heal(Target::_Self)),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: MUTAGENIC_STRENGTH,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::Multiple(vec![
                Effect::AddBuff(buffs::STRENGTH, Fixed(3), Target::_Self),
                Effect::AddBuff(buffs::STRENGTH_DOWN, Fixed(3), Target::_Self),
            ]),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: NLOTHS_GIFT,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::CardReward),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: NLOTHS_HUNGRY_FACE,
            rarity: Rarity::Event,
            activation: Activation::Uses {
                use_when: Event::ChestOpen,
                uses: 1,
            },
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: NECRONOMICON,
            rarity: Rarity::Event,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: NEOWS_LAMENT,
            rarity: Rarity::Event,
            activation: Activation::Uses {
                use_when: Event::CombatStart,
                uses: 3,
            },
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: NILRYS_CODEX,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::BeforeEnemyMove),
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: ODD_MUSHROOM,
            rarity: Rarity::Event,
            activation: Activation::Custom,
            effect: Effect::Custom,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: RED_MASK,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::CombatStart),
            effect: Effect::AddBuff(buffs::WEAK, Fixed(1), Target::AllEnemies),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SPIRIT_POOP,
            rarity: Rarity::Event,
            ..BaseRelic::default()
        },
        BaseRelic {
            name: SSSERPENT_HEAD,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::RoomEnter(RoomType::Question)),
            effect: Effect::AddGold(Fixed(50)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: WARPED_TONGS,
            rarity: Rarity::Event,
            activation: Activation::Event(Event::BeforeHandDraw),
            effect: Effect::UpgradeCard(CardLocation::PlayerHand(RelativePosition::Random)),
            ..BaseRelic::default()
        },
        BaseRelic {
            name: CIRCLET,
            rarity: Rarity::Special,
            ..BaseRelic::default()
        },
    ]
}

pub const BURNING_BLOOD: &str = "Burning Blood";
pub const RING_OF_THE_SNAKE: &str = "Ring of the Snake";
pub const CRACKED_CORE: &str = "Cracked Core";
pub const PURE_WATER: &str = "Pure Water";
pub const AKABEKO: &str = "Akabeko";
pub const ANCHOR: &str = "Anchor";
pub const ANCIENT_TEA_SET: &str = "Ancient Tea Set";
pub const ART_OF_WAR: &str = "Art of War";
pub const BAG_OF_MARBLES: &str = "Bag of Marbles";
pub const BAG_OF_PREPARATION: &str = "Bag of Preparation";
pub const BLOOD_VIAL: &str = "Blood Vial";
pub const BRONZE_SCALES: &str = "Bronze Scales";
pub const CENTENNIAL_PUZZLE: &str = "Centennial Puzzle";
pub const CERAMIC_FISH: &str = "Ceramic Fish";
pub const DREAM_CATCHER: &str = "Dream Catcher";
pub const HAPPY_FLOWER: &str = "Happy Flower";
pub const JUZU_BRACELET: &str = "Juzu Bracelet";
pub const LANTERN: &str = "Lantern";
pub const MAW_BANK: &str = "Maw Bank";
pub const MEAL_TICKET: &str = "Meal Ticket";
pub const NUNCHAKU: &str = "Nunchaku";
pub const ODDLY_SMOOTH_STONE: &str = "Oddly Smooth Stone";
pub const OMAMORI: &str = "Omamori";
pub const ORICHALCUM: &str = "Orichalcum";
pub const PEN_NIB: &str = "Pen Nib";
pub const POTION_BELT: &str = "Potion Belt";
pub const PRESERVED_INSECT: &str = "Preserved Insect";
pub const REGAL_PILLOW: &str = "Regal Pillow";
pub const SMILING_MASK: &str = "Smiling Mask";
pub const STRAWBERRY: &str = "Strawberry";
pub const THE_BOOT: &str = "The Boot";
pub const TINY_CHEST: &str = "Tiny Chest";
pub const TOY_ORNITHOPTER: &str = "Toy Ornithopter";
pub const VAJRA: &str = "Vajra";
pub const WAR_PAINT: &str = "War Paint";
pub const WHETSTONE: &str = "Whetstone";
pub const DAMARU: &str = "Damaru";
pub const DATA_DISK: &str = "Data Disk";
pub const RED_SKULL: &str = "Red Skull";
pub const SNECKO_SKULL: &str = "Snecko Skull";
pub const BLUE_CANDLE: &str = "Blue Candle";
pub const BOTTLED_FLAME: &str = "Bottled Flame";
pub const BOTTLED_LIGHTNING: &str = "Bottled Lightning";
pub const BOTTLED_TORNADO: &str = "Bottled Tornado";
pub const DARKSTONE_PERIAPT: &str = "Darkstone Periapt";
pub const ETERNAL_FEATHER: &str = "Eternal Feather";
pub const FROZEN_EGG: &str = "Frozen Egg";
pub const GREMLIN_HORN: &str = "Gremlin Horn";
pub const HORN_CLEAT: &str = "Horn Cleat";
pub const INK_BOTTLE: &str = "Ink Bottle";
pub const KUNAI: &str = "Kunai";
pub const LETTER_OPENER: &str = "Letter Opener";
pub const MATRYOSHKA: &str = "Matryoshka";
pub const MEAT_ON_THE_BONE: &str = "Meat on the Bone";
pub const MERCURY_HOURGLASS: &str = "Mercury Hourglass";
pub const MOLTEN_EGG: &str = "Molten Egg";
pub const MUMMIFIED_HAND: &str = "Mummified Hand";
pub const ORNAMENTAL_FAN: &str = "Ornamental Fan";
pub const PANTOGRAPH: &str = "Pantograph";
pub const PEAR: &str = "Pear";
pub const QUESTION_CARD: &str = "Question Card";
pub const SHURIKEN: &str = "Shuriken";
pub const SINGING_BOWL: &str = "Singing Bowl";
pub const STRIKE_DUMMY: &str = "Strike Dummy";
pub const SUNDIAL: &str = "Sundial";
pub const THE_COURIER: &str = "The Courier";
pub const TOXIC_EGG: &str = "Toxic Egg";
pub const WHITE_BEAST_STATUE: &str = "White Beast Statue";
pub const DUALITY: &str = "Duality";
pub const GOLD_PLATED_CABLES: &str = "Gold-Plated Cables";
pub const NINJA_SCROLL: &str = "Ninja Scroll";
pub const PAPER_KRANE: &str = "Paper Krane";
pub const PAPER_PHROG: &str = "Paper Phrog";
pub const SELF_FORMING_CLAY: &str = "BaseRelic-Forming Clay";
pub const SYMBIOTIC_VIRUS: &str = "Symbiotic Virus";
pub const TEARDROP_LOCKET: &str = "Teardrop Locket";
pub const BIRD_FACED_URN: &str = "Bird-Faced Urn";
pub const CALIPERS: &str = "Calipers";
pub const CAPTAINS_WHEEL: &str = "Captain's Wheel";
pub const DEAD_BRANCH: &str = "Dead Branch";
pub const DU_VU_DOLL: &str = "Du-Vu Doll";
pub const FOSSILIZED_HELIX: &str = "Fossilized Helix";
pub const GAMBLING_CHIP: &str = "Gambling Chip";
pub const GINGER: &str = "Ginger";
pub const GIRYA: &str = "Girya";
pub const ICE_CREAM: &str = "Ice Cream";
pub const INCENSE_BURNER: &str = "Incense Burner";
pub const LIZARD_TAIL: &str = "Lizard Tail";
pub const MANGO: &str = "Mango";
pub const OLD_COIN: &str = "Old Coin";
pub const PEACE_PIPE: &str = "Peace Pipe";
pub const POCKETWATCH: &str = "Pocketwatch";
pub const PRAYER_WHEEL: &str = "Prayer Wheel";
pub const SHOVEL: &str = "Shovel";
pub const STONE_CALENDAR: &str = "Stone Calendar";
pub const THREAD_AND_NEEDLE: &str = "Thread and Needle";
pub const TORII: &str = "Torii";
pub const TUNGSTEN_ROD: &str = "Tungsten Rod";
pub const TURNIP: &str = "Turnip";
pub const UNCEASING_TOP: &str = "Unceasing Top";
pub const WING_BOOTS: &str = "Wing Boots";
pub const CHAMPION_BELT: &str = "Champion Belt";
pub const CHARONS_ASHES: &str = "Charon's Ashes";
pub const CLOAK_CLASP: &str = "Cloak Clasp";
pub const EMOTION_CHIP: &str = "Emotion Chip";
pub const GOLDEN_EYE: &str = "Golden Eye";
pub const MAGIC_FLOWER: &str = "Magic Flower";
pub const THE_SPECIMEN: &str = "The Specimen";
pub const TINGSHA: &str = "Tingsha";
pub const TOUGH_BANDAGES: &str = "Tough Bandages";
pub const CAULDRON: &str = "Cauldron";
pub const CHEMICAL_X: &str = "Chemical X";
pub const CLOCKWORK_SOUVENIR: &str = "Clockwork Souvenir";
pub const DOLLYS_MIRROR: &str = "Dolly's Mirror";
pub const FROZEN_EYE: &str = "Frozen Eye";
pub const HAND_DRILL: &str = "Hand Drill";
pub const LEES_WAFFLE: &str = "Lee's Waffle";
pub const MEDICAL_KIT: &str = "Medical Kit";
pub const MEMBERSHIP_CARD: &str = "Membership Card";
pub const ORANGE_PELLETS: &str = "Orange Pellets";
pub const ORRERY: &str = "Orrery";
pub const PRISMATIC_SHARD: &str = "Prismatic Shard";
pub const SLING_OF_COURAGE: &str = "Sling of Courage";
pub const STRANGE_SPOON: &str = "Strange Spoon";
pub const THE_ABACUS: &str = "The Abacus";
pub const TOOLBOX: &str = "Toolbox";
pub const BRIMSTONE: &str = "Brimstone";
pub const MELANGE: &str = "Melange";
pub const RUNIC_CAPACITOR: &str = "Runic Capacitor";
pub const TWISTED_FUNNEL: &str = "Twisted Funnel";
pub const ASTROLABE: &str = "Astrolabe";
pub const BLACK_STAR: &str = "Black Star";
pub const BUSTED_CROWN: &str = "Busted Crown";
pub const CALLING_BELL: &str = "Calling Bell";
pub const COFFEE_DRIPPER: &str = "Coffee Dripper";
pub const CURSED_KEY: &str = "Cursed Key";
pub const ECTOPLASM: &str = "Ectoplasm";
pub const EMPTY_CAGE: &str = "Empty Cage";
pub const FUSION_HAMMER: &str = "Fusion Hammer";
pub const PANDORAS_BOX: &str = "Pandora's Box";
pub const PHILOSOPHERS_STONE: &str = "Philosopher's Stone";
pub const RUNIC_DOME: &str = "Runic Dome";
pub const RUNIC_PYRAMID: &str = "Runic Pyramid";
pub const SACRED_BARK: &str = "Sacred Bark";
pub const SLAVERS_COLLAR: &str = "Slaver's Collar";
pub const SNECKO_EYE: &str = "Snecko Eye";
pub const SOZU: &str = "Sozu";
pub const TINY_HOUSE: &str = "Tiny House";
pub const VELVET_CHOKER: &str = "Velvet Choker";
pub const BLACK_BLOOD: &str = "Black Blood";
pub const RING_OF_THE_SERPENT: &str = "Ring of the Serpent";
pub const FROZEN_CORE: &str = "Frozen Core";
pub const HOLY_WATER: &str = "Holy Water";
pub const MARK_OF_PAIN: &str = "Mark of Pain";
pub const RUNIC_CUBE: &str = "Runic Cube";
pub const WRIST_BLADE: &str = "Wrist Blade";
pub const HOVERING_KITE: &str = "Hovering Kite";
pub const INSERTER: &str = "Inserter";
pub const NUCLEAR_BATTERY: &str = "Nuclear Battery";
pub const VIOLET_LOTUS: &str = "Violet Lotus";
pub const BLOODY_IDOL: &str = "Bloody Idol";
pub const CULTIST_HEADPIECE: &str = "Cultist Headpiece";
pub const ENCHIRIDION: &str = "Enchiridion";
pub const FACE_OF_CLERIC: &str = "Face of Cleric";
pub const GOLDEN_IDOL: &str = "Golden Idol";
pub const GREMLIN_VISAGE: &str = "Gremlin Visage";
pub const MARK_OF_THE_BLOOM: &str = "Mark of the Bloom";
pub const MUTAGENIC_STRENGTH: &str = "Mutagenic Strength";
pub const NLOTHS_GIFT: &str = "N'loth's Gift";
pub const NLOTHS_HUNGRY_FACE: &str = "N'loth's Hungry Face";
pub const NECRONOMICON: &str = "Necronomicon";
pub const NEOWS_LAMENT: &str = "Neow's Lament";
pub const NILRYS_CODEX: &str = "Nilry's Codex";
pub const ODD_MUSHROOM: &str = "Odd Mushroom";
pub const RED_MASK: &str = "Red Mask";
pub const SPIRIT_POOP: &str = "Spirit Poop";
pub const SSSERPENT_HEAD: &str = "Ssserpent Head";
pub const WARPED_TONGS: &str = "Warped Tongs";
pub const CIRCLET: &str = "Circlet";
