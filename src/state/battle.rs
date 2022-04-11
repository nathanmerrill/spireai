use std::ops::Range;

use im::{HashMap, HashSet, Vector, vector};
use itertools::Itertools;
use uuid::Uuid;

use crate::{
    models::{core::{
        CardDestination, CardLocation, CardType, Condition, FightType, RelativePosition, Stance,
        Target, Class, Amount, BattleEffect, CardEffect, OrbType, When, Rarity, WhenEffect,
    }, events::BaseEvent, monsters::{Move, Intent, BaseMonster, MonsterMove}, self, cards::BaseCard, relics::{Activation, BaseRelic}},
    spireai::{
        references::{CardReference, CreatureReference, MonsterReference, BuffReference, Binding, GameAction, PotionReference},
    },
};

use super::{
    core::{Card, Monster, Orb, Creature, Buff, Vars, HpRange, CardOffer},
    probability::Probability, game::{GameState, DeckCard},
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
    pub event_battle: Option<(&'static BaseEvent, Vec<String>, Vars)>,
    pub draw_visible: bool,
    pub discard_count: u8,
    pub play_count: u8,
    pub hp_loss_count: u8,
    pub power_count: u8,
    pub last_card_played: Option<CardType>,
    pub end_turn: bool,
    pub game_state: GameState,
    pub card_choose: Option<CardChoiceState>,
}

impl BattleState {
    pub fn new(state: GameState, monster_names: &[String], fight_type: FightType, probability: &mut Probability) -> Self {
        let cards: HashMap<Uuid, Card> = state
            .deck
            .values()
            .map(|c| (c.uuid, c.duplicate()))
            .collect();
        let draw_top = if state.relics.contains("Frozen Eye") {
            cards.values().map(|c| c.uuid).collect()
        } else {
            Vector::new()
        };

        let orb_slots = if state.class == Class::Defect {
            3
        } else if state.relics.contains("Prismatic Shard") {
            1
        } else {
            0
        };

        let mut energy = 3;

        for relic in state.relics.relics.values() {
            if relic.base.energy_relic {
                energy += 1;
            }
        }

        let mut monsters: HashMap<Uuid, Monster> = monster_names
            .iter()
            .map(|n| Monster::new(n, state.asc, probability))
            .enumerate()
            .map(|(index, mut monster)| {
                monster.position = index;
                (monster.uuid, monster)
            })
            .collect();

        if let FightType::Elite { burning } = fight_type {
            let burning_type = if burning {
                probability.range(4)
            } else {
                4
            };
            let has_preserved_insect = state.relics.contains("Preserved Insect");
            if burning || has_preserved_insect {
                monsters.iter_mut().for_each(|(_, monster)| {
                    match burning_type {
                        0 => monster
                            .creature
                            .add_buff("Strength", (state.act + 1) as i16),
                        1 => {
                            let new_hp = monster.creature.hp.max + monster.creature.hp.max / 4;
                            monster.creature.hp = HpRange::new(new_hp);
                        }
                        2 => monster
                            .creature
                            .add_buff("Metallicize", (state.act * 2 + 2) as i16),
                        3 => monster
                            .creature
                            .add_buff("Regenerate", (state.act * 2 + 1) as i16),
                        4 => {}
                        _ => panic!("Unexpected burning type!"),
                    }
                    if has_preserved_insect {
                        monster.creature.hp.amount = monster.creature.hp.max * 3 / 4;
                    }
                });
            }
        }

        let mut battle_state = BattleState {
            fight_type,
            draw_top_known: draw_top,
            draw: cards.values().map(|card| card.uuid).collect(),
            deck_references: cards
                .iter()
                .map(|(uuid, card)| (card.uuid, *uuid))
                .collect(),
            cards,
            orb_slots,
            monsters,
            base_energy: energy,
            draw_bottom_known: Vector::new(),
            discard: Vector::new(),
            exhaust: HashSet::new(),
            hand: HashSet::new(),
            orbs: Vector::new(),
            player: Creature::player(state.hp),
            energy: 0,
            event_battle: None,
            draw_visible: false,
            stance: Stance::None,
            discard_count: 0,
            play_count: 0,
            hp_loss_count: 0,
            power_count: 0,
            last_card_played: None,
            end_turn: false,
            game_state: state,
            card_choose: None
        };

        for monster_ref in battle_state.available_monsters().collect_vec() {
            let creature_ref = monster_ref.creature_ref();
            if let Some(x) = monster_ref
                .base
                .x_range
                .as_ref()
                .map(|a| battle_state.eval_range(a, creature_ref, probability))
            {
                battle_state.get_monster_mut(monster_ref).unwrap().vars.x = x
            }
            if let Some(n) = monster_ref
                .base
                .n_range
                .as_ref()
                .map(|a| battle_state.eval_range(a, creature_ref, probability))
            {
                let monster = battle_state.get_monster_mut(monster_ref).unwrap();
                monster.vars.n = n;
                monster.vars.n_reset = n;
            }

            battle_state.set_monster_move(0, 0, monster_ref, probability);
        }

        battle_state.shuffle(probability);
        battle_state.eval_when(When::CombatStart, probability);
        battle_state.start_turn(true, probability);

        battle_state
    }
    
    fn shuffle(&mut self, probability: &mut Probability) {
        self.draw_top_known = Vector::new();
        self.draw_bottom_known = Vector::new();
        if self.game_state.relics.contains("Frozen Eye") {
            self.peek_top(self.draw.len(), &mut probability)
        }
    }
    
    fn set_monster_move(
        &mut self,
        mut move_index: usize,
        mut phase_index: usize,
        monster_ref: MonsterReference,
        probability: &mut Probability,
    ) {
        let (base, last_move, last_move_count) = {
            let monster = self
                .get_monster(monster_ref)
                .unwrap();
            (monster.base, monster.last_move, monster.last_move_count)
        };

        let binding = Binding::Creature(CreatureReference::Creature(monster_ref));

        let next_move = loop {
            let mut phase = base.phases.get(phase_index).unwrap();
            if move_index == phase.moves.len() {
                if !phase.next.is_empty() {
                    let position = base
                        .phases
                        .iter()
                        .find_position(|a| a.name == phase.next)
                        .unwrap();
                    phase_index = position.0;
                    phase = position.1;
                }
                move_index = 0;
            }

            let next = match &phase.moves[move_index] {
                Move::Fixed(name) => vec![(name, 1)],
                Move::Probability(probabilities) => {
                    let available_probabilites = probabilities
                        .iter()
                        .filter(|p| {
                            let max_repeats = self.eval_amount(&p.max_repeats, binding) as u8;
                            let same_move = last_move.map_or(false, |a| a.name == p.name);
                            let maxxed_out = same_move && last_move_count >= max_repeats;
                            !maxxed_out
                        })
                        .map(|a| {
                            let weight = self.eval_amount(&a.weight, binding) as u8;
                            (&a.name, weight)
                        })
                        .collect_vec();

                    if self.game_state.relics.contains("Runic Dome") {
                        available_probabilites
                    } else {
                        probability
                            .choose_weighted(&available_probabilites)
                            .into_iter()
                            .map(|f| (*f, 1))
                            .collect_vec()
                    }
                }
                Move::If {
                    condition,
                    then_phase,
                    else_phase,
                } => {
                    let next_phase = if self.eval_condition(condition, binding, None) {
                        then_phase
                    } else {
                        else_phase
                    };

                    if !next_phase.is_empty() {
                        let position = base
                            .phases
                            .iter()
                            .find_position(|a| &a.name == next_phase)
                            .unwrap();
                        phase_index = position.0;
                        move_index = 0;
                    }
                    vec![]
                }
            };

            if !next.is_empty() {
                break next;
            } else {
                move_index += 1;
            }
        };

        let monster = self
            .get_monster_mut(monster_ref)
            .unwrap();

        monster.current_move_options = next_move
            .into_iter()
            .map(|(m, p)| {
                (
                    monster.base.moveset.iter().find(|a| &a.name == m).unwrap(),
                    p,
                )
            })
            .collect();

        monster.index = move_index;
        monster.phase = phase_index;
    }

    fn eval_range(
        &mut self,
        range: &crate::models::monsters::Range,
        creature: CreatureReference,
        probability: &mut Probability,
    ) -> i16 {
        let binding = Binding::Creature(creature);
        let min = self.eval_amount(&range.min, binding);
        let max = self.eval_amount(&range.max, binding);
        probability.range((max - min + 1) as usize) as i16 + min
    }
    
    fn eval_amount(&self, amount: &Amount, binding: Binding) -> i16 {
        match amount {
            Amount::ByAsc { amount, low, high } => {
                let fight_type = binding
                    .monster_ref()
                    .map_or_else(
                        || self.fight_type,
                        |a| a.base.fight_type,
                    );
                match fight_type {
                    FightType::Common => {
                        if self.game_state.asc >= 17 {
                            *high
                        } else if self.game_state.asc >= 2 {
                            *low
                        } else {
                            *amount
                        }
                    }
                    FightType::Elite { .. } => {
                        if self.game_state.asc >= 18 {
                            *high
                        } else if self.game_state.asc >= 3 {
                            *low
                        } else {
                            *amount
                        }
                    }
                    FightType::Boss => {
                        if self.game_state.asc >= 19 {
                            *high
                        } else if self.game_state.asc >= 4 {
                            *low
                        } else {
                            *amount
                        }
                    }
                }
            }
            Amount::Custom => unimplemented!(),
            Amount::EnemyCount => self.monsters.len() as i16,
            Amount::N => self.get_vars(binding).n as i16,
            Amount::NegX => -self.get_vars(binding).x as i16,
            Amount::OrbCount => self.orbs.len() as i16,
            Amount::MaxHp => {
                self.get_creature(binding.creature_ref())
                    .unwrap()
                    .hp.max as i16
            }
            Amount::X => self.get_vars(binding).x as i16,
            Amount::PlayerBlock => self.player.block as i16,
            Amount::Fixed(amount) => *amount,
            Amount::Mult(amount_mult) => {
                let mut product = 1;
                for amount in amount_mult {
                    product *= self.eval_amount(amount, binding);
                }
                product
            }
            Amount::Sum(amount_sum) => {
                let mut sum = 0;
                for amount in amount_sum {
                    sum += self.eval_amount(amount, binding);
                }
                sum
            }
            Amount::Upgradable { amount, upgraded } => match self.is_upgraded(binding) {
                true => *upgraded,
                false => *amount,
            },
        }
    }

    pub fn eval_condition(
        &self,
        condition: &Condition,
        binding: Binding,
        action: Option<GameAction>,
    ) -> bool {
        match condition {
            Condition::Act(act) => &self.game_state.act == act,
            Condition::Always => true,
            Condition::Asc(asc) => &self.game_state.asc >= asc,
            Condition::Attacking { target } => match target.creature_ref(binding, action) {
                CreatureReference::Creature(monster) => self
                    .get_monster(monster)
                    .map(|m| {
                        matches!(
                            m.intent,
                            Intent::Attack
                                | Intent::AttackBuff
                                | Intent::AttackDebuff
                                | Intent::AttackDefend
                        )
                    })
                    .unwrap_or(false),
                _ => panic!("Unexpected target that is not a monster in Condition::Attacking"),
            },
            Condition::Buff { target, buff } => {
                let creature = target.creature_ref(binding, action);
                self
                    .get_creature(creature)
                    .map(|c| c.buff_names.contains_key(buff))
                    .unwrap_or(false)
            }
            Condition::BuffX {
                target,
                buff,
                amount: x,
            } => {
                let val = self.eval_amount(x, binding);
                let creature = self.get_creature(target.creature_ref(binding, action));

                if let Some(b) = creature.and_then(|f| f.find_buff(buff)) {
                    b.vars.x >= val
                } else {
                    false
                }
            }
            Condition::Class(class) => self.game_state.class == *class,
            Condition::Custom => unimplemented!(),
            Condition::Equals(amount1, amount2) => {
                self.eval_amount(amount1, binding) == self.eval_amount(amount2, binding)
            }
            Condition::FriendlyDead(name) => self
                .monsters
                .values()
                .any(|m| m.base.name == *name),
            Condition::HalfHp => self
                .get_creature(match binding {
                    Binding::Creature(creature) => creature,
                    _ => CreatureReference::Player,
                })
                .map(|creature| creature.hp.amount * 2 <= creature.hp.max)
                .unwrap_or(false),
            Condition::HasCard { location, card } => match location {
                CardLocation::DiscardPile => self
                    .discard()
                    .any(|c| c.base._type == *card),
                CardLocation::PlayerHand => self
                    .hand()
                    .any(|c| c.base._type == *card),
                CardLocation::ExhaustPile => self
                    .exhaust()
                    .any(|c| c.base._type == *card),
                CardLocation::DrawPile => self
                    .draw()
                    .any(|c| c.base._type == *card),
                CardLocation::None => false,
            },
            Condition::HasDiscarded => self.discard_count > 0,
            Condition::HasFriendlies(count) => {
                let creature = self.get_monster_binding(binding)
                    .expect("Monster did not resolve");
                let friendly_count = self
                    .monsters
                    .values()
                    .filter(|a| a.targetable && a.creature != creature.creature)
                    .count();

                if *count == 0 {
                    friendly_count == 0
                } else {
                    friendly_count >= *count
                }
            }
            Condition::HasGold(amount) => {
                self.game_state.gold >= self.eval_amount(amount, binding) as u16
            }
            Condition::HasOrbSlot => self.orb_slots > 0,
            Condition::HasRelic(relic) => self.game_state.relics.contains(relic),
            
            /*Condition::HasRemoveableCards { count, card_type } => {
                self.removable_cards()
                    .filter(|card| card.base._type.matches(*card_type))
                    .count()
                    > *count as usize
            }*/
            Condition::HasUpgradableCard => self.game_state.upgradable_cards().any(|_| true),
            Condition::InPosition(position) => {
                if let Some(monster) = self.get_monster_binding(binding) {
                    monster.position == *position
                } else {
                    panic!("Unexpected player in InPosition check")
                }
            }
            /* 
            Condition::IsVariant(variant) => match binding {
                Binding::CurrentEvent => {
                    self.state
                        .floor_state
                        .event()
                        .variant
                        .as_ref()
                        .expect("Expected variant")
                        == variant
                }
                _ => panic!("Unexpected binding!"),
            },*/
            Condition::LastCard(_type) => match self.last_card_played {
                Some(last_type) => last_type == *_type,
                None => false,
            },
            Condition::LessThan(amount1, amount2) => {
                self.eval_amount(amount1, binding) < self.eval_amount(amount2, binding)
            }
            Condition::MultipleAnd(conditions) => conditions
                .iter()
                .all(|condition| self.eval_condition(condition, binding, action)),
            Condition::MultipleOr(conditions) => conditions
                .iter()
                .any(|condition| self.eval_condition(condition, binding, action)),
            Condition::Never => false,
            Condition::NoBlock => self.player.block == 0,
            Condition::Not(condition) => !self.eval_condition(condition, binding, action),
            Condition::OnFloor(floor) => self.game_state.map.floor >= *floor as i8,
            Condition::RemainingHp { amount, target } => {
                let creature = target.creature_ref(binding, action);
                let hp = self.eval_amount(amount, binding);
                self
                    .get_creature(creature)
                    .map(|c| c.hp.amount >= hp as u16)
                    .unwrap_or(false)
            }
            Condition::Stance(stance) => &self.stance == stance,
            Condition::Upgraded => self.is_upgraded(binding),
        }
    }
    
    pub fn get_monster_binding(&self, binding: Binding) -> Option<&Monster> {
        match binding {
            Binding::Buff(buff) => buff
                .creature
                .monster_ref()
                .and_then(|m| self.get_monster(m)),
            Binding::Creature(creature) => {
                creature.monster_ref().and_then(|m| self.get_monster(m))
            }
            Binding::Card(_) | Binding::Potion(_) | Binding::Relic(_) => {
                None
            }
        }
    }

    pub fn get_vars(&self, binding: Binding) -> &Vars {
        match binding {
            Binding::Buff(buff) => &self.get_buff(buff).unwrap().vars,
            Binding::Card(card) => &self.get_card(card).vars,
            Binding::Creature(creature) => {
                &self.get_monster(creature.monster_ref().unwrap())
                    .unwrap()
                    .vars
            }
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.index)
            }
            Binding::Relic(relic) => &self.game_state.relics.get(relic).vars,
        }
    }

    pub fn get_mut_vars(&mut self, binding: Binding) -> &mut Vars {
        match binding {
            Binding::Buff(buff) => &mut self.get_buff_mut(buff).unwrap().vars,
            Binding::Card(card) => &mut self.get_card_mut(card).vars,
            Binding::Creature(creature) => {
                &mut self.get_monster_mut(creature.monster_ref().unwrap())
                    .unwrap()
                    .vars
            }
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.index)
            }
            Binding::Relic(relic) => &mut self.game_state.relics.get_mut(relic).vars,
        }
    }

    pub fn is_upgraded(&self, binding: Binding) -> bool {
        match binding {
            Binding::Card(card) => self.get_card(card).upgrades > 0,
            Binding::Potion(_) => self.game_state.relics.contains("Sacred Bark"),
            _ => panic!("Unexpected is_upgraded check on {:?}", self),
        }
    }

    
    fn eval_effects(&mut self, effect: &[BattleEffect], binding: Binding, action: Option<GameAction>, probability: &mut Probability) {
        for effect in effect {
            self.eval_effect( effect, binding, action, probability);
        }
    }

    fn eval_effect(&mut self, effect: &BattleEffect, binding: Binding, action: Option<GameAction>, probability: &mut Probability) {
        match effect {
            BattleEffect::AddBuff {
                buff: buff_name,
                amount: buff_amount,
                target,
            } => {
                let amount = self.eval_amount(buff_amount, binding);
                for creature in self.eval_target(*target, binding, action, probability) {
                    if let Some(creature) = self.get_creature_mut(creature) {
                        creature.add_buff(buff_name, amount);
                    }
                }
            }
            BattleEffect::AddEnergy(energy_amount) => {
                let amount = self.eval_amount(energy_amount, binding) as u8;
                self.energy += amount
            }
            BattleEffect::AddGold(gold_amount) => {
                let amount = self.eval_amount(gold_amount, binding) as u16;
                self.game_state.add_gold(amount)
            }
            BattleEffect::AddMaxHp(hp_amount) => {
                let amount = self.eval_amount(hp_amount, binding) as u16;
                self.game_state.add_max_hp(amount)
            }
            BattleEffect::AddN(n_amount) => {
                let amount = self.eval_amount(n_amount, binding);
                self.get_mut_vars(binding).n += amount;
            }
            BattleEffect::AddOrbSlot(amount) => {
                let count = self.eval_amount(amount, binding) as u8;
                self.orb_slots = 10.min(count + self.orb_slots);
            }
            /*
            BattleEffect::AddPotionSlot(amount) => {
                for _ in 0..*amount {
                    self.game_state.potions.push_back(None)
                }
            }
            BattleEffect::AddRelic(name) => {
                self.game_state.relics.add(name);
            } */
            BattleEffect::AddX(amount) => {
                self.get_mut_vars(binding).x += self.eval_amount(amount, binding);
            }
            BattleEffect::AttackDamage {
                amount,
                target,
                if_fatal,
                times,
            } => {
                let mut attack_amount = self.eval_amount(amount, binding);
                if let Some(creature) = self.get_creature_mut(binding.creature_ref()) {
                    if let Some(buff) = creature.find_buff("Vigor").map(|a| a.vars.x) {
                        attack_amount += buff;
                        creature.remove_buff_by_name("Vigor")
                    }
                    if let Some(buff) = creature.find_buff("Strength").map(|a| a.vars.x) {
                        attack_amount += buff;
                    }
                }

                for creature in self.eval_target(*target, binding, action, probability) {
                    for _ in 0..self.eval_amount(times, binding) {
                        if self.damage(
                            attack_amount as u16,
                            creature,
                            Some(action.unwrap().creature),
                            false,
                        ) {
                            self.eval_effects(if_fatal, binding, action, probability);
                        }
                    }
                }
            }
            BattleEffect::Block { amount, target } => {
                let block_amount = self.eval_amount(amount, binding) as u16;

                for creature in self.eval_target(*target, binding, action, probability) {
                    if let Some(creature) = self.get_creature_mut(creature) {
                        let new_block = std::cmp::min(creature.block + block_amount, 999);
                        creature.block = new_block;
                    }
                }
            }
            BattleEffect::ChannelOrb(orb_type) => self.channel_orb(*orb_type, probability),
            BattleEffect::ChooseCardByType {
                destination,
                _type,
                rarity,
                class,
                then,
                choices,
                exclude_healing,
            } => {
                let amount = self.eval_amount(choices, binding) as usize;

                let choice =
                    self.random_cards_by_type(amount, *class, *_type, *rarity, *exclude_healing, probability);

                let mut card_choices = Vector::new();
                for base_card in choice {
                    card_choices.push_back(Card::new(base_card));
                }

                let mut effects = vector![CardEffect::MoveTo(*destination)];
                effects.extend(then.clone());

                self.card_choose = Some(CardChoiceState {
                    choices: card_choices
                        .iter()
                        .map(|card| card.reference(CardLocation::None))
                        .collect(),
                    count_range: (amount..amount + 1),
                    then: effects,
                    scry: false,
                });

                for card in card_choices {
                    self.cards
                        .insert(card.uuid, card);
                }
            }

            BattleEffect::ChooseCards {
                location,
                then,
                min,
                max,
            } => {
                let min_count = self.eval_amount(min, binding) as usize;
                let max_count = self.eval_amount(max, binding) as usize;
                let choices = match location {
                    CardLocation::DiscardPile => {
                        self.discard().collect()
                    }
                    CardLocation::DrawPile => self.draw().collect(),
                    CardLocation::ExhaustPile => {
                        self.exhaust().collect()
                    }
                    CardLocation::PlayerHand => self.hand().collect(),
                    CardLocation::None => panic!("Cannot choose from None!"),
                };

                self.card_choose = Some(CardChoiceState {
                    choices,
                    count_range: (min_count..max_count + 1),
                    then: Vector::from(then),
                    scry: false,
                });
            }
            BattleEffect::CreateCard {
                name,
                destination,
                then,
            } => {
                let card = Card::by_name(name);
                let card_ref = self.add_card(card, *destination, probability);
                self.eval_card_effects(then, card_ref, probability);
            }

            BattleEffect::CreateCardByType {
                destination,
                _type,
                rarity,
                class,
                exclude_healing,
                then,
            } => {
                let card =
                    self.random_cards_by_type(1, *class, *_type, *rarity, *exclude_healing, probability)[0];
                let card = self.add_card(Card::new(card), *destination, probability);
                self.eval_card_effects(then, card, probability);
            }
            BattleEffect::Custom => {
                match binding {
                    Binding::Buff(buff) => {
                        match buff.base.name.as_str() {
                            "Time Warp" => {
                                self.end_turn = true
                            }
                            _ => panic!("Unexpected custom effect in {}", buff.base.name)
                        }
                    }
                    Binding::Card(card) => {
                        match card.base.name.as_str() {
                            "All For One" => {
                                let card_count = (10 - self.hand.len()).min(self.discard.len());
                                let mut cards = self.discard.split_off(card_count);
                                std::mem::swap(&mut cards, &mut self.discard);
                                self.hand.extend(cards);
                            }
                            "Calculated Gamble" => {
                                let card_count = self.hand.len();
                                let cards = self.hand().collect_vec();
                                for card in &cards {
                                    self.move_card(
                                        CardDestination::DiscardPile,
                                        *card,
                                        probability,
                                    );
                                    self.discard_count += 1;
                                }

                                self.draw_card(card_count as u8, probability);
                                
                                for card in cards {
                                    self.eval_effects(&card.base.on_discard, Binding::Card(card), None, probability);
                                }
                            }
                            "Claw" => {
                                for (_, card) in self.cards.iter_mut() {
                                    if card.base.name == "Claw" {
                                        card.vars.x += 2;
                                    }
                                }
                            }
                            "Conjure Blade" => {
                                let mut card = Card::by_name("Expunger");
                                card.vars.n = self.energy as i16;
                                self.add_card(card, CardDestination::DrawPile(RelativePosition::Random), probability);
                                
                                self.energy = 0;
                            }
                            "Darkness" => {
                                let orbs = self.orbs.iter().map(|a| a.base).enumerate().filter(|(_, a)| *a == OrbType::Dark).collect_vec();
                                for (index, orb) in orbs {
                                    self.trigger_passive(orb, index)
                                }
                            }
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!()
                }
            },
            BattleEffect::Damage { amount, target } => {
                let total = self.eval_amount(amount, binding) as u16;
                for creature in self.eval_target(*target, binding, action, probability) {
                    self.damage(total, creature, None, false);
                }
            }
            /*
            BattleEffect::DeckAdd(name) =>  {
                self.game_state.add_card(Card::by_name(name));
            },
            BattleEffect::DeckOperation {
                random,
                count,
                operation,
            } => {
                if *random {
                    assert!(*operation == DeckOperation::Upgrade);
                    let choices = self.game_state.upgradable_cards().collect_vec();
                    let selected = probability.choose_multiple(choices, *count as usize);
                    for card in selected {
                        self.game_state.deck.get_mut(&card.uuid).unwrap().upgrade();
                    }
                } else {
                    self.game_state.screen_state = ScreenState::DeckChoose(*count, *operation);
                }
            }, */
            BattleEffect::Die { target } => {
                let creature = target.creature_ref(binding, action);
                self.die(creature);
            }
            BattleEffect::DoCardEffect {
                location,
                position,
                effect,
            } => {
                for card in self.get_cards_in_location(*location, *position, probability) {
                    self.eval_card_effect(effect, card, probability)
                }
            }
            BattleEffect::Draw(amount) => {
                let n = self.eval_amount(amount, binding);
                self.draw_card(n as u8, probability);
            }
            BattleEffect::EvokeOrb(amount) => self.evoke_orb(self.eval_amount(amount, binding) as u8, probability),
            BattleEffect::Heal { amount, target } => {
                let total = self.eval_amount(amount, binding);
                for creature in self.eval_target(*target, binding, action, probability) {
                    self.heal_creature(total as f64, creature);
                }
            }
            BattleEffect::HealPercentage { amount, target } => {
                let percentage = self.eval_amount(amount, binding) as f64 / 100.0;
                for creature_ref in self.eval_target(*target, binding, action, probability) {
                    let max_hp = self.get_creature(creature_ref).map(|c| c.hp.max).unwrap_or(0);
                    
                    let total = max_hp as f64 * percentage;
                    self.heal_creature(total, creature_ref);
                }
            }
            BattleEffect::If {
                condition,
                then,
                _else,
            } => {
                if self.eval_condition(condition, binding, action) {
                    self.eval_effects(then, binding, action, probability);
                } else {
                    self.eval_effects(_else, binding, action, probability);
                }
            }
            BattleEffect::LoseHp { amount, target } => {
                let total = self.eval_amount(amount, binding);
                for creature in self.eval_target(*target, binding, action, probability) {
                    self.lose_hp(total as u16, creature, false);
                }
            }
            /*
            BattleEffect::LoseHpPercentage(amount) => {
                let percentage = self.eval_amount(amount, binding) as f64 / 1000.0;
                let damage = (self.game_state.max_hp() as f64 * percentage).floor() as u16;
                self.lose_hp(damage, CreatureReference::Player, false);
            } */
            BattleEffect::RandomChance(chances) => {
                let evaluated_chances = chances
                    .iter()
                    .map(|chance| {
                        (
                            &chance.effect,
                            self.eval_amount(&chance.amount, binding) as u8,
                        )
                    })
                    .collect_vec();

                let choice = probability
                    .choose_weighted(&evaluated_chances)
                    .unwrap();

                self.eval_effects(choice, binding, action, probability);
            }
            /*
            BattleEffect::RandomPotion => {
                let potion = self.random_potion(matches!(self.game_state.floor_state, FloorState::Battle(_)));
                self.game_state.add_potion(potion);
            }
            BattleEffect::RandomRelic => {
                let relic = self.random_relic(None, None, None, false);
                self.add_relic(relic)
            }
            BattleEffect::ReduceMaxHpPercentage(amount) => {
                let percentage = self.eval_amount(amount, binding);
                let total = (self.game_state.max_hp() as f64 * (percentage as f64 / 100.0))
                    .floor() as u16;
                self.game_state.reduce_max_hp(total);
            }*/
            BattleEffect::RemoveDebuffs => {
                let creature_ref = binding.creature_ref();
                if let Some(creature) = self.get_creature_mut(creature_ref) {
                    let buffs: Vec<Uuid> = creature
                        .buffs
                        .values()
                        .filter(|buff| buff.base.debuff || buff.vars.x < 0)
                        .map(|buff| buff.uuid)
                        .collect_vec();
                    for buff in buffs {
                        creature.remove_buff_by_uuid(buff)
                    }
                }
            }/*
            BattleEffect::RemoveRelic(relic) => {
                self.game_state.relics.remove(relic);
            } */
            BattleEffect::Repeat { n, effect } => {
                let amount = self.eval_amount(n, binding);
                for _ in 0..amount {
                    self.eval_effects(effect, binding, action, probability);
                }
            }
            BattleEffect::ResetN => {
                let vars = self.get_mut_vars(binding);
                vars.n = vars.n_reset;
            }
            BattleEffect::Scry(count) => {
                let amount = self.eval_amount(count, binding);
                self.scry(amount as usize, probability);
            }
            BattleEffect::SelfEffect(effect) => {
                if let Binding::Card(card) = binding {
                    if effect != &CardEffect::Exhaust
                        || !self.game_state.relics.contains("Strange Spoon")
                        || probability.range(2) == 0
                    {
                        self.eval_card_effect(effect, card, probability);
                    }
                } else {
                    panic!("SelfBattleEffect on a non-card!")
                }
            }
            BattleEffect::SetN(n) => {
                let amount = self.eval_amount(n, binding);
                let vars = self.get_mut_vars(binding);
                vars.n = amount;
                vars.n_reset = amount;
            }
            BattleEffect::SetStance(stance) => {
                self.stance = *stance;
            }
            BattleEffect::SetX(x) => {
                let amount = self.eval_amount(x, binding);
                let vars = self.get_mut_vars(binding);
                vars.x = amount;
            }
            /*
            BattleEffect::ShowChoices(choices) => {
                let event = self.game_state.floor_state.event_mut();
                event.available_choices = choices.clone();
            }
            BattleEffect::ShowReward(rewards) => {
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
            }
            */
            BattleEffect::Shuffle => {
                let cards = self
                    .discard
                    .iter()
                    .copied()
                    .collect_vec();
                self.draw.extend(cards);
                self.discard.clear();
                self.shuffle(probability);
            }
            BattleEffect::Spawn { choices, count } => {
                let amount = self.eval_amount(count, binding);
                for _ in 0..amount {
                    let choice = probability.choose(choices.clone()).unwrap();
                    let base = models::monsters::by_name(&choice);
                    self.add_monster(base, 0, probability);
                }
            }
            BattleEffect::Split(left, right) => {
                if let Binding::Creature(CreatureReference::Creature(monster_ref)) = binding {
                    let monster = self.remove_monster(monster_ref.uuid);
                    let hp = HpRange::new(monster.creature.hp.amount);

                    let left_base = models::monsters::by_name(left);
                    let right_base = models::monsters::by_name(right);
                    let left_ref = self.add_monster(left_base, monster.position, probability);
                    let right_ref = self.add_monster(right_base, monster.position + 1, probability);
                    let battle = self;
                    
                    {
                        let left = battle.get_creature_mut(left_ref.creature_ref()).unwrap();
                        left.hp = hp;
                    }
                    {
                        let right = battle.get_creature_mut(right_ref.creature_ref()).unwrap();
                        right.hp = hp;

                    }
                } else {
                    panic!("Unepxected binding")
                }
            }
            BattleEffect::Unbuff(buff) => {
                if let Some(creature) = self.get_creature_mut(binding.creature_ref()) {
                    creature.remove_buff_by_name(buff);
                }
            }
        }
    }
    pub fn eval_card_effects(&mut self, effects: &[CardEffect], card: CardReference, probability: &mut Probability) {
        for effect in effects {
            self.eval_card_effect(effect, card, probability);
        }
    }

    fn eval_card_effect(&mut self, effect: &CardEffect, card: CardReference, probability: &mut Probability) {
        match effect {
            CardEffect::AutoPlay => {
                let binding = Binding::Card(card);
                let target = if self.eval_condition(&card.base.targeted, binding, None) {
                    self
                        .random_monster(probability)
                        .map(|a| a.creature_ref())
                } else {
                    None
                };
                if !self.end_turn {
                    self.play_card(card, target, probability);
                }
            }
            CardEffect::CopyTo { destination, then } => {
                let battle = self;
                let card = battle.get_card(card).duplicate();
                let card_ref = battle.add_card(card, *destination, probability);
                self.eval_card_effects(then, card_ref, probability);
            }
            CardEffect::Custom => panic!("Unexpected custom card effect"),
            CardEffect::Discard => {
                if !self.discard.contains(&card.uuid) {
                    self.move_card(
                        CardDestination::DiscardPile,
                        card,
                        probability,
                    );
                    self.eval_effects(&card.base.on_discard, Binding::Card(card), None, probability);
                    self.discard_count += 1;
                }
            }
            CardEffect::Exhaust => {
                if !self.exhaust.contains(&card.uuid) {
                    self.move_card(
                        CardDestination::ExhaustPile,
                        card,
                        probability,
                    );
                    self.eval_effects(&card.base.on_exhaust, Binding::Card(card), None, probability);
                }
            }
            CardEffect::MoveTo(destination) => {
                self.move_card(
                    *destination,
                    card,
                    probability,
                );
            }
            CardEffect::ReduceCost(amount) => {
                let reduction = self.eval_amount(amount, Binding::Card(card));
                let card = self.get_card_mut(card);
                if card.base.cost != Amount::X {
                    card.cost = std::cmp::max(card.cost as i16 - reduction, 0) as u8;
                    card.base_cost = std::cmp::max(card.base_cost as i16 - reduction, 0) as u8;
                }
            }
            CardEffect::Retain => {
                self
                    .get_card_mut(card)
                    .retain = true;
            }
            CardEffect::Scry => {
                if !self.discard.contains(&card.uuid) {
                    self.move_card(
                        CardDestination::DiscardPile,
                        card,
                        probability,
                    );
                }
            }
            CardEffect::Upgrade => {
                self
                    .get_card_mut(card)
                    .upgrade();
            }
            CardEffect::ZeroCombatCost => {
                let card = self.get_card_mut(card);
                card.cost = 0;
                card.base_cost = 0;
            }
            CardEffect::ZeroCostUntilPlayed => {
                let card = self.get_card_mut(card);
                card.cost = 0;
                card.cost_until_played = true;
            }
            CardEffect::ZeroTurnCost => {
                let card = self.get_card_mut(card);
                card.cost = 0;
            }
        }
    }

    pub fn play_card(&mut self, card: CardReference, target: Option<CreatureReference>, probability: &mut Probability) {
        self.move_out(card);
        for effect in &card.base.on_play {
            self.eval_effect(
                effect,
                Binding::Card(card),
                Some(GameAction {
                    is_attack: card.base._type == CardType::Attack,
                    creature: CreatureReference::Player,
                    target,
                }),
                probability,
            )
        }

        self.eval_when(When::PlayCard(CardType::All), probability);
        self.eval_when(When::PlayCard(card.base._type), probability);

        let battle_mut = self;
        if !battle_mut.exhaust.contains(&card.uuid) {
            battle_mut.discard.push_back(card.uuid);
        }
        
        self.move_out(card);
    }

    fn transform_card(&mut self, card: CardReference, probability: &mut Probability) {
        self.game_state.remove_card(card.uuid);

        let choices = models::cards::available_cards_by_class(card.base._class)
            .iter()
            .filter(|a| a.name != card.base.name)
            .collect();
        let new_card = probability.choose(choices).unwrap();
        self.game_state.add_card(Card::new(new_card));
    }

    fn remove_monster(&mut self, uuid: Uuid) -> Monster {
        let removed = self
            .monsters
            .remove(&uuid)
            .unwrap();
        for (_, monster) in self.monsters.iter_mut() {
            if monster.position > removed.position {
                monster.position -= 1;
            }
        }
        removed
    }

    fn add_monster(&mut self, base: &'static BaseMonster, position: usize, probability: &mut Probability) -> MonsterReference {
        let hp_asc = match self.fight_type {
            FightType::Boss => 9,
            FightType::Elite { .. } => 8,
            FightType::Common => 7,
        };
        let hp_range = if self.game_state.asc < hp_asc {
            &base.hp_range
        } else {
            &base.hp_range_asc
        };

        let hp = probability.range((hp_range.max - hp_range.min) as usize) as u16 + hp_range.min;

        let mut monster = Monster::create(base, hp);

        monster.position = position;

        let binding = Binding::Creature(monster.creature_ref());
        if let Some(range) = &base.n_range {
            let min = self.eval_amount(&range.min, binding);
            let max = self.eval_amount(&range.max, binding);
            let n = probability.range((max - min) as usize) as i16 + min;
            monster.vars.n = n;
            monster.vars.n_reset = n;
        }

        if let Some(range) = &base.x_range {
            let min = self.eval_amount(&range.min, binding);
            let max = self.eval_amount(&range.max, binding);
            let x = probability.range((max - min) as usize) as i16 + min;
            monster.vars.x = x;
        }

        self.eval_effects(&monster.base.on_create, binding, None, probability);

        for (_, m) in self.monsters.iter_mut() {
            if m.position >= position {
                m.position += 1;
            }
        }

        let monster_ref = MonsterReference {
            base,
            uuid: monster.uuid,
        };

        self.monsters.insert(monster.uuid, monster);

        monster_ref
    }

    fn scry(&mut self, count: usize, probability: &mut Probability) {
        self.peek_top(count, probability);
        
        let mut choices = vec![];
        let mut unknown_cards = self.draw.clone();
        for card in &self.draw_top_known {
            unknown_cards.remove(card);
        }
        for card in &self.draw_bottom_known {
            unknown_cards.remove(card);
        }

        let mut unknown_cards = unknown_cards.into_iter().collect_vec();

        for i in 0..count {
            if let Some(top) = self.draw_top_known.get(i) {
                choices.push(*top);
            } else if let Some(bottom) = self.draw_bottom_known.get(self.draw.len() - i - 1) {
                choices.push(*bottom);
            } else {
                let choice = probability.range(unknown_cards.len());
                choices.push(unknown_cards.remove(choice));
            }
        }

        self.card_choose = Some(CardChoiceState {
            count_range: (0..choices.len() + 1),
            choices: choices
                .into_iter()
                .map(|uuid| CardReference {
                    location: CardLocation::DrawPile,
                    uuid,
                    base: self.cards[&uuid].base,
                })
                .collect(),
            then: vector![],
            scry: true,
        });
    }

    pub fn end_turn(&mut self, probability: &mut Probability) {
        self.eval_when(When::BeforeHandDiscard, probability);
        let has_runic_pyramid = self.game_state.relics.contains("Runic Pyramid");
        for card_ref in self.hand().collect_vec() {
            let binding = Binding::Card(card_ref);
            if self.get_card(card_ref).retain {
                self.eval_effects(&card_ref.base.on_retain, binding, None, probability);
                if !self.eval_condition(&card_ref.base.retain, binding, None) {
                    self.get_card_mut(card_ref)
                        .retain = false;
                }
            } else if !has_runic_pyramid {
                self.move_card(
                    CardDestination::DiscardPile,
                    card_ref,
                    probability,
                );
            }
            self.eval_effects(&card_ref.base.on_turn_end, binding, None, probability);
        }

        let passives = self.orbs.iter().map(|a| a.base).enumerate().filter(|(_, a)| *a != OrbType::Plasma).collect_vec();

        for (index, orb) in passives {
            self.trigger_passive(orb, index);
        }

        self.eval_when(When::BeforeEnemyMove, probability);
        
        for (_, monster) in self
            .monsters
            .iter_mut()
            .sorted_by_key(|(_, a)| a.index)
        {
            if !monster.creature.has_buff("Barricade") {
                monster.creature.block = 0;
            }
        }

        for monster in self.available_monsters().collect_vec() {
            self.next_monster_move(monster, probability);
        }
        self.eval_when(When::AfterEnemyMove, probability);
        self.eval_when(When::TurnEnd, probability);
        self.start_turn(false, probability);
    }

    fn start_turn(&mut self, combat_start: bool, probability: &mut Probability) {
        
        if !self.player.has_buff("Barricade")
            && !self.player.has_buff("Blur")
        {
            if self.game_state.relics.contains("Calipers") {
                self.player.block =
                    self.player.block.saturating_sub(15);
            } else {
                self.player.block = 0;
            }
        }

        self.eval_when(When::BeforeHandDraw, probability);

        let mut cards_to_draw = 5;
        if self.game_state.relics.contains("Snecko Eye") {
            cards_to_draw += 2;
        }
        if combat_start && self.game_state.relics.contains("Bag of Preparation") {
            cards_to_draw += 2;
        }
        if let Some(buff) = self.player.find_buff("Draw Card") {
            cards_to_draw += buff.vars.x;
            self.player.remove_buff_by_name("Draw Card");
        }
        self.draw_card(cards_to_draw as u8, probability);

        self.eval_when(When::AfterHandDraw, probability);
    }

    fn next_monster_move(&mut self, monster: MonsterReference, probability: &mut Probability) {
        let current_move =
            if let Some(monster) = self.get_monster(monster) {
                let choices = monster.current_move_options.iter().copied().collect_vec();
                let choice = probability
                    .choose_weighted(&choices)
                    .expect("No current moves listed!");
                Some(*choice)
            } else {
                None
            };

        if let Some(current_move) = current_move {
            self.eval_effects(
                &current_move.effects,
                Binding::Creature(CreatureReference::Creature(monster)),
                None,
                probability
            );
            self.next_move(monster, current_move, probability);
        }
    }

    fn next_move(&mut self, monster_ref: MonsterReference, performed_move: &'static MonsterMove, probability: &mut Probability) {
        if let Some((index, phase)) = self
            .get_monster_mut(monster_ref)
            .map(|monster| {
                if let Some(last_move) = monster.last_move {
                    if last_move == performed_move {
                        monster.last_move_count += 1;
                    } else {
                        monster.last_move = Some(last_move);
                        monster.last_move_count = 1;
                    }
                };
                (monster.index, monster.phase)
            })
        {
            self.set_monster_move(index + 1, phase, monster_ref, probability);
        }
    }

    fn draw_card(&mut self, mut n: u8, probability: &mut Probability) {
        let mut cards = vec![];
        while n > 0 {
            if self.hand.len() == 10 {
                break;
            }
            if self.draw.is_empty() {
                self.shuffle(probability);
            }

            let battle = self;

            if battle.draw.is_empty() {
                break;
            }

            battle.peek_top(n as usize, probability);

            let mut to_draw = battle
                .draw_top_known
                .split_off(battle.draw_top_known.len().min(n as usize));
            std::mem::swap(&mut to_draw, &mut battle.draw_top_known); // Split off splits the wrong way, so we swap the two vectors

            n -= to_draw.len() as u8;

            for uuid in to_draw {
                let reference = battle
                    .cards
                    .get(&uuid)
                    .unwrap()
                    .reference(CardLocation::DrawPile);
                battle.draw.remove(&uuid).unwrap();
                cards.push(reference);
            }
        }

        for card in cards {
            self.eval_effects(&card.base.on_draw, Binding::Card(card), None, probability);
            self.eval_target_when(When::DrawCard(card.base._type), CreatureReference::Player, probability);
            self.eval_target_when(When::DrawCard(CardType::All), CreatureReference::Player, probability);
        }
    }

    fn get_cards_in_location(
        &mut self,
        location: CardLocation,
        position: RelativePosition,
        probability: &mut Probability
    ) -> Vec<CardReference> {
        match position {
            RelativePosition::All => self.cards_in_location(location),
            RelativePosition::Random => probability
                .choose(self.cards_in_location(location))
                .into_iter()
                .collect(),
            RelativePosition::Top => match location {
                CardLocation::DrawPile => {
                    self.peek_top(1, probability);

                    self.draw_top_known
                        .get(0)
                        .map(|uuid| CardReference {
                            location: CardLocation::DrawPile,
                            uuid: *uuid,
                            base: self.cards[uuid].base,
                        })
                        .into_iter()
                        .collect()
                }
                _ => panic!("Unepxected location in RelativePosition::Bottom"),
            },
            RelativePosition::Bottom => panic!("Unepxected RelativePosition::Bottom"),
        }
    }

    pub fn random_cards_by_type(
        &mut self,
        amount: usize,
        class: Option<Class>,
        _type: CardType,
        rarity: Option<Rarity>,
        exclude_healing: bool,
        probability: &mut Probability
    ) -> Vec<&'static BaseCard> {
        let cards = models::cards::available_cards_by_class(class.unwrap_or(self.game_state.class))
            .iter()
            .filter(|card| {
                (_type == CardType::All || card._type == _type)
                    && (rarity == None || rarity.unwrap() == card.rarity)
                    && (!exclude_healing
                        || !matches!(
                            card.name.as_str(),
                            "Feed"
                                | "Reaper"
                                | "Lesson Learned"
                                | "Alchemize"
                                | "Wish"
                                | "Bandage Up"
                                | "Self Repair"
                        ))
            })
            .cloned();

        probability.choose_multiple(cards.collect(), amount as usize)
    }

    fn channel_orb(&mut self, orb_type: OrbType, probability: &mut Probability) {
        if self.orbs.len()
            == self.orb_slots as usize
        {
            self.evoke_orb(1, probability);
        }

        let n = match orb_type {
            OrbType::Any => panic!("Unexpected Any orb type"),
            OrbType::Dark => {
                let focus = self.player.get_buff_amount("Focus");
                std::cmp::max(focus + 6, 0) as u16
            }
            _ => 0,
        };

        let orb = Orb { base: orb_type, n };

        self.orbs.push_back(orb);
    }

    fn add_block(&mut self, amount: u16, target: CreatureReference, probability: &mut Probability) {
        if let Some(mut_creature) = self.get_creature_mut(target) {
            let new_block = std::cmp::min(mut_creature.block + amount, 999);
            mut_creature.block = new_block;
            self.eval_target_when(When::OnBlock, target, probability)
        }
    }

    pub fn eval_when(&mut self, when: When, probability: &mut Probability) {
        self.eval_target_when(when.clone(), CreatureReference::Player, probability);

        for creature in self.available_creatures().collect_vec() {
            self.eval_target_when(when.clone(), creature, probability);
        }
    }

    fn eval_target_when(&mut self, when: When, target: CreatureReference, probability: &mut Probability) {
        self.eval_creature_buff_when(target, when.clone(), probability);
        if let Some(monster_ref) = target.monster_ref() {
            self.eval_monster_when(monster_ref, when, probability);
        } else {
            self.eval_relic_when(when, probability);
        }
    }

    fn eval_monster_when(&mut self, monster_ref: MonsterReference, when: When, probability: &mut Probability) {
        let phase = {
            if let Some(monster) = self.get_monster(monster_ref) {
                monster.whens.get(&when).map(|a| a.as_str())
            } else {
                None
            }
        };

        if let Some(phase_name) = phase {
            self.set_monster_phase(phase_name, monster_ref, probability)
        }
    }

    fn set_monster_phase(&mut self, phase: &str, monster: MonsterReference, probability: &mut Probability) {
        let new_phase = self
            .get_monster(monster)
            .unwrap()
            .base
            .phases
            .iter()
            .position(|p| p.name == phase)
            .unwrap();
        self.set_monster_move(0, new_phase, monster, probability);
    }


    fn eval_relic_when(&mut self, when: When, probability: &mut Probability) {
        if let Some(relic_ids) = self.game_state.relics.relic_whens.get(&when).cloned() {
            for relic_id in relic_ids {
                let (base, mut x, mut enabled, relic_ref) = {
                    let relic = &self.game_state.relics.relics[&relic_id];
                    (
                        relic.base,
                        relic.vars.x as u16,
                        relic.enabled,
                        relic.reference(),
                    )
                };

                match &base.activation {
                    Activation::Counter {
                        increment,
                        reset,
                        auto_reset,
                        target,
                    } => {
                        if increment == &when && x < *target {
                            x += 1;
                            if x == *target {
                                self.eval_effects(&base.effect, Binding::Relic(relic_ref), None, probability);
                                if *auto_reset {
                                    x = 0;
                                }
                            }
                        }
                        if reset == &when {
                            x = 0;
                        }
                    }
                    Activation::Immediate | Activation::Custom => {}
                    Activation::Uses { .. } => {
                        if x != 0 {
                            x -= 1;
                            self.eval_effects(&base.effect, Binding::Relic(relic_ref), None, probability);
                        }
                    }
                    Activation::When(_) => {
                        self.eval_effects(&base.effect, Binding::Relic(relic_ref), None, probability);
                    }
                    Activation::WhenEnabled {
                        activated_at,
                        enabled_at,
                        disabled_at,
                    } => {
                        if activated_at == &when && enabled {
                            self.eval_effects(&base.effect, Binding::Relic(relic_ref), None, probability);
                        }
                        if enabled_at == &when {
                            enabled = false;
                        }
                        if disabled_at == &when {
                            enabled = true;
                        }
                    }
                }
                {
                    let relic = self.game_state.relics.get_mut(relic_ref);
                    relic.vars.x = x as i16;
                    relic.enabled = enabled;
                }
            }
        }
    }

    fn eval_creature_buff_when(&mut self, creature_ref: CreatureReference, when: When, probability: &mut Probability) {
        if let Some(buff_ids) = self
            .get_creature(creature_ref)
            .and_then(|c| c.buffs_when.get(&when).cloned())
        {
            for buff_id in buff_ids {
                let (base, buff_ref) = {
                    let buff = &self.get_creature(creature_ref).unwrap().buffs[&buff_id];
                    (buff.base, buff.reference(creature_ref))
                };

                for WhenEffect {
                    when: _when,
                    effect,
                } in &base.effects
                {
                    if when == *_when {
                        self.eval_effects(effect, Binding::Buff(buff_ref), None, probability);
                    }
                }

                if base.expire_at == when {
                    self.get_creature_mut(creature_ref)
                        .unwrap()
                        .remove_buff(buff_ref);
                } else if base.reduce_at == when {
                    let should_remove = {
                        if let Some(buff) = self.get_buff_mut(buff_ref) {
                            if buff.stacked_vars.is_empty() {
                                buff.vars.x += 1;
                            } else {
                                for mut var in buff.stacked_vars.iter_mut() {
                                    var.x -= 1;
                                }

                                buff.stacked_vars = buff
                                    .stacked_vars
                                    .iter()
                                    .filter(|var| var.x > 0)
                                    .cloned()
                                    .collect();
                            }
                            buff.stacked_vars.is_empty() && buff.vars.x == 0
                        } else {
                            false
                        }
                    };

                    if should_remove {
                        self.get_creature_mut(creature_ref)
                            .unwrap()
                            .remove_buff(buff_ref);
                    }
                }
            }
        }
    }

    fn evoke_orb(&mut self, times: u8, probability: &mut Probability) {
        if let Some(orb) = self.orbs.pop_front() {
            match orb.base {
                OrbType::Any => panic!("Unexpected OrbType of any"),
                OrbType::Dark => {
                    for _ in 0..times {
                        let lowest_monster = self
                            .monsters
                            .values()
                            .filter(|m| m.targetable)
                            .min_by_key(|m| m.creature.hp.amount)
                            .map(|m| m.creature_ref());

                        if let Some(creature_ref) = lowest_monster {
                            self.damage(orb.n as u16, creature_ref, None, true);
                        }
                    }
                }
                OrbType::Frost => {
                    let focus = self.player.get_buff_amount("Focus");
                    let block_amount = std::cmp::max(focus + 5, 0) as u16;

                    for _ in 0..times {
                        self.add_block(block_amount, CreatureReference::Player, probability);
                    }
                }
                OrbType::Lightning => {
                    let has_electro_dynamics = self.player.has_buff("Electro");
                    let focus = self.player.get_buff_amount("Focus");
                    let orb_damage = std::cmp::max(8 + focus, 0) as u16;
                    for _ in 0..times {
                        if has_electro_dynamics {
                            for monster in self
                                .available_monsters()
                                .collect_vec()
                            {
                                self.damage(orb_damage, monster.creature_ref(), None, true);
                            }
                        } else {
                            let monsters = self
                                .available_monsters()
                                .collect_vec();
                            if let Some(selected) = probability.choose(monsters) {
                                self.damage(orb_damage, selected.creature_ref(), None, true);
                            }
                        }
                    }
                }
                OrbType::Plasma => self.energy += 2 * times,
            }
        }
    }

    fn damage(
        &mut self,
        amount: u16,
        creature_ref: CreatureReference,
        attacker: Option<CreatureReference>,
        is_orb: bool,
        probability: &mut Probability
    ) -> bool {
        let hp_loss = {
            if let Some(creature) = self.get_creature(creature_ref) {
                let mut multiplier = 1.0;
                if let Some(attacker) = attacker {
                    if creature.has_buff("Vulnerable") {
                        if creature.is_player() && self.game_state.relics.contains("Odd Mushroom") {
                            multiplier += 0.25;
                        } else if !creature.is_player() && self.game_state.relics.contains("Paper Phrog") {
                            multiplier += 0.75;
                        } else {
                            multiplier += 0.5;
                        }
                    }
                    if let Some(attacker_creature) = self.get_creature(attacker) {
                        if attacker_creature.has_buff("Weak") {
                            if creature.is_player() && self.game_state.relics.contains("Paper Krane") {
                                multiplier -= 0.4;
                            } else {
                                multiplier -= 0.25;
                            }
                        }
                    }
                }

                if is_orb && creature.has_buff("Lock On") {
                    multiplier += 0.5;
                }

                if let Some(buff) = creature.find_buff("Slow") {
                    multiplier += 0.1 * buff.vars.x as f64;
                }
                let mut full_amount = (amount as f64 * multiplier).floor() as u16;

                if creature.has_buff("Intangible") {
                    full_amount = 1;
                }

                let blocked_amount = full_amount.saturating_sub(creature.block);
                let mut unblocked_amount = full_amount - blocked_amount;

                if unblocked_amount > 0 {
                    if creature.is_player() {
                        if unblocked_amount <= 5 && self.game_state.relics.contains("Torii") {
                            unblocked_amount = 1;
                        }

                        if self.game_state.relics.contains("Tungsten Rod") {
                            unblocked_amount -= 1;
                        }
                    } else if unblocked_amount < 5 && self.game_state.relics.contains("The Boot") {
                        unblocked_amount = 5;
                    }
                }

                self.get_creature_mut(creature_ref).unwrap().block -= blocked_amount;

                unblocked_amount
            } else {
                0
            }
        };

        if hp_loss > 0 && self.lose_hp(hp_loss, creature_ref, true, probability) {
            return true;
        }

        false
    }

    fn lose_hp(
        &mut self,
        mut amount: u16,
        creature_ref: CreatureReference,
        ignore_intangible: bool,
        probability: &mut Probability
    ) -> bool {
        let new_hp = {
            if let Some(creature) = self.get_creature_mut(creature_ref) {
                if !ignore_intangible && creature.has_buff("Intangible") {
                    amount = std::cmp::max(amount, 1);
                }

                if let Some(buff) = creature.find_buff_mut("Invincible") {
                    amount = std::cmp::min(amount, buff.vars.x as u16);
                    buff.vars.x -= amount as i16;
                }

                if let Some(mut buff_amount) =
                    creature.find_buff("Mode Shift").map(|b| b.vars.x as u16)
                {
                    buff_amount = buff_amount.saturating_sub(amount);
                    if buff_amount == 0 {
                        creature.remove_buff_by_name("Mode Shift")
                    } else {
                        creature.find_buff_mut("Mode Shift").unwrap().vars.x = buff_amount as i16;
                    }
                }

                creature.hp = creature.hp.saturating_sub(amount);
                creature.hp
            } else {
                return false;
            }
        };

        if amount > 0 && creature_ref == CreatureReference::Player {
            self.hp_loss_count += 1;
        }

        if new_hp == 0 {
            self.die(creature_ref, probability)
        } else {
            false
        }
    }

    fn die(&mut self, creature_ref: CreatureReference, probability: &mut Probability) -> bool {
        match creature_ref {
            CreatureReference::Player => {
                let recovery: f64 =
                    if let Some(potion_ref) = self.game_state.find_potion("Fairy In A Bottle") {
                        self.game_state.potions[potion_ref.index] = None;
                        if self.game_state.relics.contains("Sacred Bark") {
                            0.6
                        } else {
                            0.3
                        }
                    } else if let Some(relic) = self.game_state.relics.find_mut("Lizard Tail") {
                        if relic.enabled {
                            relic.enabled = false;
                            0.5
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    };

                if recovery != 0.0 {
                    let max_hp = self.player.hp.max;
                    self.heal_creature(max_hp as f64 * recovery, creature_ref);
                }

                if self.player.hp.amount == 0 {
                    self.game_state.won = Some(false);
                    true
                } else {
                    false
                }
            }
            CreatureReference::Creature(monster_ref) => {
                self.eval_target_when(When::OnDie, creature_ref, probability);

                let monster_name = monster_ref.base.name.as_str();

                let dies = match monster_name {
                    "Awakened One" => {
                        let monster = self.get_monster_mut(monster_ref).unwrap();
                        if monster.vars.x == 0 {
                            monster.vars.x = 1;
                            monster.targetable = false;
                            monster.creature.hp = 0;
                            false
                        } else {
                            true
                        }
                    }
                    "Darkling" => {
                        if self
                            .monsters
                            .values()
                            .all(|a| !a.targetable || a.uuid == monster_ref.uuid)
                        {
                            true
                        } else {
                            let monster_mut = self
                                .get_monster_mut(monster_ref)
                                .unwrap();
                            monster_mut.targetable = false;
                            monster_mut.creature.hp = 0;
                            false
                        }
                    }
                    "Bronze Orb" => {
                        if let Some(buff) = self
                            .get_creature(creature_ref)
                            .and_then(|a| a.find_buff("Stasis"))
                            .map(|b| b.card_stasis.unwrap())
                        {
                            self.move_in(
                                buff,
                                CardDestination::PlayerHand,
                                probability,
                            );
                        }
                        true
                    }
                    _ => true,
                };

                if dies {
                    self.remove_monster(monster_ref.uuid);
                }

                dies
            }
        }
    }

    pub fn add_relic(&mut self, base: &'static BaseRelic) {
        let state = self.game_state.game_state_mut();
        let reference = state.add(base);

        match reference.base.activation {
            Activation::Immediate => {
                self.eval_effects(&reference.base.effect, Binding::Relic(reference), None);
            }
            Activation::Custom => {
                match base.name.as_str() {
                    "War Paint" | "Whetstone" => {
                        let card_type = if base.name == "War Paint" {
                            CardType::Skill
                        } else {
                            CardType::Attack
                        };
                        let available_cards: Vec<DeckCard> = self
                            .state
                            .upgradable_cards()
                            .filter(|card| card_type.matches(card.base._type))
                            .collect();

                        let cards = probability.choose_multiple(available_cards, 2);

                        for card in cards {
                            self.game_state.deck[&card.uuid].upgrade();
                        }
                    }
                    _ => panic!("Unexpected custom activation"),
                };
            }
            _ => {}
        }
    }

    pub fn heal_creature(&mut self, mut amount: f64, creature_ref: CreatureReference) {
        let creature: &mut Creature = match creature_ref {
            CreatureReference::Player => {
                self.heal(amount)
            }
            CreatureReference::Creature(monster_ref) => {
                let monster = self
                    .get_monster_mut(monster_ref)
                    .unwrap();
                monster.targetable = true;
                monster.creature.hp.add(amount);
            }
        };
    }

    pub fn drink_potion(&mut self, potion: PotionReference, target: Option<CreatureReference>) {
        self.eval_effects(
            &potion.base.on_drink,
            Binding::Potion(potion),
            Some(GameAction {
                creature: CreatureReference::Player,
                is_attack: false,
                target,
            }),
        );
    }

    

    pub fn trigger_passive(&mut self, orb: OrbType, orb_index: usize) {
        let focus = self.player.get_buff_amount("Focus");
        match orb {
            OrbType::Any => panic!("Unexpected any type of orb"),
            OrbType::Dark => self.orbs.get_mut(orb_index).unwrap().n += (6 + focus).max(0) as u16,
            OrbType::Frost => self.player.block += (2 + focus).max(0) as u16,
            OrbType::Lightning => {
                let amount = (3 + focus).max(0) as u16;
                if self.player.has_buff("Electro") {
                    for creature in self.available_creatures().collect_vec() {
                        self.damage(amount, creature, None, true);
                    }
                } else {
                    let creatures = self.available_creatures().collect_vec();
                    if let Some(creature) = probability.choose(creatures) {
                        self.damage(amount, creature, None, true);
                    }                        
                }
            }
            OrbType::Plasma => {
                self.energy += 1;
            }
        }
    }

    pub fn heal(&mut self, mut amount: f64) 
    {
        if self.game_state.relics.contains("Magic Flower") 
        {
            amount *= 1.5;
        }

        self.game_state.heal(amount)
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
    
    pub fn eval_target(
        &self,
        target: Target,
        binding: Binding,
        action: Option<GameAction>,
        probability: &mut Probability,
    ) -> Vec<CreatureReference> {
        let creatures = match target {
            Target::AllMonsters => self.available_creatures().collect(),
            Target::RandomMonster => self
                .random_monster(probability)
                .map(|a| a.creature_ref())
                .into_iter()
                .collect(),
            Target::OtherMonster => match binding.creature_ref() {
                CreatureReference::Player => panic!("Unexpected player in OtherMonster"),
                CreatureReference::Creature(uuid) => {
                    let monsters = self
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
            _ => vec![self.creature_ref(binding, action)],
        };

        creatures
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct CardChoiceState {
    pub choices: Vector<CardReference>,
    pub count_range: Range<usize>,
    pub then: Vector<CardEffect>,
    pub scry: bool,
}