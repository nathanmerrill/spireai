use crate::models::cards::BaseCard;
use crate::models::monsters::Move;
use crate::models::{self, core::*, monsters::Intent, relics::Activation};
use crate::state::battle::BattleState;
use crate::state::core::{Card, Creature, Monster, Orb};
use crate::state::game::{CardChoiceEffect, CardChoiceState, GameState};
use crate::state::probability::Probability;
use im::HashMap;
use im::{HashSet, Vector};
use itertools::Itertools;
use uuid::Uuid;

use super::references::*;


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GameAction {
    pub is_attack: bool,
    pub creature: CreatureReference,
    pub target: Option<CreatureReference>,
}

pub struct GamePossibility {
    pub state: GameState,
    pub probability: Probability
}

impl GamePossibility {
    fn eval_targets(
        &mut self,
        target: &Target,
        binding: Binding,
        action: Option<GameAction>) -> Vec<CreatureReference> {
        let creatures = match target {
            Target::AllEnemies => match binding.get_monster(&self.state) {
                Some(_) => vec![CreatureReference::Player],
                None => self.state.battle_state.available_creatures().collect(),
            },
            Target::AnyFriendly => match binding.get_monster(&self.state) {
                Some(_) => self.state.battle_state.available_creatures().collect(),
                None => vec![CreatureReference::Player],
            },
            Target::RandomEnemy => match binding.get_monster(&self.state) {
                Some(_) => vec![CreatureReference::Player],
                None => {
                    self.state.battle_state.random_monster(&mut self.probability).map(|a|a.creature_ref()).into_iter().collect()
                }
            },
            Target::RandomFriendly => {
                let creature_reference = match binding {
                    Binding::Buff(buff) => buff.creature,
                    Binding::Creature(creature) => creature,
                    _ => CreatureReference::Player,
                };
                match creature_reference {
                    CreatureReference::Player => vec![CreatureReference::Player],
                    CreatureReference::Creature(uuid) => {
                        let monsters = self.state.battle_state.available_monsters().filter(|a| {
                            a.uuid != uuid
                        }).collect_vec();

                        self.probability.choose(monsters).into_iter().map(|a| a.creature_ref()).collect()
                    }
                }
            },
            _ => vec![self.eval_target(target, binding, action)]
        };

        creatures
    }

    fn eval_target(
        &self,
        target: &Target,
        binding: Binding,
        action: Option<GameAction>,
    ) -> CreatureReference {
        match target {
            Target::_Self => match binding {
                Binding::Buff(buff) => buff.creature,
                Binding::Creature(creature) => creature,
                _ => CreatureReference::Player,
            },
            Target::Attacker => match action {
                Some(_action) => match _action.is_attack {
                    true => _action.creature,
                    false => panic!("Expected attack action!"),
                },
                None => panic!("Expected action!"),
            },
            Target::TargetEnemy => match action {
                Some(_action) => match _action.creature {
                    CreatureReference::Player => _action.target.expect("Expected target!"),
                    CreatureReference::Creature(_) => CreatureReference::Player
                },
                None => panic!("Expected action!")
            },
            Target::Player => CreatureReference::Player,
            _ => panic!("Target does not resolve to a single creature! {:?}", target)
        }
    }

    pub fn spend_money(&mut self, amount: u16, at_shop: bool) {
        self.state.gold -= amount;

        if at_shop {
            if let Some(relic) = self.state.find_relic_mut("Maw Bank") {
                relic.enabled = false;
            }
        }
    }

    fn eval_card_effects(&mut self, effects: &[CardEffect], card: CardReference) {
        for effect in effects {
            self.eval_card_effect(effect, card);
        }
    }

    fn eval_card_effect(&mut self, effect: &CardEffect, card: CardReference){
        match effect {
            CardEffect::AutoPlay => {
                let binding = Binding::Card(card);
                let target = if self.eval_condition(&card.base.targeted, binding, None) {
                    self.state.battle_state.random_monster(&mut self.probability)
                } else {
                    None
                };

                for effect in &card.base.on_play {
                    self.eval_effect(effect, binding, Some(GameAction {
                        is_attack: card.base._type == CardType::Attack,
                        creature: CreatureReference::Player,
                        target: target.map(|a| a.creature_ref()),
                    }))
                };
            }
            CardEffect::CopyTo {destination, then} => {
                if *destination == CardDestination::DeckPile {
                    let upgrades = self.state.get(card).upgrades;
                    if let Some(card_ref) = self.add_card(Card::new(card.base), *destination) {
                        let card = self.state.get_mut(card_ref);
                        card.upgrades = std::cmp::max(card.upgrades, upgrades);
                        self.eval_card_effects(then, card_ref)
                    }
                } else {
                    let new_card = self.state.get(card).duplicate();
                    if let Some(card_ref) = self.add_card(new_card, *destination) {
                        self.eval_card_effects(then, card_ref)
                    }
                }

            }
            CardEffect::Custom => panic!("Unexpected custom card effect"),
            CardEffect::Discard => {
                if !self.state.battle_state.discard.contains(&card.uuid){
                    self.state.battle_state.move_card(CardDestination::DiscardPile, card, &mut self.probability);
                    self.eval_effects(&card.base.on_discard, Binding::Card(card), None);
                    self.state.battle_state.discard_count += 1;
                }
            }
            CardEffect::Exhaust => {
                if !self.state.battle_state.exhaust.contains(&card.uuid){
                    self.state.battle_state.move_card(CardDestination::ExhaustPile, card, &mut self.probability);
                    self.eval_effects(&card.base.on_exhaust, Binding::Card(card), None);
                }
            }
            CardEffect::MoveTo (destination) => {
                self.state.battle_state.move_card(*destination, card, &mut self.probability);
            }
            CardEffect::ReduceCost(amount) => {
                let reduction = self.eval_amount(amount, Binding::Card(card));
                let card = self.state.get_mut(card);
                if card.base.cost != Amount::X {
                    card.cost = std::cmp::max(card.cost as i16 - reduction, 0) as u8;
                    card.base_cost = std::cmp::max(card.base_cost as i16 - reduction, 0) as u8;
                }
            }
            CardEffect::Retain => {
                self.state.get_mut(card).retain = true;
            }
            CardEffect::Upgrade => {
                self.state.get_mut(card).upgrade();
            }
            CardEffect::ZeroCombatCost => {
                let card = self.state.get_mut(card);
                card.cost = 0;
                card.base_cost = 0;
            }
            CardEffect::ZeroCostUntilPlayed => {
                let card = self.state.get_mut(card);
                card.cost = 0;
                card.cost_until_played = true;
            }
            CardEffect::ZeroTurnCost => {
                let card = self.state.get_mut(card);
                card.cost = 0;
            }
        }
    }


