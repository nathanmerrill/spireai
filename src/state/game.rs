use im::{vector, HashMap, Vector, HashSet};
use itertools::Itertools;
use uuid::Uuid;

use crate::{
    models::{
        self,
        cards::BaseCard,
        core::{CardType, ChestType, Class, When, FightType, Rarity},
        potions::BasePotion,
        relics::{Activation, BaseRelic},
    },
    spireai::references::{
        PotionReference, RelicReference,
    }
};

use super::{
    core::{Card, CardOffer, Potion, Relic, HpRange},
    map::MapState, shop::ShopState, probability::Probability, floor::KeyState,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub relics: Relics,
    pub act: u8,
    pub asc: u8,
    pub deck: HashMap<Uuid, Card>,
    pub potions: Vector<Option<Potion>>,
    pub gold: u16,
    pub hp: HpRange,
    pub map: MapState,
    pub keys: Option<KeyState>,
    pub won: Option<bool>,
    pub purge_count: u8,
    pub rare_probability_offset: u8,
}

impl GameState {
    pub fn add_max_hp(&mut self, amount: u16) {
        self.hp.max += amount;
        self.heal(amount as f64);
    }

    pub fn heal(&mut self, amount: f64) {
        if self.relics.contains("Mark Of The Bloom") {
            return;
        }

        self.hp.add(amount);
    }

    pub fn remove_card(&mut self, card: Uuid) {
        self.deck.remove(&card);
    }

    pub fn add_card(&mut self, mut card: Card) {
        if card.base._type == CardType::Curse {
            if let Some(relic) = self.relics.find_mut("Omamori") {
                if relic.vars.x > 0 {
                    relic.vars.x -= 1;
                    return;
                }
            }

            if self.relics.contains("Darkstone Periapt") {
                self.add_max_hp(6);
            }
        }

        if self.relics.contains("Ceramic Fish") {
            self.add_gold(9);
        }
        let should_upgrade = 
        match card.base._type {
            CardType::Attack => self.relics.contains("Molten Egg"),
            CardType::Skill => self.relics.contains("Toxic Egg"),
            CardType::Power => self.relics.contains("Frozen Egg"),
            CardType::Curse => false,
            CardType::Status => false,
            CardType::All => panic!("Unexpected card type of All"),
        };

        if should_upgrade && card.upgrades == 0 {
            card.upgrades = 1;
        }

        card.bottled = false;

        self.deck.insert(card.uuid, card);
    }

    pub fn add_gold(&mut self, amount: u16) {
        if self.relics.contains("Ectoplasm") {
            return;
        }

        if self.relics.contains("Bloody Idol") {
            self.heal(5_f64);
        }

        self.gold += amount;
    }
    
    pub fn random_potion(&mut self, no_healing: bool, probability: &mut Probability) -> &'static BasePotion {
        let rarities = vec![
            (Rarity::Common, 70),
            (Rarity::Uncommon, 25),
            (Rarity::Rare, 5),
        ];

        let rarity = *probability.choose_weighted(&rarities).unwrap();

        let potions = models::potions::POTIONS
            .values()
            .filter(|a| a.rarity == rarity && !(no_healing && a.name == "Fruit Juice"))
            .collect_vec();

