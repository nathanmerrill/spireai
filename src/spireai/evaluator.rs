use crate::models::cards::BaseCard;
use crate::models::monsters::{BaseMonster, MonsterMove, Move};
use crate::models::potions::BasePotion;
use crate::models::relics::BaseRelic;
use crate::models::{self, core::*, monsters::Intent, relics::Activation};
use crate::state::battle::BattleState;
use crate::state::core::{Card, CardOffer, Creature, Monster, Orb, Potion, Relic};
use crate::state::game::{CardChoiceState, DeckCard, FloorState, GameState, Reward, ScreenState};
use crate::state::probability::Probability;
use im::{vector, HashMap, Vector};
use itertools::Itertools;
use uuid::Uuid;

use super::references::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GameAction {
    pub is_attack: bool,
    pub creature: CreatureReference,
    pub target: Option<CreatureReference>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct GamePossibility {
    pub state: GameState,
    pub probability: Probability,
}

impl GamePossibility {
    fn eval_card_effects(&mut self, effects: &[CardEffect], card: CardReference) {
        for effect in effects {
            self.eval_card_effect(effect, card);
        }
    }

    fn eval_card_effect(&mut self, effect: &CardEffect, card: CardReference) {
        match effect {
            CardEffect::AutoPlay => {
                let binding = Binding::Card(card);
                let target = if self.eval_condition(&card.base.targeted, binding, None) {
                    self.state
                        .floor_state
                        .battle()
                        .random_monster(&mut self.probability)
                        .map(|a| a.creature_ref())
                } else {
                    None
                };
                self.play_card(card, target);
            }
            CardEffect::CopyTo { destination, then } => {
                let new_card = self.state.floor_state.battle().get_card(card).duplicate();
                let card_ref = self.add_card(new_card, *destination);
                self.eval_card_effects(then, card_ref);
            }
            CardEffect::Custom => panic!("Unexpected custom card effect"),
            CardEffect::Discard => {
                if !self.state.floor_state.battle().discard.contains(&card.uuid) {
                    self.state.floor_state.battle_mut().move_card(
                        CardDestination::DiscardPile,
                        card,
                        &mut self.probability,
                    );
                    self.eval_effects(&card.base.on_discard, Binding::Card(card), None);
                    self.state.floor_state.battle_mut().discard_count += 1;
                }
            }
            CardEffect::Exhaust => {
                if !self.state.floor_state.battle().exhaust.contains(&card.uuid) {
                    self.state.floor_state.battle_mut().move_card(
                        CardDestination::ExhaustPile,
                        card,
                        &mut self.probability,
                    );
                    self.eval_effects(&card.base.on_exhaust, Binding::Card(card), None);
                }
            }
            CardEffect::MoveTo(destination) => {
                self.state.floor_state.battle_mut().move_card(
                    *destination,
                    card,
                    &mut self.probability,
                );
            }
            CardEffect::ReduceCost(amount) => {
                let reduction = self.eval_amount(amount, Binding::Card(card));
                let card = self.state.floor_state.battle_mut().get_card_mut(card);
                if card.base.cost != Amount::X {
                    card.cost = std::cmp::max(card.cost as i16 - reduction, 0) as u8;
                    card.base_cost = std::cmp::max(card.base_cost as i16 - reduction, 0) as u8;
                }
            }
            CardEffect::Retain => {
                self.state
                    .floor_state
                    .battle_mut()
                    .get_card_mut(card)
                    .retain = true;
            }
            CardEffect::Upgrade => {
                self.state
                    .floor_state
                    .battle_mut()
                    .get_card_mut(card)
                    .upgrade();
            }
            CardEffect::ZeroCombatCost => {
                let card = self.state.floor_state.battle_mut().get_card_mut(card);
                card.cost = 0;
                card.base_cost = 0;
            }
            CardEffect::ZeroCostUntilPlayed => {
                let card = self.state.floor_state.battle_mut().get_card_mut(card);
                card.cost = 0;
                card.cost_until_played = true;
            }
            CardEffect::ZeroTurnCost => {
                let card = self.state.floor_state.battle_mut().get_card_mut(card);
                card.cost = 0;
            }
        }
    }

    pub fn play_card(&mut self, card: CardReference, target: Option<CreatureReference>) {
        for effect in &card.base.on_play {
            self.eval_effect(
                effect,
                Binding::Card(card),
                Some(GameAction {
                    is_attack: card.base._type == CardType::Attack,
                    creature: CreatureReference::Player,
                    target,
                }),
            )
        }

        self.eval_when(When::PlayCard(CardType::All));
        self.eval_when(When::PlayCard(card.base._type));
    }

    pub fn add_card(&mut self, card: Card, destination: CardDestination) -> CardReference {
        self.state
            .floor_state
            .battle_mut()
            .new_card(card, destination, &mut self.probability)
    }

    pub fn eval_effects(
        &mut self,
        effects: &'static [Effect],
        binding: Binding,
        action: Option<GameAction>,
    ) {
        for effect in effects {
            self.eval_effect(effect, binding, action);
        }
    }

    fn eval_effect(
        &mut self,
        effect: &'static Effect,
        binding: Binding,
        action: Option<GameAction>,
    ) {
        match effect {
            Effect::AddBuff {
                buff: buff_name,
                amount: buff_amount,
                target,
            } => {
                let amount = self.eval_amount(buff_amount, binding);
                for creature in target.to_creatures(
                    binding,
                    action,
                    self.state.floor_state.battle(),
                    &mut self.probability,
                ) {
                    if let Some(creature) = self.state.get_creature_mut(creature) {
                        creature.add_buff(buff_name, amount);
                    }
                }
            }
            Effect::AddEnergy(energy_amount) => {
                let amount = self.eval_amount(energy_amount, binding) as u8;
                self.state.floor_state.battle_mut().energy += amount
            }
            Effect::AddGold(gold_amount) => {
                let amount = self.eval_amount(gold_amount, binding) as u16;
                self.state.add_gold(amount)
            }
            Effect::AddMaxHp(hp_amount) => {
                let amount = self.eval_amount(hp_amount, binding) as u16;
                self.state.player.add_max_hp(amount, &self.state.relics)
            }
            Effect::AddN(n_amount) => {
                let amount = self.eval_amount(n_amount, binding);
                binding.get_mut_vars(&mut self.state).n += amount;
            }
            Effect::AddOrbSlot(amount) => {
                let count = self.eval_amount(amount, binding) as u8;
                self.state.floor_state.battle_mut().orb_slots =
                    std::cmp::min(count + self.state.floor_state.battle().orb_slots, 10)
                        - self.state.floor_state.battle().orb_slots;
            }
            Effect::AddPotionSlot(amount) => {
                for _ in 0..*amount {
                    self.state.potions.push_back(None)
                }
            }
            Effect::AddRelic(name) => {
                self.add_relic(models::relics::by_name(name));
            }
            Effect::AddX(amount) => {
                binding.get_mut_vars(&mut self.state).x += self.eval_amount(amount, binding);
            }
            Effect::AttackDamage {
                amount,
                target,
                if_fatal,
                times,
            } => {
                let attack_amount = self.eval_amount(amount, binding);

                for creature in target.to_creatures(
                    binding,
                    action,
                    self.state.floor_state.battle(),
                    &mut self.probability,
                ) {
                    for _ in 0..self.eval_amount(times, binding) {
                        if self.damage(
                            attack_amount as u16,
                            creature,
                            Some(action.unwrap().creature),
                            false,
                        ) {
                            self.eval_effects(if_fatal, binding, action);
                        }
                    }
                }
            }
            Effect::Block { amount, target } => {
                let block_amount = self.eval_amount(amount, binding) as u16;

                for creature in target.to_creatures(
                    binding,
                    action,
                    self.state.floor_state.battle(),
                    &mut self.probability,
                ) {
                    if let Some(creature) = self.state.get_creature_mut(creature) {
                        let new_block = std::cmp::min(creature.block + block_amount, 999);
                        creature.block = new_block;
                    }
                }
            }
            Effect::ChannelOrb(orb_type) => self.channel_orb(*orb_type),
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

                let choice =
                    self.random_cards_by_type(amount, *class, *_type, *rarity, *exclude_healing);

                let mut card_choices = Vector::new();
                for base_card in choice {
                    card_choices.push_back(Card::new(base_card));
                }

                let mut effects = vector![CardEffect::MoveTo(*destination)];
                effects.extend(then.clone());

                self.state.screen_state = ScreenState::CardChoose(CardChoiceState {
                    choices: card_choices
                        .iter()
                        .map(|card| card.reference(CardLocation::None))
                        .collect(),
                    count_range: (amount..amount + 1),
                    then: effects,
                });

                for card in card_choices {
                    self.state
                        .floor_state
                        .battle_mut()
                        .cards
                        .insert(card.uuid, card);
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
                    CardLocation::DiscardPile => {
                        self.state.floor_state.battle().discard().collect()
                    }
                    CardLocation::DrawPile => self.state.floor_state.battle().draw().collect(),
                    CardLocation::ExhaustPile => {
                        self.state.floor_state.battle().exhaust().collect()
                    }
                    CardLocation::PlayerHand => self.state.floor_state.battle().hand().collect(),
                    CardLocation::None => panic!("Cannot choose from None!"),
                };

                self.state.screen_state = ScreenState::CardChoose(CardChoiceState {
                    choices,
                    count_range: (min_count..max_count + 1),
                    then: Vector::from(then),
                });
            }
            Effect::CreateCard {
                name,
                destination,
                then,
            } => {
                let card = Card::by_name(name);
                let card_ref = self.add_card(card, *destination);
                self.eval_card_effects(then, card_ref);
            }

            Effect::CreateCardByType {
                destination,
                _type,
                rarity,
                class,
                exclude_healing,
                then,
            } => {
                let card =
                    self.random_cards_by_type(1, *class, *_type, *rarity, *exclude_healing)[0];
                let card = self.add_card(Card::new(card), *destination);
                self.eval_card_effects(then, card);
            }
            Effect::Custom => unimplemented!(),
            Effect::Damage { amount, target } => {
                let total = self.eval_amount(amount, binding) as u16;
                let creature = target.to_creature(binding, action);
                self.damage(total, creature, None, false);
            }
            Effect::DeckAdd(_) => unimplemented!(),
            Effect::DeckOperation {
                random,
                count,
                operation,
            } => unimplemented!(),
            Effect::Die { target } => {
                let creature = target.to_creature(binding, action);
                self.die(creature);
            }
            Effect::DoCardEffect {
                location,
                position,
                effect,
            } => {
                for card in self.get_cards_in_location(*location, *position) {
                    self.eval_card_effect(effect, card)
                }
            }
            Effect::Draw(amount) => {
                let n = self.eval_amount(amount, binding);
                self.draw(n as u8);
            }
            Effect::EvokeOrb(amount) => self.evoke_orb(self.eval_amount(amount, binding) as u8),
            Effect::Fight { monsters, room } => {
                self.fight(monsters, *room);
            }
            Effect::Heal { amount, target } => {
                let total = self.eval_amount(amount, binding);
                for creature in target.to_creatures(
                    binding,
                    action,
                    self.state.floor_state.battle(),
                    &mut self.probability,
                ) {
                    self.heal(total as f64, creature);
                }
            }
            Effect::HealPercentage { amount, target } => {
                let percentage = self.eval_amount(amount, binding) as f64 / 100.0;
                for creature in target.to_creatures(
                    binding,
                    action,
                    self.state.floor_state.battle(),
                    &mut self.probability,
                ) {
                    let total =
                        self.state.get_creature(creature).unwrap().max_hp as f64 * percentage;
                    self.heal(total, creature);
                }
            }
            Effect::If {
                condition,
                then,
                _else,
            } => {
                if self.eval_condition(condition, binding, action) {
                    self.eval_effects(then, binding, action);
                } else {
                    self.eval_effects(_else, binding, action);
                }
            }
            Effect::LoseAllGold => self.state.gold = 0,
            Effect::LoseHp { amount, target } => {
                let total = self.eval_amount(amount, binding);
                for creature in target.to_creatures(
                    binding,
                    action,
                    self.state.floor_state.battle(),
                    &mut self.probability,
                ) {
                    self.lose_hp(total as u16, creature, false);
                }
            }
            Effect::LoseHpPercentage(amount) => {
                let percentage = self.eval_amount(amount, binding) as f64 / 1000.0;
                let damage = (self.state.player.creature.max_hp as f64 * percentage).floor() as u16;
                self.lose_hp(damage, CreatureReference::Player, false);
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

                let choice = self
                    .probability
                    .choose_weighted(&evaluated_chances)
                    .unwrap();

                self.eval_effects(choice, binding, action);
            }
            Effect::RandomPotion => {
                let potion = self.random_potion(self.state.floor_state.battle().active);
                self.state.add_potion(potion);
            }
            Effect::RandomRelic => {
                let relic = self.random_relic(None, None, None, false);
                self.add_relic(relic)
            }
            Effect::ReduceMaxHpPercentage(amount) => {
                let percentage = self.eval_amount(amount, binding);
                let total = (self.state.player.creature.max_hp as f64 * (percentage as f64 / 100.0))
                    .floor() as u16;
                self.state.player.reduce_max_hp(total);
            }
            Effect::RemoveDebuffs => {
                let creature_ref = binding.get_creature();
                if let Some(creature) = self.state.get_creature_mut(creature_ref) {
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
            }
            Effect::RemoveRelic(relic) => {
                self.state.relics.remove(relic);
            }
            Effect::Repeat { n, effect } => {
                let amount = self.eval_amount(n, binding);
                for _ in 0..amount {
                    self.eval_effects(effect, binding, action);
                }
            }
            Effect::ResetN => {
                let vars = binding.get_mut_vars(&mut self.state);
                vars.n = vars.n_reset;
            }
            Effect::Scry(count) => {
                let amount = self.eval_amount(count, binding);
                self.scry(amount as usize);
            }
            Effect::SelfEffect(effect) => {
                if let Binding::Card(card) = binding {
                    if effect != &CardEffect::Exhaust
                        || !self.state.relics.contains("Strange Spoon")
                        || self.probability.range(2) == 0
                    {
                        self.eval_card_effect(effect, card);
                    }
                } else {
                    panic!("SelfEffect on a non-card!")
                }
            }
            Effect::SetN(n) => {
                let amount = self.eval_amount(n, binding);
                let vars = binding.get_mut_vars(&mut self.state);
                vars.n = amount;
                vars.n_reset = amount;
            }
            Effect::SetStance(stance) => {
                self.state.floor_state.battle_mut().stance = *stance;
            }
            Effect::SetX(x) => {
                let amount = self.eval_amount(x, binding);
                let vars = binding.get_mut_vars(&mut self.state);
                vars.x = amount;
            }
            Effect::ShowChoices(choices) => {
                let event = self.state.floor_state.event_mut();
                event.available_choices = choices.clone();
            }
            Effect::ShowReward(rewards) => {
                self.state.floor_state = FloorState::Rewards(
                    rewards
                        .iter()
                        .map(|reward| match reward {
                            RewardType::ColorlessCard => Reward::CardChoice(
                                self.generate_card_rewards(FightType::Common, true),
                            ),
                            RewardType::EliteCard => {
                                Reward::CardChoice(self.generate_card_rewards(
                                    FightType::Elite { burning: false },
                                    false,
                                ))
                            }
                            RewardType::Gold { min, max } => {
                                let amount =
                                    self.probability.range((max - min) as usize) as u16 + min;
                                Reward::Gold(amount)
                            }
                            RewardType::RandomBook => {
                                let book = self
                                    .probability
                                    .choose(vec!["Necronomicon", "Enchiridion", "Nilry's Codex"])
                                    .unwrap();
                                Reward::Relic(Relic::by_name(book))
                            }
                            RewardType::RandomPotion => {
                                let base = self.random_potion(false);
                                Reward::Potion(Potion { base })
                            }
                            RewardType::RandomRelic => {
                                let base = self.random_relic(None, None, None, false);
                                Reward::Relic(Relic::new(base))
                            }
                            RewardType::Relic(rarity) => {
                                let base = self.random_relic(None, Some(*rarity), None, false);
                                Reward::Relic(Relic::new(base))
                            }
                            RewardType::RelicName(name) => Reward::Relic(Relic::by_name(name)),
                            RewardType::StandardCard => Reward::CardChoice(
                                self.generate_card_rewards(FightType::Common, false),
                            ),
                        })
                        .collect(),
                )
            }
            Effect::Shuffle => {
                let cards = self
                    .state
                    .floor_state
                    .battle()
                    .discard
                    .iter()
                    .copied()
                    .collect_vec();
                self.state.floor_state.battle_mut().draw.extend(cards);
                self.state.floor_state.battle_mut().discard.clear();
                self.shuffle();
            }
            Effect::Spawn { choices, count } => {
                let amount = self.eval_amount(count, binding);
                for _ in 0..amount {
                    let choice = self.probability.choose(choices.clone()).unwrap();
                    let base = models::monsters::by_name(&choice);
                    self.add_monster(base, 0);
                }
            }
            Effect::Split(left, right) => {
                if let Binding::Creature(CreatureReference::Creature(monster_ref)) = binding {
                    let monster = self.remove_monster(monster_ref.uuid);
                    let hp = monster.creature.hp;

                    let left_base = models::monsters::by_name(left);
                    let right_base = models::monsters::by_name(right);
                    let left_ref = self.add_monster(left_base, monster.position);
                    let right_ref = self.add_monster(right_base, monster.position + 1);

                    let left = self
                        .state
                        .get_creature_mut(left_ref.creature_ref())
                        .unwrap();
                    left.max_hp = hp;
                    left.hp = hp;

                    let right = self
                        .state
                        .get_creature_mut(right_ref.creature_ref())
                        .unwrap();
                    right.max_hp = hp;
                    right.hp = hp;
                } else {
                    panic!("Unepxected binding")
                }
            }
            Effect::Unbuff(buff) => {
                if let Some(creature) = self.state.get_creature_mut(binding.get_creature()) {
                    creature.remove_buff_by_name(buff);
                }
            }
        }
    }

    fn transform_card(&mut self, card: CardReference) {
        self.state.remove_card(card.uuid);

        let choices = models::cards::available_cards_by_class(card.base._class)
            .iter()
            .filter(|a| a.name != card.base.name)
            .collect();
        let new_card = self.probability.choose(choices).unwrap();
        self.state.add_card(Card::new(new_card));
    }

    fn remove_monster(&mut self, uuid: Uuid) -> Monster {
        let removed = self
            .state
            .floor_state
            .battle_mut()
            .monsters
            .remove(&uuid)
            .unwrap();
        for (_, monster) in self.state.floor_state.battle_mut().monsters.iter_mut() {
            if monster.position > removed.position {
                monster.position -= 1;
            }
        }
        removed
    }

    fn add_monster(&mut self, base: &'static BaseMonster, position: usize) -> MonsterReference {
        let hp_asc = match self.state.floor_state.battle().fight_type {
            FightType::Boss => 9,
            FightType::Elite { .. } => 8,
            FightType::Common => 7,
        };
        let hp_range = if self.state.asc < hp_asc {
            &base.hp_range
        } else {
            &base.hp_range_asc
        };

        let hp = self
            .probability
            .range((hp_range.max - hp_range.min) as usize) as u16
            + hp_range.min;

        let mut monster = Monster::new(base, hp);

        monster.position = position;

        let binding = Binding::Creature(CreatureReference::Creature(monster.monster_ref()));
        if let Some(range) = &base.n_range {
            let min = self.eval_amount(&range.min, binding);
            let max = self.eval_amount(&range.max, binding);
            let n = self.probability.range((max - min) as usize) as i16 + min;
            monster.vars.n = n;
            monster.vars.n_reset = n;
        }

        if let Some(range) = &base.x_range {
            let min = self.eval_amount(&range.min, binding);
            let max = self.eval_amount(&range.max, binding);
            let x = self.probability.range((max - min) as usize) as i16 + min;
            monster.vars.x = x;
        }

        self.eval_effects(&monster.base.on_create, binding, None);

        for (_, m) in self.state.floor_state.battle_mut().monsters.iter_mut() {
            if m.position >= position {
                m.position += 1;
            }
        }

        let monster_ref = MonsterReference {
            base,
            uuid: monster.uuid,
        };

        self.state
            .floor_state
            .battle_mut()
            .monsters
            .insert(monster.uuid, monster);

        monster_ref
    }

    fn generate_card_rewards(
        &mut self,
        fight_type: FightType,
        colorless: bool,
    ) -> Vector<CardOffer> {
        let cards = {
            if colorless {
                models::cards::available_cards_by_class(Class::None)
            } else if self.state.relics.contains("Prismatic Shard") {
                models::cards::available_cards_by_class(Class::All)
            } else {
                models::cards::available_cards_by_class(self.state.class)
            }
        };

        let count =
            1 + {
                if self.state.relics.contains("Busted Crown") {
                    0
                } else {
                    2
                }
            } + {
                if self.state.relics.contains("Question Card") {
                    1
                } else {
                    0
                }
            };

        self.generate_card_offers(Some(fight_type), cards, count, true)
    }

    fn generate_card_offers(
        &mut self,
        fight_type: Option<FightType>,
        available: &[&'static BaseCard],
        count: usize,
        reset_rarity: bool,
    ) -> Vector<CardOffer> {
        let mut cards = available.to_owned();

        (0..count)
            .map(|_| {
                let offer = self.generate_card_offer(fight_type, &cards);
                let index = cards.iter().position(|b| b == &offer.base).unwrap();
                cards.remove(index);
                match offer.base.rarity {
                    Rarity::Rare => {
                        if reset_rarity {
                            self.state.rare_probability_offset = 0;
                        }
                    }
                    Rarity::Common => {
                        self.state.rare_probability_offset =
                            std::cmp::min(self.state.rare_probability_offset + 1, 40);
                    }
                    _ => {}
                }
                offer
            })
            .collect()
    }

    pub fn generate_card_offer(
        &mut self,
        fight_type: Option<FightType>,
        available: &[&'static BaseCard],
    ) -> CardOffer {
        let has_nloth = self.state.relics.contains("N'loth's Gift");

        let rarity_probabilities = match fight_type {
            Some(FightType::Common) => {
                if has_nloth {
                    [
                        4 + self.state.rare_probability_offset,
                        37,
                        59 - self.state.rare_probability_offset,
                    ]
                } else if self.state.rare_probability_offset < 2 {
                    [
                        0,
                        35 + self.state.rare_probability_offset,
                        65 - self.state.rare_probability_offset,
                    ]
                } else {
                    [
                        self.state.rare_probability_offset - 2,
                        37,
                        65 - self.state.rare_probability_offset,
                    ]
                }
            }
            Some(FightType::Elite { .. }) => {
                if has_nloth {
                    if self.state.rare_probability_offset < 31 {
                        [
                            25 + self.state.rare_probability_offset,
                            40,
                            35 - self.state.rare_probability_offset,
                        ]
                    } else {
                        [
                            25 + self.state.rare_probability_offset,
                            75 - self.state.rare_probability_offset,
                            0,
                        ]
                    }
                } else {
                    [
                        5 + self.state.rare_probability_offset,
                        40,
                        55 - self.state.rare_probability_offset,
                    ]
                }
            }
            Some(FightType::Boss) => [100, 0, 0],
            None => [
                4 + self.state.rare_probability_offset,
                37,
                59 - self.state.rare_probability_offset,
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

        let rarity = *self
            .probability
            .choose_weighted(&[
                (Rarity::Rare, rare),
                (Rarity::Uncommon, uncommon),
                (Rarity::Common, common),
            ])
            .unwrap();

        let card = self
            .probability
            .choose(
                available
                    .iter()
                    .filter(|card| card.rarity == rarity)
                    .collect(),
            )
            .unwrap();

        let is_default_upgraded = match card._type {
            CardType::Attack => self.state.relics.contains("Molten Egg"),
            CardType::Skill => self.state.relics.contains("Toxic Egg"),
            CardType::Power => self.state.relics.contains("Frozen Egg"),
            _ => panic!("Unexpected card type!"),
        };

        let is_upgraded = is_default_upgraded || {
            let chance = match self.state.act {
                1 => 0,
                2 => 2,
                3 | 4 => 4,
                _ => panic!("Unexpected ascension"),
            } / if self.state.asc < 12 { 1 } else { 2 };

            *self
                .probability
                .choose_weighted(&[(true, chance), (false, 8 - chance)])
                .unwrap()
        };

        CardOffer {
            base: card,
            upgraded: is_upgraded,
        }
    }

    fn scry(&mut self, count: usize) {
        let cards = self.state.floor_state.battle().draw_top_known.take(count);

        let remaining_cards = count - cards.len();
        if remaining_cards > 0 {
            let mut to_draw = self.state.floor_state.battle().draw.clone();
            for card in cards {
                to_draw.remove(&card);
            }

            let additional_cards = self
                .probability
                .choose_multiple(to_draw.into_iter().collect(), remaining_cards);
            for card in additional_cards {
                self.state
                    .floor_state
                    .battle_mut()
                    .draw_top_known
                    .push_back(card);
            }
        }
        let battle = self.state.floor_state.battle();
        let mut choices = vec![];
        let mut unknown_cards = battle.draw.clone();
        for card in &battle.draw_top_known {
            unknown_cards.remove(card);
        }
        for card in &battle.draw_bottom_known {
            unknown_cards.remove(card);
        }

        let mut unknown_cards = unknown_cards.into_iter().collect_vec();

        for i in 0..count {
            if let Some(top) = battle.draw_top_known.get(i) {
                choices.push(*top);
            } else if let Some(bottom) = battle.draw_bottom_known.get(battle.draw.len() - i - 1) {
                choices.push(*bottom);
            } else {
                let choice = self.probability.range(unknown_cards.len());
                choices.push(unknown_cards.remove(choice));
            }
        }

        self.state.screen_state = ScreenState::CardChoose(CardChoiceState {
            count_range: (0..choices.len() + 1),
            choices: choices
                .into_iter()
                .map(|uuid| CardReference {
                    location: CardLocation::DrawPile,
                    uuid,
                    base: battle.cards[&uuid].base,
                })
                .collect(),
            then: vector![CardEffect::MoveTo(CardDestination::DiscardPile)],
        });
    }

    pub fn random_potion(&mut self, no_healing: bool) -> &'static BasePotion {
        let rarities = vec![
            (Rarity::Common, 70),
            (Rarity::Uncommon, 25),
            (Rarity::Rare, 5),
        ];

        let rarity = *self.probability.choose_weighted(&rarities).unwrap();

        let potions = models::potions::POTIONS
            .values()
            .filter(|a| a.rarity == rarity && !(no_healing && a.name == "Fruit Juice"))
            .collect_vec();

        self.probability.choose(potions).unwrap()
    }

    pub fn fight(&mut self, monsters: &[String], fight_type: FightType) {
        self.create_battle(monsters, fight_type);
        self.eval_when(When::CombatStart);
        self.start_turn(true);
    }

    fn create_battle(&mut self, monster_names: &[String], fight_type: FightType) {
        let cards: HashMap<Uuid, Card> = self
            .state
            .deck
            .values()
            .map(|c| (c.uuid, c.duplicate()))
            .collect();
        let draw_top = if self.state.relics.contains("Frozen Eye") {
            cards.values().map(|c| c.uuid).collect()
        } else {
            Vector::new()
        };

        let orb_slots = if self.state.class == Class::Defect {
            3
        } else if self.state.relics.contains("Prismatic Shard") {
            1
        } else {
            0
        };

        let mut energy = 3;

        for relic in self.state.relics.relics.values() {
            if relic.base.energy_relic {
                energy += 1;
            }
        }

        let mut monsters: HashMap<Uuid, Monster> = monster_names
            .iter()
            .map(|n| self.create_monster(n))
            .enumerate()
            .map(|(index, mut monster)| {
                monster.position = index;
                (monster.uuid, monster)
            })
            .collect();

        if let FightType::Elite { burning } = fight_type {
            let burning_type = if burning {
                self.probability.range(4)
            } else {
                4
            };
            let has_preserved_insect = self.state.relics.contains("Preserved Insect");
            if burning || has_preserved_insect {
                monsters.iter_mut().for_each(|(_, monster)| {
                    match burning_type {
                        0 => monster
                            .creature
                            .add_buff("Strength", (self.state.act + 1) as i16),
                        1 => {
                            let new_hp = monster.creature.max_hp + monster.creature.max_hp / 4;
                            monster.creature.max_hp = new_hp;
                            monster.creature.hp = new_hp;
                        }
                        2 => monster
                            .creature
                            .add_buff("Metallicize", (self.state.act * 2 + 2) as i16),
                        3 => monster
                            .creature
                            .add_buff("Regenerate", (self.state.act * 2 + 1) as i16),
                        4 => {}
                        _ => panic!("Unexpected burning type!"),
                    }
                    if has_preserved_insect {
                        monster.creature.hp = monster.creature.max_hp * 3 / 4;
                    }
                });
            }
        }

        let mut battle_state = BattleState {
            active: true,
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
            ..BattleState::new()
        };

        if let Some(relic) = self.state.relics.find_mut("Magic Flower") {
            relic.enabled = true;
        }

        for monster_ref in battle_state.available_monsters().collect_vec() {
            let creature_ref = monster_ref.creature_ref();
            if let Some(x) = monster_ref
                .base
                .x_range
                .as_ref()
                .map(|a| self.eval_range(a, creature_ref))
            {
                battle_state.get_monster_mut(monster_ref).unwrap().vars.x = x
            }
            if let Some(n) = monster_ref
                .base
                .n_range
                .as_ref()
                .map(|a| self.eval_range(a, creature_ref))
            {
                let monster = battle_state.get_monster_mut(monster_ref).unwrap();
                monster.vars.n = n;
                monster.vars.n_reset = n;
            }

            self.set_monster_move(0, 0, monster_ref);
        }

        self.state.floor_state = FloorState::Battle(battle_state);

        self.shuffle();
    }

    fn create_monster(&mut self, name: &str) -> Monster {
        let base = crate::models::monsters::by_name(name);
        let upgrade_asc = match base.fight_type {
            FightType::Common => 7,
            FightType::Elite { .. } => 8,
            FightType::Boss => 9,
        };

        let hp_range = if self.state.asc >= upgrade_asc {
            &base.hp_range_asc
        } else {
            &base.hp_range
        };

        let hp = self
            .probability
            .range((hp_range.max - hp_range.min + 1) as usize) as u16
            + hp_range.min;

        Monster::new(base, hp)
    }

    fn eval_range(
        &mut self,
        range: &crate::models::monsters::Range,
        creature: CreatureReference,
    ) -> i16 {
        let binding = Binding::Creature(creature);
        let min = self.eval_amount(&range.min, binding);
        let max = self.eval_amount(&range.max, binding);
        self.probability.range((max - min + 1) as usize) as i16 + min
    }

    pub fn end_turn(&mut self) {
        self.eval_when(When::BeforeHandDiscard);
        let has_runic_pyramid = self.state.relics.contains("Runic Pyramid");
        for card_ref in self.state.floor_state.battle().hand().collect_vec() {
            let binding = Binding::Card(card_ref);
            if self.state.floor_state.battle().get_card(card_ref).retain {
                self.eval_effects(&card_ref.base.on_retain, binding, None);
                if !self.eval_condition(&card_ref.base.retain, binding, None) {
                    self.state
                        .floor_state
                        .battle_mut()
                        .get_card_mut(card_ref)
                        .retain = false;
                }
            } else if !has_runic_pyramid {
                self.state.floor_state.battle_mut().move_card(
                    CardDestination::DiscardPile,
                    card_ref,
                    &mut self.probability,
                );
            }
            self.eval_effects(&card_ref.base.on_turn_end, binding, None);
        }
        self.eval_when(When::BeforeEnemyMove);
        {
            let battle_state = self.state.floor_state.battle_mut();
            for (_, monster) in battle_state
                .monsters
                .iter_mut()
                .sorted_by_key(|(_, a)| a.index)
            {
                if !monster.creature.has_buff("Barricade") {
                    monster.creature.block = 0;
                }
            }

            for monster in battle_state.available_monsters().collect_vec() {
                self.next_monster_move(monster);
            }
        }
        self.eval_when(When::AfterEnemyMove);
        self.eval_when(When::TurnEnd);
        self.start_turn(false);
    }

    fn start_turn(&mut self, combat_start: bool) {
        if !self.state.player.creature.has_buff("Barricade")
            && !self.state.player.creature.has_buff("Blur")
        {
            if self.state.relics.contains("Calipers") {
                self.state.player.creature.block =
                    self.state.player.creature.block.saturating_sub(15);
            } else {
                self.state.player.creature.block = 0;
            }
        }

        self.eval_when(When::BeforeHandDraw);

        let mut cards_to_draw = 5;
        if self.state.relics.contains("Snecko Eye") {
            cards_to_draw += 2;
        }
        if combat_start && self.state.relics.contains("Bag of Preparation") {
            cards_to_draw += 2;
        }
        if let Some(buff) = self.state.player.creature.find_buff("Draw Card") {
            cards_to_draw += buff.vars.x;
            self.state.player.creature.remove_buff_by_name("Draw Card");
        }
        self.draw(cards_to_draw as u8);

        self.eval_when(When::AfterHandDraw);
    }

    fn next_monster_move(&mut self, monster: MonsterReference) {
        let current_move =
            if let Some(monster) = self.state.floor_state.battle().get_monster(monster) {
                let choices = monster.current_move_options.iter().copied().collect_vec();
                let choice = self
                    .probability
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
            );
            self.next_move(monster, current_move);
        }
    }

    fn next_move(&mut self, monster_ref: MonsterReference, performed_move: &'static MonsterMove) {
        if let Some((index, phase)) = self
            .state
            .floor_state
            .battle_mut()
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
            self.set_monster_move(index + 1, phase, monster_ref);
        }
    }

    fn draw(&mut self, mut n: u8) {
        let mut cards = vec![];
        while n > 0 {
            if self.state.floor_state.battle().hand.len() == 10 {
                break;
            }
            if self.state.floor_state.battle().draw.is_empty() {
                self.shuffle();
            }

            let battle = self.state.floor_state.battle_mut();

            if battle.draw.is_empty() {
                break;
            }

            battle.peek_top(n, &mut self.probability);

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
            self.eval_effects(&card.base.on_draw, Binding::Card(card), None);
            self.eval_target_when(When::DrawCard(card.base._type), CreatureReference::Player);
            self.eval_target_when(When::DrawCard(CardType::All), CreatureReference::Player);
        }
    }

    fn shuffle(&mut self) {
        if self.state.relics.contains("Frozen Eye") {
            unimplemented!();
        } else {
            self.state.floor_state.battle_mut().draw_top_known = Vector::new();
            self.state.floor_state.battle_mut().draw_bottom_known = Vector::new();
        }
    }

    fn get_cards_in_location(
        &mut self,
        location: CardLocation,
        position: RelativePosition,
    ) -> Vec<CardReference> {
        match position {
            RelativePosition::All => self.state.floor_state.battle().cards_in_location(location),
            RelativePosition::Random => self
                .probability
                .choose(self.state.floor_state.battle().cards_in_location(location))
                .into_iter()
                .collect(),
            RelativePosition::Top => match location {
                CardLocation::DrawPile => {
                    self.state
                        .floor_state
                        .battle_mut()
                        .peek_top(1, &mut self.probability);

                    self.state
                        .floor_state
                        .battle()
                        .draw_top_known
                        .get(0)
                        .map(|uuid| CardReference {
                            location: CardLocation::DrawPile,
                            uuid: *uuid,
                            base: self.state.floor_state.battle().cards[uuid].base,
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
    ) -> Vec<&'static BaseCard> {
        let cards = models::cards::available_cards_by_class(class.unwrap_or(self.state.class))
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

        self.probability
            .choose_multiple(cards.collect(), amount as usize)
    }

    fn channel_orb(&mut self, orb_type: OrbType) {
        if self.state.floor_state.battle().orbs.len()
            == self.state.floor_state.battle().orb_slots as usize
        {
            self.evoke_orb(1);
        }

        let n = match orb_type {
            OrbType::Any => panic!("Unexpected Any orb type"),
            OrbType::Dark => {
                let focus = self.state.player.creature.get_buff_amount("Focus");
                std::cmp::max(focus + 6, 0) as u16
            }
            _ => 0,
        };

        let orb = Orb { base: orb_type, n };

        self.state.floor_state.battle_mut().orbs.push_back(orb);
    }

    fn add_block(&mut self, amount: u16, target: CreatureReference) {
        if let Some(mut_creature) = self.state.get_creature_mut(target) {
            let new_block = std::cmp::min(mut_creature.block + amount, 999);
            mut_creature.block = new_block;
            self.eval_target_when(When::OnBlock, target)
        }
    }

    pub fn eval_when(&mut self, when: When) {
        self.eval_target_when(when.clone(), CreatureReference::Player);

        for creature in self
            .state
            .floor_state
            .battle()
            .available_creatures()
            .collect_vec()
        {
            self.eval_target_when(when.clone(), creature);
        }
    }

    fn eval_target_when(&mut self, when: When, target: CreatureReference) {
        self.eval_creature_buff_when(target, when.clone());
        if let Some(monster_ref) = target.monster_ref() {
            self.eval_monster_when(monster_ref, when);
        } else {
            self.eval_relic_when(when);
        }
    }

    fn eval_monster_when(&mut self, monster_ref: MonsterReference, when: When) {
        let phase = {
            if let Some(monster) = self.state.floor_state.battle().get_monster(monster_ref) {
                monster.whens.get(&when).map(|a| a.as_str())
            } else {
                None
            }
        };

        if let Some(phase_name) = phase {
            self.set_monster_phase(phase_name, monster_ref)
        }
    }

    fn set_monster_phase(&mut self, phase: &str, monster: MonsterReference) {
        let new_phase = self
            .state
            .floor_state
            .battle()
            .get_monster(monster)
            .unwrap()
            .base
            .phases
            .iter()
            .position(|p| p.name == phase)
            .unwrap();
        self.set_monster_move(0, new_phase, monster);
    }

    fn set_monster_move(
        &mut self,
        mut move_index: usize,
        mut phase_index: usize,
        monster_ref: MonsterReference,
    ) {
        let (base, last_move, last_move_count) = {
            let monster = self
                .state
                .floor_state
                .battle()
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

                    if self.state.relics.contains("Runic Dome") {
                        available_probabilites
                    } else {
                        self.probability
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
            .state
            .floor_state
            .battle_mut()
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

    fn eval_relic_when(&mut self, when: When) {
        if let Some(relic_ids) = self.state.relics.relic_whens.get(&when).cloned() {
            for relic_id in relic_ids {
                let (base, mut x, mut enabled, relic_ref) = {
                    let relic = &self.state.relics.relics[&relic_id];
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
                                self.eval_effects(&base.effect, Binding::Relic(relic_ref), None);
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
                            self.eval_effects(&base.effect, Binding::Relic(relic_ref), None);
                        }
                    }
                    Activation::When(_) => {
                        self.eval_effects(&base.effect, Binding::Relic(relic_ref), None);
                    }
                    Activation::WhenEnabled {
                        activated_at,
                        enabled_at,
                        disabled_at,
                    } => {
                        if activated_at == &when && enabled {
                            self.eval_effects(&base.effect, Binding::Relic(relic_ref), None);
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
                    let relic = self.state.relics.get_mut(relic_ref);
                    relic.vars.x = x as i16;
                    relic.enabled = enabled;
                }
            }
        }
    }

    fn eval_creature_buff_when(&mut self, creature_ref: CreatureReference, when: When) {
        if let Some(buff_ids) = self
            .state
            .get_creature(creature_ref)
            .and_then(|c| c.buffs_when.get(&when).cloned())
        {
            for buff_id in buff_ids {
                let (base, buff_ref) = {
                    let buff = &self.state.get_creature(creature_ref).unwrap().buffs[&buff_id];
                    (buff.base, buff.reference(creature_ref))
                };

                for WhenEffect {
                    when: _when,
                    effect,
                } in &base.effects
                {
                    if when == *_when {
                        self.eval_effects(effect, Binding::Buff(buff_ref), None);
                    }
                }

                if base.expire_at == when {
                    self.state
                        .get_creature_mut(creature_ref)
                        .unwrap()
                        .remove_buff(buff_ref);
                } else if base.reduce_at == when {
                    let should_remove = {
                        if let Some(buff) = self.state.get_buff_mut(buff_ref) {
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
                        self.state
                            .get_creature_mut(creature_ref)
                            .unwrap()
                            .remove_buff(buff_ref);
                    }
                }
            }
        }
    }

    fn evoke_orb(&mut self, times: u8) {
        if let Some(orb) = self.state.floor_state.battle_mut().orbs.pop_front() {
            match orb.base {
                OrbType::Any => panic!("Unexpected OrbType of any"),
                OrbType::Dark => {
                    for _ in 0..times {
                        let lowest_monster = self
                            .state
                            .floor_state
                            .battle()
                            .monsters
                            .values()
                            .filter(|m| m.targetable)
                            .min_by_key(|m| m.creature.hp)
                            .map(|m| m.creature_ref());

                        if let Some(creature_ref) = lowest_monster {
                            self.damage(orb.n as u16, creature_ref, None, true);
                        }
                    }
                }
                OrbType::Frost => {
                    let focus = self.state.player.creature.get_buff_amount("Focus");
                    let block_amount = std::cmp::max(focus + 5, 0) as u16;

                    for _ in 0..times {
                        self.add_block(block_amount, CreatureReference::Player);
                    }
                }
                OrbType::Lightning => {
                    let has_electro_dynamics = self.state.player.creature.has_buff("Electro");
                    let focus = self.state.player.creature.get_buff_amount("Focus");
                    let orb_damage = std::cmp::max(8 + focus, 0) as u16;
                    for _ in 0..times {
                        if has_electro_dynamics {
                            for monster in self
                                .state
                                .floor_state
                                .battle()
                                .available_monsters()
                                .collect_vec()
                            {
                                self.damage(orb_damage, monster.creature_ref(), None, true);
                            }
                        } else {
                            let monsters = self
                                .state
                                .floor_state
                                .battle()
                                .available_monsters()
                                .collect_vec();
                            if let Some(selected) = self.probability.choose(monsters) {
                                self.damage(orb_damage, selected.creature_ref(), None, true);
                            }
                        }
                    }
                }
                OrbType::Plasma => self.state.floor_state.battle_mut().energy += 2 * times,
            }
        }
    }

    fn damage(
        &mut self,
        amount: u16,
        creature_ref: CreatureReference,
        attacker: Option<CreatureReference>,
        is_orb: bool,
    ) -> bool {
        let hp_loss = {
            if let Some(creature) = self.state.get_creature(creature_ref) {
                let mut multiplier = 1.0;
                if let Some(attacker) = attacker {
                    if creature.has_buff("Vulnerable") {
                        if creature.is_player && self.state.relics.contains("Odd Mushroom") {
                            multiplier += 0.25;
                        } else if !creature.is_player && self.state.relics.contains("Paper Phrog") {
                            multiplier += 0.75;
                        } else {
                            multiplier += 0.5;
                        }
                    }
                    if let Some(attacker_creature) = self.state.get_creature(attacker) {
                        if attacker_creature.has_buff("Weak") {
                            if creature.is_player && self.state.relics.contains("Paper Krane") {
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
                    if creature.is_player {
                        if unblocked_amount <= 5 && self.state.relics.contains("Torii") {
                            unblocked_amount = 1;
                        }

                        if self.state.relics.contains("Tungsten Rod") {
                            unblocked_amount -= 1;
                        }
                    } else if unblocked_amount < 5 && self.state.relics.contains("The Boot") {
                        unblocked_amount = 5;
                    }
                }

                self.state.get_creature_mut(creature_ref).unwrap().block -= blocked_amount;

                unblocked_amount
            } else {
                0
            }
        };

        if hp_loss > 0 && self.lose_hp(hp_loss, creature_ref, true) {
            return true;
        }

        false
    }

    fn lose_hp(
        &mut self,
        mut amount: u16,
        creature_ref: CreatureReference,
        ignore_intangible: bool,
    ) -> bool {
        let new_hp = {
            if let Some(creature) = self.state.get_creature_mut(creature_ref) {
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
            self.state.floor_state.battle_mut().hp_loss_count += 1;
        }

        if new_hp == 0 {
            self.die(creature_ref)
        } else {
            false
        }
    }

    fn die(&mut self, creature_ref: CreatureReference) -> bool {
        match creature_ref {
            CreatureReference::Player => {
                let recovery: f64 =
                    if let Some(potion_ref) = self.state.find_potion("Fairy In A Bottle") {
                        self.state.potions[potion_ref.index] = None;
                        if self.state.relics.contains("Sacred Bark") {
                            0.6
                        } else {
                            0.3
                        }
                    } else if let Some(relic) = self.state.relics.find_mut("Lizard Tail") {
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
                    let max_hp = self.state.player.creature.max_hp;
                    self.heal(max_hp as f64 * recovery, creature_ref);
                }

                if self.state.player.creature.hp == 0 {
                    self.state.won = Some(false);
                    true
                } else {
                    false
                }
            }
            CreatureReference::Creature(monster_ref) => {
                self.eval_target_when(When::OnDie, creature_ref);

                let monster_name = monster_ref.base.name.as_str();

                let dies = match monster_name {
                    "Awakened One" => {
                        let monster = self
                            .state
                            .floor_state
                            .battle_mut()
                            .get_monster_mut(monster_ref)
                            .unwrap();
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
                            .state
                            .floor_state
                            .battle()
                            .monsters
                            .values()
                            .all(|a| !a.targetable || a.uuid == monster_ref.uuid)
                        {
                            true
                        } else {
                            let monster_mut = self
                                .state
                                .floor_state
                                .battle_mut()
                                .get_monster_mut(monster_ref)
                                .unwrap();
                            monster_mut.targetable = false;
                            monster_mut.creature.hp = 0;
                            false
                        }
                    }
                    "Bronze Orb" => {
                        if let Some(buff) = self
                            .state
                            .get_creature(creature_ref)
                            .and_then(|a| a.find_buff("Stasis"))
                            .map(|b| b.card_stasis.unwrap())
                        {
                            self.state.floor_state.battle_mut().move_in(
                                buff,
                                CardDestination::PlayerHand,
                                &mut self.probability,
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
        let reference = self.state.relics.add(base);

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

                        let cards = self.probability.choose_multiple(available_cards, 2);

                        for card in cards {
                            self.state.deck[&card.uuid].upgrade();
                        }
                    }
                    _ => panic!("Unexpected custom activation"),
                };
            }
            _ => {}
        }
    }

    pub fn heal(&mut self, mut amount: f64, creature_ref: CreatureReference) {
        let creature: &mut Creature = match creature_ref {
            CreatureReference::Player => {
                if self.state.relics.contains("Mark Of The Bloom") {
                    return;
                }

                if self.state.floor_state.battle().active
                    && self.state.relics.contains("Magic Flower")
                {
                    amount *= 1.5;
                }

                &mut self.state.player.creature
            }
            CreatureReference::Creature(monster_ref) => {
                let monster = self
                    .state
                    .floor_state
                    .battle_mut()
                    .get_monster_mut(monster_ref)
                    .unwrap();
                monster.targetable = true;
                &mut monster.creature
            }
        };

        creature.hp = std::cmp::min(
            (amount - 0.0001).ceil() as u16 + creature.hp,
            creature.max_hp,
        )
    }
    /*

    fn create_card(&self, name: &str) -> Card {
        let base = models::cards::by_name(name);
        let uuid = Uuid::new_v4();

        let cost = match base.cost {
            Amount::Fixed(cost) => cost as u8,
            Amount::Upgradable{amount, ..} => amount as u8,
            Amount::X => 0,
            Amount::Custom => {
                match name {
                    "Blood for Blood" => {
                        4 - std::cmp::min(self.state.floor_state.battle().hp_loss_count, 4)
                    },
                    "Eviscerate" => {
                        3 - std::cmp::min(self.state.floor_state.battle().discard_count, 3)
                    },
                    "Force Field" => {
                        4 - std::cmp::min(self.state.floor_state.battle().power_count, 4)
                    },
                    _ => panic!("Custom cost amount on an unknown card")
                }
            },
            _ => panic!("Unexpected cost amount")
        };

        let upgrades = match self.state.floor_state.battle().active {
            true => {
                if self.state.player.buff_names.contains_key("Master Reality") {
                    if base.name == "Searing Blow" {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                }
            }
            false => {
                if match base._type {
                    CardType::Attack => self.state.relics.contains("Molten Egg"),
                    CardType::Skill => self.state.relics.contains("Toxic Egg"),
                    CardType::Power => self.state.relics.contains("Frozen Egg"),
                    CardType::Curse => false,
                    CardType::Status => false,
                    CardType::All => panic!("Unexpected card type of All"),
                } {1} else {0}
            }
        };

        let retain = match base.retain {
            Condition::Always => true,
            Condition::Never => false,
            Condition::Upgraded => upgrades > 0,
            _ => panic!("Unexpected retain condition")
        };



        Card {
            base,
            uuid,
            base_cost: cost,
            cost,
            cost_until_played: false,
            retain,
            vars: empty_vars(),
            upgrades,
            bottled: false,
        }
    }

    */

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

    pub fn random_relic(
        &mut self,
        chest_type: Option<ChestType>,
        rarity: Option<Rarity>,
        exclude: Option<&'static BaseRelic>,
        in_shop: bool,
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

        let rarity = self.probability.choose_weighted(&choices).unwrap();

        let available_relics = models::relics::RELICS
            .values()
            .filter(|relic| {
                relic.rarity == **rarity
                    && (relic.class == self.state.class || relic.class == Class::All)
                    && !self.state.relics.contains(&relic.name)
                    && (relic.max_floor == 0 || relic.max_floor as i8 >= self.state.map.floor)
                    && match relic.name.as_str() {
                        "Maw Bank" | "Smiling Mask" | "The Courier" | "Old Coin" => !in_shop,
                        "Bottled Flame" => self.state.deck.values().any(|c| {
                            c.base._type == CardType::Attack && c.base.rarity != Rarity::Starter
                        }),
                        "Bottled Lightning" => self.state.deck.values().any(|c| {
                            c.base._type == CardType::Skill && c.base.rarity != Rarity::Starter
                        }),
                        "Bottled Tornado" => self
                            .state
                            .deck
                            .values()
                            .any(|c| c.base._type == CardType::Power),
                        "Girya" => {
                            !self.state.relics.contains("Peace Pipe")
                                || !self.state.relics.contains("Shovel")
                        }
                        "Shovel" => {
                            !self.state.relics.contains("Peace Pipe")
                                || !self.state.relics.contains("Girya")
                        }
                        "Peace Pipe" => {
                            !self.state.relics.contains("Girya")
                                || !self.state.relics.contains("Shovel")
                        }
                        "Black Blood" => self.state.relics.contains("Burning Blood"),
                        "Frozen Core" => self.state.relics.contains("Cracked Core"),
                        "Holy Water" => self.state.relics.contains("Pure Water"),
                        "Ring of the Snake" => self.state.relics.contains("Ring of the Serpent"),
                        _ => true,
                    }
                    && match &exclude {
                        None => true,
                        Some(e) => relic != e,
                    }
            })
            .collect();

        self.probability
            .choose(available_relics)
            .expect("No available relics to be chosen!")
    }

    fn eval_amount(&self, amount: &Amount, binding: Binding) -> i16 {
        match amount {
            Amount::ByAsc { amount, low, high } => {
                let fight_type = binding
                    .get_monster(self.state.floor_state.battle())
                    .map_or_else(
                        || self.state.floor_state.battle().fight_type,
                        |a| a.base.fight_type,
                    );
                match fight_type {
                    FightType::Common => {
                        if self.state.asc >= 17 {
                            *high
                        } else if self.state.asc >= 2 {
                            *low
                        } else {
                            *amount
                        }
                    }
                    FightType::Elite { .. } => {
                        if self.state.asc >= 18 {
                            *high
                        } else if self.state.asc >= 3 {
                            *low
                        } else {
                            *amount
                        }
                    }
                    FightType::Boss => {
                        if self.state.asc >= 19 {
                            *high
                        } else if self.state.asc >= 4 {
                            *low
                        } else {
                            *amount
                        }
                    }
                }
            }
            Amount::Custom => panic!("Unhandled custom amount: {:?}", binding),
            Amount::EnemyCount => self.state.floor_state.battle().monsters.len() as i16,
            Amount::N => binding.get_vars(&self.state).n as i16,
            Amount::NegX => -binding.get_vars(&self.state).x as i16,
            Amount::OrbCount => self.state.floor_state.battle().orbs.len() as i16,
            Amount::MaxHp => {
                self.state
                    .get_creature(binding.get_creature())
                    .unwrap()
                    .max_hp as i16
            }
            Amount::X => binding.get_vars(&self.state).x as i16,
            Amount::PlayerBlock => self.state.player.creature.block as i16,
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
            Amount::Upgradable { amount, upgraded } => match binding.is_upgraded(&self.state) {
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
            Condition::Act(act) => &self.state.act == act,
            Condition::Always => true,
            Condition::Asc(asc) => &self.state.asc >= asc,
            Condition::Attacking { target } => match target.to_creature(binding, action) {
                CreatureReference::Creature(monster) => self
                    .state
                    .floor_state
                    .battle()
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
                let creature = target.to_creature(binding, action);
                self.state
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
                let creature = self.state.get_creature(target.to_creature(binding, action));

                if let Some(b) = creature.and_then(|f| f.find_buff(buff)) {
                    b.vars.x >= val
                } else {
                    false
                }
            }
            Condition::Class(class) => self.state.class == *class,
            Condition::Custom => panic!("Unhandled custom condition: {:?}", binding),
            Condition::Equals(amount1, amount2) => {
                self.eval_amount(amount1, binding) == self.eval_amount(amount2, binding)
            }
            Condition::FriendlyDead(name) => self
                .state
                .floor_state
                .battle()
                .monsters
                .values()
                .any(|m| m.base.name == *name),
            Condition::HalfHp => self
                .state
                .get_creature(match binding {
                    Binding::Creature(creature) => creature,
                    _ => CreatureReference::Player,
                })
                .map(|creature| creature.hp * 2 <= creature.max_hp)
                .unwrap_or(false),
            Condition::HasCard { location, card } => match location {
                CardLocation::DiscardPile => self
                    .state
                    .floor_state
                    .battle()
                    .discard()
                    .any(|c| c.base._type == *card),
                CardLocation::PlayerHand => self
                    .state
                    .floor_state
                    .battle()
                    .hand()
                    .any(|c| c.base._type == *card),
                CardLocation::ExhaustPile => self
                    .state
                    .floor_state
                    .battle()
                    .exhaust()
                    .any(|c| c.base._type == *card),
                CardLocation::DrawPile => self
                    .state
                    .floor_state
                    .battle()
                    .draw()
                    .any(|c| c.base._type == *card),
                CardLocation::None => false,
            },
            Condition::HasDiscarded => self.state.floor_state.battle().discard_count > 0,
            Condition::HasFriendlies(count) => {
                let creature = binding
                    .get_monster(self.state.floor_state.battle())
                    .expect("Monster did not resolve");
                let friendly_count = self
                    .state
                    .floor_state
                    .battle()
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
                self.state.gold >= self.eval_amount(amount, binding) as u16
            }
            Condition::HasOrbSlot => self.state.floor_state.battle().orb_slots > 0,
            Condition::HasRelic(relic) => self.state.relics.contains(relic),
            Condition::HasRemoveableCards { count, card_type } => {
                self.state
                    .removable_cards()
                    .filter(|card| card.base._type.matches(*card_type))
                    .count()
                    > *count as usize
            }
            Condition::HasUpgradableCard => self.state.upgradable_cards().any(|_| true),
            Condition::InPosition(position) => {
                if let Some(monster) = binding.get_monster(self.state.floor_state.battle()) {
                    monster.position == *position
                } else {
                    panic!("Unexpected player in InPosition check")
                }
            }
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
            },
            Condition::LastCard(_type) => match self.state.floor_state.battle().last_card_played {
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
            Condition::NoBlock => self.state.player.creature.block == 0,
            Condition::Not(condition) => !self.eval_condition(condition, binding, action),
            Condition::OnFloor(floor) => self.state.map.floor >= *floor as i8,
            Condition::RemainingHp { amount, target } => {
                let creature = target.to_creature(binding, action);
                let hp = self.eval_amount(amount, binding);
                self.state
                    .get_creature(creature)
                    .map(|c| c.hp >= hp as u16)
                    .unwrap_or(false)
            }
            Condition::Stance(stance) => &self.state.floor_state.battle().stance == stance,
            Condition::Upgraded => binding.is_upgraded(&self.state),
        }
    }
}