    pub fn add_card(&mut self, card: Card, destination: CardDestination) -> Option<CardReference> {
        if destination == CardDestination::DeckPile {
            if card.base._type == CardType::Curse {
                if let Some(relic) = self.state.find_relic_mut("Omamori") {
                    if relic.vars.x > 0 {
                        relic.vars.x -= 1;
                        return None;
                    }
                }
        
                if self.state.has_relic("Darkstone Periapt")
                {
                    self.add_max_hp(6);
                }
            }
        
        
            if self.state.has_relic("Ceramic Fish") {
                self.add_gold(9);
            }
        }
        let card_ref = Some(CardReference {
            location: destination.location(),
            uuid: card.uuid,
            base: card.base
        });

        self.state.add_card(card, destination, &mut self.probability);
        card_ref
    }


    fn eval_effects(
        &mut self,
        effects: &'static [Effect],
        binding: Binding,
        action: Option<GameAction>
    ){
        for effect in effects {
            self.eval_effect(effect, binding, action);
        }
    }

    fn eval_effect(
        &mut self, 
        effect: &'static Effect,
        binding: Binding,
        action: Option<GameAction>
    )
    {
        match effect {
            Effect::AddBuff {
                buff: buff_name,
                amount: buff_amount,
                target,
            } => {
                let amount = self.eval_amount(buff_amount, binding);
                for creature in self.eval_targets(target, binding, action)
                {
                    self.state.get_mut(creature).add_buff(buff_name, amount);
                }
            }
            Effect::AddEnergy(energy_amount) => {
                let amount = self.eval_amount(energy_amount, binding) as u8;
                self.state.battle_state.energy += amount
            }
            Effect::AddGold(gold_amount) => {
                let amount = self.eval_amount(gold_amount, binding) as u16;
                self.add_gold(amount)
            }
            Effect::AddMaxHp(hp_amount) => {
                let amount = self.eval_amount(hp_amount, binding) as u16;
                self.add_max_hp(amount)
            }
            Effect::AddN(n_amount) => {
                let amount = self.eval_amount(n_amount, binding);
                binding.get_mut_vars(&mut self.state).n += amount;
            }
            Effect::AddOrbSlot(amount) => {
                let count = self.eval_amount(amount, binding) as u8;
                self.state.battle_state.orb_slots = std::cmp::min(count + self.state.battle_state.orb_slots, 10) - self.state.battle_state.orb_slots;            
            }
            Effect::AddPotionSlot(amount) => {
                for _ in 0 .. *amount {
                    self.state.potions.push_back(None)
                }
            }
            Effect::AddRelic(name) => {
                self.add_relic(name);
            }
            Effect::AddX(amount ) => {
                binding.get_mut_vars(&mut self.state).x += self.eval_amount(amount, binding);
            }
            Effect::AttackDamage {amount, target, if_fatal, times} => {
                let attack_amount = self.eval_amount(&amount, binding);
                
                for creature in self.eval_targets(&target, binding, action) {
                    for _ in 0 .. self.eval_amount(times, binding){
                        if self.damage(attack_amount as u16, creature) {
                            self.eval_effects(if_fatal, binding, action);
                        }
                    }
                }
            }
            Effect::Block {amount, target} => {
                let block_amount = self.eval_amount(amount, binding) as u16;

                for creature in self.eval_targets(target, binding, action) {
                    let mut_creature = self.state.get_mut(creature);
                    let new_block = std::cmp::min(mut_creature.block + block_amount, 999);
                    mut_creature.block = new_block;
                }
            }
            Effect::ChannelOrb(orb_type) => {
                self.channel_orb(*orb_type)
            }
            Effect::ChooseCardByType {
                destination, 
                _type, 
                rarity, 
                class, 
                then, 
                choices,
                exclude_healing,
            } => {
                let amount = self.eval_amount(&choices, binding);

                let choice = self.random_cards_by_type(amount as usize, *class, *_type, *rarity, *exclude_healing);

                let mut card_choices = Vector::new();
                for base_card in choice {
                    card_choices.push_back(Card::new(base_card));
                }

                self.state.card_choices = CardChoiceState {
                    choices: card_choices.iter().map(|card| card.uuid).collect(),
                    location: CardLocation::Stasis,
                    count: Some((amount as usize, amount as usize)),
                    effect: CardChoiceEffect::AddToLocation(*destination, then.clone()),
                };

                for card in card_choices {
                    self.state.battle_state.cards.insert(card.uuid, card);
                }
            }

            Effect::ChooseCards{location, then, min, max} => {
                let min_count = self.eval_amount(min, binding);
                let max_count = self.eval_amount(max, binding);
                let cards: Vector<Uuid> = match location {
                    CardLocation::DeckPile => self.state.deck().map(|c| c.uuid).collect(),
                    CardLocation::DiscardPile => self.state.battle_state.discard().map(|c| c.uuid).collect(),
                    CardLocation::DrawPile => self.state.battle_state.draw().map(|c| c.uuid).collect(),
                    CardLocation::ExhaustPile => self.state.battle_state.exhaust().map(|c| c.uuid).collect(),
                    CardLocation::PlayerHand => self.state.battle_state.hand().map(|c| c.uuid).collect(),
                    CardLocation::Stasis => panic!("Cannot choose from stasis!"),
                };
                
                self.state.card_choices = CardChoiceState {
                    choices: cards,
                    location: *location,
                    count: Some((min_count as usize, max_count as usize)),
                    effect: CardChoiceEffect::Then(then)
                };
            }
            Effect::CreateCard{name, destination, then} => {
                let card = Card::by_name(name);
                if let Some(card_ref) = self.add_card(card, *destination){
                    self.eval_card_effects(then, card_ref);
                }            
            }
            Effect::CreateCardByType{destination, _type, rarity, class, exclude_healing, then} => {
                let card = self.random_cards_by_type(1, *class, *_type, *rarity, *exclude_healing)[0];
                if let Some(card_ref) = self.add_card(Card::new(card), *destination) {
                    self.eval_card_effects(then, card_ref);
                }
            }
            Effect::Custom => unimplemented!(),
            Effect::Damage {amount, target} => {
                let total = self.eval_amount(amount, binding) as u16;
                let creature = self.eval_target(target, binding, action);
                self.damage(total, creature);
            }
            Effect::Die {target} => {
                let creature = self.eval_target(target, binding, action);
                self.die(creature);
            }
            Effect::DoCardEffect {location, position, effect} => {
                for card in self.get_cards_in_location(*location, *position) {
                    self.eval_card_effect(effect, card)
                }
            }
            Effect::Draw(amount) => {
                let n = self.eval_amount(amount, binding);
                self.draw(n as u8);
            }
            Effect::EvokeOrb(amount) => {
                self.evoke_orb(self.eval_amount(amount, binding) as u8)
            }
            Effect::Fight {monsters, room} => {
                self.fight(monsters, *room);
            }
            Effect::Heal {amount, target} => {
                let total = self.eval_amount(amount, binding);
                for creature in self.eval_targets(target, binding, action) {
                    self.heal(total as f64, creature);
                }
            }
            Effect::HealPercentage {amount, target} => {
                let percentage = self.eval_amount(amount, binding) as f64 / 100.0;
                for creature in self.eval_targets(target, binding, action) {
                    let total = self.state.get(creature).hp as f64 * percentage;
                    self.heal(total, creature);
                }
            }
            Effect::If {condition, then, _else} => {
                if self.eval_condition(condition, binding, action) {
                    self.eval_effects(then, binding, action);
                } else {
                    self.eval_effects(_else, binding, action);
                }
            }
            Effect::LoseAllGold => {
                self.state.gold = 0
            }
            Effect::LoseHp {amount, target} => {
                let total = self.eval_amount(amount, binding);
                for creature in self.eval_targets(target, binding, action) {
                    self.lose_hp(total as u16, creature);
                }
            }
            Effect::LoseHpPercentage(amount) => {
                let percentage = self.eval_amount(amount, binding) as f64 / 1000.0;
                let damage = (self.state.player.max_hp as f64 * percentage).floor() as u16;
                self.lose_hp(damage, CreatureReference::Player);
            }
            Effect::RandomChance(chances) => {
                let evaluated_chances = chances.iter().map(|chance| {
                    (&chance.effect, self.eval_amount(&chance.amount, binding) as u8)
                }).collect_vec();

                let choice = self.probability.choose_weighted(&evaluated_chances).unwrap();

                self.eval_effects(choice, binding, action);
            }
            Effect::RandomPotion => {
                let potion = self.random_potion();
                self.state.add_potion(potion);
            }
            Effect::RandomRelic => {
                let relic = self.random_relic(None);
                self.add_relic(relic)
            }
            Effect::ReduceMaxHpPercentage(amount) => {
                let percentage = self.eval_amount(amount, binding);
                let total = (self.state.player.max_hp as f64 * (percentage as f64 / 100.0)).floor() as u16;
                self.state.reduce_max_hp(total);            
            }
            Effect::RemoveCard(count) => {
                self.state.card_choices = CardChoiceState {
                    choices: self.state.deck().map(|a| a.uuid).collect(),
                    location: CardLocation::DeckPile,
                    count: Some((*count as usize, *count as usize)),
                    effect: CardChoiceEffect::Remove
                }
            }
            Effect::RemoveDebuffs => {
                let creature_ref = binding.get_creature();
                let creature = self.state.get(creature_ref);
                let debuffs = creature.buffs.values()
                    .filter(|buff|buff.base.debuff || buff.vars.x < 0)
                    .map(|buff|buff.reference(creature_ref))
                    .collect_vec();
                
                for debuff in debuffs {
                    self.state.get_mut(debuff.creature).remove_buff(debuff);
                }
            }
            
            _ => unimplemented!(),
        }
    }




