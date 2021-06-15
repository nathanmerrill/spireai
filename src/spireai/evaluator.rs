use crate::models::cards::BaseCard;
use crate::models::monsters::Move;
use crate::models::{self, core::*, monsters::Intent, relics::Activation, state::*};
use crate::spireai::GamePossibility;
use im::{HashSet, Vector, vector};
use itertools::Itertools;
use uuid::Uuid;

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub struct CardReference {
    pub storage: CardStorage,
    pub uuid: Uuid,
    pub base: &'static models::cards::BaseCard,
}

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub enum CardStorage {
    Deck,
    Battle,
    Stasis
}

impl BindingReference for CardReference {
    type Item = Card;
    fn get(self, state: &GameState) -> &Card {
        match self.storage {
            CardStorage::Deck => state.deck.get(&self.uuid).unwrap(),
            CardStorage::Battle => state.battle_state.cards.get(&self.uuid).unwrap(),
            CardStorage::Stasis => state.card_stasis.get(&self.uuid).unwrap()
        }
    }

    fn get_mut(self, state: &mut GameState) -> &mut Card {
        match self.storage {
            CardStorage::Deck => state.deck.get_mut(&self.uuid).unwrap(),
            CardStorage::Battle => state.battle_state.cards.get_mut(&self.uuid).unwrap(),
            CardStorage::Stasis => state.card_stasis.get_mut(&self.uuid).unwrap()
        }
    }
}

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub enum CreatureReference {
    Player,
    Creature(Uuid),
}

impl BindingReference for CreatureReference {
    type Item = Creature;
    fn get(self, state: &GameState) -> &Creature {
        match self {
            CreatureReference::Creature(uuid) => {
                &state.battle_state.monsters[&uuid].creature
            }
            CreatureReference::Player => &state.player,
        }
    }

    fn get_mut(self, state: &mut GameState) -> &mut Creature {
        match self {
            CreatureReference::Creature(uuid) => {
                &mut state
                    .battle_state
                    .monsters
                    .get_mut(&uuid)
                    .unwrap()
                    .creature
            }
            CreatureReference::Player => &mut state.player,
        }
    }
}

impl CreatureReference {
    pub fn get_monster(self, state: &GameState) -> Option<&Monster> {
        match self {
            CreatureReference::Creature(uuid) => {
                Some(&state.battle_state.monsters[&uuid])
            }
            CreatureReference::Player => None,
        }
    }

