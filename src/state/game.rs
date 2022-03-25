use std::ops::Range;

use im::{vector, HashMap, Vector};
use uuid::Uuid;

use crate::{
    models::{
        self,
        cards::BaseCard,
        core::{CardEffect, CardType, ChestType, Class, DeckOperation, When, FightType},
        potions::BasePotion,
        relics::{Activation, BaseRelic},
    },
    spireai::references::{
        BuffReference, CardReference, CreatureReference, PotionReference, RelicReference,
    },
};

use super::{
    battle::BattleState,
    core::{Buff, Card, CardOffer, Creature, Event, Potion, Relic},
    map::MapState,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub map: MapState,
    pub floor_state: FloorState,
    pub screen_state: ScreenState,
    pub relics: Relics,
    pub act: u8,
    pub asc: u8,
    pub deck: HashMap<Uuid, Card>,
    pub potions: Vector<Option<Potion>>,
    pub gold: u16,
    pub keys: Option<KeyState>,
    pub won: Option<bool>,
    pub purge_count: u8,
    pub rare_probability_offset: u8,
    pub max_hp: u16,
    pub hp: u16,
}

impl GameState {
    pub fn add_max_hp(&mut self, amount: u16) {
        if let FloorState::Battle(battle) = self.floor_state {
            battle.player.max_hp += amount;
        } else {
            self.max_hp += amount;
        }

        self.heal(amount as f64);
    }

    pub fn reduce_max_hp(&mut self, reduction: u16) {
        if let FloorState::Battle(battle) = self.floor_state {
            battle.player.max_hp -= reduction;
            battle.player.hp = battle.player.hp.min(battle.player.max_hp);
        } else {
            self.max_hp += reduction;
            self.hp = self.hp.min(self.max_hp);
        }
    }

    pub fn heal(&mut self, mut amount: f64) {
        if self.relics.contains("Mark Of The Bloom") {
            return;
        }
        
        if let FloorState::Battle(battle) = self.floor_state {
            if self.relics.contains("Magic Flower") {
                amount *= 1.5;
            }
            battle.player.hp = battle.player.max_hp.min((amount - 0.0001).ceil() as u16 + battle.player.hp)
        } else {
            self.hp = self.max_hp.min((amount - 0.0001).ceil() as u16 + self.hp)
        }        
    }

    pub fn max_hp(&self) -> u16 {
        if let FloorState::Battle(battle) = self.floor_state {
            battle.player.max_hp
        } else {
            self.max_hp
        }
    }

    pub fn remove_card(&mut self, card: Uuid) {
        self.deck.remove(&card);
    }

    pub fn purge_cost(&self) -> u16 {
        if self.relics.contains("Smiling Mask") {
            50
        } else {
            let discount = if self.relics.contains("Membership Card") {
                if self.relics.contains("The Courier") {
                    0.6
                } else {
                    0.5
                }
            } else if self.relics.contains("The Courier") {
                0.8
            } else {
                1.0
            };

            ((self.purge_count * 25 + 75) as f32 * discount).ceil() as u16
        }
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

    pub fn spend_gold(&mut self, amount: u16) {
        self.gold -= amount;

        if let FloorState::Shop(_) = self.floor_state {
            if let Some(relic) = self.relics.find_mut("Maw Bank") {
                relic.enabled = false;
            }
        }
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
            floor_state: FloorState::Event(Event::by_name("Neow")),
            screen_state: ScreenState::Normal,
            relics: Relics::new(),
            act: 1,
            asc,
            deck,
            potions,
            max_hp: hp,
            hp,
            gold: 99,
            keys: None,
            won: None,
            purge_count: 0,
            rare_probability_offset: 0,
        };

        state.relics.add(models::relics::by_name(starting_relic));

        state
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
    pub relic_whens: HashMap<When, Vector<Uuid>>,
    pub relic_names: HashMap<String, Uuid>,
}

impl Relics {
    pub fn new() -> Self {
        Self {
            relics: HashMap::new(),
            relic_whens: HashMap::new(),
            relic_names: HashMap::new(),
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


#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum FloorState {
    Event(Event),
    Rest,
    Chest(ChestType),
    Battle(BattleState),
    GameOver,
    Rewards(Vector<Reward>),
    Shop(ShopState),
}

impl FloorState {
    pub fn battle(&self) -> &BattleState {
        match &self {
            FloorState::Battle(a) => a,
            _ => panic!("Not in a battle!"),
        }
    }

    pub fn battle_mut(&mut self) -> &mut BattleState {
        match self {
            FloorState::Battle(a) => a,
            _ => panic!("Not in a battle!"),
        }
    }

    pub fn event(&self) -> &Event {
        match &self {
            FloorState::Event(a) => a,
            _ => panic!("Not in an event!"),
        }
    }

    pub fn event_mut(&mut self) -> &mut Event {
        match self {
            FloorState::Event(a) => a,
            _ => panic!("Not in an event!"),
        }
    }

    pub fn shop(&self) -> &ShopState {
        match &self {
            FloorState::Shop(a) => a,
            _ => panic!("Not in a shop!"),
        }
    }

    pub fn shop_mut(&mut self) -> &mut ShopState {
        match self {
            FloorState::Shop(a) => a,
            _ => panic!("Not in a shop!"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ShopState {
    pub generated: bool,
    pub cards: Vector<(CardOffer, u16)>,
    pub potions: Vector<(&'static BasePotion, u16)>,
    pub relics: Vector<(&'static BaseRelic, u16)>,
    pub can_purge: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ScreenState {
    Normal,
    InShop,
    CardReward(Vector<CardOffer>, Option<usize>), // usize is the index of the reward in the rewards
    CardChoose(CardChoiceState),
    DeckChoose(u8, DeckOperation),
    Proceed,
    Map,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Reward {
    CardChoice(Vector<CardOffer>, FightType, bool), // True if colorless
    Gold(u16),
    Relic(&'static BaseRelic),
    Potion(Potion),
    EmeraldKey,
    SapphireKey,
    SapphireLinkedRelic(&'static BaseRelic),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy,Debug)]
pub struct KeyState {
    pub ruby: bool,
    pub emerald: bool,
    pub sapphire: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CardChoiceState {
    pub choices: Vector<CardReference>,
    pub count_range: Range<usize>,
    pub then: Vector<CardEffect>,
    pub scry: bool,
}