        probability.choose(potions).unwrap()
    }

    pub fn add_potion(&mut self, base: &'static BasePotion) {
        if let Some(slot) = self.potions.iter().position(|a| a.is_none()) {
            self.potions.set(slot, Some(Potion { base }));
        }
    }

    pub fn find_potion(&mut self, name: &str) -> Option<PotionReference> {
        self.potions()
            .find_map(|p| if p.base.name == name { Some(p) } else { None })
    }

    pub fn potion_at(&self, slot: usize) -> Option<PotionReference> {
        self.potions[slot]
            .as_ref()
            .map(|potion| potion.reference(slot))
    }

    pub fn new(class: Class, asc: u8) -> Self {
        let mut cards = match class {
            Class::Ironclad => vec![
                "Strike", "Strike", "Strike", "Strike", "Strike", "Defend", "Defend", "Defend",
                "Defend", "Bash",
            ],
            Class::Silent => vec![
                "Strike",
                "Strike",
                "Strike",
                "Strike",
                "Strike",
                "Defend",
                "Defend",
                "Defend",
                "Defend",
                "Defend",
                "Survivor",
                "Neutralize",
            ],
            Class::Defect => vec![
                "Strike", "Strike", "Strike", "Strike", "Defend", "Defend", "Defend", "Defend",
                "Zap", "Dualcast",
            ],
            Class::Watcher => vec![
                "Strike",
                "Strike",
                "Strike",
                "Strike",
                "Defend",
                "Defend",
                "Defend",
                "Defend",
                "Eruption",
                "Vigilance",
            ],
            _ => panic!("Unexpected class!"),
        };

        if asc >= 10 {
            cards.push("Ascender's Bane")
        };

        let deck = cards
            .iter()
            .map(|name| {
                let card = Card::by_name(name);
                (card.uuid, card)
            })
            .collect();

        let potions = if asc >= 11 {
            vector![None, None]
        } else {
            vector![None, None, None]
        };

        let mut max_hp = match class {
            Class::Ironclad => 80,
            Class::Silent => 70,
            Class::Defect => 75,
            Class::Watcher => 72,
            _ => panic!("Unexpected class!"),
        };

        if asc >= 14 {
            if class == Class::Ironclad {
                max_hp -= 5
            } else {
                max_hp -= 4
            }
        }

        let hp = if asc >= 6 {
            (max_hp as f64 * 0.90).ceil() as u16
        } else {
            max_hp
        };

        let starting_relic = match class {
            Class::Ironclad => "Burning Blood",
            Class::Silent => "Ring of the Snake",
            Class::Defect => "Cracked Core",
            Class::Watcher => "Pure Water",
            _ => panic!("Unexpected class!"),
        };

        let mut state = Self {
            class,
            map: MapState::new(),
            relics: Relics::new(),
            act: 1,
            asc,
            deck,
            potions,
            hp: HpRange { amount: hp, max: max_hp },
            gold: 99,
            keys: None,
            won: None,
            purge_count: 0,
            rare_probability_offset: 0,
        };

        state.relics.add(models::relics::by_name(starting_relic));

        state
    }
    
    pub fn random_relic(
        &mut self,
        chest_type: Option<ChestType>,
        rarity: Option<Rarity>,
        in_shop: bool,
        probability: &mut Probability
    ) -> &'static BaseRelic {
        let probabilities = match chest_type {
            None => match rarity {
                None => [50, 33, 17, 0, 0],
                Some(Rarity::Shop) => [0, 0, 0, 0, 100],
                Some(Rarity::Boss) => [0, 0, 0, 100, 0],
                Some(Rarity::Rare) => [0, 0, 100, 0, 0],
                Some(Rarity::Uncommon) => [0, 100, 0, 0, 0],
                Some(Rarity::Common) => [100, 0, 0, 0, 0],
                _ => panic!("Unexpected rarity"),
            },
            Some(ChestType::Small) => [75, 25, 0, 0, 0],
            Some(ChestType::Medium) => [35, 50, 15, 0, 0],
            Some(ChestType::Large) => [0, 75, 25, 0, 0],
            Some(ChestType::Boss) => [0, 0, 0, 100, 0],
        };

        let rarities = [
            Rarity::Common,
            Rarity::Uncommon,
            Rarity::Rare,
            Rarity::Boss,
            Rarity::Shop,
        ];

        let choices = rarities
            .iter()
            .zip(probabilities.iter().copied())
            .collect_vec();

        let rarity = probability.choose_weighted(&choices).unwrap();

        let available_relics = models::relics::RELICS
            .values()
            .filter(|relic| {
                relic.rarity == **rarity
                    && (relic.class == self.class || relic.class == Class::All)
                    && !self.relics.contains(&relic.name)
                    && !self.relics.seen.contains(relic)
                    && (relic.max_floor == 0 || relic.max_floor as i8 >= self.map.floor)
                    && match relic.name.as_str() {
                        "Maw Bank" | "Smiling Mask" | "The Courier" | "Old Coin" => !in_shop,
                        "Bottled Flame" => self.deck.values().any(|c| {
                            c.base._type == CardType::Attack && c.base.rarity != Rarity::Starter
                        }),
                        "Bottled Lightning" => self.deck.values().any(|c| {
                            c.base._type == CardType::Skill && c.base.rarity != Rarity::Starter
                        }),
                        "Bottled Tornado" => self
                            .deck
                            .values()
                            .any(|c| c.base._type == CardType::Power),
                        "Girya" => {
                            !self.relics.contains("Peace Pipe")
                                || !self.relics.contains("Shovel")
                        }
                        "Shovel" => {
                            !self.relics.contains("Peace Pipe")
                                || !self.relics.contains("Girya")
                        }
                        "Peace Pipe" => {
                            !self.relics.contains("Girya")
                                || !self.relics.contains("Shovel")
                        }
                        "Black Blood" => self.relics.contains("Burning Blood"),
                        "Frozen Core" => self.relics.contains("Cracked Core"),
                        "Holy Water" => self.relics.contains("Pure Water"),
                        "Ring of the Snake" => self.relics.contains("Ring of the Serpent"),
                        _ => true,
                    }
            })
            .collect();

        let relic = probability
            .choose(available_relics)
            .expect("No available relics to be chosen!");

        self.relics.seen.insert(relic);

        relic
    }

    pub fn generate_card_rewards(
        &mut self,
        fight_type: FightType,
        colorless: bool,
        probability: &mut Probability
    ) -> Vector<CardOffer> {
        let cards = {
            if colorless {
                models::cards::available_cards_by_class(Class::None)
            } else if self.relics.contains("Prismatic Shard") {
                models::cards::available_cards_by_class(Class::All)
            } else {
                models::cards::available_cards_by_class(self.class)
            }
        };

        let count =
            if self.relics.contains("Busted Crown") {
                1
            } else {
                2
            }
            +
            if self.relics.contains("Question Card") {
                1
            } else {
                0
            };

        self.generate_card_offers(Some(fight_type), cards, count, true, probability)
    }

    fn generate_card_offers(
        &mut self,
        fight_type: Option<FightType>,
        available: &[&'static BaseCard],
        count: usize,
        reset_rarity: bool,
        probability: &mut Probability
    ) -> Vector<CardOffer> {
        let mut cards = available.to_owned();

        (0..count)
            .map(|_| {
                let offer = self.generate_card_offer(fight_type, &cards, probability);
                let index = cards.iter().position(|b| b == &offer.base).unwrap();
                cards.remove(index);
                match offer.base.rarity {
                    Rarity::Rare => {
                        if reset_rarity {
                            self.rare_probability_offset = 0;
                        }
                    }
                    Rarity::Common => {
                        self.rare_probability_offset =
                            std::cmp::min(self.rare_probability_offset + 1, 40);
                    }
                    _ => {}
                }
                offer
            })
            .collect()
    }

    
    pub fn generate_card_offer(
        &self,
        fight_type: Option<FightType>,
        available: &[&'static BaseCard],
        probability: &mut Probability
    ) -> CardOffer {
        let has_nloth = self.relics.contains("N'loth's Gift");

        let rarity_probabilities = match fight_type {
            Some(FightType::Common) => {
                if has_nloth {
                    [
                        4 + self.rare_probability_offset,
                        37,
                        59 - self.rare_probability_offset,
                    ]
                } else if self.rare_probability_offset < 2 {
                    [
                        0,
                        35 + self.rare_probability_offset,
                        65 - self.rare_probability_offset,
                    ]
                } else {
                    [
                        self.rare_probability_offset - 2,
                        37,
                        65 - self.rare_probability_offset,
                    ]
                }
            }
            Some(FightType::Elite { .. }) => {
                if has_nloth {
                    if self.rare_probability_offset < 31 {
                        [
                            25 + self.rare_probability_offset,
                            40,
                            35 - self.rare_probability_offset,
                        ]
                    } else {
                        [
                            25 + self.rare_probability_offset,
                            75 - self.rare_probability_offset,
                            0,
                        ]
                    }
                } else {
                    [
                        5 + self.rare_probability_offset,
                        40,
                        55 - self.rare_probability_offset,
                    ]
                }
            }
            Some(FightType::Boss) => [100, 0, 0],
            None => [
                4 + self.rare_probability_offset,
                37,
                59 - self.rare_probability_offset,
            ],
        };

        let [mut rare, mut uncommon, mut common] = rarity_probabilities;

        let (mut has_rare, mut has_uncommon, mut has_common) = (false, false, false);
        for card in available {
            match card.rarity {
                Rarity::Rare => has_rare = true,
                Rarity::Uncommon => has_uncommon = true,
                Rarity::Common => has_common = true,
                _ => panic!("Unexpected rarity!"),
            }
        }

        if !has_rare {
            rare = 0;
        }
        if !has_uncommon {
            uncommon = 0;
        }
        if !has_common {
            common = 0;
        }

        let rarity = *probability
            .choose_weighted(&[
                (Rarity::Rare, rare),
                (Rarity::Uncommon, uncommon),
                (Rarity::Common, common),
            ])
            .unwrap();

        let card = probability
            .choose(
                available
                    .iter()
                    .filter(|card| card.rarity == rarity)
                    .collect(),
            )
            .unwrap();

        let is_default_upgraded = match card._type {
            CardType::Attack => self.relics.contains("Molten Egg"),
            CardType::Skill => self.relics.contains("Toxic Egg"),
            CardType::Power => self.relics.contains("Frozen Egg"),
            _ => panic!("Unexpected card type!"),
        };

        let is_upgraded = is_default_upgraded || {
            let chance = match self.act {
                1 => 0,
                2 => 2,
                3 | 4 => 4,
                _ => panic!("Unexpected ascension"),
            } / if self.asc < 12 { 1 } else { 2 };

            *probability
                .choose_weighted(&[(true, chance), (false, 8 - chance)])
                .unwrap()
        };

        CardOffer {
            base: card,
            upgraded: is_upgraded,
        }
    }

    pub fn deck(&self) -> impl Iterator<Item = DeckCard> + '_ {
        self.deck.iter().map(|(u, c)| DeckCard {
            uuid: *u,
            base: c.base,
        })
    }

    pub fn removable_cards(&self) -> impl Iterator<Item = DeckCard> + '_ {
        self.deck.iter().filter_map(|(u, c)| {
            if c.removable() {
                Some(DeckCard {
                    uuid: *u,
                    base: c.base,
                })
            } else {
                None
            }
        })
    }

    pub fn upgradable_cards(&self) -> impl Iterator<Item = DeckCard> + '_ {
        self.deck.iter().filter_map(|(u, c)| {
            if c.upgradable() {
                Some(DeckCard {
                    uuid: *u,
                    base: c.base,
                })
            } else {
                None
            }
        })
    }

    pub fn potions(&self) -> impl Iterator<Item = PotionReference> + '_ {
        self.potion_slots().flatten()
    }

    pub fn potion_slots(&self) -> impl Iterator<Item = Option<PotionReference>> + '_ {
        self.potions
            .iter()
            .enumerate()
            .map(|(position, opt)| opt.as_ref().map(|potion| potion.reference(position)))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct DeckCard {
    pub uuid: Uuid,
    pub base: &'static BaseCard,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Relics {
    pub relics: HashMap<Uuid, Relic>,
    pub seen: HashSet<&'static BaseRelic>,
    pub relic_whens: HashMap<When, Vector<Uuid>>,
    pub relic_names: HashMap<String, Uuid>,
}