    fn random_potion(&mut self) -> &'static String {
        let rarities = [
            (Rarity::Common, 70),
            (Rarity::Uncommon, 25),
            (Rarity::Rare, 5)
        ];

        let rarity = *self.probability.choose_weighted(&rarities).unwrap();

        let in_combat = self.state.battle_state.active;

        let potions = models::potions::POTIONS.values().filter(|a| a.rarity == rarity && !(in_combat && a.name == "Fruit Juice")).collect_vec();

        &self.probability.choose(potions).unwrap().name
    }


    fn fight(&mut self, monsters: &'static Vec<String>, fight_type: FightType) {
        self.create_battle(monsters, fight_type);
        self.eval_when(When::CombatStart);
        self.start_turn(true);
    }

    fn create_battle(&mut self, monster_names: &'static Vec<String>, fight_type: FightType) {
        let cards: HashMap<Uuid, Card> = self.state.deck.values().map(|c| (c.uuid, c.duplicate())).collect();
        let draw_top = if self.state.has_relic("Frozen Eye") {
            cards.values().map(|c| c.uuid).collect()
        } else {
            Vector::new()
        };

        let orb_slots = if self.state.class == Class::Defect {
            3
        } else if self.state.has_relic("Prismatic Shard") {
            1
        } else {
            0
        };

        let mut energy = 3;

        for relic in self.state.relics.values() {
            if relic.base.energy_relic {
                energy += 1;
            }
        }

        let monsters = monster_names.iter()
            .map(|n| self.create_monster(n))
            .enumerate()
            .map(|(index, mut monster)| {
                monster.position = index;
                (monster.uuid, monster)
            }).collect();

        self.state.battle_state = BattleState {
            active: true,
            fight_type,
            draw_top_known: draw_top,
            draw: cards.values().map(|card| card.uuid).collect(),
            deck_references: cards.iter().map(|(uuid, card)| (card.uuid, *uuid)).collect(),
            cards,
            orb_slots,
            monsters,
            base_energy: energy, 
            ..BattleState::new()
        };

        for monster_ref in self.state.battle_state.available_monsters().collect_vec() {
            let creature_ref = monster_ref.creature_ref();
            if let Some(x) = monster_ref.base.x_range.as_ref().map(|a| self.eval_range(a, creature_ref)) {
                self.state.get_mut(monster_ref).vars.x = x
            }
            if let Some(n) = monster_ref.base.n_range.as_ref().map(|a| self.eval_range(a, creature_ref)) {
                let monster = self.state.get_mut(monster_ref);
                monster.vars.n = n;
                monster.vars.n_reset = n;
            }

            self.set_monster_move(0, 0, monster_ref);
        }

        self.shuffle();
    }

    fn create_monster(&mut self, name: &'static str) -> Monster {
        let base = crate::models::monsters::by_name(name);
        let upgrade_asc = match base.fight_type {
            FightType::Common => 7,
            FightType::Elite => 8,
            FightType::Boss => 9
        };

        let hp_range = if self.state.asc >= upgrade_asc {
            &base.hp_range_asc
        } else {
            &base.hp_range
        };

        let hp = self.probability.range((hp_range.max - hp_range.min + 1) as usize) as u16 + hp_range.min;
        
        Monster::new(base, hp)
    }

    fn eval_range(&mut self, range: &crate::models::monsters::Range, creature: CreatureReference) -> i16 {
        let binding = Binding::Creature(creature);
        let min = self.eval_amount(&range.min, binding);
        let max = self.eval_amount(&range.max, binding);
        self.probability.range((max - min + 1) as usize) as i16 + min
    }

    pub fn end_turn(&mut self) {
        self.eval_when(When::BeforeHandDiscard);
        let has_runic_pyramid = self.state.has_relic("Runic Pyramid");
        for card_ref in self.state.battle_state.hand().collect_vec() {
            let binding = Binding::Card(card_ref);
            if self.state.get(card_ref).retain {
                self.eval_effects(&card_ref.base.on_retain, binding, None);
                if !self.eval_condition(&card_ref.base.retain, binding, None) {
                    self.state.get_mut(card_ref).retain = false;
                }
            } else if !has_runic_pyramid {
                self.state.battle_state.move_card(CardDestination::DiscardPile, card_ref, &mut self.probability);
            }
            self.eval_effects(&card_ref.base.on_turn_end, binding, None);
        }
        self.eval_when(When::BeforeEnemyMove);
        let monsters = self.state.battle_state.monsters.values().sorted_by_key(|a| a.index).map(|a| a.monster_ref()).collect_vec();
        for monster in monsters {
            self.next_monster_move(monster);
        }
        self.eval_when(When::AfterEnemyMove);
        self.eval_when(When::TurnEnd);
        self.start_turn(false);
    }

    fn start_turn(&mut self, combat_start: bool) {
        self.eval_when(When::BeforeHandDraw);
        
        let mut cards_to_draw = 5;
        if self.state.has_relic("Snecko Eye") {
            cards_to_draw += 2;
        }
        if combat_start && self.state.has_relic("Bag of Preparation") {
            cards_to_draw += 2;
        }
        if let Some(buff) = self.state.player.find_buff("Draw Card") {
            cards_to_draw += buff.vars.x;
            self.state.player.remove_buff_by_name("Draw Card");
        }
        self.draw(cards_to_draw as u8);
        
        self.eval_when(When::AfterHandDraw);
    }

    fn next_monster_move(&mut self, monster: MonsterReference) {
        self.eval_effects(&self.state.get(monster).current_move.effects, Binding::Monster(monster), None);
        self.next_move(monster);
    }

    fn next_move(&mut self, monster_ref: MonsterReference) {
        let (index, phase) = {
            let monster = self.state.get_mut(monster_ref);
            if let Some(last_move) = monster.last_move{
                if last_move == monster.current_move {
                    monster.last_move_count += 1;
                } else {
                    monster.last_move = Some(last_move);
                    monster.last_move_count = 1;
                }
            };
            (monster.index, monster.phase)
        };

        self.set_monster_move(index+1, phase, monster_ref);    
    }

    fn draw(&mut self, n: u8) {
        let mut cards = Vec::new();
        for _ in  0 .. n {
            if self.state.battle_state.draw.is_empty() {
                self.shuffle();
            }
            if self.state.battle_state.draw.is_empty() {
                break;
            }
            let next_card = self.state.battle_state.draw_top_known.pop_back();

            let uuid = match next_card {
                Some(uuid) => {
                    uuid
                }
                None => {
                    let choices = self.state.battle_state.draw.iter().cloned().collect_vec();
                    self.probability.choose(choices).unwrap()
                }
            };
            
            self.state.battle_state.draw.remove(&uuid);
            let card = self.state.battle_state.cards.get(&uuid).unwrap().reference(CardLocation::DrawPile);
            cards.push(card)
        }

        for card in cards {
            self.eval_effects(&card.base.on_draw, Binding::Card(card), None);
            self.eval_target_when(When::DrawCard(card.base._type), CreatureReference::Player);
            self.eval_target_when(When::DrawCard(CardType::All), CreatureReference::Player);
        }
    }

    fn shuffle(&mut self) {
        if self.state.has_relic("Frozen Eye") {
            self.shuffle()
        } else {
            self.state.battle_state.draw_top_known = Vector::new();
            self.state.battle_state.draw_bottom_known = Vector::new();
        }
    }

    fn get_cards_in_location(&mut self, location: CardLocation, position: RelativePosition) -> Vec<CardReference> {
        match position {
            RelativePosition::All => {
                self.state.in_location(location)
            }
            RelativePosition::Random => {
                self.probability.choose(self.state.in_location(location)).into_iter().collect()
            }
            RelativePosition::Top => {
                match location {
                    CardLocation::DrawPile => {
                        let uuid = self.state.battle_state.draw_top_known.back().cloned().unwrap_or_else(|| {
                            let draw_top: HashSet<Uuid> = self.state.battle_state.draw_top_known.iter().cloned().collect();
                            let difference = self.state.battle_state.draw.iter().filter(|uuid| draw_top.contains(uuid)).cloned().collect_vec();
                            self.probability.choose(difference).unwrap()
                        });

                        vec![CardReference {
                            location: CardLocation::DrawPile,
                            uuid,
                            base: self.state.battle_state.cards[&uuid].base
                        }]
                    },
                    _ => panic!("Unepxected location in RelativePosition::Bottom")
                }
            }
            RelativePosition::Bottom => {
                match location {
                    CardLocation::DrawPile => {
                        if self.state.battle_state.draw_top_known.len() == self.state.battle_state.draw.len() {
                            self.state.battle_state.draw_top_known.front().map(|uuid| CardReference {
                                location: CardLocation::DrawPile,
                                uuid: *uuid,
                                base: self.state.battle_state.cards[uuid].base
                            }).into_iter().collect_vec()
                        } else {
                            let uuid = 
                            self.state.battle_state.draw_bottom_known.back().cloned().unwrap_or_else(|| {
                                let draw_top: HashSet<Uuid> = self.state.battle_state.draw_top_known.iter().cloned().collect();
                                let difference = self.state.battle_state.draw.iter().filter(|uuid| draw_top.contains(uuid)).cloned().collect_vec();
                                self.probability.choose(difference).unwrap()
                            });
                            vec![CardReference {
                                location: CardLocation::DrawPile,
                                uuid,
                                base: self.state.battle_state.cards[&uuid].base
                            }]
                        }
                    },
                    _ => panic!("Unepxected location in RelativePosition::Bottom")
                }
            }
        }
    }

    fn random_cards_by_type(&mut self, amount: usize, class: Option<Class>, _type: CardType, rarity: Option<Rarity>, exclude_healing: bool) -> Vec<&'static BaseCard> {
        let cards = models::cards::available_cards_by_class(class.unwrap_or(self.state.class)).iter().filter(|card| {
            (_type == CardType::All || card._type == _type) &&
            (rarity == None || rarity.unwrap() == card.rarity) &&
            (!exclude_healing || 
            !matches!(card.name.as_str(),
                "Feed" | "Reaper" | "Lesson Learned" | "Alchemize" | "Wish" | "Bandage Up" | "Self Repair"))
        }).cloned();

        self.probability.choose_multiple(cards.collect(), amount as usize)
    }

    fn channel_orb(&mut self, orb_type: OrbType) {
        if self.state.battle_state.orbs.len() == self.state.battle_state.orb_slots as usize {
            self.evoke_orb(1);
        }

        let n = match orb_type {
            OrbType::Any => panic!("Unexpected Any orb type"),
            OrbType::Dark => {
                let focus = self.state.player.get_buff_amount("Focus");
                std::cmp::max(focus + 6, 0) as u16
            },
            _ => 0
        };

        let orb = Orb {
            base: orb_type,
            n
        };

        self.state.battle_state.orbs.push_back(orb);
    }

    fn add_block(&mut self, amount: u16, target: CreatureReference) {
        let mut_creature = self.state.get_mut(target);
        let new_block = std::cmp::min(mut_creature.block + amount, 999);
        mut_creature.block = new_block;
        self.eval_target_when(When::OnBlock, target)
    }

    fn eval_when(&mut self, when: When) {
        self.eval_target_when(when.clone(), CreatureReference::Player);

        for creature in self.state.battle_state.available_creatures().collect_vec() {
            self.eval_target_when(when.clone(), creature);
        }
    }

    fn eval_target_when(&mut self, when: When, target: CreatureReference) {
        self.eval_creature_buff_when(target, when.clone());
        if target == CreatureReference::Player {
            self.eval_relic_when(when);    
        } else {
            self.eval_monster_when(target.monster_ref(&self.state), when);
        }
    }

    fn eval_monster_when(&mut self, monster_ref: MonsterReference, when: When) {
        let phase = {
            if let Some(monster ) = self.state.get_opt(monster_ref){
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
        let new_phase = self.state.get_mut(monster).base.phases.iter().position(|p| p.name == phase).unwrap();
        self.set_monster_move(0, new_phase, monster);
    }

    fn set_monster_move(&mut self, mut move_index: usize, mut phase_index: usize, monster_ref: MonsterReference) {
        let (base, last_move, last_move_count) = {
            let monster = self.state.get(monster_ref);
            (monster.base, monster.last_move, monster.last_move_count)
        };
        let binding = Binding::Monster(monster_ref);

        let next_move = loop {
            let mut phase = base.phases.get(phase_index).unwrap();
            if move_index == phase.moves.len() {
                if !phase.next.is_empty() {
                    let position = base.phases.iter().find_position(|a| a.name == phase.next).unwrap();
                    phase_index = position.0;
                    phase = position.1;            
                }
                move_index = 0;
            }
            

            let name =  match &phase.moves[move_index] {
                Move::Fixed(name) => {
                    Some(name)
                }
                Move::Probability(probabilities) => {
                    let available_probabilites = 
                    probabilities.iter().filter(|p| {
                        let max_repeats = self.eval_amount(&p.max_repeats, binding) as u8;
                        let same_move = last_move.map_or(false, |a| a.name == p.name);
                        let maxxed_out = same_move && last_move_count >= max_repeats;
                        !maxxed_out
                    }).map(|a| {
                        let weight = self.eval_amount(&a.weight, binding) as u8;
                        (&a.name, weight)
                    }).collect_vec();

                    self.probability.choose_weighted(&available_probabilites).cloned()
                }
                Move::If{condition, then_phase, else_phase} => {
                    let next_phase = if self.eval_condition(&condition, binding, None) {
                        then_phase
                    } else {
                        else_phase
                    };

                    if !next_phase.is_empty() {
                        let position = base.phases.iter().find_position(|a| &a.name == next_phase).unwrap();
                        phase_index = position.0;
                        move_index = 0;
                    }
                    None
                }
            };

            if let Some(value) = name {
                break value;
            } else {
                move_index += 1;
            }
        };

        let monster = self.state.get_mut(monster_ref);

        monster.current_move = monster.base.moveset.iter().find(|a| &a.name == next_move).unwrap();
        monster.index = move_index;
        monster.phase = phase_index;
    }

    fn eval_relic_when(&mut self, when: When) {
        if let Some(relic_ids) = self.state.relic_whens.get(&when).cloned() {
            for relic_id in relic_ids {
                
                let (base, mut x, mut enabled, relic_ref) = {
                    let relic = &self.state.relics[&relic_id];
                    (relic.base, relic.vars.x as u16, relic.enabled, relic.reference())
                };

                match &base.activation {
                    Activation::Counter {increment, reset, auto_reset, target} => {
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
                    Activation::Immediate | Activation::Custom => {},
                    Activation::Uses { .. } => {
                        if x != 0 {
                            x -= 1;
                            self.eval_effects(&base.effect, Binding::Relic(relic_ref), None);
                        }
                    }
                    Activation::When(_) => {
                        self.eval_effects(&base.effect, Binding::Relic(relic_ref), None);
                    }
                    Activation::WhenEnabled {activated_at, enabled_at, disabled_at} => {
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
                    let relic = self.state.get_mut(relic_ref);
                    relic.vars.x = x as i16;
                    relic.enabled = enabled;
                }
            } 
        }
    }

    fn eval_creature_buff_when(&mut self, creature_ref: CreatureReference, when: When) {
        if let Some(buff_ids) = self.state.get(creature_ref).buffs_when.get(&when).cloned() {
            for buff_id in buff_ids {
                let (base, buff_ref) = {
                    let buff = &self.state.get(creature_ref).buffs[&buff_id];
                    (buff.base, buff.reference(creature_ref))
                };

                for WhenEffect { when: _when, effect} in &base.effects {
                    if when == *_when {
                        self.eval_effects(effect, Binding::Buff(buff_ref), None);
                    }
                }
                
                if base.expire_at == when {
                    self.state.get_mut(creature_ref).remove_buff(buff_ref);
                } else if base.reduce_at == when {
                    let should_remove = {
                        let buff = self.state.get_mut(buff_ref);
                        if buff.stacked_vars.is_empty() {
                            buff.vars.x += 1;
                        } else {
                            for mut var in buff.stacked_vars.iter_mut() {
                                var.x -= 1;
                            }

                            buff.stacked_vars = buff.stacked_vars.iter().filter(|var| var.x > 0).cloned().collect();
                        }
                        buff.stacked_vars.is_empty() && buff.vars.x == 0
                    };

                    if should_remove {
                        self.state.get_mut(creature_ref).remove_buff(buff_ref);
                    }
                }
            }
        }
    }


    fn evoke_orb(&mut self, times: u8) {
        if let Some(orb) = self.state.battle_state.orbs.pop_front() {
            match orb.base {
                OrbType::Any => panic!("Unexpected OrbType of any"),
                OrbType::Dark => {
                    for _ in 0 .. times {
                        let lowest_monster =  
                            self.state.battle_state.monsters.values()
                                .filter(|m| m.targetable)
                                .min_by_key(|m|m.creature.hp)
                                .map(|m|m.uuid);

                        if let Some(uuid) = lowest_monster {
                            let mut orb_damage = orb.n as f64;
                            let creature = CreatureReference::Creature(uuid);
                            if self.state.get(creature).has_buff("Lock On") {
                                orb_damage *= 1.5;
                            }

                            self.damage(orb_damage.floor() as u16, creature);
                        }
                    }
                }
                OrbType::Frost => {
                    let focus = self.state.player.get_buff_amount("Focus");
                    let block_amount = std::cmp::max(focus+5, 0) as u16;
                    
                    for _ in 0 .. times {
                        self.add_block(block_amount, CreatureReference::Player);
                    }
                }
                OrbType::Lightning => {
                    let has_electro_dynamics = self.state.player.has_buff("Electro");
                    let focus = self.state.player.get_buff_amount("Focus");
                    let orb_damage = std::cmp::max(8 + focus, 0) as f64;
                    for _ in 0 .. times {
                        if has_electro_dynamics {
                            for monster in self.state.battle_state.available_monsters().collect_vec() {
                                let multiplier = if self.state.get(monster).creature.has_buff("Lock On") {
                                    1.5
                                } else {
                                    1.0
                                };
                                
                                self.damage((orb_damage * multiplier).floor() as u16, monster.creature_ref());
                            }
                        } else {
                            let monsters = self.state.battle_state.available_monsters().collect_vec();
                            if let Some(selected) = self.probability.choose(monsters) {
                                let multiplier = if self.state.get(selected).creature.has_buff("Lock On") {
                                    1.5
                                } else {
                                    1.0
                                };
                                
                                self.damage((orb_damage * multiplier).floor() as u16, selected.creature_ref());
                            }
                        }
                    }
                }
                OrbType::Plasma => {
                    self.state.battle_state.energy += 2 * times
                }
            }
        }
    }

    fn damage(&mut self, amount: u16, creature_ref: CreatureReference) -> bool {
        let mut block = self.state.get(creature_ref).block;
        if block < amount as u16 {
            if self.lose_hp(amount - block, creature_ref) {
                return true;
            }
            block = 0
        } else {
            block -= amount
        }

        self.state.get_mut(creature_ref).block = block;

        false
    }

    fn lose_hp(&mut self, amount: u16, creature_ref: CreatureReference) -> bool {
        let new_hp = {
            let creature = self.state.get_mut(creature_ref);
            creature.hp = std::cmp::max(creature.hp - amount, 0);
            creature.hp
        };

        if amount > 0 && creature_ref == CreatureReference::Player {
            self.state.battle_state.hp_loss_count += 1; 
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
                    if self.state.has_relic("Sacred Bark") {
                        0.6
                    } else {
                        0.3
                    }
                } else if let Some(relic) = self.state.find_relic_mut("Lizard Tail") {
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
                    let max_hp = self.state.get(creature_ref).max_hp;
                    self.heal(max_hp as f64 * recovery, creature_ref);
                }
                
                if self.state.player.hp == 0 {
                    self.state.won = Some(false);
                    true
                } else {
                    false
                }
            }
            CreatureReference::Creature(uuid) => {
                let monster = &self.state.battle_state.monsters[&uuid];

                match monster.base.name.as_str() {
                    "Awakened One" => {
                        if monster.vars.x == 0 {
                            self.eval_target_when(When::OnDie, creature_ref);
                            let monster_mut = creature_ref.get_monster_mut(&mut self.state).unwrap();
                            monster_mut.vars.x = 1;
                            monster_mut.targetable = false;
                            monster_mut.creature.hp = 0;
                            false
                        } else {
                            true
                        }
                    }
                    "Darkling" => {
                        if self.state.battle_state.monsters.values().all(|a| a.targetable == false || a.uuid == uuid) {
                            true
                        } else {
                            let monster_mut = creature_ref.get_monster_mut(&mut self.state).unwrap();
                            monster_mut.targetable = false;
                            monster_mut.creature.hp = 0;
                            false
                        }
                    }
                    _ => true
                }
            }
        }
    }

    pub fn add_relic(&mut self, name: &str) {
        let reference = self.state.add_relic(name);

        match reference.base.activation {
            Activation::Immediate => {
                self.eval_effects(&reference.base.effect, Binding::Relic(reference), None);
            }
            Activation::Custom => {
                match name {
                    "War Paint" | "Whetstone" => {
                        let card_type = if name == "War Paint" {
                            CardType::Skill
                        } else {
                            CardType::Attack
                        };
                        let available_cards: Vec<CardReference> = 
                            self
                            .state
                            .deck()
                            .filter(|card_ref| {
                                let card = self.state.get(*card_ref);
                                card_type.matches(card.base._type) && card.upgradable()
                            })
                            .collect();
                
                        let cards = self.probability.choose_multiple(available_cards, 2);
                
                        for card in cards {
                            self.state.get_mut(card).upgrade();
                        }
                    }
                    _ => panic!("Unexpected custom activation")
                };
            }
            _ => {}
        }
    }

    fn add_max_hp(&mut self, amount: u16) {
        self.state.player.max_hp += amount;
        self.heal(amount as f64, CreatureReference::Player)
    }

    fn heal(&mut self, mut amount: f64, creature_ref: CreatureReference) {
        let creature: &mut Creature = match creature_ref {
            CreatureReference::Player => {
                if self.state.has_relic("Mark Of The Bloom")
                {
                    return;
                }

                if self.state.battle_state.active && self.state.has_relic("Magic Flower") {
                    amount *= 1.5;
                }
                &mut self.state.player
            },
            CreatureReference::Creature(_) => {
                let monster = creature_ref.get_monster_mut(&mut self.state).unwrap();
                monster.targetable = true;
                &mut monster.creature
            }
        };
        
        creature.hp = std::cmp::min((amount - 0.0001).ceil() as u16 + creature.hp, creature.max_hp)
    }

    fn add_gold(&mut self, amount: u16) {
        if self.state.has_relic("Ectoplasm") {
            return;
        }

        if self.state.has_relic("Bloody Idol") {
            self.heal(5_f64, CreatureReference::Player);
        }

        self.state.gold += amount;
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
                        4 - std::cmp::min(self.state.battle_state.hp_loss_count, 4)
                    },
                    "Eviscerate" => {
                        3 - std::cmp::min(self.state.battle_state.discard_count, 3)
                    },
                    "Force Field" => {
                        4 - std::cmp::min(self.state.battle_state.power_count, 4)
                    },
                    _ => panic!("Custom cost amount on an unknown card")
                }
            },
            _ => panic!("Unexpected cost amount")
        };

        let upgrades = match self.state.battle_state.active {
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
                    CardType::Attack => self.state.has_relic("Molten Egg"),
                    CardType::Skill => self.state.has_relic("Toxic Egg"),
                    CardType::Power => self.state.has_relic("Frozen Egg"),
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
                target: target,
            }),
        );
    }
    
    pub fn random_relic(&mut self, chest_type: Option<ChestType>) -> &'static str {
        let probabilities = match chest_type {
            None => [50, 33, 17, 0],
            Some(ChestType::Small) => [75, 25, 0, 0],
            Some(ChestType::Medium) => [35, 50, 15, 0],
            Some(ChestType::Large) => [0, 75, 25, 0],
            Some(ChestType::Boss) => [0, 0, 0, 100]
        };
        let rarities = [Rarity::Common, Rarity::Uncommon, Rarity::Rare, Rarity::Boss];

        let choices = rarities.iter().zip(probabilities.iter().copied()).collect_vec();
        
        let rarity = self.probability.choose_weighted(&choices).unwrap();

        let available_relics = models::relics::RELICS
            .values()
            .filter(|relic| {
                relic.rarity == **rarity
                    && (relic.class == self.state.class || relic.class == Class::All)
                    && !self.state.has_relic(&relic.name)
            })
            .map(|relic| &relic.name)
            .collect();

        self.probability.choose(available_relics)
            .expect("No available relics to be chosen!")
    }
    /*

 */
    fn eval_amount(&self, amount: &Amount, binding: Binding) -> i16 {
        match amount {
            Amount::ByAsc { amount, low, high } => {
                let fight_type = binding.get_monster(&self.state).map_or_else(|| self.state.battle_state.fight_type, |a|a.base.fight_type);
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
                    FightType::Elite => {
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
            Amount::EnemyCount => {
                self.state.battle_state.monsters.len() as i16
            }
            Amount::N => binding.get_vars(&self.state).n as i16,
            Amount::NegX => -binding.get_vars(&self.state).x as i16,
            Amount::OrbCount => {
                self.state.battle_state.orbs.len() as i16
            }
            Amount::MaxHp => self.state.get(binding.get_creature()).max_hp as i16,
            Amount::X => binding.get_vars(&self.state).x as i16,
            Amount::PlayerBlock => self.state.player.block as i16,
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

    fn eval_condition(
        &self,
        condition: &Condition,
        binding: Binding,
        action: Option<GameAction>,
    ) -> bool {
        match condition {
            Condition::Act(act) => &self.state.act == act,
            Condition::Always => true,
            Condition::Asc(asc) => &self.state.asc >= asc,
            Condition::Attacking { target } => {
                match self.eval_target(target, binding, action) {
                    CreatureReference::Creature(uuid) => matches!(
                        self.state.battle_state.monsters[&uuid].intent,
                        Intent::Attack
                            | Intent::AttackBuff
                            | Intent::AttackDebuff
                            | Intent::AttackDefend
                    ),
                    _ => panic!("Unexpected target that is not a monster in Condition::Attacking"),
                }
            }
            Condition::Buff { target, buff } => {
                let creature = self.eval_target(target, binding, action);
                self.state.get(creature).buff_names.contains_key(buff)
            }
            Condition::BuffX {
                target,
                buff,
                amount: x,
            } => {
                let val = self.eval_amount(x, binding);
                let creature = self.state.get(self.eval_target(target, binding, action));
            
                if let Some(b) = creature.find_buff(buff) {
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
            Condition::FriendlyDead(name) => {
                self.state.battle_state.monsters.values().any(|m|m.base.name == *name)
            }
            Condition::HalfHp => {
                let creature = self.state.get(match binding {
                    Binding::Creature(creature) => creature,
                    _ => CreatureReference::Player,
                });

                creature.hp * 2 <= creature.max_hp
            }
            Condition::HasCard { location, card } => {
                match location {
                    CardLocation::DeckPile => {
                        self.state.deck().any(|c|c.base._type == *card)
                    }
                    CardLocation::DiscardPile => {
                        self.state.battle_state.discard().any(|c|c.base._type == *card)
                    }
                    CardLocation::PlayerHand => {
                        self.state.battle_state.hand().any(|c|c.base._type == *card)
                    }
                    CardLocation::ExhaustPile => {
                        self.state.battle_state.exhaust().any(|c|c.base._type == *card)
                    }
                    CardLocation::DrawPile => {
                        self.state.battle_state.draw().any(|c|c.base._type == *card)
                    }
                    &CardLocation::Stasis => panic!("Cannot detect if card is in stasis")
                }
            }
            Condition::HasDiscarded => {
                self.state.battle_state.discard_count > 0
            }
            Condition::HasFriendlies(count) => {
                let creature = binding.get_monster(&self.state).expect("Monster did not resolve");
                let friendly_count = self.state
                    .battle_state
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
            Condition::HasGold(amount) => self.state.gold >= self.eval_amount(amount, binding) as u16,
            Condition::HasOrbSlot => {
                self.state.battle_state.orb_slots > 0
            }
            Condition::HasRelic(relic) => self.state.has_relic(relic),
            Condition::HasRemoveableCards { count, card_type } => {
                self.state
                    .deck()
                    .filter(|card_ref| {
                        let card = self.state.get(*card_ref);
                        card.removable() && card.base._type.matches(*card_type)
                    })
                    .count()
                    > *count as usize
            }
            Condition::HasUpgradableCard => self.state.deck.values().any(|card| card.upgradable()),
            Condition::InPosition(position) => {
                if let Some(monster) = binding.get_monster(&self.state) {
                    monster.position == *position
                } else {
                    panic!("Unexpected player in InPosition check")
                }
            },
            Condition::IsVariant(variant) => match binding {
                Binding::Event(event) => {
                    self.state.get(event).variant.as_ref().expect("Expected variant") == variant
                }
                _ => panic!("Unexpected binding!"),
            },
            Condition::LastCard(_type) => {
                match self.state.battle_state.last_card_played
                {
                    Some(last_type) => last_type == *_type,
                    None => false,
                }
            }
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
            Condition::NoBlock => self.state.player.block == 0,
            Condition::Not(condition) => !self.eval_condition(condition, binding, action),
            Condition::OnFloor(floor) => self.state.floor >= *floor,
            Condition::RemainingHp { amount, target } => {
                let creature = self.eval_target(target, binding, action);
                let hp = self.eval_amount(amount, binding);
                self.state.get(creature).hp >= hp as u16
            }
            Condition::Stance(stance) => {
                &self.state.battle_state.stance == stance
            }
            Condition::Upgraded => binding.is_upgraded(&self.state),
        }
    }
}