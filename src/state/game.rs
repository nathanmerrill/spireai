use im::{vector, HashMap, HashSet, Vector};
use itertools::Itertools;
use uuid::Uuid;

use crate::{
    models::{
        self,
        cards::BaseCard,
        core::{Amount, CardType, ChestType, Class, Condition, FightType, Rarity, When, Effect, DeckOperation},
        potions::BasePotion,
        relics::{Activation, BaseRelic, self},
    },
    spireai::references::{PotionReference, RelicReference},
};

use super::{
    core::{Card, CardOffer, HpRange, Relic},
    floor::KeyState,
    map::MapState,
    probability::Probability,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub relics: Relics,
    pub act: u8,
    pub asc: u8,
    pub deck: HashMap<Uuid, Card>,
    pub potions: Vector<Option<&'static BasePotion>>,
    pub gold: u16,
    pub hp: HpRange,
    pub map: MapState,
    pub keys: Option<KeyState>,
    pub won: Option<bool>,
    pub purge_count: u8,
    pub rare_probability_offset: u8,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            class: Class::All,
            relics: Relics::new(),
            act: 0,
            asc: 0,
            deck: Default::default(),
            potions: Default::default(),
            gold: Default::default(),
            hp: HpRange::new(0),
            map: MapState::new(),
            keys: Default::default(),
            won: Default::default(),
            purge_count: Default::default(),
            rare_probability_offset: Default::default(),
        }
    }
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
        let should_upgrade = match card.base._type {
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

    pub fn add_potion(&mut self, potion: &'static BasePotion) -> bool {
        if let Some(slot) = self.potions.iter().position(|a| a.is_none()) {
            self.potions.set(slot, Some(potion));
            true
        } else {
            false
        }
    }

    pub fn find_potion(&mut self, name: &str) -> Option<PotionReference> {
        self.potions().find(|p| p.base.name == name)
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
            Class::Silent => "Ring Of The Snake",
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
            hp: HpRange {
                amount: hp,
                max: max_hp,
            },
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
        probability: &mut Probability,
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
                        "Bottled Tornado" => {
                            self.deck.values().any(|c| c.base._type == CardType::Power)
                        }
                        "Girya" => {
                            !self.relics.contains("Peace Pipe") || !self.relics.contains("Shovel")
                        }
                        "Shovel" => {
                            !self.relics.contains("Peace Pipe") || !self.relics.contains("Girya")
                        }
                        "Peace Pipe" => {
                            !self.relics.contains("Girya") || !self.relics.contains("Shovel")
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
        fight_type: Option<FightType>,
        colorless: bool,
        probability: &mut Probability,
    ) -> Vector<CardOffer> {
        let cards = {
            if colorless {
                models::cards::available_cards_by_class(Class::None)
            } else if fight_type.is_some() && self.relics.contains("Prismatic Shard") {
                models::cards::available_cards_by_class(Class::All)
            } else {
                models::cards::available_cards_by_class(self.class)
            }
        };

        let count = if self.relics.contains("Busted Crown") {
            1
        } else {
            2
        } + if self.relics.contains("Question Card") {
            1
        } else {
            0
        };

        self.generate_card_offers(fight_type, cards, count, true, probability)
    }

    fn generate_card_offers(
        &mut self,
        fight_type: Option<FightType>,
        available: &[&'static BaseCard],
        count: usize,
        reset_rarity: bool,
        probability: &mut Probability,
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
        probability: &mut Probability,
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

    pub fn eval_condition(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Asc(i) => self.asc >= *i,
            Condition::Act(i) => self.act >= *i,
            Condition::Not(c) => !self.eval_condition(c),
            Condition::MultipleAnd(conditions) => conditions.iter().all(|c| self.eval_condition(c)),
            Condition::MultipleOr(conditions) => conditions.iter().any(|c| self.eval_condition(c)),
            Condition::HasRelic(relic) => self.relics.contains(relic.as_str()),
            Condition::HasGold(amount) => {
                if let Amount::Fixed(i) = amount {
                    self.gold >= *i as u16
                } else {
                    panic!("Cannot handle non-fixed amount in static condition")
                }
            }
            Condition::Always => true,
            Condition::Class(c) => self.class == *c,
            Condition::HasUpgradableCard => self.upgradable_cards().any(|_| true),
            Condition::OnFloor(i) => self.map.floor >= *i,
            Condition::Never => false,
            Condition::Custom => unimplemented!(),
            _ => panic!("Cannot handle game state condition: {:?}", condition),
        }
    }

    pub fn eval_effect(&mut self, effect: &Effect, probability: &mut Probability) 
    {
        match effect {
            Effect::AddPotionSlot(amount) => {
                for _ in 0..*amount {
                    self.potions.push_back(None)
                }
            }
            Effect::AddRelic(name) => {
                self.relics.add(relics::by_name(name));
            } 
            /*
            Effect::ShowChoices(choices) => {
                let event = self.game_state.floor_state.event_mut();
                event.available_choices = choices.clone();
            }
            Effect::ShowReward(rewards) => {
                self.game_state.floor_state = FloorState::Rewards(
                    rewards
                        .iter()
                        .map(|reward| match reward {
                            RewardType::ColorlessCard => Reward::CardChoice(vector![], FightType::Common, true),
                            RewardType::EliteCard => Reward::CardChoice(vector![], FightType::Common, false),
                            RewardType::Gold { min, max } => {
                                let amount =
                                    probability.range((max - min) as usize) as u16 + min;
                                Reward::Gold(amount)
                            }
                            RewardType::RandomBook => {
                                let book = self
                                    .probability
                                    .choose(vec!["Necronomicon", "Enchiridion", "Nilry's Codex"])
                                    .unwrap();
                                Reward::Relic(models::relics::by_name(book))
                            }
                            RewardType::RandomPotion => {
                                let base = self.random_potion(false);
                                Reward::Potion(Potion { base })
                            }
                            RewardType::RandomRelic => {
                                let base = self.random_relic(None, None, None, false);
                                Reward::Relic(base)
                            }
                            RewardType::Relic(rarity) => {
                                let base = self.random_relic(None, Some(*rarity), None, false);
                                Reward::Relic(base)
                            }
                            RewardType::RelicName(name) => Reward::Relic(models::relics::by_name(name)),
                            RewardType::StandardCard => Reward::CardChoice(vector![], FightType::Common, false),
                        })
                        .collect(),
                )
            } */
            
            Effect::RemoveRelic(relic) => {
                self.relics.remove(relic);
            }
            Effect::RandomPotion => {
                let potion = random_potion(false, probability);
                self.add_potion(potion);
            }
            Effect::RandomRelic => {
                let relic = self.random_relic(None, None, false, probability);
                self.add_relic(relic, probability);
            }
            Effect::ReduceMaxHpPercentage(amount) => {
                let percentage = self.eval_amount(amount);
                let total = (self.hp.max as f64 * (percentage as f64 / 100.0))
                    .floor() as u16;
                self.hp.reduce_max_hp(total)
            }
            Effect::LoseHpPercentage(amount) => {
                let percentage = self.eval_amount(amount) as f64 / 1000.0;
                let damage = (self.hp.max as f64 * percentage).floor() as u16;
                self.hp.amount -= damage
            }
            Effect::DeckAdd(name) =>  {
                self.add_card(Card::by_name(name));
            },
            Effect::DeckOperation {
                random,
                count,
                operation,
            } => {
                if *random {
                    assert!(*operation == DeckOperation::Upgrade);
                    let choices = self.upgradable_cards().collect_vec();
                    let selected = probability.choose_multiple(choices, *count as usize);
                    for card in selected {
                        self.deck.get_mut(&card.uuid).unwrap().upgrade();
                    }
                } else {
                    panic!("Deck operation must occur during an event!")
                }
            },
            Effect::AddMaxHp(amount) => {
                let amount = self.eval_amount(amount) as u16;
                self.hp.max += amount;
                self.hp.amount += amount;
            }
            _ => {
                panic!("Cannot handle effect in Game: {:?}", effect)
            }
        }
    }

    
    pub fn eval_amount(&self, amount: &Amount) -> i16 {
        match amount {
            Amount::ByAsc { amount, high, .. } => {
                if self.asc >= 15 {
                    *high
                } else {
                    *amount
                }
            }
            Amount::Custom => unimplemented!(),
            Amount::MaxHp => self.hp.max as i16,
            Amount::Fixed(amount) => *amount,
            Amount::Mult(amount_mult) => {
                let mut product = 1;
                for amount in amount_mult {
                    product *= self.eval_amount(amount);
                }
                product
            }
            Amount::Sum(amount_sum) => {
                let mut sum = 0;
                for amount in amount_sum {
                    sum += self.eval_amount(amount);
                }
                sum
            }
            _ => panic!("Unexpected amount in game.eval_amount: {:?}", amount)
        }
    }

    pub fn drink_potion(
        &mut self,
        potion: PotionReference,
        eval_effects: bool,
        probability: &mut Probability,
    ) {
        if eval_effects {
            match potion.base.name.as_str() {
                "Fruit Juice" => {
                    let amount = if self.relics.contains("Sacred Bark") {
                        10
                    } else {
                        5
                    };

                    self.add_max_hp(amount)
                }
                "Blood Potion" => {
                    let amount = if self.relics.contains("Sacred Bark") {
                        0.40
                    } else {
                        0.20
                    };
                    self.heal(self.hp.max as f64 * amount)
                }
                "Entropic Brew" => {
                    let amount = self.potion_slots().filter(|a| a.is_none()).count();
                    (0..amount).for_each(|_| {
                        self.add_potion(random_potion(true, probability));
                    })
                }
                _ => panic!("Unexpected potion!"),
            }
        }
    }

    pub fn add_relic(&mut self, base: &'static BaseRelic, probability: &mut Probability) {
        self.relics.add(base);

        if base.activation == Activation::Immediate {
            match base.name.as_str() {
                "Potion Belt" => self.potions.append(vector![None, None]),
                "Strawberry" => self.add_max_hp(7),
                "Pear" => self.add_max_hp(10),
                "Mango" => self.add_max_hp(14),
                "Old Coin" => self.add_gold(300),
                "Lees Waffle" => {
                    self.hp.max += 7;
                    self.heal(self.hp.max as f64)
                }
                "War Paint" | "Whetstone" => {
                    let card_type = if base.name == "War Paint" {
                        CardType::Skill
                    } else {
                        CardType::Attack
                    };
                    let available_cards: Vec<DeckCard> = self
                        .upgradable_cards()
                        .filter(|card| card_type.matches(card.base._type))
                        .collect();

                    let cards = probability.choose_multiple(available_cards, 2);

                    for card in cards {
                        self.deck[&card.uuid].upgrade();
                    }
                }
                "Bottled Flame" | "Bottled Lightning" | "Bottled Tornado" | "Astrolabe"
                | "Calling Bell" | "Empty Cage" | "Pandoras Box" | "Tiny House" => {
                    unimplemented!("Add to chest activation")
                }
                "Cauldron" | "Dollys Mirror" | "Orrery" => {}
                _ => {
                    panic!("Unexpected immediate activation: {}", base.name)
                }
            }
        }
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

    fn add(&mut self, base: &'static BaseRelic) -> RelicReference {
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

pub fn random_potion(no_healing: bool, probability: &mut Probability) -> &'static BasePotion {
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
