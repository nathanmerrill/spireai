use im::{vector, HashMap, Vector};
use uuid::Uuid;

use crate::{models::{self, core::{CardDestination, CardEffect, CardLocation, ChestType, Class, Condition, When, CardType}, potions::BasePotion, relics::{Activation, BaseRelic}}, spireai::references::{
        BindingReference, BuffReference, CardReference, CreatureReference, PotionReference,
        RelicReference,
    }};

use super::{battle::BattleState, core::{Card, CardOffer, Creature, Event, Potion, Relic}, map::MapState, probability::Probability};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct GameState {
    pub class: Class,
    pub map: MapState,
    pub floor_state: FloorState,
    pub last_elite: Option<usize>,
    pub last_normal: Option<usize>,
    pub easy_fight_count: u8,
    pub battle_state: BattleState,
    pub event_state: Option<Event>,
    pub card_choices: CardChoiceState,
    pub act: u8,
    pub asc: u8,
    pub deck: HashMap<Uuid, Card>,
    pub potions: Vector<Option<Potion>>,
    pub relics: HashMap<Uuid, Relic>,
    pub relic_whens: HashMap<When, Vector<Uuid>>,
    pub relic_names: HashMap<String, Uuid>,
    pub player: Creature,
    pub gold: u16,
    pub keys: Option<KeyState>,
    pub won: Option<bool>,
    pub base_purge_cost: u16,
    pub card_rarity_offset: u8,
}

impl GameState {
    pub fn remove_card(&mut self, card: CardReference) {
        match card.location {
            CardLocation::DeckPile => self.deck.remove(&card.uuid),
            _ => {
                self.battle_state.move_out(card);
                self.battle_state.cards.remove(&card.uuid)
            }
        }
        .unwrap();
    }

    pub fn purge_cost(&self) -> u16 {
        if self.has_relic("Smiling Mask") {
            50
        } else {
            let discount = if self.has_relic("Membership Card") {
                if self.has_relic("The Courier") {
                    0.6
                } else {
                    0.5
                }
            } else {
                if self.has_relic("The Courier") {
                    0.8
                } else {
                    1.0
                }
            };

            (self.base_purge_cost as f32 * discount).ceil() as u16
        }
    }

    pub fn add_card(
        &mut self,
        card: Card,
        destination: CardDestination,
        probability: &mut Probability,
    ) {
        match destination {
            CardDestination::DeckPile => {
                self.deck.insert(card.uuid, card);
            }
            _ => {
                self.battle_state.move_in(
                    card.uuid,
                    destination,
                    probability,
                );
                self.battle_state.cards.insert(card.uuid, card);
            }
        }
    }

    pub fn find_relic_mut(&mut self, name: &str) -> Option<&mut Relic> {
        if let Some(uuid) = self.relic_names.get(name) {
            Some(self.relics.get_mut(uuid).unwrap())
        } else {
            None
        }
    }

    pub fn has_relic(&self, name: &str) -> bool {
        self.relic_names.contains_key(name)
    }

