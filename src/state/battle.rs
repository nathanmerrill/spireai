use im::{HashMap, HashSet, Vector};
use uuid::Uuid;

use crate::{
    models::core::{CardDestination, CardLocation, CardType, FightType, RelativePosition, Stance},
    spireai::references::{CardReference, CreatureReference, MonsterReference},
};

use super::{
    core::{Card, Monster, Orb},
    probability::Probability,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BattleState {
    pub active: bool,
    pub deck_references: HashMap<Uuid, Uuid>,
    pub cards: HashMap<Uuid, Card>,
    pub draw: HashSet<Uuid>,
    pub draw_top_known: Vector<Uuid>,
    pub draw_bottom_known: Vector<Uuid>,
    pub discard: HashSet<Uuid>,
    pub exhaust: HashSet<Uuid>,
    pub hand: HashSet<Uuid>,
    pub monsters: HashMap<Uuid, Monster>,
    pub orbs: Vector<Orb>,
    pub orb_slots: u8,
    pub energy: u8,
    pub base_energy: u8,
    pub stance: Stance,
    pub fight_type: FightType,
    pub event_battle: bool,
    pub draw_visible: bool,
    pub discard_count: u8,
    pub play_count: u8,
    pub hp_loss_count: u8,
    pub power_count: u8,
    pub last_card_played: Option<CardType>,
}

impl BattleState {
    pub fn new() -> Self {
        Self {
            active: false,
            deck_references: HashMap::new(),
            draw_top_known: Vector::new(),
            draw_bottom_known: Vector::new(),
            draw: HashSet::new(),
            discard: HashSet::new(),
            exhaust: HashSet::new(),
            hand: HashSet::new(),
            cards: HashMap::new(),
            monsters: HashMap::new(),
            orbs: Vector::new(),
            orb_slots: 0,
            base_energy: 0,
            energy: 0,
            event_battle: false,
            draw_visible: false,
            stance: Stance::None,
            fight_type: FightType::Common,
            discard_count: 0,
            play_count: 0,
            hp_loss_count: 0,
            power_count: 0,
            last_card_played: None,
        }
    }

    pub fn move_card(
        &mut self,
        destination: CardDestination,
        mut card: CardReference,
        probability: &mut Probability,
    ) -> CardReference {
        self.move_out(card);
        self.move_in(card, destination, probability);
        card.location = destination.location();
        card
    }

    pub fn move_out(&mut self, card: CardReference) {
        match card.location {
            CardLocation::DeckPile => {
                panic!("Deck cannot be moved between")
            }
            CardLocation::DiscardPile => self.discard.remove(&card.uuid),
            CardLocation::DrawPile => {
                if let Some(index) = self.draw_top_known.iter().position(|a| a == &card.uuid) {
                    self.draw_top_known.remove(index);
                }
                if let Some(index) = self.draw_bottom_known.iter().position(|a| a == &card.uuid) {
                    self.draw_bottom_known.remove(index);
                }
                self.draw.remove(&card.uuid)
            }
            CardLocation::ExhaustPile => self.exhaust.remove(&card.uuid),
            CardLocation::PlayerHand => self.hand.remove(&card.uuid),
            CardLocation::Stasis => {
                return;
            }
        }
        .unwrap();
    }

    pub fn move_in(
        &mut self,
        card: CardReference,
        destination: CardDestination,
        probability: &mut Probability,
    ) {
        match destination {
            CardDestination::DeckPile => {
                panic!("Deck cannot be moved between")
            }
            CardDestination::DiscardPile => {
                self.discard.insert(card.uuid);
            }
            CardDestination::DrawPile(position) => {
                let uuid = card.uuid;
                self.draw.insert(uuid);
                match position {
                    RelativePosition::All => {
                        panic!("Unexpected RelativePosition::All when inserting into draw pile")
                    }
                    RelativePosition::Bottom => {
                        if self.draw_top_known.len() == self.draw.len() - 1 {
                            self.draw_top_known.push_front(uuid)
                        } else {
                            self.draw_bottom_known.push_back(uuid)
                        }
                    }
                    RelativePosition::Top => self.draw_top_known.push_back(uuid),
                    RelativePosition::Random => {
                        if self.draw_visible {
                            let position = probability.range(self.draw.len());
                            self.draw_top_known.insert(position, uuid);
                        } else {
                            self.draw_top_known = Vector::new();
                            self.draw_bottom_known = Vector::new();
                        }
                    }
                };
            }
            CardDestination::ExhaustPile => {
                self.exhaust.insert(card.uuid);
            }
            CardDestination::PlayerHand => {
                if self.hand.len() == 10 {
                    self.discard.insert(card.uuid);
                } else {
                    self.hand.insert(card.uuid);
                }
            }
        }
    }

    pub fn discard(&self) -> impl Iterator<Item = CardReference> + '_ {
        self.discard.iter().map(move |u| CardReference {
            uuid: *u,
            location: CardLocation::DiscardPile,
            base: self.cards[u].base,
        })
    }

    pub fn exhaust(&self) -> impl Iterator<Item = CardReference> + '_ {
        self.exhaust.iter().map(move |u| CardReference {
            uuid: *u,
            location: CardLocation::ExhaustPile,
            base: self.cards[u].base,
        })
    }

    pub fn hand(&self) -> impl Iterator<Item = CardReference> + '_ {
        self.hand.iter().map(move |u| CardReference {
            uuid: *u,
            location: CardLocation::PlayerHand,
            base: self.cards[u].base,
        })
    }

    pub fn draw(&self) -> impl Iterator<Item = CardReference> + '_ {
        self.draw.iter().map(move |u| CardReference {
            uuid: *u,
            location: CardLocation::DrawPile,
            base: self.cards[u].base,
        })
    }

    pub fn all_monsters(&self) -> impl Iterator<Item = MonsterReference> + '_ {
        self.monsters.values().map(|m| MonsterReference {
            base: m.base,
            uuid: m.uuid,
        })
    }

    pub fn available_monsters(&self) -> impl Iterator<Item = MonsterReference> + '_ {
        self.monsters
            .values()
            .filter(|m| m.targetable)
            .map(|m| MonsterReference {
                base: m.base,
                uuid: m.uuid,
            })
    }

    pub fn available_creatures(&self) -> impl Iterator<Item = CreatureReference> + '_ {
        self.monsters
            .values()
            .filter(|m| m.targetable)
            .map(|m| CreatureReference::Creature(m.uuid))
    }

    pub fn random_monster(&self, probability: &mut Probability) -> Option<MonsterReference> {
        probability.choose(self.available_monsters().collect())
    }
}
