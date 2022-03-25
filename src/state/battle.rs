use im::{HashMap, HashSet, Vector};
use itertools::Itertools;
use uuid::Uuid;

use crate::{
    models::core::{
        CardDestination, CardLocation, CardType, Condition, FightType, RelativePosition, Stance,
        Target,
    },
    spireai::{
        evaluator::GameAction,
        references::{Binding, CardReference, CreatureReference, MonsterReference, BuffReference},
    },
};

use super::{
    core::{Card, Monster, Orb, Creature, Buff},
    probability::Probability,
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct BattleState {
    pub deck_references: HashMap<Uuid, Uuid>,
    pub player: Creature,
    pub cards: HashMap<Uuid, Card>,
    pub draw: HashSet<Uuid>,
    pub draw_top_known: Vector<Uuid>,
    pub draw_bottom_known: Vector<Uuid>,
    pub discard: Vector<Uuid>,
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
    pub end_turn: bool,
}

impl BattleState {
    pub fn new(max_hp: u16, current_hp: u16) -> Self {
        let player = Creature::new(max_hp);
        player.hp = current_hp;

        Self {
            deck_references: HashMap::new(),
            draw_top_known: Vector::new(),
            draw_bottom_known: Vector::new(),
            draw: HashSet::new(),
            discard: Vector::new(),
            exhaust: HashSet::new(),
            hand: HashSet::new(),
            cards: HashMap::new(),
            monsters: HashMap::new(),
            orbs: Vector::new(),
            player,
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
            end_turn: false,
        }
    }

    pub fn add_card(
        &mut self,
        mut card: Card,
        destination: CardDestination,
        probability: &mut Probability
    ) -> CardReference {
        if self.player.has_buff("Master Reality") {
            if card.base.name == "Searing Blow" {
                card.upgrades = 2;
            }
            card.upgrades = 1;
        }

        let cost = match card.base.name.as_str() {
            "Blood for Blood" => {
                4_u8.saturating_sub(self.hp_loss_count)
            }
            "Eviscerate" => {
                3_u8.saturating_sub(self.discard_count)
            }
            "Force Field" => {
                4_u8.saturating_sub(self.power_count)
            }
            _ => card.cost
        };

        card.cost = card.cost.min(cost);

        let reference = card.reference(destination.location());
        let uuid = card.uuid;
        self.cards.insert(uuid, card);
        self.move_in(uuid, destination, probability);
        reference
    }

    pub fn move_card(
        &mut self,
        destination: CardDestination,
        mut card: CardReference,
        probability: &mut Probability,
    ) -> CardReference {
        self.move_out(card);
        self.move_in(card.uuid, destination, probability);
        card.location = destination.location();
        card
    }

    pub fn cards_in_location(&self, location: CardLocation) -> Vec<CardReference> {
        match location {
            CardLocation::DiscardPile => self.discard().collect(),
            CardLocation::ExhaustPile => self.exhaust().collect(),
            CardLocation::PlayerHand => self.hand().collect(),
            CardLocation::DrawPile => self.draw().collect(),
            CardLocation::None => vec![],
        }
    }

    pub fn card_playable(&self, card: CardReference) -> bool {
        let card = self.get_card(card);
        card.cost <= self.energy
            && match card.base.playable_if {
                Condition::Always => true,
                Condition::Never => false,
                Condition::Custom => match card.base.name.as_str() {
                    "Clash" => self.hand().all(|f| f.base._type == CardType::Attack),
                    "Grand Finale" => self.draw().count() == 0,
                    "Impatience" => self.hand().all(|f| f.base._type != CardType::Attack),
                    "Signature Move" => {
                        self.hand()
                            .filter(|f| f.base._type == CardType::Attack)
                            .count()
                            == 1
                    }
                    _ => panic!("Unexpected custom condition on card: {}", card.base.name),
                },
                _ => panic!("Unexpected condition!"),
            }
    }

    pub fn move_out(&mut self, card: CardReference) {
        match card.location {
            CardLocation::DiscardPile => self.discard.index_of(&card.uuid).map(|i| self.discard.remove(i)),
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
            CardLocation::None => None,
        };
    }

    pub fn move_in(
        &mut self,
        card: Uuid,
        destination: CardDestination,
        probability: &mut Probability,
    ) {
        match destination {
            CardDestination::DiscardPile => {
                self.discard.push_back(card);
            }
            CardDestination::DrawPile(position) => {
                self.draw.insert(card);
                match position {
                    RelativePosition::All => {
                        panic!("Unexpected RelativePosition::All when inserting into draw pile")
                    }
                    RelativePosition::Bottom => {
                        if self.draw_top_known.len() == self.draw.len() - 1 {
                            self.draw_top_known.push_front(card)
                        } else {
                            self.draw_bottom_known.push_back(card)
                        }
                    }
                    RelativePosition::Top => self.draw_top_known.push_back(card),
                    RelativePosition::Random => {
                        if self.draw_visible {
                            let position = probability.range(self.draw.len());
                            self.draw_top_known.insert(position, card);
                        } else {
                            self.draw_top_known = Vector::new();
                            self.draw_bottom_known = Vector::new();
                        }
                    }
                };
            }
            CardDestination::ExhaustPile => {
                self.exhaust.insert(card);
            }
            CardDestination::PlayerHand => {
                if self.hand.len() == 10 {
                    self.discard.push_back(card);
                } else {
                    self.hand.insert(card);
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
        self.monsters.values().map(|m| m.monster_ref())
    }

    pub fn available_monsters(&self) -> impl Iterator<Item = MonsterReference> + '_ {
        self.monsters
            .values()
            .filter(|m| m.targetable)
            .map(|m| m.monster_ref())
    }

    pub fn available_creatures(&self) -> impl Iterator<Item = CreatureReference> + '_ {
        self.monsters
            .values()
            .filter(|m| m.targetable)
            .map(|m| m.creature_ref())
    }

    pub fn random_monster(&self, probability: &mut Probability) -> Option<MonsterReference> {
        probability.choose(self.available_monsters().collect())
    }

    pub fn get_monster(&self, monster: MonsterReference) -> Option<&Monster> {
        self.monsters.get(&monster.uuid)
    }

    pub fn get_monster_mut(&mut self, monster: MonsterReference) -> Option<&mut Monster> {
        self.monsters.get_mut(&monster.uuid)
    }

    pub fn get_buff(&self, buff: BuffReference) -> Option<&Buff> {
        self.get_creature(buff.creature)
            .and_then(|f| f.buffs.get(&buff.buff))
    }

    pub fn get_buff_mut(&mut self, buff: BuffReference) -> Option<&mut Buff> {
        self.get_creature_mut(buff.creature)
            .and_then(|f| f.buffs.get_mut(&buff.buff))
    }

    pub fn get_creature(&self, creature: CreatureReference) -> Option<&Creature> {
        match creature {
            CreatureReference::Player => Some(&self.player),
            CreatureReference::Creature(monster) => self.get_monster(monster)
                .map(|m| &m.creature),
        }
    }
    pub fn get_creature_mut(&mut self, creature: CreatureReference) -> Option<&mut Creature> {
        match creature {
            CreatureReference::Player => Some(&mut self.player),
            CreatureReference::Creature(monster) => self
                .get_monster_mut(monster)
                .map(|m| &mut m.creature),
        }
    }

    pub fn get_card(&self, card: CardReference) -> &Card {
        debug_assert!(self.location_matches(card));
        self.cards.get(&card.uuid).unwrap()
    }

    pub fn get_card_mut(&mut self, card: CardReference) -> &mut Card {
        debug_assert!(self.location_matches(card));
        self.cards.get_mut(&card.uuid).unwrap()
    }

    fn location_matches(&self, card: CardReference) -> bool {
        match card.location {
            CardLocation::DiscardPile => self.discard.contains(&card.uuid),
            CardLocation::ExhaustPile => self.exhaust.contains(&card.uuid),
            CardLocation::PlayerHand => self.hand.contains(&card.uuid),
            CardLocation::DrawPile => self.draw.contains(&card.uuid),
            CardLocation::None => {
                !self.draw.contains(&card.uuid)
                    && !self.hand.contains(&card.uuid)
                    && !self.discard.contains(&card.uuid)
                    && !self.exhaust.contains(&card.uuid)
            }
        }
    }

    pub fn peek_top(&mut self, n: usize, probability: &mut Probability) {
        if self.draw.is_empty() {
            return;
        }

        let remaining_picks = n - self.draw_top_known.len();

        let choices = self
            .draw
            .clone()
            .difference(self.draw_top_known.iter().copied().collect())
            .difference(self.draw_bottom_known.iter().copied().collect())
            .iter()
            .cloned()
            .collect_vec();
        let max_picks = choices.len().min(remaining_picks);

        let choices = probability.choose_multiple(choices, max_picks);

        self.draw_top_known.extend(choices);

        if max_picks < remaining_picks {
            let bottom_peek = (remaining_picks - max_picks).min(self.draw_bottom_known.len());
            let mut top = self.draw_bottom_known.split_off(bottom_peek);
            std::mem::swap(&mut top, &mut self.draw_bottom_known);
            self.draw_top_known.extend(top);
        }
    }

    pub fn peek_bottom(&mut self, n: u8, probability: &mut Probability) {
        if self.draw.is_empty() {
            return;
        }

        let remaining_picks = n as usize - self.draw_bottom_known.len();

        let choices = self
            .draw
            .clone()
            .difference(self.draw_top_known.iter().copied().collect())
            .difference(self.draw_bottom_known.iter().copied().collect())
            .iter()
            .cloned()
            .collect_vec();
        let max_picks = choices.len().min(remaining_picks);

        let choices = probability.choose_multiple(choices, max_picks);

        self.draw_bottom_known.extend(choices);

        if max_picks < remaining_picks {
            let bottom_peek = (remaining_picks - max_picks).min(self.draw_bottom_known.len());
            let mut bottom = self.draw_bottom_known.split_off(bottom_peek);
            std::mem::swap(&mut bottom, &mut self.draw_bottom_known);
            self.draw_top_known.extend(bottom);
        }
    }
}

impl Target {
    pub fn to_creature(self, binding: Binding, action: Option<GameAction>) -> CreatureReference {
        match self {
            Target::_Self => binding.get_creature(),
            Target::Attacker => {
                let action = action.expect("Expected action!");
                debug_assert!(action.is_attack, "Expected attack action!");
                action.creature
            }
            Target::OneEnemy => {
                let action = action.expect("Expected action!");
                match action.creature {
                    CreatureReference::Player => action.target.expect("Expected target!"),
                    CreatureReference::Creature(_) => CreatureReference::Player,
                }
            }
            Target::Player => CreatureReference::Player,
            _ => panic!("Target does not resolve to a single creature! {:?}", self),
        }
    }

    pub fn to_creatures(
        self,
        binding: Binding,
        action: Option<GameAction>,
        state: &BattleState,
        probability: &mut Probability,
    ) -> Vec<CreatureReference> {
        let creatures = match self {
            Target::AllEnemies => match binding.get_creature() {
                CreatureReference::Player => state.available_creatures().collect(),
                _ => vec![CreatureReference::Player],
            },
            Target::AnyFriendly => state.available_creatures().collect(),
            Target::RandomEnemy => state
                .random_monster(probability)
                .map(|a| a.creature_ref())
                .into_iter()
                .collect(),
            Target::RandomFriendly => match binding.get_creature() {
                CreatureReference::Player => vec![CreatureReference::Player],
                CreatureReference::Creature(uuid) => {
                    let monsters = state
                        .available_monsters()
                        .filter(|a| *a != uuid)
                        .collect_vec();

                    probability
                        .choose(monsters)
                        .into_iter()
                        .map(|a| a.creature_ref())
                        .collect()
                }
            },
            _ => vec![self.to_creature(binding, action)],
        };

        creatures
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Player {
    pub creature: Creature,
}

impl Player {
    pub fn new(max_hp: u16, current_hp: u16) -> Player {
        let creature = Creature::new(max_hp);
        creature.hp = current_hp;
        Player {
            creature,
        }
    }

    

    pub fn buffs(&self) -> impl Iterator<Item = BuffReference> + '_ {
        self.creature.buffs.values().map(move |b| BuffReference {
            base: b.base,
            creature: CreatureReference::Player,
            buff: b.uuid,
        })
    }
}