impl Relics {
    pub fn new() -> Self {
        Self {
            relics: HashMap::new(),
            relic_whens: HashMap::new(),
            relic_names: HashMap::new(),
            seen: HashSet::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = RelicReference> + '_ {
        self.relics.iter().map(|(u, c)| RelicReference {
            base: c.base,
            relic: *u,
        })
    }

    pub fn find_mut(&mut self, name: &str) -> Option<&mut Relic> {
        if let Some(uuid) = self.relic_names.get(name) {
            Some(self.relics.get_mut(uuid).unwrap())
        } else {
            None
        }
    }

    pub fn find(&self, name: &str) -> Option<&Relic> {
        if let Some(uuid) = self.relic_names.get(name) {
            Some(self.relics.get(uuid).unwrap())
        } else {
            None
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.relic_names.contains_key(name)
    }

    pub fn add(&mut self, base: &'static BaseRelic) -> RelicReference {
        let relic = Relic::new(base);
        self.relic_names
            .insert(relic.base.name.to_string(), relic.uuid);
        let whens = match &relic.base.activation {
            Activation::Immediate | Activation::Custom => {
                vec![]
            }
            Activation::Counter {
                increment, reset, ..
            } => {
                if increment == reset {
                    vec![increment]
                } else {
                    vec![increment, reset]
                }
            }
            Activation::Uses { use_when, .. } => vec![use_when],
            Activation::When(when) => vec![when],
            Activation::WhenEnabled {
                activated_at,
                enabled_at,
                disabled_at,
            } => {
                if activated_at == enabled_at {
                    if activated_at == disabled_at {
                        vec![activated_at]
                    } else {
                        vec![activated_at, disabled_at]
                    }
                } else if enabled_at == disabled_at {
                    vec![activated_at, enabled_at]
                } else {
                    vec![activated_at, enabled_at, disabled_at]
                }
            }
        };

        for when in whens {
            self.relic_whens
                .entry(when.clone())
                .or_insert_with(Vector::new)
                .push_back(relic.uuid)
        }

        let reference = relic.reference();

        self.relics.insert(relic.uuid, relic);

        reference
    }

    pub fn remove(&mut self, name: &str) {
        let uuid = self.relic_names.remove(name).unwrap();
        self.relics.remove(&uuid);
        for (_, uuids) in self.relic_whens.iter_mut() {
            if let Some(index) = uuids.index_of(&uuid) {
                uuids.remove(index);
            }
        }
    }

    pub fn get(&self, relic: RelicReference) -> &Relic {
        self.relics.get(&relic.relic).unwrap()
    }

    pub fn get_mut(&mut self, relic: RelicReference) -> &mut Relic {
        self.relics.get_mut(&relic.relic).unwrap()
    }
}


