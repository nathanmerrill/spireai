use std::ops::Range;

use im::{vector, HashMap, HashSet, Vector};
use itertools::Itertools;
use uuid::Uuid;

use crate::{
    models::{
        self,
        buffs::{self, RUSHDOWN},
        cards::BaseCard,
        core::{
            Amount, CardDestination, CardEffect, CardLocation, CardType, Class, Condition, Effect,
            FightType, OrbType, Rarity, RelativePosition, Stance, Target,
        },
        events::BaseEvent,
        monsters::{BaseMonster, Intent, MonsterMove, Move},
        relics,
    },
    spireai::references::{
        Binding, BuffReference, CardReference, CreatureReference, GameAction, MonsterReference,
        PotionReference,
    },
};

use super::{
    core::{Buff, Card, Creature, HpRange, Monster, Orb, Vars},
    game::{random_potion, GameState},
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
    pub draw_inserted: Vector<Uuid>,
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
    pub gold_recovered: u16,
    pub skip_monsters: bool,
    pub wish: u8,
    pub blizzard: u8,
    pub stance_pot: bool,
    pub game_state: GameState,
    pub card_choose: Option<CardChoiceState>,
    pub battle_over: bool,
    pub skip_rewards: bool,
}

impl BattleState {
    pub fn new(
        state: GameState,
        monster_names: &[String],
        fight_type: FightType,
        probability: &mut Probability,
    ) -> Self {
        let cards: HashMap<Uuid, Card> = state
            .deck
            .values()
            .map(|c| (c.uuid, c.duplicate()))
            .collect();
        let draw_top = if state.has_relic(relics::FROZEN_EYE) {
            cards.values().map(|c| c.uuid).collect()
        } else {
            Vector::new()
        };

        let orb_slots = if state.class == Class::Defect {
            3
        } else if state.has_relic(relics::PRISMATIC_SHARD) {
            1
        } else {
            0
        };

        let mut energy = 3;

        for relic in state.relics {
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
            let burning_type = if burning { probability.range(4) } else { 4 };
            let has_preserved_insect = state.has_relic(relics::PRESERVED_INSECT);
            if burning || has_preserved_insect {
                monsters.iter_mut().for_each(|(_, monster)| {
                    match burning_type {
                        0 => {
                            monster
                                .creature
                                .add_buff(buffs::STRENGTH, (state.act + 1) as i16);
                        }
                        1 => {
                            let new_hp = monster.creature.hp.max + monster.creature.hp.max / 4;
                            monster.creature.hp = HpRange::new(new_hp);
                        }
                        2 => {
                            monster
                                .creature
                                .add_buff(buffs::METALLICIZE, (state.act * 2 + 2) as i16);
                        }
                        3 => {
                            monster
                                .creature
                                .add_buff(buffs::REGENERATE, (state.act * 2 + 1) as i16);
                        }
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
            draw_inserted: Vector::new(),
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
            skip_monsters: false,
            gold_recovered: 0,
            wish: 0,
            blizzard: 0,
            stance_pot: false,
            game_state: state,
            card_choose: None,
            battle_over: false,
            skip_rewards: false,
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
        for relic in battle_state.game_state.relics {}
        if fight_type == FightType::Boss && battle_state.game_state.has_relic(relics::PANTOGRAPH) {
            battle_state.heal(25.0)
        }

        if matches!(fight_type, FightType::Elite { .. })
            && battle_state.game_state.has_relic(relics::SLING_OF_COURAGE)
        {
            battle_state.player.add_buff(buffs::STRENGTH, 2);
        }

        battle_state.start_turn(true, probability);

        battle_state
    }

    fn shuffle(&mut self, probability: &mut Probability) {
        self.draw_top_known = Vector::new();
        self.draw_bottom_known = Vector::new();
        self.draw_inserted = Vector::new();
        if self.game_state.has_relic(relics::FROZEN_EYE) {
            self.peek_top(self.draw.len(), probability)
        }
    }

    pub fn combat_end(&mut self, probability: &mut Probability) {
        self.battle_over = true;
        todo!("CombatEnd relics");
        self.game_state.hp = self.player.hp;
    }

    fn set_monster_move(
        &mut self,
        mut move_index: usize,
        mut phase_index: usize,
        monster_ref: MonsterReference,
        probability: &mut Probability,
    ) {
        let (base, last_move, last_move_count) = {
            let monster = self.get_monster(monster_ref).unwrap();
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

                    if self.game_state.has_relic(relics::RUNIC_DOME) {
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

        let monster = self.get_monster_mut(monster_ref).unwrap();

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
                    .map_or_else(|| self.fight_type, |a| a.base.fight_type);
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
            Amount::Blizzard => self.blizzard as i16,
            Amount::Shield => self.player.block as i16,
            Amount::Custom => unimplemented!(),
            Amount::EnemyCount => self.monsters.len() as i16,
            Amount::N => self.get_vars(binding).n as i16,
            Amount::NegX => -self.get_vars(binding).x as i16,
            Amount::OrbCount => self.orbs.len() as i16,
            Amount::MaxHp => self.get_creature(binding.creature_ref()).unwrap().hp.max as i16,
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
                self.get_creature(creature)
                    .map(|c| c.has_buff(buff))
                    .unwrap_or(false)
            }
            Condition::BuffX {
                target,
                buff,
                amount: x,
            } => {
                let val = self.eval_amount(x, binding);
                let creature = self.get_creature(target.creature_ref(binding, action));
                creature.is_some_and(|c| c.get_buff_amount(buff) >= val)
            }
            Condition::Custom => unimplemented!(),
            Condition::Equals(amount1, amount2) => {
                self.eval_amount(amount1, binding) == self.eval_amount(amount2, binding)
            }
            Condition::FriendlyDead(name) => self.monsters.values().any(|m| m.base.name == *name),
            Condition::HalfHp => self
                .get_creature(match binding {
                    Binding::Creature(creature) => creature,
                    _ => CreatureReference::Player,
                })
                .map(|creature| creature.hp.amount * 2 <= creature.hp.max)
                .unwrap_or(false),
            Condition::HasCard { location, card } => match location {
                CardLocation::DiscardPile => self.discard().any(|c| c.base._type == *card),
                CardLocation::PlayerHand => self.hand().any(|c| c.base._type == *card),
                CardLocation::ExhaustPile => self.exhaust().any(|c| c.base._type == *card),
                CardLocation::DrawPile => self.draw().any(|c| c.base._type == *card),
                CardLocation::None => false,
            },
            Condition::HasDiscarded => self.discard_count > 0,
            Condition::HasFriendlies(count) => {
                let creature = self
                    .get_monster_binding(binding)
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
            Condition::HasOrbSlot => self.orb_slots > 0,

            /*Condition::HasRemoveableCards { count, card_type } => {
                self.removable_cards()
                    .filter(|card| card.base._type.matches(*card_type))
                    .count()
                    > *count as usize
            }*/
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
            Condition::NoBlock => self.player.block == 0,
            Condition::Not(condition) => !self.eval_condition(condition, binding, action),
            Condition::RemainingHp { amount, target } => {
                let creature = target.creature_ref(binding, action);
                let hp = self.eval_amount(amount, binding);
                self.get_creature(creature)
                    .map(|c| c.hp.amount >= hp as u16)
                    .unwrap_or(false)
            }
            Condition::Stance(stance) => &self.stance == stance,
            Condition::Upgraded => self.is_upgraded(binding),
            _ => self.game_state.eval_condition(condition),
        }
    }

    pub fn get_monster_binding(&self, binding: Binding) -> Option<&Monster> {
        match binding {
            Binding::Buff(buff) => buff
                .creature
                .monster_ref()
                .and_then(|m| self.get_monster(m)),
            Binding::Creature(creature) => creature.monster_ref().and_then(|m| self.get_monster(m)),
            Binding::Card(_) | Binding::Potion(_) | Binding::Relic(_) => None,
        }
    }

    pub fn get_vars(&self, binding: Binding) -> &Vars {
        match binding {
            Binding::Buff(buff) => &self.get_buff(buff).unwrap().vars,
            Binding::Card(card) => &self.get_card(card).vars,
            Binding::Creature(creature) => {
                &self
                    .get_monster(creature.monster_ref().unwrap())
                    .unwrap()
                    .vars
            }
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.index)
            }
            Binding::Relic(relic) => &self.game_state.get_relic(relic.base).unwrap().vars,
        }
    }

    pub fn get_mut_vars(&mut self, binding: Binding) -> &mut Vars {
        match binding {
            Binding::Buff(buff) => &mut self.get_buff_mut(buff).unwrap().vars,
            Binding::Card(card) => &mut self.get_card_mut(card).vars,
            Binding::Creature(creature) => {
                &mut self
                    .get_monster_mut(creature.monster_ref().unwrap())
                    .unwrap()
                    .vars
            }
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.index)
            }
            Binding::Relic(relic) => &mut self.game_state.get_relic(relic.base).unwrap().vars,
        }
    }

    pub fn is_upgraded(&self, binding: Binding) -> bool {
        match binding {
            Binding::Card(card) => self.get_card(card).upgrades > 0,
            Binding::Potion(_) => self.game_state.has_relic(relics::SACRED_BARK),
            _ => panic!("Unexpected is_upgraded check on {:?}", self),
        }
    }

    fn eval_effects(
        &mut self,
        effect: &[Effect],
        binding: Binding,
        action: Option<GameAction>,
        probability: &mut Probability,
    ) {
        for effect in effect {
            self.eval_effect(effect, binding, action, probability);
        }
    }

    fn eval_effect(
        &mut self,
        effect: &Effect,
        binding: Binding,
        action: Option<GameAction>,
        probability: &mut Probability,
    ) {
        match effect {
            Effect::AddBuff {
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
            Effect::AddEnergy(energy_amount) => {
                let amount = self.eval_amount(energy_amount, binding) as u8;
                self.energy += amount
            }
            Effect::AddGold(gold_amount) => {
                let amount = self.eval_amount(gold_amount, binding) as u16;
                self.game_state.add_gold(amount)
            }
            Effect::AddMaxHp(hp_amount) => {
                let amount = self.eval_amount(hp_amount, binding) as u16;
                self.game_state.add_max_hp(amount)
            }
            Effect::AddN(n_amount) => {
                let amount = self.eval_amount(n_amount, binding);
                self.get_mut_vars(binding).n += amount;
            }
            Effect::AddOrbSlot(amount) => {
                let count = self.eval_amount(amount, binding) as u8;
                self.orb_slots = 10.min(count + self.orb_slots);
            }
            Effect::AddX(amount) => {
                self.get_mut_vars(binding).x += self.eval_amount(amount, binding);
            }
            Effect::AttackDamage {
                amount,
                target,
                if_fatal,
                times,
            } => {
                let attack_amount = self.eval_amount(amount, binding).min(0);

                let (is_fatal, _) = self.attack_damage(
                    attack_amount,
                    self.eval_amount(times, binding) as u16,
                    self.eval_target(*target, binding, action, probability),
                    binding.creature_ref(),
                    probability,
                );

                if is_fatal {
                    self.eval_effects(if_fatal, binding, action, probability)
                }
            }
            Effect::Block { amount, target } => {
                let block_amount = self.eval_amount(amount, binding) as u16;
                for creature in self.eval_target(*target, binding, action, probability) {
                    self.add_block(
                        block_amount,
                        creature,
                        matches!(binding, Binding::Card(_)),
                        probability,
                    );
                }
            }
            Effect::ChannelOrb(orb_type) => self.channel_orb(*orb_type, probability),
            Effect::ChooseCardByType {
                destination,
                _type,
                rarity,
                class,
                then,
                choices,
                exclude_healing,
            } => {
                let amount = self.eval_amount(choices, binding) as usize;

                let choice = self.random_cards_by_type(
                    amount,
                    *class,
                    *_type,
                    *rarity,
                    *exclude_healing,
                    probability,
                );

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
                    self.cards.insert(card.uuid, card);
                }
            }

            Effect::ChooseCards {
                location,
                then,
                min,
                max,
            } => {
                let min_count = self.eval_amount(min, binding) as usize;
                let max_count = self.eval_amount(max, binding) as usize;
                let choices = match location {
                    CardLocation::DiscardPile => self.discard().collect(),
                    CardLocation::DrawPile => self.draw().collect(),
                    CardLocation::ExhaustPile => self.exhaust().collect(),
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
            Effect::CreateCard {
                name,
                destination,
                then,
            } => {
                let card = Card::by_name(name);
                let card_ref = self.add_card(card, *destination, probability);
                self.eval_card_effects(then, card_ref, probability);
            }

            Effect::CreateCardByType {
                destination,
                _type,
                rarity,
                class,
                exclude_healing,
                then,
            } => {
                let card = self.random_cards_by_type(
                    1,
                    *class,
                    *_type,
                    *rarity,
                    *exclude_healing,
                    probability,
                )[0];
                let card = self.add_card(Card::new(card), *destination, probability);
                self.eval_card_effects(then, card, probability);
            }
            Effect::Catalyst => {
                if let Some(creature) = action
                    .and_then(|a| a.target)
                    .and_then(|b| self.get_creature_mut(b))
                {
                    if let Some(buff) = creature.get_singular_buff_mut(buffs::POISON) {
                        if self.is_upgraded(binding) {
                            buff.vars.x *= 3
                        } else {
                            buff.vars.x *= 2
                        }
                    }
                }
            }
            Effect::Custom => match binding {
                Binding::Buff(buff) => match buff.base.name.as_str() {
                    "Time Warp" => self.end_turn = true,
                    _ => panic!("Unexpected custom effect in {}", buff.base.name),
                },
                Binding::Card(card) => match card.base.name.as_str() {
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
                            self.move_card(CardDestination::DiscardPile, *card, probability);
                            self.discard_count += 1;
                        }

                        self.draw_card(card_count as u8, probability);

                        for card in cards {
                            self.eval_effects(
                                &card.base.on_discard,
                                Binding::Card(card),
                                None,
                                probability,
                            );
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
                        self.add_card(
                            card,
                            CardDestination::DrawPile(RelativePosition::Random),
                            probability,
                        );

                        self.energy = 0;
                    }
                    "Darkness" => {
                        let orbs = self
                            .orbs
                            .iter()
                            .map(|a| a.base)
                            .enumerate()
                            .filter(|(_, a)| *a == OrbType::Dark)
                            .collect_vec();
                        for (index, orb) in orbs {
                            self.trigger_passive(orb, index, probability)
                        }
                    }
                    "Escape Plan" => {
                        if let Some(card) = self.draw_card(1, probability).get(0) {
                            if card.base._type == CardType::Skill {
                                self.add_block(4, CreatureReference::Player, true, probability)
                            }
                        }
                    }
                    "Fission" => {
                        let amount = self.orbs.len() as u8;
                        if self.is_upgraded(binding) {
                            for _ in 0..amount {
                                self.evoke_orb(1, probability);
                            }
                        } else {
                            self.orbs.clear()
                        }

                        self.draw_card(amount, probability);
                        self.energy += amount;
                    }
                    "Lesson Learned" => {
                        let choices = self
                            .game_state
                            .deck
                            .iter_mut()
                            .filter(|(_, a)| a.upgradable())
                            .collect_vec();
                        if let Some((_, card)) = probability.choose(choices) {
                            card.upgrade()
                        }
                    }
                    "Meditate" => {
                        self.card_choose = Some(CardChoiceState {
                            count_range: (2..2),
                            choices: self.discard().collect(),
                            then: vector![CardEffect::Retain],
                            scry: true,
                        });

                        self.end_turn = true
                    }
                    "Melter" => {
                        if let Some(creature) = self.get_creature_mut(binding.creature_ref()) {
                            creature.block = 0;
                        }
                    }
                    "Nightmare" => {
                        self.card_choose = Some(CardChoiceState {
                            count_range: (1..1),
                            choices: self.hand().collect(),
                            then: vector![CardEffect::Custom("Nightmare".to_string())],
                            scry: true,
                        });
                    }
                    "Pressure Points" => {
                        for creature_ref in self.available_creatures().collect_vec() {
                            if let Some(amount) = self
                                .get_creature(creature_ref)
                                .map(|a| a.get_buff_amount(buffs::MARK))
                            {
                                self.lose_hp(amount as u16, creature_ref, false, probability);
                            }
                        }
                    }
                    "Reaper" => {
                        let amount = if self.is_upgraded(binding) { 4 } else { 5 };
                        let (_, amount_dealt) = self.attack_damage(
                            amount,
                            1,
                            self.available_creatures().collect_vec(),
                            CreatureReference::Player,
                            probability,
                        );

                        self.heal(amount_dealt as f64);
                    }
                    "Recursion" => {
                        if let Some(orb) = self.orbs.front().copied() {
                            self.evoke_orb(1, probability);
                            self.orbs.push_back(orb);
                        }
                    }
                    "Recycle" => {
                        self.card_choose = Some(CardChoiceState {
                            count_range: (1..1),
                            choices: self.hand().collect(),
                            then: vector![CardEffect::Custom("Recycle".to_string())],
                            scry: true,
                        });
                    }
                    "Scrape" => {
                        let amount = if self.is_upgraded(binding) { 4 } else { 5 };
                        let cards = self.draw_card(amount, probability);
                        for card in cards {
                            if self.get_card(card).cost != 0 {
                                self.discard_card(card, probability);
                            }
                        }
                    }
                    "Second Wind" => {
                        let cards = self
                            .hand()
                            .filter(|a| a.base._type != CardType::Attack)
                            .collect_vec();
                        let times = cards.len();
                        let block_amount = if self.is_upgraded(binding) { 5 } else { 7 };

                        self.exhaust_cards(cards, probability);

                        for _ in 0..times {
                            self.add_block(
                                block_amount,
                                CreatureReference::Player,
                                true,
                                probability,
                            );
                        }
                    }
                    "Secret Technique" => {
                        let cards = self.random_cards_by_type(
                            1,
                            Some(self.game_state.class),
                            CardType::Skill,
                            None,
                            true,
                            probability,
                        );
                        for card in cards {
                            let mut card = Card::new(card);
                            card.cost_until_played = true;
                            card.cost = 0;
                            self.add_card(card, CardDestination::PlayerHand, probability);
                        }
                    }
                    "Secret Weapon" => {
                        let cards = self.random_cards_by_type(
                            1,
                            Some(self.game_state.class),
                            CardType::Attack,
                            None,
                            true,
                            probability,
                        );
                        for card in cards {
                            let mut card = Card::new(card);
                            card.cost_until_played = true;
                            card.cost = 0;
                            self.add_card(card, CardDestination::PlayerHand, probability);
                        }
                    }
                    "Sever Soul" => {
                        let cards = self
                            .hand()
                            .filter(|a| a.base._type != CardType::Attack)
                            .collect_vec();

                        self.exhaust_cards(cards, probability);
                    }
                    "Storm Of Steel" => {
                        let cards = self.hand().collect_vec();
                        let count = cards.len();
                        for card in cards {
                            self.discard_card(card, probability);
                        }
                        for _ in 0..count {
                            self.add_card(
                                Card::by_name("Shiv"),
                                CardDestination::PlayerHand,
                                probability,
                            );
                        }
                    }
                    "Unload" => {
                        let cards = self
                            .hand()
                            .filter(|a| a.base._type != CardType::Attack)
                            .collect_vec();

                        for card in cards {
                            self.discard_card(card, probability);
                        }
                    }
                    "Vault" => {
                        self.end_turn = true;
                        self.skip_monsters = true;
                    }
                    "Violence" => {
                        let mut count = if self.is_upgraded(binding) { 3 } else { 4 };
                        count = count.max(10 - self.hand.len());
                        let attacks = self
                            .draw()
                            .filter(|a| a.base._type == CardType::Attack)
                            .collect_vec();

                        let chosen = probability.choose_multiple(attacks, count);
                        for card in chosen {
                            self.move_card(CardDestination::PlayerHand, card, probability);
                        }
                    }
                    "Wallop" => {
                        let target = action.unwrap().target.unwrap();
                        let amount = if self.is_upgraded(binding) { 9 } else { 12 };

                        let (_, damage_dealt) = self.attack_damage(
                            amount,
                            1,
                            vec![target],
                            CreatureReference::Player,
                            probability,
                        );
                        self.add_block(damage_dealt, CreatureReference::Player, true, probability);
                    }
                    "Wish" => {
                        self.wish += 1;
                    }
                    _ => panic!("Unrecognized custom card!"),
                },
                Binding::Creature(creature) => {
                    match action.unwrap().monster_move.unwrap().name.as_str() {
                        "Stasis" => {
                            let options = if self.draw.is_empty() {
                                self.discard().max_set_by_key(|a| a.base.rarity)
                            } else {
                                self.draw().max_set_by_key(|a| a.base.rarity)
                            };

                            if let Some(selected) = probability.choose(options) {
                                if let Some(creature) = self.get_creature_mut(creature) {
                                    creature.add_buff(buffs::STASIS, 1);
                                    if let Some(buff) = creature.buffs.last_mut() {
                                        buff.card_stasis = Some(selected.uuid);
                                    }
                                    self.move_out(selected);
                                }
                            }
                        }
                        "Support Beam" => {
                            let boss = self
                                .monsters
                                .iter()
                                .find(|a| a.1.base.name == "Bronze Automaton")
                                .unwrap()
                                .1
                                .creature_ref();

                            self.add_block(12, boss, false, probability);
                        }
                        "Inferno" => {
                            for (_, card) in self.cards.iter_mut() {
                                if card.base.name == "Burn" {
                                    card.upgrades = 1
                                }
                            }
                        }
                        "Mug" | "Lunge" => {
                            let max_gold = self.game_state.gold as i16;
                            let stolen = if let Some(monster) =
                                self.get_monster_mut(creature.monster_ref().unwrap())
                            {
                                let amount =
                                    (monster.creature.get_buff_amount(buffs::INNATE_THEIVERY))
                                        .min(max_gold);
                                monster.vars.x += amount;
                                amount
                            } else {
                                0
                            };

                            self.game_state.gold -= stolen as u16;
                        }
                        "Escape" => {
                            self.remove_monster(creature.monster_ref().unwrap().uuid);
                            if self.monsters.is_empty() {
                                self.combat_end(probability);
                            }
                        }
                        "Suck" => {
                            let amount = if self.game_state.asc > 1 { 12 } else { 10 };

                            let (_, damage_dealt) = self.attack_damage(
                                amount,
                                1,
                                vec![CreatureReference::Player],
                                creature,
                                probability,
                            );
                            self.heal_creature(damage_dealt as f64, creature);
                        }
                        "Smash" => {
                            let amount = if self.game_state.asc > 2 { 38 } else { 39 };

                            let (_, damage_dealt) = self.attack_damage(
                                amount,
                                1,
                                vec![CreatureReference::Player],
                                creature,
                                probability,
                            );
                            if self.game_state.asc > 17 {
                                self.add_block(99, creature, false, probability);
                            } else {
                                self.add_block(damage_dealt, creature, false, probability);
                            }
                        }
                        "Implant" => {
                            let curse = probability
                                .choose(
                                    models::cards::available_cards_by_class(
                                        models::core::Class::Curse,
                                    )
                                    .to_vec(),
                                )
                                .unwrap();

                            self.game_state.add_card(Card::new(curse))
                        }

                        a => panic!("Unexpected Custom in creature move: {}", a),
                    }
                }
                Binding::Potion(potion) => match potion.base.name.as_str() {
                    "Smoke Bomb" => {
                        self.skip_rewards = true;
                        self.combat_end(probability);
                    }
                    "Snecko Oil" => {
                        for card in self.hand().collect_vec() {
                            let card = self.get_card_mut(card);
                            card.base_cost = probability.range(5) as u8;
                            card.cost = card.base_cost;
                        }
                    }
                    "Stance Potion" => {
                        self.stance_pot = true;
                    }
                    a => panic!("Unexpected Custom effect in potion: {}", a),
                },
                Binding::Relic(_) => {
                    panic!("Unexpected Custom effect in relic");
                }
            },
            Effect::Damage { amount, target } => {
                let total = self.eval_amount(amount, binding) as u16;
                for creature in self.eval_target(*target, binding, action, probability) {
                    self.damage(total, creature, None, false, probability);
                }
            }
            Effect::Die { target } => {
                let creature = target.creature_ref(binding, action);
                self.die(creature, probability);
            }
            Effect::DoCardEffect {
                location,
                position,
                effect,
            } => {
                for card in self.get_cards_in_location(*location, *position, probability) {
                    self.eval_card_effect(effect, card, probability)
                }
            }
            Effect::Draw(amount) => {
                let n = self.eval_amount(amount, binding);
                self.draw_card(n as u8, probability);
            }
            Effect::EvokeOrb(amount) => {
                self.evoke_orb(self.eval_amount(amount, binding) as u8, probability)
            }
            Effect::Heal { amount, target } => {
                let total = self.eval_amount(amount, binding);
                for creature in self.eval_target(*target, binding, action, probability) {
                    self.heal_creature(total as f64, creature);
                }
            }
            Effect::HealPercentage { amount, target } => {
                let percentage = self.eval_amount(amount, binding) as f64 / 100.0;
                for creature_ref in self.eval_target(*target, binding, action, probability) {
                    let max_hp = self
                        .get_creature(creature_ref)
                        .map(|c| c.hp.max)
                        .unwrap_or(0);

                    let total = max_hp as f64 * percentage;
                    self.heal_creature(total, creature_ref);
                }
            }
            Effect::If {
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
            Effect::LoseHp { amount, target } => {
                let total = self.eval_amount(amount, binding);
                for creature in self.eval_target(*target, binding, action, probability) {
                    self.lose_hp(total as u16, creature, false, probability);
                }
            }
            Effect::RandomChance(chances) => {
                let evaluated_chances = chances
                    .iter()
                    .map(|chance| {
                        (
                            &chance.effect,
                            self.eval_amount(&chance.amount, binding) as u8,
                        )
                    })
                    .collect_vec();

                let choice = probability.choose_weighted(&evaluated_chances).unwrap();

                self.eval_effects(choice, binding, action, probability);
            }
            Effect::RemoveDebuffs => {
                let creature_ref = binding.creature_ref();
                if let Some(creature) = self.get_creature_mut(creature_ref) {
                    creature.buffs = creature
                        .buffs
                        .into_iter()
                        .filter(|buff| buff.base.debuff || buff.vars.x < 0)
                        .collect();
                }
            }
            Effect::Repeat { n, effect } => {
                let amount = self.eval_amount(n, binding);
                for _ in 0..amount {
                    self.eval_effects(effect, binding, action, probability);
                }
            }
            Effect::ResetN => {
                let vars = self.get_mut_vars(binding);
                vars.n = vars.n_reset;
            }
            Effect::Scry(count) => {
                let amount = self.eval_amount(count, binding);
                self.scry(amount as usize, probability);
            }
            Effect::SelfEffect(effect) => {
                if let Binding::Card(card) = binding {
                    if effect != &CardEffect::Exhaust
                        || !self.game_state.has_relic(relics::STRANGE_SPOON)
                        || probability.range(2) == 0
                    {
                        self.eval_card_effect(effect, card, probability);
                    }
                } else {
                    panic!("SelfEffect on a non-card!")
                }
            }
            Effect::SetN(n) => {
                let amount = self.eval_amount(n, binding);
                let vars = self.get_mut_vars(binding);
                vars.n = amount;
                vars.n_reset = amount;
            }
            Effect::SetStance(stance) => self.set_stance(*stance, probability),
            Effect::SetX(x) => {
                let amount = self.eval_amount(x, binding);
                let vars = self.get_mut_vars(binding);
                vars.x = amount;
            }
            Effect::Shuffle => {
                let cards = self.discard.iter().copied().collect_vec();
                self.draw.extend(cards);
                self.discard.clear();
                self.shuffle(probability);
            }
            Effect::Spawn { choices, count } => {
                let amount = self.eval_amount(count, binding);
                for _ in 0..amount {
                    let choice = probability.choose(choices.clone()).unwrap();
                    let base = models::monsters::by_name(&choice);
                    self.add_monster(base, 0, probability);
                }
            }
            Effect::Split(left, right) => {
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
            Effect::Unbuff(buff) => {
                if let Some(creature) = self.get_creature_mut(binding.creature_ref()) {
                    creature.remove_buffs_by_type(buff);
                }
            }
            Effect::RandomPotion => {
                let potion = random_potion(true, probability);
                self.game_state.add_potion(potion);
            }
            _ => {
                self.game_state.eval_effect(effect, probability);
            }
        }
    }

    pub fn set_stance(&mut self, stance: Stance, probability: &mut Probability) {
        if self.stance == stance {
            return;
        }

        if self.stance == Stance::Calm {
            self.energy += if self.game_state.has_relic(relics::VIOLET_LOTUS) {
                2
            } else {
                3
            }
        }

        match stance {
            Stance::Calm | Stance::None => {}
            Stance::Divinity => {
                self.energy += 3;
            }
            Stance::Wrath => {
                self.draw_card(self.player.get_buff_amount(RUSHDOWN) as u8, probability);
            }
            Stance::All => panic!("Unexpected All stance enter"),
        }
    }
    pub fn eval_card_effects(
        &mut self,
        effects: &[CardEffect],
        card: CardReference,
        probability: &mut Probability,
    ) {
        for effect in effects {
            self.eval_card_effect(effect, card, probability);
        }
    }

    fn eval_card_effect(
        &mut self,
        effect: &CardEffect,
        card: CardReference,
        probability: &mut Probability,
    ) {
        match effect {
            CardEffect::AutoPlay => {
                let binding = Binding::Card(card);
                let target = if self.eval_condition(&card.base.targeted, binding, None) {
                    self.random_monster(probability)
                } else {
                    None
                };
                if !self.end_turn {
                    self.play_card(card, target, false, probability);
                }
            }
            CardEffect::CopyTo { destination, then } => {
                let battle = self;
                let card = battle.get_card(card).duplicate();
                let card_ref = battle.add_card(card, *destination, probability);
                battle.eval_card_effects(then, card_ref, probability);
            }
            CardEffect::Custom(_name) => unimplemented!(),
            CardEffect::Discard => {
                self.discard_card(card, probability);
            }
            CardEffect::Exhaust => {
                self.exhaust_cards(vec![card], probability);
            }
            CardEffect::If { condition, then } => {
                if self.eval_condition(condition, Binding::Card(card), None) {
                    self.eval_card_effects(then, card, probability)
                }
            }
            CardEffect::MoveTo(destination) => {
                self.move_card(*destination, card, probability);
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
                self.get_card_mut(card).retain = true;
            }
            CardEffect::Scry => {
                if !self.discard.contains(&card.uuid) {
                    self.move_card(CardDestination::DiscardPile, card, probability);
                }
            }
            CardEffect::Upgrade => {
                self.get_card_mut(card).upgrade();
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

    pub fn play_card(
        &mut self,
        card: CardReference,
        target: Option<MonsterReference>,
        use_energy: bool,
        probability: &mut Probability,
    ) {
        self.move_out(card);
        for effect in &card.base.on_play {
            self.eval_effect(
                effect,
                Binding::Card(card),
                Some(GameAction {
                    is_attack: card.base._type == CardType::Attack,
                    creature: CreatureReference::Player,
                    target: target.map(|a| a.creature_ref()),
                    monster_move: None,
                }),
                probability,
            )
        }

        todo!("When play cards");

        if !self.exhaust.contains(&card.uuid) {
            self.discard.push_back(card.uuid);
        }

        self.move_out(card);
        let (cost, card_type) = {
            let card = self.get_card_mut(card);
            let cost = card.cost;
            card.cost = card.base_cost;
            (cost, card.base._type)
        };

        if use_energy && cost > 0 {
            if card_type == CardType::Attack && self.player.has_buff(buffs::FREE_ATTACK_POWER) {
                self.player.add_buff(buffs::FREE_ATTACK_POWER, -1);
            } else {
                self.energy -= cost;
            }
        }
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
        let removed = self.monsters.remove(&uuid).unwrap();
        for (_, monster) in self.monsters.iter_mut() {
            if monster.position > removed.position {
                monster.position -= 1;
            }
        }
        removed
    }

    fn add_monster(
        &mut self,
        base: &'static BaseMonster,
        position: usize,
        probability: &mut Probability,
    ) -> MonsterReference {
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
        todo!("When BeforeHandDiscard");
        let has_runic_pyramid = self.game_state.has_relic(relics::RUNIC_PYRAMID);
        for card_ref in self.hand().collect_vec() {
            let binding = Binding::Card(card_ref);
            if self.get_card(card_ref).retain {
                self.eval_effects(&card_ref.base.on_retain, binding, None, probability);
                if !self.eval_condition(&card_ref.base.retain, binding, None) {
                    self.get_card_mut(card_ref).retain = false;
                }
            } else if !has_runic_pyramid {
                self.move_card(CardDestination::DiscardPile, card_ref, probability);
            }
            self.eval_effects(&card_ref.base.on_turn_end, binding, None, probability);
        }

        let passives = self
            .orbs
            .iter()
            .map(|a| a.base)
            .enumerate()
            .filter(|(_, a)| *a != OrbType::Plasma)
            .collect_vec();

        for (index, orb) in passives {
            self.trigger_passive(orb, index, probability);
        }

        todo!("When BeforeEnemyMove");

        for (_, monster) in self.monsters.iter_mut().sorted_by_key(|(_, a)| a.index) {
            if !monster.creature.has_buff(buffs::BARRICADE) {
                monster.creature.block = 0;
            }
        }

        for monster in self.available_monsters().collect_vec() {
            self.next_monster_move(monster, probability);
        }
        todo!("When AfterEnemyMove");
        todo!("When TurnEnd");
        self.start_turn(false, probability);
    }

    fn start_turn(&mut self, combat_start: bool, probability: &mut Probability) {
        if !self.player.has_buff(buffs::BARRICADE) && !self.player.has_buff(buffs::BLUR) {
            if self.game_state.has_relic(relics::CALIPERS) {
                self.player.block = self.player.block.saturating_sub(15);
            } else {
                self.player.block = 0;
            }
        }

        todo!("When BeforeHandDraw");

        let mut cards_to_draw = 5;
        if self.game_state.has_relic(relics::SNECKO_EYE) {
            cards_to_draw += 2;
        }
        if combat_start && self.game_state.has_relic(relics::BAG_OF_PREPARATION) {
            cards_to_draw += 2;
        }
        let extra_cards = self.player.get_buff_amount(buffs::DRAW_CARD);
        cards_to_draw += extra_cards;
        if extra_cards != 0 {
            self.player.remove_buffs_by_type(buffs::DRAW_CARD);
        }
        self.draw_card(cards_to_draw as u8, probability);

        todo!("When AfterHandDraw");
    }

    fn next_monster_move(&mut self, monster: MonsterReference, probability: &mut Probability) {
        let current_move = if let Some(monster) = self.get_monster(monster) {
            let choices = monster.current_move_options.iter().copied().collect_vec();
            let choice = probability
                .choose_weighted(&choices)
                .expect("No current moves listed!");
            Some(*choice)
        } else {
            None
        };

        if let Some(current_move) = current_move {
            let reference = CreatureReference::Creature(monster);
            self.eval_effects(
                &current_move.effects,
                Binding::Creature(reference),
                Some(GameAction {
                    is_attack: false,
                    creature: reference,
                    target: Some(CreatureReference::Player),
                    monster_move: Some(current_move),
                }),
                probability,
            );
            self.next_move(monster, current_move, probability);
        }
    }

    fn next_move(
        &mut self,
        monster_ref: MonsterReference,
        performed_move: &'static MonsterMove,
        probability: &mut Probability,
    ) {
        if let Some((index, phase)) = self.get_monster_mut(monster_ref).map(|monster| {
            if let Some(last_move) = monster.last_move {
                if last_move == performed_move {
                    monster.last_move_count += 1;
                } else {
                    monster.last_move = Some(last_move);
                    monster.last_move_count = 1;
                }
            };
            (monster.index, monster.phase)
        }) {
            self.set_monster_move(index + 1, phase, monster_ref, probability);
        }
    }

    fn draw_card(&mut self, mut n: u8, probability: &mut Probability) -> Vec<CardReference> {
        let mut cards = vec![];
        while n > 0 {
            if self.hand.len() == 10 {
                break;
            }
            if self.draw.is_empty() {
                self.shuffle(probability);
            }

            if self.draw.is_empty() {
                break;
            }

            self.peek_top(n as usize, probability);

            let mut to_draw = self
                .draw_top_known
                .split_off(self.draw_top_known.len().min(n as usize));
            std::mem::swap(&mut to_draw, &mut self.draw_top_known); // Split off splits the wrong way, so we swap the two vectors

            n -= to_draw.len() as u8;

            for uuid in to_draw {
                let reference = self
                    .cards
                    .get(&uuid)
                    .unwrap()
                    .reference(CardLocation::DrawPile);
                self.draw.remove(&uuid).unwrap();
                cards.push(reference);
            }
        }

        for card in cards.iter() {
            self.eval_effects(&card.base.on_draw, Binding::Card(*card), None, probability);
            todo!("When DrawCard");
        }

        cards
    }

    fn get_cards_in_location(
        &mut self,
        location: CardLocation,
        position: RelativePosition,
        probability: &mut Probability,
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
        probability: &mut Probability,
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
        if self.orbs.len() == self.orb_slots as usize {
            self.evoke_orb(1, probability);
        }

        let n = match orb_type {
            OrbType::Any => panic!("Unexpected Any orb type"),
            OrbType::Frost => {
                self.blizzard += 1;
                0
            }
            OrbType::Dark => {
                let focus = self.player.get_buff_amount(buffs::FOCUS);
                std::cmp::max(focus + 6, 0) as u16
            }
            _ => 0,
        };

        let orb = Orb { base: orb_type, n };

        self.orbs.push_back(orb);
    }

    fn add_block(
        &mut self,
        mut amount: u16,
        target: CreatureReference,
        from_card: bool,
        probability: &mut Probability,
    ) {
        if let Some(mut_creature) = self.get_creature_mut(target) {
            if from_card {
                amount = (amount as i16 + mut_creature.get_buff_amount(buffs::FOCUS)).clamp(0, 999)
                    as u16;
            }
            let new_block = std::cmp::min(mut_creature.block + amount, 999);
            mut_creature.block = new_block;
            todo!("When OnBlock");
        }
    }

    fn set_monster_phase(
        &mut self,
        phase: &str,
        monster: MonsterReference,
        probability: &mut Probability,
    ) {
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
                            self.damage(orb.n as u16, creature_ref, None, true, probability);
                        }
                    }
                }
                OrbType::Frost => {
                    let focus = self.player.get_buff_amount(buffs::FOCUS);
                    let block_amount = std::cmp::max(focus + 5, 0) as u16;

                    for _ in 0..times {
                        self.add_block(block_amount, CreatureReference::Player, false, probability);
                    }
                }
                OrbType::Lightning => {
                    let has_electro_dynamics = self.player.has_buff(buffs::ELECTRO);
                    let focus = self.player.get_buff_amount(buffs::FOCUS);
                    let orb_damage = std::cmp::max(8 + focus, 0) as u16;
                    for _ in 0..times {
                        if has_electro_dynamics {
                            for monster in self.available_monsters().collect_vec() {
                                self.damage(
                                    orb_damage,
                                    monster.creature_ref(),
                                    None,
                                    true,
                                    probability,
                                );
                            }
                        } else {
                            let monsters = self.available_monsters().collect_vec();
                            if let Some(selected) = probability.choose(monsters) {
                                self.damage(
                                    orb_damage,
                                    selected.creature_ref(),
                                    None,
                                    true,
                                    probability,
                                );
                            }
                        }
                    }
                }
                OrbType::Plasma => self.energy += 2 * times,
            }
        }
    }

    fn attack_damage(
        &mut self,
        mut amount: i16,
        times: u16,
        creatures: Vec<CreatureReference>,
        attacker: CreatureReference,
        probability: &mut Probability,
    ) -> (bool, u16) {
        if let Some(creature) = self.get_creature_mut(attacker) {
            let vigor_amount = creature.get_buff_amount(buffs::VIGOR);
            amount += vigor_amount;
            if amount == 0 {
                creature.remove_buffs_by_type(buffs::VIGOR)
            }

            amount += creature.get_buff_amount(buffs::STRENGTH);

            if creature.is_player() {
                match self.stance {
                    Stance::Wrath => amount *= 2,
                    Stance::Divinity => amount *= 3,
                    _ => {}
                }
            }
        }

        let mut is_fatal = false;
        let mut total_amount = 0;

        for creature in creatures {
            for _ in 0..times {
                if !is_fatal {
                    let (fatal, a) =
                        self.damage(amount as u16, creature, Some(attacker), false, probability);
                    is_fatal = fatal;

                    total_amount += a;
                }
            }
        }

        (is_fatal, total_amount)
    }

    fn damage(
        &mut self,
        amount: u16,
        creature_ref: CreatureReference,
        attacker: Option<CreatureReference>,
        is_orb: bool,
        probability: &mut Probability,
    ) -> (bool, u16) {
        let hp_loss = {
            if let Some(creature) = self.get_creature(creature_ref) {
                let mut multiplier = 1.0;
                if let Some(attacker) = attacker {
                    if creature.has_buff(buffs::VULNERABLE) {
                        if creature.is_player() && self.game_state.has_relic(relics::ODD_MUSHROOM) {
                            multiplier += 0.25;
                        } else if !creature.is_player()
                            && self.game_state.has_relic(relics::PAPER_PHROG)
                        {
                            multiplier += 0.75;
                        } else {
                            multiplier += 0.5;
                        }
                    }
                    if let Some(attacker_creature) = self.get_creature(attacker) {
                        if attacker_creature.has_buff(buffs::WEAK) {
                            if creature.is_player()
                                && self.game_state.has_relic(relics::PAPER_KRANE)
                            {
                                multiplier -= 0.4;
                            } else {
                                multiplier -= 0.25;
                            }
                        }
                    }
                }

                if is_orb && creature.has_buff(buffs::LOCK_ON) {
                    multiplier += 0.5;
                }

                multiplier += 0.1 * creature.get_buff_amount(buffs::SLOW) as f64;

                let mut full_amount = (amount as f64 * multiplier).floor() as u16;

                if creature.has_buff(buffs::INTANGIBLE) {
                    full_amount = 1;
                }

                let blocked_amount = full_amount.saturating_sub(creature.block);
                let mut unblocked_amount = full_amount - blocked_amount;

                if unblocked_amount > 0 {
                    if creature.is_player() {
                        if unblocked_amount <= 5 && self.game_state.has_relic(relics::TORII) {
                            unblocked_amount = 1;
                        }

                        if self.game_state.has_relic(relics::TUNGSTEN_ROD) {
                            unblocked_amount -= 1;
                        }
                    } else if unblocked_amount < 5 && self.game_state.has_relic(relics::THE_BOOT) {
                        unblocked_amount = 5;
                    }
                }

                self.get_creature_mut(creature_ref).unwrap().block -= blocked_amount;

                unblocked_amount
            } else {
                0
            }
        };

        if hp_loss > 0 {
            self.lose_hp(hp_loss, creature_ref, true, probability)
        } else {
            (false, 0)
        }
    }

    fn lose_hp(
        &mut self,
        mut amount: u16,
        creature_ref: CreatureReference,
        ignore_intangible: bool,
        probability: &mut Probability,
    ) -> (bool, u16) {
        let new_hp = {
            if let Some(creature) = self.get_creature_mut(creature_ref) {
                if !ignore_intangible && creature.has_buff(buffs::INTANGIBLE) {
                    amount = std::cmp::max(amount, 1);
                }

                if let Some(buff) = creature.get_singular_buff_mut(buffs::INVINCIBLE) {
                    amount = std::cmp::min(amount, buff.vars.x as u16);
                    buff.vars.x -= amount as i16;
                }

                if let Some(buff) = creature.get_singular_buff_mut(buffs::MODE_SHIFT) {
                    buff.vars.x.saturating_sub(amount as i16);
                    if buff.vars.x == 0 {
                        creature.remove_buffs_by_type(buffs::MODE_SHIFT);
                        unimplemented!("Switch mode!")
                    }
                }

                creature.hp.amount = creature.hp.amount.saturating_sub(amount);
                creature.hp.amount
            } else {
                return (false, 0);
            }
        };

        if amount > 0 && creature_ref == CreatureReference::Player {
            self.hp_loss_count += 1;
        }

        if new_hp == 0 {
            (self.die(creature_ref, probability), amount)
        } else {
            (false, amount)
        }
    }

    fn die(&mut self, creature_ref: CreatureReference, probability: &mut Probability) -> bool {
        match creature_ref {
            CreatureReference::Player => {
                let recovery: f64 =
                    if let Some(potion_ref) = self.game_state.find_potion("Fairy In A Bottle") {
                        self.game_state.potions[potion_ref.index] = None;
                        if self.game_state.has_relic(relics::SACRED_BARK) {
                            0.6
                        } else {
                            0.3
                        }
                    } else if let Some(relic) = self.game_state.get_relic_mut(relics::LIZARD_TAIL) {
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
                todo!("When OnDie");

                let monster_name = monster_ref.base.name.as_str();

                let dies = match monster_name {
                    "Awakened One" => {
                        let monster = self.get_monster_mut(monster_ref).unwrap();
                        if monster.vars.x == 0 {
                            monster.vars.x = 1;
                            monster.targetable = false;
                            monster.creature.hp.amount = 0;
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
                            let monster_mut = self.get_monster_mut(monster_ref).unwrap();
                            monster_mut.targetable = false;
                            monster_mut.creature.hp.amount = 0;
                            false
                        }
                    }
                    "Bronze Orb" => {
                        if let Some(card) = self
                            .get_creature(creature_ref)
                            .and_then(|a| a.get_singular_buff(buffs::STASIS))
                            .and_then(|b| b.card_stasis)
                        {
                            self.move_in(card, CardDestination::PlayerHand, probability);
                        }
                        true
                    }
                    "Mugger" | "Looter" => {
                        self.gold_recovered += self.get_monster(monster_ref).unwrap().vars.x as u16;
                        true
                    }
                    _ => true,
                };

                if dies {
                    self.remove_monster(monster_ref.uuid);

                    if self.monsters.is_empty() {
                        self.combat_end(probability);
                    }
                }

                dies
            }
        }
    }

    pub fn heal_creature(&mut self, amount: f64, creature_ref: CreatureReference) {
        match creature_ref {
            CreatureReference::Player => self.heal(amount),
            CreatureReference::Creature(monster_ref) => {
                let monster = self.get_monster_mut(monster_ref).unwrap();
                monster.targetable = true;
                monster.creature.hp.add(amount);
            }
        };
    }

    pub fn drink_potion(
        &mut self,
        potion: PotionReference,
        target: Option<MonsterReference>,
        probability: &mut Probability,
    ) {
        self.eval_effects(
            &potion.base.on_drink,
            Binding::Potion(potion),
            Some(GameAction {
                creature: CreatureReference::Player,
                is_attack: false,
                target: target.map(|a| a.creature_ref()),
                monster_move: None,
            }),
            probability,
        );

        if self.game_state.has_relic(relics::TOY_ORNITHOPTER) {
            self.heal(5.0);
        }

        if potion.base.name == "Entropic Brew" {
            self.game_state.drink_potion(potion, false, probability);
        }
    }

    pub fn trigger_passive(
        &mut self,
        orb: OrbType,
        orb_index: usize,
        probability: &mut Probability,
    ) {
        let focus = self.player.get_buff_amount(buffs::FOCUS);
        match orb {
            OrbType::Any => panic!("Unexpected any type of orb"),
            OrbType::Dark => self.orbs.get_mut(orb_index).unwrap().n += (6 + focus).max(0) as u16,
            OrbType::Frost => self.player.block += (2 + focus).max(0) as u16,
            OrbType::Lightning => {
                let amount = (3 + focus).max(0) as u16;
                if self.player.has_buff(buffs::ELECTRO) {
                    for creature in self.available_creatures().collect_vec() {
                        self.damage(amount, creature, None, true, probability);
                    }
                } else {
                    let creatures = self.available_creatures().collect_vec();
                    if let Some(creature) = probability.choose(creatures) {
                        self.damage(amount, creature, None, true, probability);
                    }
                }
            }
            OrbType::Plasma => {
                self.energy += 1;
            }
        }
    }

    pub fn heal(&mut self, mut amount: f64) {
        if self.game_state.has_relic(relics::MAGIC_FLOWER) {
            amount *= 1.5;
        }

        self.game_state.heal(amount)
    }

    pub fn add_card(
        &mut self,
        mut card: Card,
        destination: CardDestination,
        probability: &mut Probability,
    ) -> CardReference {
        if self.player.has_buff(buffs::MASTER_REALITY) {
            if card.base.name == "Searing Blow" {
                card.upgrades = 2;
            }
            card.upgrades = 1;
        }

        let cost = match card.base.name.as_str() {
            "Blood for Blood" => 4_u8.saturating_sub(self.hp_loss_count),
            "Eviscerate" => 3_u8.saturating_sub(self.discard_count),
            "Force Field" => 4_u8.saturating_sub(self.power_count),
            _ => card.cost,
        };

        card.cost = card.cost.min(cost);

        let reference = card.reference(destination.location());
        let uuid = card.uuid;
        self.cards.insert(uuid, card);
        self.move_in(uuid, destination, probability);
        reference
    }

    fn exhaust_cards(&mut self, cards: Vec<CardReference>, probability: &mut Probability) {
        for card in cards {
            if !self.exhaust.contains(&card.uuid) {
                self.move_card(CardDestination::ExhaustPile, card, probability);
                self.eval_effects(
                    &card.base.on_exhaust,
                    Binding::Card(card),
                    None,
                    probability,
                );
            }
        }
    }

    fn discard_card(&mut self, card: CardReference, probability: &mut Probability) {
        if !self.discard.contains(&card.uuid) {
            self.move_card(CardDestination::DiscardPile, card, probability);
            self.eval_effects(
                &card.base.on_discard,
                Binding::Card(card),
                None,
                probability,
            );
            self.discard_count += 1;
        }
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
        (card.cost <= self.energy
            || (card.base._type == CardType::Attack
                && self.player.has_buff(buffs::FREE_ATTACK_POWER)))
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
            CardLocation::DiscardPile => self
                .discard
                .index_of(&card.uuid)
                .map(|i| self.discard.remove(i)),
            CardLocation::DrawPile => {
                if let Some(index) = self.draw_top_known.iter().position(|a| a == &card.uuid) {
                    self.draw_top_known.remove(index);
                } else if let Some(index) =
                    self.draw_bottom_known.iter().position(|a| a == &card.uuid)
                {
                    self.draw_bottom_known.remove(index);
                } else if let Some(index) = self.draw_inserted.iter().position(|a| a == &card.uuid)
                {
                    self.draw_inserted.remove(index);
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
                        } else if self.draw_top_known.contains(&card)
                            || self.draw_bottom_known.contains(&card)
                        {
                            self.draw_inserted.push_back(card);
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
            .and_then(|f| f.get_buff(buff))
    }

    pub fn get_buff_mut(&mut self, buff: BuffReference) -> Option<&mut Buff> {
        self.get_creature_mut(buff.creature)
            .and_then(|f| f.get_buff_mut(buff))
    }

    pub fn get_creature(&self, creature: CreatureReference) -> Option<&Creature> {
        match creature {
            CreatureReference::Player => Some(&self.player),
            CreatureReference::Creature(monster) => self.get_monster(monster).map(|m| &m.creature),
        }
    }
    pub fn get_creature_mut(&mut self, creature: CreatureReference) -> Option<&mut Creature> {
        match creature {
            CreatureReference::Player => Some(&mut self.player),
            CreatureReference::Creature(monster) => {
                self.get_monster_mut(monster).map(|m| &mut m.creature)
            }
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

        let mut remaining_choices: Vector<Uuid> = self
            .draw
            .clone()
            .difference(self.draw_top_known.iter().copied().collect())
            .difference(self.draw_bottom_known.iter().copied().collect())
            .difference(self.draw_inserted.iter().copied().collect())
            .into_iter()
            .collect();

        let mut new_top = vector![];

        for _ in 0..n {
            if !self.draw_inserted.is_empty() {
                let chosen = probability.range(self.draw.len());
                if chosen < self.draw_inserted.len() {
                    new_top.push_back(self.draw_inserted.remove(chosen));
                    continue;
                }
            }

            if let Some(top) = self.draw_top_known.pop_front() {
                new_top.push_back(top);
                continue;
            }

            if !remaining_choices.is_empty() {
                let choice = probability.range(remaining_choices.len());
                new_top.push_back(remaining_choices.remove(choice))
            } else {
                break;
            }
        }

        self.draw_top_known.extend(new_top);
        if remaining_choices.is_empty() {
            while let Some(card) = self.draw_bottom_known.pop_back() {
                self.draw_top_known.push_back(card)
            }
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
            _ => vec![target.creature_ref(binding, action)],
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
