use crate::models::core::*;
use crate::models::cards;
use crate::models::buffs;
use Amount::*;

impl BaseRelic {
    fn default() -> Self {
        Self {
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
    
    pub fn by_name(name: &str) -> Self {
        match name {
            BURNING_BLOOD => Self { 
                activation: Activation::Event(Event::CombatEnd),
                effect: Effect::Heal(Fixed(6)),
                rarity: Rarity::Starter,
                class: Class::Ironclad,
                ..Self::default()
            },
            RING_OF_THE_SNAKE => Self { 
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::Draw(Fixed(2)),
                rarity: Rarity::Starter,
                class: Class::Silent,
                ..Self::default()
            },
            CRACKED_CORE => Self { 
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::ChannelOrb(Orb::Lightning),
                rarity: Rarity::Starter,
                class: Class::Defect,
                ..Self::default()
            },
            PURE_WATER => Self { 
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddCard{
                    card: CardReference::ByName(cards::MIRACLE),
                    destination: CardLocation::PlayerHand(RelativePosition::Top), 
                    copies: Fixed(1),
                    modifier: CardModifier::None,
                },
                rarity: Rarity::Starter,
                class: Class::Watcher,
                ..Self::default()
            },
            AKABEKO => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::VIGOR, Fixed(8), EffectTarget::_Self),
                ..Self::default()
            },
            ANCHOR => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::Block(Fixed(10), EffectTarget::_Self),
                ..Self::default()
            },
            ANCIENT_TEA_SET => Self {
                activation: Activation::WhenEnabled{
                    enabled_at: Event::RoomEnter(RoomType::Battle),
                    disabled_at: Event::CombatStart,
                    activated_at: Event::CombatStart,
                },
                effect: Effect::AddEnergy(Fixed(2)),
                ..Self::default()
            },
            ART_OF_WAR => Self {
                activation: Activation::WhenEnabled{
                    enabled_at: Event::CombatStart,
                    disabled_at: Event::PlayCard(CardType::Attack),
                    activated_at: Event::CombatStart,
                },
                effect: Effect::AddEnergy(Fixed(1)),
                ..Self::default()
            },
            BAG_OF_MARBLES => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::VULNERABLE, Fixed(1), EffectTarget::AllEnemies),
                ..Self::default()
            },
            BAG_OF_PREPARATION => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::Draw(Fixed(2)),
                ..Self::default()
            },
            BLOOD_VIAL => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::Heal(Fixed(2)),
                ..Self::default()
            },
            BRONZE_SCALES => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::THORNS, Fixed(3), EffectTarget::_Self),
                ..Self::default()
            },
            CENTENNIAL_PUZZLE => Self {
                activation: Activation::Event(Event::HpLoss(EffectTarget::_Self)),
                effect: Effect::Draw(Fixed(3)),
                ..Self::default()
            },
            CERAMIC_FISH => Self {
                activation: Activation::Event(Event::AddToDeck(CardType::All)),
                effect: Effect::AddGold(Fixed(9)),
                ..Self::default()
            },
            DREAM_CATCHER => Self {
                activation: Activation::Event(Event::Rest),
                effect: Effect::CardReward,
                ..Self::default()
            },
            HAPPY_FLOWER => Self {
                activation: Activation::Counter {
                    increment: Event::TurnStart,
                    reset: Event::Never,
                    auto_reset: true,
                    target: 3,
                },
                effect: Effect::AddEnergy(Fixed(1)),
                ..Self::default()
            },
            JUZU_BRACELET => Self {
                effect: Effect::Custom,
                ..Self::default()
            },
            LANTERN => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddEnergy(Fixed(1)),
                ..Self::default()
            },
            MAW_BANK => Self {
                activation: Activation::Event(Event::RoomEnter(RoomType::All)),
                effect: Effect::AddGold(Fixed(12)),
                disable_at: Event::SpendGold,
                ..Self::default()
            },
            MEAL_TICKET => Self {
                activation: Activation::Event(Event::RoomEnter(RoomType::Shop)),
                effect: Effect::Heal(Fixed(15)),
                ..Self::default()
            },
            NUNCHAKU => Self {
                activation: Activation::Counter {
                    increment: Event::PlayCard(CardType::Attack),
                    reset: Event::Never,
                    auto_reset: true,
                    target: 10,
                },
                effect: Effect::Heal(Fixed(15)),
                ..Self::default()
            },
            ODDLY_SMOOTH_STONE => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::DEXTERITY, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            OMAMORI => Self {
                activation: Activation::Uses {
                    use_when: Event::AddToDeck(CardType::Curse),
                    uses: 2,
                },
                effect: Effect::Custom,
                ..Self::default()
            },
            ORICHALCUM => Self {
                activation: Activation::Event(Event::TurnEnd),
                effect: Effect::IfNoBlock(EffectTarget::_Self, vec![
                    Effect::Block(Fixed(6), EffectTarget::_Self)
                ]),
                ..Self::default()
            },
            PEN_NIB => Self {
                activation: Activation::Counter {
                    increment: Event::PlayCard(CardType::Attack),
                    reset: Event::Never,
                    auto_reset: true,
                    target: 10,
                },
                effect: Effect::Custom,
                ..Self::default()
            },
            POTION_BELT => Self {
                activation: Activation::Immediate,
                effect: Effect::AddPotionSlot(Fixed(2)),
                ..Self::default()
            },
            PRESERVED_INSECT => Self {
                activation: Activation::Event(Event::RoomEnter(RoomType::Elite)),
                effect: Effect::Custom,
                ..Self::default()
            },
            REGAL_PILLOW => Self {
                activation: Activation::Event(Event::Rest),
                effect: Effect::Heal(Fixed(15)),
                ..Self::default()
            },
            SMILING_MASK => Self {
                effect: Effect::Custom,
                ..Self::default()
            },
            STRAWBERRY => Self {
                activation: Activation::Immediate,
                effect: Effect::AddMaxHp(Fixed(7)),
                ..Self::default()
            },
            THE_BOOT => Self {
                effect: Effect::Custom,
                ..Self::default()
            },
            TINY_CHEST => Self {
                activation: Activation::Counter {
                    increment: Event::RoomEnter(RoomType::Question),
                    reset: Event::Never,
                    auto_reset: true,
                    target: 4,
                },
                effect: Effect::Custom,
                ..Self::default()
            },
            TOY_ORNITHOPTER => Self {
                activation: Activation::Event(Event::UsePotion),
                effect: Effect::Heal(Fixed(5)),
                ..Self::default()
            },
            VAJRA => Self {
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::STRENGTH, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            WAR_PAINT => Self {
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            WHETSTONE => Self {
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            DAMARU => Self {
                class: Class::Watcher,
                activation: Activation::Event(Event::TurnStart),
                effect: Effect::AddBuff(buffs::MANTRA, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            DATA_DISK => Self {
                class: Class::Defect,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::FOCUS, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            RED_SKULL => Self {
                class: Class::Ironclad,
                activation: Activation::Event(Event::HpChange(EffectTarget::_Self)),
                effect: Effect::AddBuff(buffs::STRENGTH, Amount::Custom, EffectTarget::_Self),
                ..Self::default()
            },
            SNECKO_SKULL => Self {
                class: Class::Silent,
                effect: Effect::Custom,
                ..Self::default()
            },
            BLUE_CANDLE => Self {
                rarity: Rarity::Uncommon,
                effect: Effect::Custom,
                ..Self::default()
            },
            BOTTLED_FLAME => Self {
                activation: Activation::Immediate,
                rarity: Rarity::Uncommon,
                effect: Effect::Custom,
                ..Self::default()
            },
            BOTTLED_LIGHTNING => Self {
                activation: Activation::Immediate,
                rarity: Rarity::Uncommon,
                effect: Effect::Custom,
                ..Self::default()
            },
            BOTTLED_TORNADO => Self {
                activation: Activation::Immediate,
                rarity: Rarity::Uncommon,
                effect: Effect::Custom,
                ..Self::default()
            },
            DARKSTONE_PERIAPT => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::AddToDeck(CardType::Curse)),
                effect: Effect::AddMaxHp(Fixed(6)),
                ..Self::default()
            },
            ETERNAL_FEATHER => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::RoomEnter(RoomType::Rest)),
                effect: Effect::Custom,
                ..Self::default()
            },
            FROZEN_EGG => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::AddToDeck(CardType::Power)),
                effect: Effect::Custom,
                ..Self::default()
            },
            GREMLIN_HORN => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::Die(EffectTarget::AllEnemies)),
                effect: Effect::Multiple(vec![
                    Effect::AddEnergy(Fixed(1)),
                    Effect::Draw(Fixed(1)),
                ]),
                ..Self::default()
            },
            HORN_CLEAT => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Counter {
                    increment: Event::TurnStart,
                    reset: Event::CombatStart,
                    auto_reset: false,
                    target: 2, 
                },
                effect: Effect::Block(Fixed(14), EffectTarget::_Self),
                ..Self::default()
            },
            INK_BOTTLE => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Counter {
                    increment: Event::PlayCard(CardType::All),
                    reset: Event::Never,
                    auto_reset: true,
                    target: 10, 
                },
                effect: Effect::Draw(Fixed(1)),
                ..Self::default()
            },
            KUNAI => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Counter {
                    increment: Event::PlayCard(CardType::Attack),
                    reset: Event::TurnStart,
                    auto_reset: true,
                    target: 3, 
                },
                effect: Effect::AddBuff(buffs::DEXTERITY, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            LETTER_OPENER => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Counter {
                    increment: Event::PlayCard(CardType::Skill),
                    reset: Event::TurnStart,
                    auto_reset: true,
                    target: 3, 
                },
                effect: Effect::Damage(Fixed(5), EffectTarget::AllEnemies),
                ..Self::default()
            },
            MATRYOSHKA => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Uses {
                    use_when: Event::ChestOpen,
                    uses: 2,
                },
                effect: Effect::Custom,
                ..Self::default()
            },
            MEAT_ON_THE_BONE => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::CombatEnd),
                effect: Effect::IfHalfHp(EffectTarget::_Self, vec![
                    Effect::Heal(Fixed(12))
                ]),
                ..Self::default()
            },
            MERCURY_HOURGLASS => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::TurnStart),
                effect: Effect::Damage(Fixed(3), EffectTarget::AllEnemies),
                ..Self::default()
            },
            MOLTEN_EGG => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::AddToDeck(CardType::Attack)),
                effect: Effect::Custom,
                ..Self::default()
            },
            MUMMIFIED_HAND => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::PlayCard(CardType::Power)),
                effect: Effect::SetCardModifier(CardLocation::PlayerHand(RelativePosition::Random), CardModifier::SetZeroTurnCost),
                ..Self::default()
            },
            ORNAMENTAL_FAN => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Counter {
                    increment: Event::PlayCard(CardType::Attack),
                    reset: Event::TurnStart,
                    auto_reset: true,
                    target: 3, 
                },
                effect: Effect::Block(Fixed(4), EffectTarget::_Self),
                ..Self::default()
            },
            PANTOGRAPH => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::RoomEnter(RoomType::Boss)),
                effect: Effect::Heal(Fixed(25)),
                ..Self::default()
            },
            PEAR => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Immediate,
                effect: Effect::AddMaxHp(Fixed(10)),
                ..Self::default()
            },
            QUESTION_CARD => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::CardReward),
                effect: Effect::Custom,
                ..Self::default()
            },
            SHURIKEN => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Counter {
                    increment: Event::PlayCard(CardType::Attack),
                    reset: Event::TurnStart,
                    auto_reset: true,
                    target: 3, 
                },
                effect: Effect::AddBuff(buffs::STRENGTH, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            SINGING_BOWL => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::CardReward),
                effect: Effect::Custom,
                ..Self::default()
            },
            STRIKE_DUMMY => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::PlayCard(CardType::Attack)),
                effect: Effect::Custom,
                ..Self::default()
            },
            SUNDIAL => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Counter {
                    increment: Event::Shuffle,
                    reset: Event::Never,
                    auto_reset: true,
                    target: 3, 
                },
                effect: Effect::AddEnergy(Fixed(2)),
                ..Self::default()
            },
            THE_COURIER => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            TOXIC_EGG => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::AddToDeck(CardType::Skill)),
                effect: Effect::Custom,
                ..Self::default()
            },
            WHITE_BEAST_STATUE => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::CardReward),
                effect: Effect::Custom,
                ..Self::default()
            },
            DUALITY => Self {
                class: Class::Watcher,
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::PlayCard(CardType::Attack)),
                effect: Effect::Multiple(vec![
                    Effect::AddBuff(buffs::DEXTERITY, Fixed(1), EffectTarget::_Self),
                    Effect::AddBuff(buffs::DEXTERITY_DOWN, Fixed(1), EffectTarget::_Self),
                ]),
                ..Self::default()
            },
            GOLD_PLATED_CABLES => Self {
                class: Class::Defect,
                rarity: Rarity::Uncommon,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            NINJA_SCROLL => Self {
                class: Class::Silent,
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddCard {
                    card: CardReference::ByName(cards::SHIV), 
                    destination: CardLocation::PlayerHand(RelativePosition::Top), 
                    copies: Fixed(3),
                    modifier: CardModifier::None,
                },
                ..Self::default()
            },
            PAPER_KRANE => Self {
                class: Class::Silent,
                rarity: Rarity::Uncommon,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            PAPER_PHROG => Self {
                class: Class::Ironclad,
                rarity: Rarity::Uncommon,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            SELF_FORMING_CLAY => Self {
                class: Class::Ironclad,
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::HpLoss(EffectTarget::_Self)),
                effect: Effect::AddBuff(buffs::NEXT_TURN_BLOCK, Fixed(3), EffectTarget::_Self),
                ..Self::default()
            },
            SYMBIOTIC_VIRUS => Self {
                class: Class::Defect,
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::ChannelOrb(Orb::Dark),
                ..Self::default()
            },
            TEARDROP_LOCKET => Self {
                class: Class::Watcher,
                rarity: Rarity::Uncommon,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::SetStance(Stance::Calm),
                ..Self::default()
            },
            BIRD_FACED_URN => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::PlayCard(CardType::Power)),
                effect: Effect::Heal(Fixed(2)),
                ..Self::default()
            },
            CALIPERS => Self {
                rarity: Rarity::Rare,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            CAPTAINS_WHEEL => Self {
                rarity: Rarity::Uncommon,
                activation: Activation::Counter {
                    increment: Event::TurnStart,
                    reset: Event::CombatStart,
                    auto_reset: false,
                    target: 3, 
                },
                effect: Effect::Block(Fixed(18), EffectTarget::_Self),
                ..Self::default()
            },
            DEAD_BRANCH => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Exhaust),
                effect: Effect::AddCard {
                    card: CardReference::RandomType(CardType::All), 
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: Fixed(1),
                    modifier: CardModifier::None,
                },
                ..Self::default()
            },
            DU_VU_DOLL => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::STRENGTH, Amount::Custom, EffectTarget::_Self),
                ..Self::default()
            },
            FOSSILIZED_HELIX => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::BUFFER, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            GINGER => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Buff(buffs::WEAK, EffectTarget::_Self)),
                effect: Effect::Custom,
                ..Self::default()
            },
            GIRYA => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::STRENGTH, X, EffectTarget::_Self),
                ..Self::default()
            },
            ICE_CREAM => Self {
                rarity: Rarity::Rare,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            INCENSE_BURNER => Self {
                rarity: Rarity::Rare,
                activation: Activation::Counter {
                    increment: Event::TurnStart,
                    reset: Event::Never,
                    auto_reset: true,
                    target: 6, 
                },
                effect: Effect::AddBuff(buffs::INTANGIBLE, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            LIZARD_TAIL => Self {
                rarity: Rarity::Rare,
                activation: Activation::Uses {
                    use_when: Event::Die(EffectTarget::_Self),
                    uses: 1,
                },
                effect: Effect::HealPercentage(50),
                ..Self::default()
            },
            MANGO => Self {
                rarity: Rarity::Rare,
                activation: Activation::Immediate,
                effect: Effect::AddMaxHp(Fixed(14)),
                ..Self::default()
            },
            OLD_COIN => Self {
                rarity: Rarity::Rare,
                activation: Activation::Immediate,
                effect: Effect::AddGold(Fixed(300)),
                ..Self::default()
            },
            PEACE_PIPE => Self {
                rarity: Rarity::Rare,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            POCKETWATCH => Self {
                rarity: Rarity::Rare,
                activation: Activation::Custom,
                effect: Effect::Draw(Fixed(3)),
                ..Self::default()
            },
            PRAYER_WHEEL => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::CardReward),
                effect: Effect::Custom,
                ..Self::default()
            },
            SHOVEL => Self {
                rarity: Rarity::Rare,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            STONE_CALENDAR => Self {
                rarity: Rarity::Rare,
                activation: Activation::Counter {
                    increment: Event::TurnEnd,
                    reset: Event::CombatStart,
                    auto_reset: false,
                    target: 7, 
                },
                effect: Effect::Damage(Fixed(50), EffectTarget::AllEnemies),
                ..Self::default()
            },
            THREAD_AND_NEEDLE => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::PLATED_ARMOR, Fixed(4), EffectTarget::_Self),
                ..Self::default()
            },
            TORII => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::UnblockedDamage(EffectTarget::_Self)),
                effect: Effect::Custom,
                ..Self::default()
            },
            TUNGSTEN_ROD => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::HpLoss(EffectTarget::_Self)),
                effect: Effect::Custom,
                ..Self::default()
            },
            TURNIP => Self {
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Buff(buffs::FRAIL, EffectTarget::_Self)),
                effect: Effect::Custom,
                ..Self::default()
            },
            UNCEASING_TOP => Self {
                rarity: Rarity::Rare,
                activation: Activation::Custom,
                effect: Effect::Draw(Fixed(1)),
                ..Self::default()
            },
            WING_BOOTS => Self {
                rarity: Rarity::Rare,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            CHAMPION_BELT => Self {
                class: Class::Ironclad,
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Buff(buffs::VULNERABLE, EffectTarget::AllEnemies)),
                effect: Effect::Custom,
                ..Self::default()
            },
            CHARONS_ASHES => Self {
                class: Class::Ironclad,
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Exhaust),
                effect: Effect::Damage(Fixed(3), EffectTarget::AllEnemies),
                ..Self::default()
            },
            EMOTION_CHIP => Self {
                class: Class::Defect,
                rarity: Rarity::Rare,
                activation: Activation::WhenEnabled{
                    enabled_at: Event::HpLoss(EffectTarget::_Self),
                    disabled_at: Event::TurnStart,
                    activated_at: Event::TurnStart,
                },
                effect: Effect::Custom,
                ..Self::default()
            },
            GOLDEN_EYE => Self {
                class: Class::Watcher,
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Scry),
                effect: Effect::Custom,
                ..Self::default()
            },
            MAGIC_FLOWER => Self {
                class: Class::Ironclad,
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Heal(EffectTarget::_Self)),
                effect: Effect::Custom,
                ..Self::default()
            },
            THE_SPECIMEN => Self {
                class: Class::Silent,
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Die(EffectTarget::AllEnemies)),
                effect: Effect::Custom,
                ..Self::default()
            },
            TINGSHA => Self {
                class: Class::Silent,
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Discard),
                effect: Effect::Damage(Fixed(3), EffectTarget::RandomEnemy),
                ..Self::default()
            },
            TOUGH_BANDAGES => Self {
                class: Class::Silent,
                rarity: Rarity::Rare,
                activation: Activation::Event(Event::Discard),
                effect: Effect::Block(Fixed(3), EffectTarget::_Self),
                ..Self::default()
            },
            CAULDRON => Self {
                rarity: Rarity::Shop,
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            CHEMICAL_X => Self {
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::PlayCard(CardType::All)),
                effect: Effect::Custom,
                ..Self::default()
            },
            CLOCKWORK_SOUVENIR => Self {
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::ARTIFACT, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            DOLLYS_MIRROR => Self {
                rarity: Rarity::Shop,
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            FROZEN_EYE => Self {
                rarity: Rarity::Shop,
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            HAND_DRILL => Self {
                rarity: Rarity::Shop,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            LEES_WAFFLE => Self {
                rarity: Rarity::Shop,
                activation: Activation::Immediate,
                effect: Effect::Multiple(vec![
                    Effect::HealPercentage(100),
                    Effect::AddMaxHp(Fixed(7)),
                ]),
                ..Self::default()
            },
            MEDICAL_KIT => Self {
                rarity: Rarity::Shop,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            MEMBERSHIP_CARD => Self {
                rarity: Rarity::Shop,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            ORANGE_PELLETS => Self {
                rarity: Rarity::Shop,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            ORRERY => Self {
                rarity: Rarity::Shop,
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            PRISMATIC_SHARD => Self {
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::CardReward),
                effect: Effect::Custom,
                ..Self::default()
            },
            SLING_OF_COURAGE => Self {
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::RoomEnter(RoomType::Elite)),
                effect: Effect::AddBuff(buffs::STRENGTH, Fixed(2), EffectTarget::_Self),
                ..Self::default()
            },
            STRANGE_SPOON => Self {
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::Exhaust),
                effect: Effect::Custom,
                ..Self::default()
            },
            THE_ABACUS => Self {
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::Shuffle),
                effect: Effect::Block(Fixed(6), EffectTarget::_Self),
                ..Self::default()
            },
            TOOLBOX => Self {
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::Custom,
                ..Self::default()
            },
            BRIMSTONE => Self {
                class: Class::Ironclad,
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::TurnStart),
                effect: Effect::Multiple(vec![
                    Effect::AddBuff(buffs::STRENGTH, Fixed(2), EffectTarget::_Self),
                    Effect::AddBuff(buffs::STRENGTH, Fixed(1), EffectTarget::AllEnemies),
                ]),
                ..Self::default()
            },
            MELANGE => Self {
                class: Class::Watcher,
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::Shuffle),
                effect: Effect::Scry(Fixed(3)),
                ..Self::default()
            },
            RUNIC_CAPACITOR => Self {
                class: Class::Defect,
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddOrbSlot(Fixed(3)),
                ..Self::default()
            },
            TWISTED_FUNNEL => Self {
                class: Class::Defect,
                rarity: Rarity::Shop,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::POISON, Fixed(4), EffectTarget::AllEnemies),
                ..Self::default()
            },
            ASTROLABE => Self {
                rarity: Rarity::Boss,
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            BLACK_STAR => Self {
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::CardReward),
                effect: Effect::Custom,
                ..Self::default()
            },
            BUSTED_CROWN => Self {
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::CardReward),
                effect: Effect::Custom,
                energy_relic: true,
                ..Self::default()
            },
            CALLING_BELL => Self {
                rarity: Rarity::Boss,
                activation: Activation::Immediate,
                effect: Effect::Multiple(vec![
                    Effect::AddCard{
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
                ..Self::default()
            },
            COFFEE_DRIPPER => Self {
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::RoomEnter(RoomType::Rest)),
                effect: Effect::Custom,
                energy_relic: true,
                ..Self::default()
            },
            CURSED_KEY => Self {
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::ChestOpen),
                effect: Effect::AddCard{
                    card: CardReference::RandomType(CardType::Curse),
                    destination: CardLocation::DeckPile(RelativePosition::Bottom), 
                    copies: Fixed(1),
                    modifier: CardModifier::None,
                },
                energy_relic: true,
                ..Self::default()
            },
            ECTOPLASM => Self {
                rarity: Rarity::Boss,
                activation: Activation::Custom,
                effect: Effect::Custom,
                energy_relic: true,
                ..Self::default()
            },
            EMPTY_CAGE => Self {
                rarity: Rarity::Boss,
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            FUSION_HAMMER => Self {
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::RoomEnter(RoomType::Rest)),
                effect: Effect::Custom,
                energy_relic: true,
                ..Self::default()
            },
            PANDORAS_BOX => Self {
                rarity: Rarity::Boss,
                activation: Activation::Immediate,
                effect: Effect::Custom,
                ..Self::default()
            },
            PHILOSOPHERS_STONE => Self {
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::STRENGTH, Fixed(1), EffectTarget::AllEnemies),
                energy_relic: true,
                ..Self::default()
            },
            RUNIC_DOME => Self {
                rarity: Rarity::Boss,
                activation: Activation::Custom,
                effect: Effect::Custom,
                energy_relic: true,
                ..Self::default()
            },
            RUNIC_PYRAMID => Self {
                rarity: Rarity::Boss,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            SACRED_BARK => Self {
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::UsePotion),
                effect: Effect::Custom,
                ..Self::default()
            },
            SLAVERS_COLLAR => Self {
                rarity: Rarity::Boss,
                activation: Activation::Event(
                    Event::Multiple(vec![
                        Event::RoomEnter(RoomType::Boss),
                        Event::RoomEnter(RoomType::Elite),
                    ])),
                effect: Effect::Custom,
                ..Self::default()
            },
            SNECKO_EYE => Self {
                rarity: Rarity::Boss,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            SOZU => Self {
                rarity: Rarity::Boss,
                activation: Activation::Custom,
                effect: Effect::Custom,
                energy_relic: true,
                ..Self::default()
            },
            TINY_HOUSE => Self {
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
                ..Self::default()
            },
            VELVET_CHOKER => Self {
                rarity: Rarity::Boss,
                activation: Activation::Counter {
                    increment: Event::PlayCard(CardType::All),
                    reset: Event::TurnStart,
                    auto_reset: false,
                    target: 6,
                },
                effect: Effect::Custom,
                energy_relic: true,
                ..Self::default()
            },
            BLACK_BLOOD => Self {
                class: Class::Ironclad,
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::CombatEnd),
                effect: Effect::Heal(Fixed(12)),
                replaces_starter: true,
                ..Self::default()
            },
            RING_OF_THE_SERPENT => Self {
                class: Class::Silent,
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::TurnStart),
                effect: Effect::Draw(Fixed(1)),
                replaces_starter: true,
                ..Self::default()
            },
            FROZEN_CORE => Self {
                class: Class::Defect,
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::TurnEnd),
                effect: Effect::Custom,
                replaces_starter: true,
                ..Self::default()
            },
            HOLY_WATER => Self {
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
                ..Self::default()
            },
            MARK_OF_PAIN => Self {
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
                ..Self::default()
            },
            RUNIC_CUBE => Self {
                class: Class::Ironclad,
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::HpLoss(EffectTarget::_Self)),
                effect: Effect::Draw(Fixed(1)),
                ..Self::default()
            },
            WRIST_BLADE => Self {
                class: Class::Silent,
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::PlayCard(CardType::Attack)),
                effect: Effect::Custom,
                ..Self::default()
            },
            HOVERING_KITE => Self {
                class: Class::Silent,
                rarity: Rarity::Boss,
                activation: Activation::WhenEnabled{
                    activated_at: Event::Discard,
                    enabled_at: Event::TurnStart,
                    disabled_at: Event::Discard,
                },
                effect: Effect::AddEnergy(Fixed(1)),
                ..Self::default()
            },
            INSERTER => Self {
                class: Class::Defect,
                rarity: Rarity::Boss,
                activation: Activation::Counter{
                    increment: Event::TurnStart,
                    reset: Event::Never,
                    auto_reset: true,
                    target: 2,
                },
                effect: Effect::AddOrbSlot(Fixed(1)),
                ..Self::default()
            },
            NUCLEAR_BATTERY => Self {
                class: Class::Defect,
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::ChannelOrb(Orb::Plasma),
                ..Self::default()
            },
            VIOLET_LOTUS => Self {
                class: Class::Defect,
                rarity: Rarity::Boss,
                activation: Activation::Event(Event::StanceChange(Stance::Calm, Stance::All)),
                effect: Effect::AddEnergy(Fixed(1)),
                ..Self::default()
            },
            BLOODY_IDOL => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::GainGold),
                effect: Effect::Heal(Fixed(5)),
                ..Self::default()
            },
            CULTIST_HEADPIECE => Self {
                rarity: Rarity::Event,
                ..Self::default()
            },
            ENCHIRIDION => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddCard {
                    card: CardReference::RandomType(CardType::Power), 
                    destination: CardLocation::PlayerHand(RelativePosition::Bottom), 
                    copies: Fixed(1),
                    modifier: CardModifier::SetZeroTurnCost,
                },
                ..Self::default()
            },
            FACE_OF_CLERIC => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::CombatEnd),
                effect: Effect::AddMaxHp(Fixed(1)),
                ..Self::default()
            },
            GOLDEN_IDOL => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::GainGold),
                effect: Effect::Multiply(Fixed(25)),
                ..Self::default()
            },
            GREMLIN_VISAGE => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::WEAK, Fixed(1), EffectTarget::_Self),
                ..Self::default()
            },
            MARK_OF_THE_BLOOM => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::Heal(EffectTarget::_Self)),
                effect: Effect::Cancel,
                ..Self::default()
            },
            MUTAGENIC_STRENGTH => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::Multiple(vec![
                    Effect::AddBuff(buffs::STRENGTH, Fixed(3), EffectTarget::_Self),
                    Effect::AddBuff(buffs::STRENGTH_DOWN, Fixed(3), EffectTarget::_Self),
                ]),
                ..Self::default()
            },
            NLOTHS_GIFT => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::CardReward),
                effect: Effect::Custom,
                ..Self::default()
            },
            NLOTHS_HUNGRY_FACE => Self {
                rarity: Rarity::Event,
                activation: Activation::Uses {
                    use_when: Event::ChestOpen,
                    uses: 1,
                },
                effect: Effect::Cancel,
                ..Self::default()
            },
            NECRONOMICON => Self {
                rarity: Rarity::Event,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            NEOWS_LAMENT => Self {
                rarity: Rarity::Event,
                activation: Activation::Uses {
                    use_when: Event::CombatStart,
                    uses: 3,
                },
                effect: Effect::Custom,
                ..Self::default()
            },
            NILRYS_CODEX => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::TurnEnd),
                effect: Effect::Custom,
                ..Self::default()
            },
            ODD_MUSHROOM => Self {
                rarity: Rarity::Event,
                activation: Activation::Custom,
                effect: Effect::Custom,
                ..Self::default()
            },
            RED_MASK => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::CombatStart),
                effect: Effect::AddBuff(buffs::WEAK, Fixed(1), EffectTarget::AllEnemies),
                ..Self::default()
            },
            SPIRIT_POOP => Self {
                rarity: Rarity::Event,
                ..Self::default()
            },
            SSSERPENT_HEAD => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::RoomEnter(RoomType::Question)),
                effect: Effect::AddGold(Fixed(50)),
                ..Self::default()
            },
            WARPED_TONGS => Self {
                rarity: Rarity::Event,
                activation: Activation::Event(Event::TurnStart),
                effect: Effect::UpgradeCard(CardLocation::PlayerHand(RelativePosition::Random)),
                ..Self::default()
            },
            CIRCLET => Self {
                rarity: Rarity::Special,
                ..Self::default()
            },
            _ => panic!("Unexpected relic"),
        }
    }
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
pub const SELF_FORMING_CLAY: &str = "Self-Forming Clay";
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