    pub fn add_relic(&mut self, base: &'static BaseRelic) -> RelicReference {
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

    pub fn remove_relic(&mut self, name: &str) {
        let uuid = self.relic_names.remove(name).unwrap();
        self.relics.remove(&uuid);
        for (_, uuids) in self.relic_whens.iter_mut() {
            if let Some(index) = uuids.index_of(&uuid) {
                uuids.remove(index);
            }
        }
    }

    pub fn reduce_max_hp(&mut self, reduction: u16) {
        self.player.max_hp -= reduction;
        self.player.hp = std::cmp::min(self.player.hp, self.player.max_hp);
    }

    pub fn add_potion(&mut self, base: &'static BasePotion) {
        if let Some(slot) = self.potions.iter().position(|a| a.is_none()) {
            self.potions.set(slot, Some(Potion{base}));
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

    pub fn card_playable(&self, card: CardReference) -> bool {
        let card = self.get(card);
        card.cost <= self.battle_state.energy
            && match card.base.playable_if {
                Condition::Always => true,
                Condition::Never => false,
                Condition::Custom => {
                    match card.base.name.as_str() {
                        "Clash" => self.battle_state.hand().all(|f| f.base._type == CardType::Attack),
                        "Grand Finale" => self.battle_state.draw().count() == 0,
                        "Impatience" => self.battle_state.hand().all(|f| f.base._type != CardType::Attack),
                        "Signature Move" => self.battle_state.hand().filter(|f| f.base._type == CardType::Attack).count() == 1,
                        _ => panic!("Unexpected custom condition on card: {}", card.base.name),
                    }
                }
                _ => panic!("Unexpected condition!"),
            }
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

        let mut player = Creature::new(hp);
        player.hp = hp;

        let mut state = Self {
            class,
            map: MapState::new(),
            floor_state: FloorState::Event,
            battle_state: BattleState::new(),
            event_state: Some(Event::by_name("Neow")),
            card_choices: CardChoiceState::new(),
            easy_fight_count: 0,
            last_normal: None,
            last_elite: None,
            act: 0,
            asc,
            deck,
            potions,
            relics: HashMap::new(),
            relic_whens: HashMap::new(),
            relic_names: HashMap::new(),
            player,
            gold: 99,
            base_purge_cost: 50,
            keys: None,
            won: None,
            card_rarity_offset: 0,
        };

        state.add_relic(models::relics::by_name(starting_relic));

        state
    }

    pub fn get<A>(&self, _ref: A) -> &A::Item
    where
        A: BindingReference,
    {
        _ref.get(self).unwrap()
    }

    pub fn get_opt<A>(&self, _ref: A) -> Option<&A::Item>
    where
        A: BindingReference,
    {
        _ref.get(self)
    }

    pub fn get_mut<A>(&mut self, _ref: A) -> &mut A::Item
    where
        A: BindingReference,
    {
        _ref.get_mut(self).unwrap()
    }

    pub fn get_mut_ref<A>(&mut self, _ref: A) -> Option<&mut A::Item>
    where
        A: BindingReference,
    {
        _ref.get_mut(self)
    }
    pub fn deck(&self) -> impl Iterator<Item = CardReference> + '_ {
        self.deck.iter().map(|(u, c)| CardReference {
            uuid: *u,
            location: CardLocation::DeckPile,
            base: c.base,
        })
    }

    pub fn removable_cards(&self) -> impl Iterator<Item = CardReference> + '_ {
        self.deck.iter().filter_map(|(u, c)| 
            if c.removable() {
                Some(CardReference {
                    uuid: *u,
                    location: CardLocation::DeckPile,
                    base: c.base,
                })
            } else {
                None
            }
        )
    }

    pub fn upgradable_cards(&self) -> impl Iterator<Item = CardReference> + '_ {
        self.deck.iter().filter_map(|(u, c)| 
            if c.upgradable() {
                Some(CardReference {
                    uuid: *u,
                    location: CardLocation::DeckPile,
                    base: c.base,
                })
            } else {
                None
            }
        )
    }

    pub fn relics(&self) -> impl Iterator<Item = RelicReference> + '_ {
        self.relics.iter().map(|(u, c)| RelicReference {
            base: c.base,
            relic: *u,
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

    pub fn player_buffs(&self) -> impl Iterator<Item = BuffReference> + '_ {
        self.player.buffs.values().map(move |b| BuffReference {
            base: b.base,
            creature: CreatureReference::Player,
            buff: b.uuid,
        })
    }

    pub fn card_choices(&self) -> impl Iterator<Item = CardReference> + '_ {
        self.card_choices
            .choices
            .iter()
            .map(move |u| CardReference {
                uuid: *u,
                location: self.card_choices.location,
                base: match self.card_choices.location {
                    CardLocation::DeckPile => self.deck[u].base,
                    _ => self.battle_state.cards[u].base,
                },
            })
    }

    pub fn in_location(&self, location: CardLocation) -> Vec<CardReference> {
        match location {
            CardLocation::DeckPile => self.deck().collect(),
            CardLocation::DiscardPile => self.battle_state.discard().collect(),
            CardLocation::ExhaustPile => self.battle_state.exhaust().collect(),
            CardLocation::PlayerHand => self.battle_state.hand().collect(),
            CardLocation::DrawPile => self.battle_state.draw().collect(),
            CardLocation::Stasis => panic!("Cannot get cards in stasis"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum FloorState {
    Event,
    EventUpgrade(u8),
    EventTransform(u8, bool), // true if it upgrades cards
    EventRemove(u8),
    Rest,
    Chest(ChestType),
    Battle,
    Map,
    GameOver,
    Rewards(Vector<Reward>),
    CardReward(Vector<CardOffer>), // true if upgraded
    ShopEntrance,
    Shop {
        cards: Vector<(CardOffer, u16)>,
        potions: Vector<(&'static BasePotion, u16)>,
        relics: Vector<(&'static BaseRelic, u16)>,
        can_purge: bool,
    },
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Reward {
    CardChoice(Vector<CardOffer>), // true if upgraded
    Gold(u8),
    Relic(Relic),
    Potion(Potion),
    EmeraldKey,
    SapphireKey(Relic),
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct KeyState {
    pub ruby: bool,
    pub emerald: bool,
    pub sapphire: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CardChoiceState {
    pub choices: Vector<Uuid>,
    pub location: CardLocation,
    pub count: Option<(usize, usize)>,
    pub effect: CardChoiceEffect,
}

impl CardChoiceState {
    pub fn new() -> Self {
        Self {
            choices: Vector::new(),
            location: CardLocation::Stasis,
            count: None,
            effect: CardChoiceEffect::None,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum CardChoiceEffect {
    None,
    Scry,
    Then(&'static Vec<CardEffect>),
    Remove,
    AddToLocation(CardDestination, Vec<CardEffect>),
    Transform,
    Upgrade
}