    pub fn get_monster_mut(self, state: &mut GameState) -> Option<&mut Monster> {
        match self {
            CreatureReference::Creature(uuid) => state
                .battle_state
                .monsters
                .get_mut(&uuid),
            CreatureReference::Player => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BuffReference {
    pub creature: CreatureReference,
    pub buff: Uuid,
}

impl BindingReference for BuffReference {
    type Item = Buff;
    fn get(self, state: &GameState) -> &Buff {
        let creature = self.creature.get(state);
        creature.buffs.get(&self.buff).unwrap()
    }

    fn get_mut(self, state: &mut GameState) -> &mut Buff {
        let creature = self.creature.get_mut(state);
        creature.buffs.get_mut(&self.buff).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PotionReference {
    pub potion: usize,
}

impl BindingReference for PotionReference {
    type Item = Option<Potion>;
    fn get(self, state: &GameState) -> &Option<Potion> {
        &state.potions[self.potion]
    }

    fn get_mut(self, state: &mut GameState) -> &mut Option<Potion> {
        state.potions.get_mut(self.potion).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelicReference {
    pub relic: Uuid,
}

impl BindingReference for RelicReference {
    type Item = Relic;
    fn get(self, state: &GameState) -> &Relic {
        state.relics.get(&self.relic).unwrap()
    }

    fn get_mut(self, state: &mut GameState) -> &mut Relic {
        state.relics.get_mut(&self.relic).unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventReference {}

impl BindingReference for EventReference {
    type Item = EventState;
    fn get(self, state: &GameState) -> &EventState {
        &state.event_state.as_ref().unwrap()
    }

    fn get_mut(self, state: &mut GameState) -> &mut EventState {
        state.event_state.as_mut().unwrap()
    }
}

pub trait BindingReference {
    type Item;

    fn get(self, state: &GameState) -> &Self::Item;
    fn get_mut(self, state: &mut GameState) -> &mut Self::Item;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Binding {
    Buff(BuffReference),
    Card(CardReference),
    Creature(CreatureReference),
    Potion(PotionReference),
    Relic(RelicReference),
    Event(EventReference),
}

impl Binding {
    fn get_creature(self) -> CreatureReference {
        match self {
            Binding::Buff(buff) => buff.creature,
            Binding::Card(_) => CreatureReference::Player,
            Binding::Potion(_) => CreatureReference::Player,
            Binding::Relic(_) => CreatureReference::Player,
            Binding::Creature(creature) => creature,
            Binding::Event(_) => CreatureReference::Player,
        }
    }

    fn get_monster(self, state: &GameState) -> Option<&Monster> {
        match self {
            Binding::Buff(buff) => match buff.creature {
                CreatureReference::Player => None,
                _ => buff.creature.get_monster(state),
            },
            Binding::Creature(creature) => match creature {
                CreatureReference::Player => None,
                _ => creature.get_monster(state),
            },
            Binding::Card(_) | Binding::Potion(_) | Binding::Relic(_) | Binding::Event(_) => None,
        }
    }

    fn get_vars(self, state: &GameState) -> &Vars {
        match self {
            Binding::Buff(buff) => &buff.get(state).vars,
            Binding::Card(card) => &card.get(state).vars,
            Binding::Creature(creature) => &creature.get_monster(state).unwrap().vars,
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.potion)
            }
            Binding::Event(event) => &event.get(state).vars,
            Binding::Relic(relic) => &relic.get(state).vars,
        }
    }

    fn get_mut_vars(self, state: &mut GameState) -> &mut Vars {
        match self {
            Binding::Buff(buff) => &mut buff.get_mut(state).vars,
            Binding::Card(card) => &mut card.get_mut(state).vars,
            Binding::Creature(creature) => &mut creature.get_monster_mut(state).unwrap().vars,
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.potion)
            }
            Binding::Event(event) => &mut event.get_mut(state).vars,
            Binding::Relic(relic) => &mut relic.get_mut(state).vars,
        }
    }

    fn is_upgraded(self, game_state: &GameState) -> bool {
        match self {
            Binding::Card(card) => card.get(game_state).upgrades > 0,
            Binding::Potion(_) => game_state
                .relic_names
                .contains_key("Sacred Bark"),
            _ => panic!("Unexpected is_upgraded check on {:?}", self),
        }
    }
}

pub fn eval_targets(
    target: &Target,
    state: &mut GamePossibility,
    binding: Binding,
    action: Option<GameAction>) -> Vector<CreatureReference> {
    match target {
        Target::AllEnemies => match binding.get_monster(&state.state) {
            Some(_) => vector![CreatureReference::Player],
            None => state.state.battle_state.available_monsters().collect(),
        },
        Target::AnyFriendly => match binding.get_monster(&state.state) {
            Some(_) => state.state.battle_state.available_monsters().collect(),
            None => vector![CreatureReference::Player],
        },
        Target::RandomEnemy => match binding.get_monster(&state.state) {
            Some(_) => vector![CreatureReference::Player],
            None => {
                random_monster(state).into_iter().collect()
            }
        },
        Target::RandomFriendly => vector![{
            let creature_reference = match binding {
                Binding::Buff(buff) => buff.creature,
                Binding::Creature(creature) => creature,
                _ => CreatureReference::Player,
            };
            match creature_reference {
                CreatureReference::Player => CreatureReference::Player,
                CreatureReference::Creature(uuid) => {
                    let monsters = state.state.battle_state.available_monsters().filter(|a| {
                        if let CreatureReference::Creature(other_uuid) = a {
                            *other_uuid == uuid
                        } else {
                            panic!("Unexpected player reference")
                        }
                    }).collect_vec();

                    return state.choose(monsters).into_iter().collect()
                }
            }
        }],
        _ => vector![eval_target(target, binding, action)]
    }
}

pub fn eval_target(
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

pub fn eval_card_effects(effects: &[CardEffect], card: CardReference, state: &mut GamePossibility) {
    for effect in effects {
        eval_card_effect(effect, card, state);
    }
}

pub fn random_monster(state: &mut GamePossibility) -> Option<CreatureReference> {
    state.choose(state.state.battle_state.available_monsters().collect())
}

pub fn eval_card_effect(effect: &CardEffect, card: CardReference, state: &mut GamePossibility) {
    match effect {
        CardEffect::AutoPlay => {
            let binding = Binding::Card(card);
            let target = if eval_condition(&card.base.targeted, &state.state, binding, None) {
                random_monster(state)
            } else {
                None
            };

            for effect in &card.base.on_play {
                eval_effect(effect, state, binding, Some(GameAction {
                    is_attack: card.base._type == CardType::Attack,
                    creature: CreatureReference::Player,
                    target,
                }))
            }
        }
        CardEffect::CopyTo {location, position, then} => {
            if *location == CardLocation::DeckPile {
                let upgrades = card.get(&state.state).upgrades;
                if let Some(card_ref) = add_card_to_deck(&card.base.name, state) {
                    let card = card_ref.get_mut(&mut state.state);
                    card.upgrades = std::cmp::max(card.upgrades, upgrades);
                    eval_card_effects(then, card_ref, state)
                }
            } else {
                let new_card = copy_card(card.get(&state.state));
                if let Some(card_ref) = insert_card(new_card, *location, *position, state) {
                    eval_card_effects(then, card_ref, state)
                }
            }

        }
        CardEffect::Custom => panic!("Unexpected custom card effect"),
        CardEffect::Discard => {
            if state.state.battle_state.discard.contains(&card.uuid){
                return;
            }

            remove_card_from_location(card, &mut state.state);
            eval_effects(&card.base.on_discard, state, Binding::Card(card), None);
            state.state.battle_state.discard_count += 1;
            add_card_to_location(card, CardLocation::DiscardPile, RelativePosition::Bottom, state);
        }
        CardEffect::Exhaust => {
            if state.state.battle_state.exhaust.contains(&card.uuid){
                return;
            }
            remove_card_from_location(card, &mut state.state);
            eval_effects(&card.base.on_exhaust, state, Binding::Card(card), None);
            add_card_to_location(card, CardLocation::ExhaustPile, RelativePosition::Bottom, state);
        }
        CardEffect::MoveTo {location, position} => {
            remove_card_from_location(card, &mut state.state);
            add_card_to_location(card, *location, *position, state);
        }
        CardEffect::ReduceCost(amount) => {
            let reduction = eval_amount(amount, &state.state, Binding::Card(card));
            let card = card.get_mut(&mut state.state);
            if card.base.cost != Amount::X {
                card.cost = std::cmp::max(card.cost as i16 - reduction, 0) as u8;
                card.base_cost = std::cmp::max(card.base_cost as i16 - reduction, 0) as u8;
            }
        }
        CardEffect::Retain => {
            card.get_mut(&mut state.state).retain = true;
        }
        CardEffect::Upgrade => {
            upgrade_card(card, &mut state.state)
        }
        CardEffect::ZeroCombatCost => {
            let card = card.get_mut(&mut state.state);
            card.cost = 0;
            card.base_cost = 0;
        }
        CardEffect::ZeroCostUntilPlayed => {
            let card = card.get_mut(&mut state.state);
            card.cost = 0;
            card.cost_until_played = true;
        }
        CardEffect::ZeroTurnCost => {
            let card = card.get_mut(&mut state.state);
            card.cost = 0;
        }
    }
}

pub fn insert_card(card: Card, location: CardLocation, position: RelativePosition, state: &mut GamePossibility) -> Option<CardReference> {
    match location {
        CardLocation::DeckPile => {
            if card.base._type == CardType::Curse {
                if let Some(relic) = find_relic_mut("Omamori", &mut state.state) {
                    if relic.vars.x > 0 {
                        relic.vars.x -= 1;
                        return None;
                    }
                }
        
                if state.state
                    .relic_names
                    .contains_key("Darkstone Periapt")
                {
                    add_max_hp(6, &mut state.state);
                }
            }
        
        
            if state.state.relic_names.contains_key("Ceramic Fish") {
                add_gold(9, &mut state.state);
            }
            let card_ref = 
            CardReference {
                storage: CardStorage::Deck,
                uuid: card.uuid,
                base: card.base
            };
        
            state.state.deck.insert(card.uuid, card);
            Some(card_ref)
        }
        _ => {
            let card_ref = CardReference {
                storage: CardStorage::Battle,
                uuid: card.uuid,
                base: card.base
            };
            state.state.battle_state.cards.insert(card.uuid, card);
            add_card_to_location(card_ref, location, position, state);
            Some(card_ref)
        }
    }
}

pub fn upgrade_card(card_ref: CardReference, state: &mut GameState) {
    let card = card_ref.get_mut(state);
    match card.base._type {
        CardType::Status | CardType::Curse => {},
        _ => {
            if card.upgrades == 0 || card.base.name == "Searing Blow" {
                card.upgrades += 1;
                if let Amount::Upgradable {upgraded, ..} = card.base.cost {
                    card.cost = upgraded as u8
                }
            }
        }
    }
}

pub fn remove_card_from_location(card: CardReference, state: &mut GameState) {
    match card.storage {
        CardStorage::Stasis => {
            state.card_stasis.remove(&card.uuid).expect("Expected card in deck!");
        }
        CardStorage::Deck => {
            state.deck.remove(&card.uuid).expect("Expected card in deck!");
        }
        CardStorage::Battle => {
            state.battle_state.hand.remove(&card.uuid).or_else(|| 
                state.battle_state.draw.remove(&card.uuid).or_else(||
                    state.battle_state.discard.remove(&card.uuid).or_else(||
                        state.battle_state.exhaust.remove(&card.uuid)
                    )
                )
            ).expect("Expected card in battle state!");
        }
    };
}

pub fn add_card_to_location(card: CardReference, location: CardLocation, position: RelativePosition, state: &mut GamePossibility) {
    match location {
        CardLocation::DeckPile => { }
        CardLocation::DiscardPile => {
            state.state.battle_state.discard.insert(card.uuid);
        }
        CardLocation::DrawPile => {
            let uuid = card.uuid;
            state.state.battle_state.draw.insert(uuid);
            match position {
                RelativePosition::All => panic!("Unexpected RelativePosition::All when inserting into draw pile"),
                RelativePosition::Bottom => {
                    if state.state.battle_state.draw_top_known.len() == state.state.battle_state.draw.len() - 1 {
                        state.state.battle_state.draw_top_known.push_front(uuid)
                    } else {
                        state.state.battle_state.draw_bottom_known.push_back(uuid)
                    }
                }
                RelativePosition::Top => {
                    state.state.battle_state.draw_top_known.push_back(uuid)
                }
                RelativePosition::Random => {
                    if state.state.relic_names.contains_key("Frozen Eye") {
                        let position = state.range(state.state.battle_state.draw.len());
                        state.state.battle_state.draw_top_known.insert(position, uuid);
                    } else {
                        state.state.battle_state.draw_top_known = Vector::new();
                        state.state.battle_state.draw_bottom_known = Vector::new();
                    }
                }
            };
        }
        CardLocation::ExhaustPile => {
            state.state.battle_state.exhaust.insert(card.uuid);
        }
        CardLocation::PlayerHand => {
            if state.state.battle_state.hand.len() == 10 {
                state.state.battle_state.discard.insert(card.uuid);
            } else {
                state.state.battle_state.hand.insert(card.uuid);
            }
        }
    }
}

pub fn copy_card(card: &Card) -> Card {
    Card {
        base: card.base,
        cost: card.cost,
        base_cost: card.base_cost,
        cost_until_played: card.cost_until_played,
        retain: card.retain,
        uuid: Uuid::new_v4(),
        vars: Vars {
            n: card.vars.n,
            x: card.vars.x,
            n_reset: card.vars.n_reset
        },
        upgrades: card.upgrades,
        bottled: false,
    }
}


pub fn eval_effects(
    effects: &'static [Effect],
    state: &mut GamePossibility,
    binding: Binding,
    action: Option<GameAction>
){
    for effect in effects {
        eval_effect(effect, state, binding, action);
    }
}

pub fn eval_effect(
    effect: &'static Effect,
    state: &mut GamePossibility,
    binding: Binding,
    action: Option<GameAction>
){
    match effect {
        Effect::AddBuff {
            buff: buff_name,
            amount: buff_amount,
            target,
        } => {
            let amount = eval_amount(buff_amount, state.into(), binding);
            for creature in eval_targets(target, state, binding, action)
            {
                add_buff(creature.get_mut(state.into()), buff_name, amount)
            }
        }
        Effect::AddEnergy(energy_amount) => {
            let amount = eval_amount(energy_amount, state.into(), binding) as u8;
            state.state.battle_state.energy += amount
        }
        Effect::AddGold(gold_amount) => {
            let amount = eval_amount(gold_amount, state.into(), binding) as u16;
            add_gold(amount, state.into())
        }
        Effect::AddMaxHp(hp_amount) => {
            let amount = eval_amount(hp_amount, state.into(), binding) as u16;
            add_max_hp(amount, state.into())
        }
        Effect::AddN(n_amount) => {
            let amount = eval_amount(n_amount, state.into(), binding);
            binding.get_mut_vars(state.into()).n += amount;
        }
        Effect::AddOrbSlot(amount) => {
            let count = eval_amount(amount, state.into(), binding) as u8;
            state.state.battle_state.orb_slots = std::cmp::min(count + state.state.battle_state.orb_slots, 10) - state.state.battle_state.orb_slots;            
        }
        Effect::AddPotionSlot(amount) => {
            for _ in 0 .. *amount {
                state.state.potions.push_back(None)
            }
        }
        Effect::AddRelic(name) => {
            add_relic(name, state);
        }
        Effect::AddX(amount ) => {
            binding.get_mut_vars(state.into()).x += eval_amount(amount, state.into(), binding);
        }
        Effect::AttackDamage {amount, target, if_fatal, times} => {
            let attack_amount = eval_amount(&amount, state.into(), binding);
            
            for creature in eval_targets(&target, state, binding, action) {
                for _ in 0 .. eval_amount(times, state.into(), binding){
                    if damage(attack_amount as u16, creature, state.into()) {
                        eval_effects(if_fatal, state, binding, action);
                    }
                }
            }
        }
        Effect::Block {amount, target} => {
            let block_amount = eval_amount(amount, state.into(), binding) as u16;

            for creature in eval_targets(target, state, binding, action) {
                let mut_creature = creature.get_mut(state.into());
                let new_block = std::cmp::min(mut_creature.block + block_amount, 999);
                mut_creature.block = new_block;
            }
        }
        Effect::ChannelOrb(orb_type) => {
            channel_orb(*orb_type, state)
        }
        Effect::ChooseCardByType {
            location, 
            _type, 
            rarity, 
            class, 
            position, 
            then, 
            choices,
            exclude_healing,
        } => {
            let amount = eval_amount(&choices, &state.state, binding);

            let choice = random_cards_by_type(amount as usize, *class, *_type, *rarity, *exclude_healing, state);

            let mut card_choices = Vector::new();
            for card in choice {
                card_choices.push_back(create_card(&card.name, &state.state));
            }
            state.state.card_choices = card_choices.iter().map(|card| CardReference {
                storage: CardStorage::Stasis,
                uuid: card.uuid,
                base: card.base
            }).collect();

            state.state.card_stasis = card_choices.into_iter().map(|card| (card.uuid, card)).collect();
            state.state.card_choice_range = Some((1, 1));
            
            state.state.card_choice_type = CardChoiceType::AddToLocation(*location, *position, then.clone());

        }
        Effect::ChooseCards{location, then, min, max} => {
            let min_count = eval_amount(min, &state.state, binding);
            let max_count = eval_amount(max, &state.state, binding);
            match location {
                CardLocation::DeckPile => state.state.card_choices = state.state.deck().collect(),
                CardLocation::DiscardPile => state.state.card_choices = state.state.battle_state.discard().collect(),
                CardLocation::DrawPile => state.state.card_choices = state.state.battle_state.draw().collect(),
                CardLocation::ExhaustPile => state.state.card_choices = state.state.battle_state.exhaust().collect(),
                CardLocation::PlayerHand => state.state.card_choices = state.state.battle_state.hand().collect(),
            }
            state.state.card_choice_range = Some((min_count as usize, max_count as usize));
            state.state.card_choice_type = CardChoiceType::Then(then);
        }
        Effect::CreateCard{name, location, position, then} => {
            let card = create_card(name, &state.state);
            if let Some(card_ref) = insert_card(card, *location, *position, state){
                eval_card_effects(then, card_ref, state);
            }            
        }
        Effect::CreateCardByType{location, _type, rarity, class, position, exclude_healing, then} => {
            let card = random_cards_by_type(1, *class, *_type, *rarity, *exclude_healing, state)[0];
            if let Some(card_ref) = insert_card(create_card(&card.name, &state.state), *location, *position, state) {
                eval_card_effects(then, card_ref, state);
            }
        }
        Effect::Custom => unimplemented!(),
        Effect::Damage {amount, target} => {
            damage(eval_amount(amount, &state.state, binding) as u16, eval_target(target, binding, action), &mut state.state);
        }
        Effect::Die {target} => {
            die(eval_target(target, binding, action), &mut state.state);
        }
        Effect::DoCardEffect {location, position, effect} => {
            for card in get_cards_in_location(*location, *position, state) {
                eval_card_effect(effect, card, state)
            }
        }
        Effect::Draw(amount) => {
            let n = eval_amount(amount, &state.state, binding);
            draw(n as u8, state);
        }
        Effect::EvokeOrb(amount) => {
            evoke_orb(eval_amount(amount, &state.state, binding) as u8, state)
        }
        Effect::LoseHpPercentage(amount) => {
            let percentage = eval_amount(amount, &state.state, binding) as f64 / 1000.0;
            let damage = (state.state.player.max_hp as f64 * percentage).floor() as u16;
            lose_hp(damage, CreatureReference::Player, &mut state.state);
        }
        
        
        _ => unimplemented!(),
    }
}

pub fn end_turn(state: &mut GamePossibility) {
    eval_when(When::BeforeHandDiscard, CreatureReference::Player, state);
    let has_runic_pyramid = state.state.relic_names.contains_key("Runic Pyramid");
    for card_ref in state.state.battle_state.hand().collect_vec() {
        let binding = Binding::Card(card_ref);
        if card_ref.get(&state.state).retain {
            eval_effects(&card_ref.base.on_retain, state, binding, None);
            if !eval_condition(&card_ref.base.retain, &state.state, binding, None) {
                card_ref.get_mut(&mut state.state).retain = false;
            }
        } else if !has_runic_pyramid {
            remove_card_from_location(card_ref, &mut state.state);
            add_card_to_location(card_ref, CardLocation::DiscardPile, RelativePosition::Top, state);
        }
        eval_effects(&card_ref.base.on_turn_end, state, binding, None);
    }
    eval_when(When::BeforeEnemyMove, CreatureReference::Player, state);
    for creature in state.state.battle_state.available_monsters().collect_vec() {
        next_monster_move(creature, state);
    }
}

fn next_monster_move(creature: CreatureReference, state: &mut GamePossibility) {
    eval_effects(&creature.get_monster(&state.state).unwrap().current_move.effects, state, Binding::Creature(creature), None);
    next_move(creature, state);
}

fn next_move(creature: CreatureReference, state: &mut GamePossibility) {
    let monster = creature.get_monster_mut(&mut state.state).unwrap();
    if let Some(last_move) = monster.last_move{
        if last_move == monster.current_move {
            monster.last_move_count += 1;
        } else {
            monster.last_move = Some(last_move);
            monster.last_move_count = 1;
        }
    };

    set_monster_move(monster.index+1, monster.phase, creature, state);    
}

fn draw(n: u8, state: &mut GamePossibility) {
    let mut cards = Vec::new();
    for _ in  0 .. n {
        if state.state.battle_state.draw.is_empty() {
            shuffle(state);
        }
        if state.state.battle_state.draw.is_empty() {
            break;
        }
        let next_card = state.state.battle_state.draw_top_known.pop_back();

        let uuid = match next_card {
            Some(uuid) => {
                uuid
            }
            None => {
                let choices = state.state.battle_state.draw.iter().cloned().collect_vec();
                state.choose(choices).unwrap()
            }
        };
        
        state.state.battle_state.draw.remove(&uuid);
        let card = state.state.battle_state.cards.get(&uuid).unwrap().base;
        cards.push(CardReference {
            storage: CardStorage::Battle,
            uuid,
            base: card,
        });
    }

    for card in cards {
        eval_effects(&card.base.on_draw, state, Binding::Card(card), None);
        eval_when(When::DrawCard(card.base._type), CreatureReference::Player, state);
        eval_when(When::DrawCard(CardType::All), CreatureReference::Player, state);
    }
}

fn shuffle(state: &mut GamePossibility) {
    unimplemented!()
}

fn get_cards_in_location(location: CardLocation, position: RelativePosition, state: &mut GamePossibility) -> Vec<CardReference> {
    match position {
        RelativePosition::All => {
            match location {
                CardLocation::DeckPile => state.state.deck().collect_vec(),
                CardLocation::DiscardPile => state.state.battle_state.discard().collect_vec(),
                CardLocation::ExhaustPile => state.state.battle_state.exhaust().collect_vec(),
                CardLocation::PlayerHand => state.state.battle_state.hand().collect_vec(),
                CardLocation::DrawPile => state.state.battle_state.draw().collect_vec()
            }
        }
        RelativePosition::Random => {
            let items = match location {
                CardLocation::DeckPile => state.state.deck().collect_vec(),
                CardLocation::DiscardPile => state.state.battle_state.discard().collect_vec(),
                CardLocation::ExhaustPile => state.state.battle_state.exhaust().collect_vec(),
                CardLocation::PlayerHand => state.state.battle_state.hand().collect_vec(),
                CardLocation::DrawPile => state.state.battle_state.draw().collect_vec()
            };

            state.choose(items).into_iter().collect_vec()
        }
        RelativePosition::Top => {
            match location {
                CardLocation::DrawPile => {
                    let uuid = state.state.battle_state.draw_top_known.back().cloned().unwrap_or_else(|| {
                        let draw_top: HashSet<Uuid> = state.state.battle_state.draw_top_known.iter().cloned().collect();
                        let difference = state.state.battle_state.draw.iter().filter(|uuid| draw_top.contains(uuid)).cloned().collect_vec();
                        state.choose(difference).unwrap()
                    });

                    vec![CardReference {
                        storage: CardStorage::Battle,
                        uuid,
                        base: state.state.battle_state.cards[&uuid].base
                    }]
                },
                _ => panic!("Unepxected location in RelativePosition::Bottom")
            }
        }
        RelativePosition::Bottom => {
            match location {
                CardLocation::DrawPile => {
                    if state.state.battle_state.draw_top_known.len() == state.state.battle_state.draw.len() {
                        state.state.battle_state.draw_top_known.front().map(|uuid| CardReference {
                            storage: CardStorage::Battle,
                            uuid: *uuid,
                            base: state.state.battle_state.cards[uuid].base
                        }).into_iter().collect_vec()
                    } else {
                        let uuid = 
                        state.state.battle_state.draw_bottom_known.back().cloned().unwrap_or_else(|| {
                            let draw_top: HashSet<Uuid> = state.state.battle_state.draw_top_known.iter().cloned().collect();
                            let difference = state.state.battle_state.draw.iter().filter(|uuid| draw_top.contains(uuid)).cloned().collect_vec();
                            state.choose(difference).unwrap()
                        });
                        vec![CardReference {
                            storage: CardStorage::Battle,
                            uuid,
                            base: state.state.battle_state.cards[&uuid].base
                        }]
                    }
                },
                _ => panic!("Unepxected location in RelativePosition::Bottom")
            }
        }
    }
}

fn random_cards_by_type(amount: usize, class: Option<Class>, _type: CardType, rarity: Option<Rarity>, exclude_healing: bool, state: &mut GamePossibility) -> Vec<&'static BaseCard> {
    let cards = models::cards::available_cards_by_class(class.unwrap_or(state.state.class)).iter().filter(|card| {
        (_type == CardType::All || card._type == _type) &&
        (rarity == None || rarity.unwrap() == card.rarity) &&
        (!exclude_healing || 
        !matches!(card.name.as_str(),
            "Feed" | "Reaper" | "Lesson Learned" | "Alchemize" | "Wish" | "Bandage Up" | "Self Repair"))
    }).cloned();

    state.choose_multiple(cards.collect(), amount as usize)
}

fn channel_orb(orb_type: OrbType, state: &mut GamePossibility) {
    if state.state.battle_state.orbs.len() == state.state.battle_state.orb_slots as usize {
        evoke_orb(1, state);
    }

    let n = match orb_type {
        OrbType::Any => panic!("Unexpected Any orb type"),
        OrbType::Dark => {
            let focus = get_buff_amount("Focus", CreatureReference::Player, &state.state);
            std::cmp::max(focus + 6, 0) as u16
        },
        _ => 0
    };

    let orb = Orb {
        base: orb_type,
        n
    };

    state.state.battle_state.orbs.push_back(orb);
}

fn add_block(amount: u16, target: CreatureReference, state: &mut GamePossibility) {
    let mut_creature = target.get_mut(state.into());
    let new_block = std::cmp::min(mut_creature.block + amount, 999);
    mut_creature.block = new_block;
    eval_when(When::OnBlock, target, state)
}

fn eval_when(when: When, target: CreatureReference, state: &mut GamePossibility) {
    eval_creature_buff_when(target, when.clone(), state);

    if target == CreatureReference::Player {
        eval_relic_when(when, state);    
    } else {
        eval_monster_when(target, when, state);
    }
}

fn eval_monster_when(creature: CreatureReference, when: When, state: &mut GamePossibility) {
    if let Some(monster ) = creature.get_monster(&state.state){
        if let Some(phase) = monster.whens.get(&when) {
            set_monster_phase(phase, creature, state);
        }
    }
}

fn set_monster_phase(phase: &str, creature: CreatureReference, state: &mut GamePossibility) {
    let new_phase = creature.get_monster(&state.state).unwrap().base.phases.iter().position(|p| p.name == phase).unwrap();
    set_monster_move(0, new_phase, creature, state);
}

fn set_monster_move(mut move_index: usize, mut phase_index: usize, creature: CreatureReference, state: &mut GamePossibility) {
    let (base, last_move, last_move_count) = {
        let monster = creature.get_monster(&state.state).unwrap();
        (monster.base, monster.last_move, monster.last_move_count)
    };
    let binding = Binding::Creature(creature);

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
                    let max_repeats = eval_amount(&p.max_repeats, &state.state, binding) as u8;
                    let same_move = last_move.map_or(false, |a| a.name == p.name);
                    let maxxed_out = same_move && last_move_count >= max_repeats;
                    !maxxed_out
                }).map(|a| {
                    let weight = eval_amount(&a.weight, &state.state, binding) as u8;
                    (&a.name, weight)
                }).collect_vec();

                state.choose_weighted(&available_probabilites).cloned()
            }
            Move::If{condition, then_phase, else_phase} => {
                let next_phase = if eval_condition(&condition, &state.state, binding, None) {
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

    let monster = creature.get_monster_mut(&mut state.state).unwrap();

    monster.current_move = monster.base.moveset.iter().find(|a| &a.name == next_move).unwrap();
    monster.index = move_index;
    monster.phase = phase_index;
}

fn eval_relic_when(when: When, state: &mut GamePossibility) {
    if let Some(relic_ids) = state.state.relic_whens.get(&when).cloned() {
        for relic_id in relic_ids {
            let relic_ref = RelicReference {
                relic: relic_id
            };
            let (base, mut x, mut enabled) = {
                let relic = relic_ref.get(&state.state);
                (relic.base, relic.vars.x as u16, relic.enabled)
            };

            match &base.activation {
                Activation::Counter {increment, reset, auto_reset, target} => {
                    if increment == &when && x < *target {
                        x += 1;
                        if x == *target {
                            eval_effects(&base.effect, state, Binding::Relic(relic_ref), None);
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
                        eval_effects(&base.effect, state, Binding::Relic(relic_ref), None);
                    }
                }
                Activation::When(_) => {
                    eval_effects(&base.effect, state, Binding::Relic(relic_ref), None);
                }
                Activation::WhenEnabled {activated_at, enabled_at, disabled_at} => {
                    if activated_at == &when && enabled {
                        eval_effects(&base.effect, state, Binding::Relic(relic_ref), None);                                
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
                let relic = relic_ref.get_mut(&mut state.state);
                relic.vars.x = x as i16;
                relic.enabled = enabled;
            }
        } 
    }
}

fn eval_creature_buff_when(creature: CreatureReference, when: When, state: &mut GamePossibility) {
    if let Some(buff_ids) = creature.get(&state.state).buffs_when.get(&when).cloned() {
        for buff_id in buff_ids {
            let buff_ref = BuffReference {
                creature,
                buff: buff_id
            };

            let base = buff_ref.get(&state.state).base;

            for WhenEffect { when: _when, effect} in &base.effects {
                if when == *_when {
                    eval_effects(effect, state, Binding::Buff(buff_ref), None);
                }
            }
            
            if base.expire_at == when {
                remove_buff(buff_ref, &mut state.state);
            } else if base.reduce_at == when {
                let should_remove = {
                    let buff = buff_ref.get_mut(&mut state.state);
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
                    remove_buff(buff_ref, &mut state.state);
                }
            }
        }
    }
}

fn remove_buff(buff: BuffReference, state: &mut GameState) {
    let creature = buff.creature.get_mut(state);

    let removed = creature.buffs.remove(&buff.buff).unwrap();
    creature.buff_names.remove(&removed.base.name);
    
    for (_, uuids) in creature.buffs_when.iter_mut() {
        if let Some(index) = uuids.index_of(&buff.buff) {
            uuids.remove(index);
        }
    }
}

fn evoke_orb(times: u8, state: &mut GamePossibility) {
    if let Some(orb) = state.state.battle_state.orbs.pop_front() {
        match orb.base {
            OrbType::Any => panic!("Unexpected OrbType of any"),
            OrbType::Dark => {
                for _ in 0 .. times {
                    let lowest_monster =  
                        state.state.battle_state.monsters.values()
                            .filter(|m| m.targetable)
                            .min_by_key(|m|m.creature.hp)
                            .map(|m|m.uuid);

                    if let Some(uuid) = lowest_monster {
                        let mut orb_damage = orb.n as f64;
                        let creature = CreatureReference::Creature(uuid);
                        if has_buff("Lock On", creature, &state.state) {
                            orb_damage *= 1.5;
                        }

                        damage(orb_damage.floor() as u16, creature, &mut state.state);
                    }
                }
            }
            OrbType::Frost => {
                let focus = get_buff_amount("Focus", CreatureReference::Player, &state.state);
                let block_amount = std::cmp::max(focus+5, 0) as u16;
                
                for _ in 0 .. times {
                    add_block(block_amount, CreatureReference::Player, state);
                }
            }
            OrbType::Lightning => {
                let has_electro_dynamics = has_buff("Electro", CreatureReference::Player, &state.state);
                let focus = get_buff_amount("Focus", CreatureReference::Player, &state.state);
                let orb_damage = std::cmp::max(8 + focus, 0) as f64;
                for _ in 0 .. times {
                    if has_electro_dynamics {
                        for monster in state.state.battle_state.available_monsters().collect_vec() {
                            let multiplier = if has_buff("Lock On", monster, &state.state) {
                                1.5
                            } else {
                                1.0
                            };
                            
                            damage((orb_damage * multiplier).floor() as u16, monster, &mut state.state);
                        }
                    } else {
                        let monsters = state.state.battle_state.available_monsters().collect_vec();
                        if let Some(creature) = state.choose(monsters) {
                            let multiplier = if has_buff("Lock On", creature, &state.state) {
                                1.5
                            } else {
                                1.0
                            };
                            
                            damage((orb_damage * multiplier).floor() as u16, creature, &mut state.state);
                        }
                    }
                }
            }
            OrbType::Plasma => {
                state.state.battle_state.energy += 2 * times
            }
        }
    }
}

fn get_buff_amount(name: &str, creature: CreatureReference, state: &GameState) -> i16 {
    find_buff(name, creature, state).map_or(0, |b|b.get(state).vars.n)
}

fn has_buff(name: &str, creature: CreatureReference, state: &GameState) -> bool {
    creature.get(state).buff_names.contains_key(name)
}

fn find_buff(name: &str, creature: CreatureReference, state: &GameState) -> Option<BuffReference> {
    creature.get(state).buff_names.get(name)
        .map(|uuid| BuffReference{
            creature: CreatureReference::Player,
            buff: *uuid
        })
}


fn damage(amount: u16, creature_ref: CreatureReference, state: &mut GameState) -> bool {
    let mut block = creature_ref.get(state).block;
    if block < amount as u16 {
        if lose_hp(amount - block, creature_ref, state) {
            return true;
        }
        block = 0
    } else {
        block -= amount
    }

    creature_ref.get_mut(state).block = block;

    false
}

fn lose_hp(amount: u16, creature_ref: CreatureReference, state: &mut GameState) -> bool {
    let new_hp = std::cmp::max(creature_ref.get(state).hp - amount, 0);
    creature_ref.get_mut(state).hp = new_hp;
    if creature_ref == CreatureReference::Player {
        state.battle_state.hp_loss_count += 1;
        
    }

    if new_hp == 0 {
        die(creature_ref, state)
    } else {
        false
    }
}

fn die(creature_ref: CreatureReference, state: &mut GameState) -> bool {
    match creature_ref {
        CreatureReference::Player => {
            let recovery: f64 = 
            if let Some(idx) = find_potion("Fairy In A Bottle", state) {
                state.potions[idx] = None;
                if state.relic_names.contains_key("Sacred Bark") {
                    0.6
                } else {
                    0.3
                }
            } else if let Some(relic) = find_relic_mut("Lizard Tail", state) {
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
                let max_hp = creature_ref.get(state).max_hp;
                heal(max_hp as f64 * recovery, state);
            }
            
            if state.player.hp == 0 {
                state.won = Some(false);
                true
            } else {
                false
            }
        }
        CreatureReference::Creature(uuid) => {
            let monster = &state.battle_state.monsters[&uuid];
            
            unimplemented!()
        }
    }
}

fn find_potion(name: &str, state: &GameState) -> Option<usize>{
    state.potions.iter().position(|p| match p {
        Some(potion) => potion.base.name == name,
        None => false
    })
}

pub fn add_relic(name: &str, state: &mut GamePossibility) {
    let relic = create_relic(name);
    state.state.relic_names.insert(relic.base.name.to_string(), relic.uuid);
    let whens = match &relic.base.activation {
        Activation::Immediate => {
            eval_effects(&relic.base.effect, state, Binding::Relic(RelicReference{relic: relic.uuid}), None);
            vec![]
        }
        Activation::Counter{increment, reset, ..} => {
            if increment == reset {
                vec![increment]
            } else {
                vec![increment, reset]
            }
        }
        Activation::Uses{use_when, ..} => vec![use_when],
        Activation::When(when) => vec![when],
        Activation::WhenEnabled{activated_at, enabled_at, disabled_at} => {
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
        },
        Activation::Custom => {
            match name {
                "War Paint" | "Whetstone" => {
                    let card_type = if name == "War Paint" {
                        CardType::Skill
                    } else {
                        CardType::Attack
                    };
                    let available_cards: Vec<CardReference> = state
                        .state
                        .deck()
                        .filter(|card| card_types_match(*card, card_type) && card_upgradable(*card, &state.state))
                        .collect();
            
                    let cards = state.choose_multiple(available_cards, 2);
            
                    for card in cards {
                        upgrade_card(card, state.into());
                    }
                }
                _ => panic!("Unexpected custom activation")
            };
            vec![]
        }
    };

    for when in whens {
        state.state.relic_whens.entry(when.clone()).or_insert_with(Vector::new).push_back(relic.uuid)
    }
    state.state.relics.insert(relic.uuid, relic);
}

pub fn add_card_to_deck(name: &str, state: &mut GamePossibility) -> Option<CardReference> {
    insert_card(create_card(name, &state.state), CardLocation::DeckPile, RelativePosition::Bottom, state)
}

pub fn find_relic_mut<'a>(name: &str, state: &'a mut GameState) -> Option<&'a mut Relic> {
    if let Some(uuid) = state.relic_names.get(name) {
        Some(state.relics.get_mut(uuid).unwrap())
    } else {
        None
    }
}

pub fn find_relic<'a>(name: &str, state: &'a GameState) -> Option<&'a Relic> {
    state.relic_names.get(name).map(|uuid|state.relics.get(uuid).unwrap())
}

pub fn add_max_hp(amount: u16, state: &mut GameState) {
    state.player.max_hp += amount;
    heal(amount as f64, state)
}

pub fn heal(mut amount: f64, state: &mut GameState) {
    if state
        .relic_names
        .contains_key("Mark Of The Bloom")
    {
        return;
    }

    if state.battle_state.active && state.relic_names.contains_key("Magic Flower") {
        amount *= 1.5;
    }

    state.player.hp = std::cmp::min((amount - 0.0001).ceil() as u16 + state.player.hp, state.player.max_hp);
}

pub fn add_gold(amount: u16, state: &mut GameState) {
    if state.relic_names.contains_key("Ectoplasm") {
        return;
    }

    if state.relic_names.contains_key("Bloody Idol") {
        heal(5_f64, state);
    }

    state.gold += amount;
}

fn add_buff(creature: &mut Creature, name: &str, amount: i16) {
    if let Some(uuid) = creature.buff_names.get(name) {
        let buff = creature.buffs.get_mut(uuid).unwrap();
        if !buff.base.repeats {
            if !buff.base.singular {
                buff.vars.x += amount
            }
        } else {
            buff.stacked_vars.push(Vars {
                n: 0,
                n_reset: 0,
                x: amount,
            })
        }
    } else {
        let new_buff = create_buff(name, amount);
        for effects in &new_buff.base.effects {
            creature.buffs_when.entry(effects.when.clone()).or_insert_with(Vector::new).push_back(new_buff.uuid)
        }
        creature.buff_names.insert(name.to_string(), new_buff.uuid);
        creature.buffs.insert(new_buff.uuid, new_buff);
    }
}

fn empty_vars() -> Vars {
    Vars {
        n: 0,
        n_reset: 0,
        x: 0,
    }
}

pub fn create_card(name: &str, state: &GameState) -> Card {
    let base = models::cards::by_name(name);
    let uuid = Uuid::new_v4();

    let cost = match base.cost {
        Amount::Fixed(cost) => cost as u8,
        Amount::Upgradable{amount, ..} => amount as u8,
        Amount::X => 0,
        Amount::Custom => {
            match name {
                "Blood for Blood" => {
                    4 - std::cmp::min(state.battle_state.hp_loss_count, 4)
                },
                "Eviscerate" => {
                    3 - std::cmp::min(state.battle_state.discard_count, 3)
                },
                "Force Field" => {
                    4 - std::cmp::min(state.battle_state.power_count, 4)
                },
                _ => panic!("Custom cost amount on an unknown card")
            }
        },
        _ => panic!("Unexpected cost amount")
    };

    let upgrades = match state.battle_state.active {
        true => {
            if state.player.buff_names.contains_key("Master Reality") {
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
                CardType::Attack => state.relic_names.contains_key("Molten Egg"),
                CardType::Skill => state.relic_names.contains_key("Toxic Egg"),
                CardType::Power => state.relic_names.contains_key("Frozen Egg"),
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

pub fn create_relic(name: &str) -> Relic {
    let base = models::relics::by_name(name);
    let uuid = Uuid::new_v4();
    let mut relic = Relic {
        base,
        uuid,
        vars: empty_vars(),
        enabled: true,
    };
    relic.vars.x = base.starting_x;
    relic
}

pub fn create_potion(name: &str) -> Potion {
    Potion {
        base: models::potions::by_name(name),
    }
}

pub fn create_buff(name: &str, amount: i16) -> Buff {
    let base = models::buffs::by_name(name);
    if !base.repeats {
        Buff {
            base,
            uuid: Uuid::new_v4(),
            vars: Vars {
                n: 0,
                n_reset: 0,
                x: amount,
            },
            stacked_vars: vec![],
        }
    } else {
        Buff {
            base,
            uuid: Uuid::new_v4(),
            vars: empty_vars(),
            stacked_vars: vec![Vars {
                n: 0,
                n_reset: 0,
                x: amount,
            }],
        }
    }
}

pub fn card_upgradable(card_ref: CardReference, state: &GameState) -> bool {
    let card = card_ref.get(state);
    match card.base._type {
        CardType::Attack | CardType::Skill | CardType::Power => {
            card.upgrades == 0 && card.base.name != "Searing Blow"
        }
        CardType::Status => false,
        CardType::Curse => false,
        CardType::All => panic!("Unexpected All on card type"),
    }
}

pub fn card_removable(card_ref: CardReference, state: &GameState) -> bool {
    let card = card_ref.get(state);
    if card.bottled {
        return false;
    }
    card.base.name == "Ascender's Bane"
        || card.base.name == "Curse of the Bell"
        || card.base.name == "Necronomicurse"
}

pub fn card_playable(
    reference: CardReference,
    battle_state: &BattleState,
    state: &GameState,
) -> bool {
    let card = reference.get(state);
    card.cost <= battle_state.energy
        && eval_condition(
            &card.base.playable_if,
            state,
            Binding::Card(reference),
            None,
        )
}

pub fn eval_amount(amount: &Amount, state: &GameState, binding: Binding) -> i16 {
    match amount {
        Amount::ByAsc { amount, low, high } => {
            match state.battle_state.battle_type {
                BattleType::Common | BattleType::Event => {
                    if state.asc >= 17 {
                        *high
                    } else if state.asc >= 2 {
                        *low
                    } else {
                        *amount
                    }
                }
                BattleType::Elite => {
                    if state.asc >= 18 {
                        *high
                    } else if state.asc >= 3 {
                        *low
                    } else {
                        *amount
                    }
                }
                BattleType::Boss => {
                    if state.asc >= 19 {
                        *high
                    } else if state.asc >= 4 {
                        *low
                    } else {
                        *amount
                    }
                }
            }
        }
        Amount::Custom => panic!("Unhandled custom amount: {:?}", binding),
        Amount::EnemyCount => {
            state.battle_state.monsters.len() as i16
        }
        Amount::N => binding.get_vars(state).n as i16,
        Amount::NegX => -binding.get_vars(state).x as i16,
        Amount::OrbCount => {
            state.battle_state.orbs.len() as i16
        }
        Amount::MaxHp => binding.get_creature().get(state).max_hp as i16,
        Amount::X => binding.get_vars(state).x as i16,
        Amount::PlayerBlock => state.player.block as i16,
        Amount::Fixed(amount) => *amount,
        Amount::Mult(amount_mult) => {
            let mut product = 1;
            for amount in amount_mult {
                product *= eval_amount(amount, state, binding);
            }
            product
        }
        Amount::Sum(amount_sum) => {
            let mut sum = 0;
            for amount in amount_sum {
                sum += eval_amount(amount, state, binding);
            }
            sum
        }
        Amount::Upgradable { amount, upgraded } => match binding.is_upgraded(state) {
            true => *upgraded,
            false => *amount,
        },
    }
}

pub fn eval_condition(
    condition: &Condition,
    state: &GameState,
    binding: Binding,
    action: Option<GameAction>,
) -> bool {
    match condition {
        Condition::Act(act) => &state.act == act,
        Condition::Always => true,
        Condition::Asc(asc) => &state.asc >= asc,
        Condition::Attacking { target } => {
            match eval_target(target, binding, action) {
                CreatureReference::Creature(uuid) => matches!(
                    state.battle_state.monsters[&uuid].intent,
                    Intent::Attack
                        | Intent::AttackBuff
                        | Intent::AttackDebuff
                        | Intent::AttackDefend
                ),
                _ => panic!("Unexpected target that is not a monster in Condition::Attacking"),
            }
        }
        Condition::Buff { target, buff } => {
            let creature = eval_target(target, binding, action);
            creature.get(state).buff_names.contains_key(buff)
        }
        Condition::BuffX {
            target,
            buff,
            amount: x,
        } => {
            let val = eval_amount(x, state, binding);
            let creature = eval_target(target, binding, action);
            
            if let Some(b) =  find_buff(buff, creature, state) {
                b.get(state).vars.x >= val
            } else { 
                false
            }
        }
        Condition::Class(class) => state.class == *class,
        Condition::Custom => panic!("Unhandled custom condition: {:?}", binding),
        Condition::Equals(amount1, amount2) => {
            eval_amount(amount1, state, binding) == eval_amount(amount2, state, binding)
        }
        Condition::FriendlyDead(name) => {
            state.battle_state.monsters.values().any(|m|m.base.name == *name)
        }
        Condition::HalfHp => {
            let creature = match binding {
                Binding::Creature(creature) => creature,
                _ => CreatureReference::Player,
            }.get(state);

            creature.hp * 2 <= creature.max_hp
        }
        Condition::HasCard { location, card } => {
            match location {
                CardLocation::DeckPile => {
                    state.deck().any(|c|c.base._type == *card)
                }
                CardLocation::DiscardPile => {
                    state.battle_state.discard().any(|c|c.base._type == *card)
                }
                CardLocation::PlayerHand => {
                    state.battle_state.hand().any(|c|c.base._type == *card)
                }
                CardLocation::ExhaustPile => {
                    state.battle_state.exhaust().any(|c|c.base._type == *card)
                }
                CardLocation::DrawPile => {
                    state.battle_state.draw().any(|c|c.base._type == *card)
                }
            }
        }
        Condition::HasDiscarded => {
            state.battle_state.discard_count > 0
        }
        Condition::HasFriendlies(count) => {
            let creature = binding.get_monster(state).expect("Monster did not resolve");
            let friendly_count = state
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
        Condition::HasGold(amount) => state.gold >= eval_amount(amount, state, binding) as u16,
        Condition::HasOrbSlot => {
            state.battle_state.orb_slots > 0
        }
        Condition::HasRelic(relic) => state.relic_names.contains_key(relic),
        Condition::HasRemoveableCards { count, card_type } => {
            state
                .deck()
                .filter(|card| card_removable(*card, state) && card_types_match(*card, *card_type))
                .count()
                > *count as usize
        }
        Condition::HasUpgradableCard => state.deck().any(|card| card_upgradable(card, state)),
        Condition::InPosition(position) => {
            if let Some(monster) = binding.get_monster(state) {
                monster.position == *position
            } else {
                panic!("Unexpected player in InPosition check")
            }
        },
        Condition::IsVariant(variant) => match binding {
            Binding::Event(event) => {
                event.get(state).variant.as_ref().expect("Expected variant") == variant
            }
            _ => panic!("Unexpected binding!"),
        },
        Condition::LastCard(_type) => {
            match state.battle_state.last_card_played
            {
                Some(last_type) => last_type == *_type,
                None => false,
            }
        }
        Condition::LessThan(amount1, amount2) => {
            eval_amount(amount1, state, binding) < eval_amount(amount2, state, binding)
        }
        Condition::MultipleAnd(conditions) => conditions
            .iter()
            .all(|condition| eval_condition(condition, state, binding, action)),
        Condition::MultipleOr(conditions) => conditions
            .iter()
            .any(|condition| eval_condition(condition, state, binding, action)),
        Condition::Never => false,
        Condition::NoBlock => state.player.block == 0,
        Condition::Not(condition) => !eval_condition(condition, state, binding, action),
        Condition::OnFloor(floor) => state.floor >= *floor,
        Condition::RemainingHp { amount, target } => {
            let creature = eval_target(target, binding, action);
            let hp = eval_amount(amount, state, binding);
            creature.get(state).hp >= hp as u16
        }
        Condition::Stance(stance) => {
            &state.battle_state.stance == stance
        }
        Condition::Upgraded => binding.is_upgraded(state),
    }
}

pub fn card_types_match(card: CardReference, _type: CardType) -> bool {
    _type == CardType::All || card.base._type == _type
}

pub fn potion_targeted(reference: PotionReference, state: &GameState) -> bool {
    eval_condition(
        &reference.get(state).as_ref().unwrap().base.targeted,
        state,
        Binding::Potion(reference),
        None,
    )
}

pub fn card_targeted(card: CardReference, state: &GameState) -> bool {
    eval_condition(
        &card.base.targeted,
        state,
        Binding::Card(card),
        None,
    )
}
