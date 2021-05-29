use crate::models::{self, core::*, monsters::Intent, relics::Activation, state::*};
use crate::spireai::GamePossibilitySet;
use im::{Vector, vector};
use itertools::Itertools;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardReference {
    Deck(Uuid),
    Discard(Uuid),
    Draw(Uuid),
    Hand(Uuid),
    Exhaust(Uuid),
}

impl BindingReference for CardReference {
    type Item = Card;
    fn get(self, state: &GameState) -> &Card {
        match self {
            CardReference::Deck(uuid) => state.deck.get(uuid),
            CardReference::Discard(uuid) => state.battle_state.discard.get(uuid),
            CardReference::Draw(uuid) => &state.battle_state.draw[&uuid],
            CardReference::Hand(uuid) => state.battle_state.hand.get(uuid),
            CardReference::Exhaust(uuid) => state.battle_state.exhaust.get(uuid),
        }
    }

    fn get_mut(self, state: &mut GameState) -> &mut Card {
        match self {
            CardReference::Deck(uuid) => state.deck.get_mut(uuid),
            CardReference::Discard(uuid) => state.battle_state.discard.get_mut(uuid),
            CardReference::Draw(uuid) => state.battle_state.draw.get_mut(&uuid).unwrap(),
            CardReference::Hand(uuid) => state.battle_state.hand.get_mut(uuid),
            CardReference::Exhaust(uuid) => state.battle_state.exhaust.get_mut(uuid),
        }
    }
}

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub enum CreatureReference {
    Player,
    Creature(usize),
}

impl BindingReference for CreatureReference {
    type Item = Creature;
    fn get(self, state: &GameState) -> &Creature {
        match self {
            CreatureReference::Creature(position) => {
                &state.battle_state.monsters[position].creature
            }
            CreatureReference::Player => &state.player,
        }
    }

    fn get_mut(self, state: &mut GameState) -> &mut Creature {
        match self {
            CreatureReference::Creature(position) => {
                &mut state
                    .battle_state
                    .monsters
                    .get_mut(position)
                    .unwrap()
                    .creature
            }
            CreatureReference::Player => &mut state.player,
        }
    }
}

impl CreatureReference {
    fn get_monster(self, state: &GameState) -> Option<&Monster> {
        match self {
            CreatureReference::Creature(position) => {
                Some(&state.battle_state.monsters[position])
            }
            CreatureReference::Player => None,
        }
    }

    fn get_monster_mut(self, state: &mut GameState) -> Option<&mut Monster> {
        match self {
            CreatureReference::Creature(position) => state
                .battle_state
                .monsters
                .get_mut(position),
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
        creature.buffs.get(self.buff)
    }

    fn get_mut(self, state: &mut GameState) -> &mut Buff {
        let creature = self.creature.get_mut(state);
        creature.buffs.get_mut(self.buff)
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
        state.relics.get(self.relic)
    }

    fn get_mut(self, state: &mut GameState) -> &mut Relic {
        state.relics.get_mut(self.relic)
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

trait BindingReference {
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
    state: &mut GamePossibilitySet,
    binding: Binding,
    action: Option<GameAction>) -> Vector<CreatureReference> {
    match target {
        Target::AllEnemies => match binding.get_monster(&state.state) {
            Some(_) => vector![CreatureReference::Player],
            None => state.state.battle_state.available_monsters(),
        },
        Target::AnyFriendly => match binding.get_monster(&state.state) {
            Some(_) => state.state.battle_state.available_monsters(),
            None => vector![CreatureReference::Player],
        },
        Target::RandomEnemy => match binding.get_monster(&state.state) {
            Some(_) => vector![CreatureReference::Player],
            None => {
                if let Some(selection) = state.choose(state.state.battle_state.available_monsters().iter()) {
                    vector![*selection]
                } else {
                    vector![]
                }
            }
        },
        Target::RandomFriendly => vector![{
            let creature_reference = match binding {
                Binding::Buff(buff) => buff.creature,
                Binding::Creature(creature) => creature,
                _ => return vector![CreatureReference::Player],
            };
            match creature_reference {
                CreatureReference::Player => CreatureReference::Player,
                CreatureReference::Creature(position) => {
                    let monster_count = get_monster_count(&state.state);
                    if monster_count == 1 {
                        CreatureReference::Creature(0)
                    } else {
                        let mut positions: Vec<usize> = (0..position).collect();
                        positions.extend((position + 1)..monster_count);
                        CreatureReference::Creature(*state.choose(&positions).unwrap())
                    }
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
                CreatureReference::Player => CreatureReference::Creature(_action.target.expect("Expected target index!")),
                CreatureReference::Creature(_) => CreatureReference::Player
            },
            None => panic!("Expected action!")
        },
        Target::Player => CreatureReference::Player,
        _ => panic!("Target does not resolve to a single creature! {:?}", target)
    }
}

pub fn eval_card_effects(effects: &Vec<CardEffect>, card: CardReference, state: &mut GamePossibilitySet) {
    for effect in effects {
        eval_card_effect(effect, card, state);
    }
}

pub fn random_monster(state: &mut GamePossibilitySet) -> CreatureReference {
    state.choose(state.state.battle_state.available_monsters()).unwrap()
}

pub fn eval_card_effect(effect: &CardEffect, card_ref: CardReference, state: &mut GamePossibilitySet) {
    match effect {
        CardEffect::AutoPlay => {
            let card = card_ref.get(&state.state).base;
            let binding = Binding::Card(card_ref);
            let target: Option<usize> = if eval_condition(&card.targeted, &state.state, binding, None) {
                if let CreatureReference::Creature(position) = random_monster(state){
                    Some(position)
                } else {
                    panic!("Unexpected player reference")
                }
            } else {
                None
            };

            for effect in &card.on_play {
                eval_effect(effect, state, binding, Some(GameAction {
                    is_attack: card._type == CardType::Attack,
                    creature: CreatureReference::Player,
                    target: target
                }))
            }
        }
        CardEffect::CopyTo {location, position, then} => {
            let card = copy_card(card_ref.get(&state.state));
            let new_card = add_card_to_location(card, *location, *position, state);
            eval_card_effects(then, new_card, state)
        }
        CardEffect::Custom => panic!("Unexpected custom card effect"),
        CardEffect::Discard => {
            let card = remove_card_from_location(card_ref, state);
            unimplemented!();
            add_card_to_location(card, CardLocation::DiscardPile, RelativePosition::Bottom, state);
        }

        _ => unimplemented!()
    }
}

pub fn remove_card_from_location(card: CardReference, state: &mut GamePossibilitySet) -> Card {
    match card {
        CardReference::Deck(uuid) => {
            let position = state.state.deck.uuids.index_of(&uuid).unwrap();
            state.state.deck.remove(position)
        }
        CardReference::Discard(uuid) => {
            let position = state.state.battle_state.discard.uuids.index_of(&uuid).unwrap();
            state.state.battle_state.discard.remove(position)
        }
        CardReference::Hand(uuid) => {
            let position = state.state.battle_state.hand.uuids.index_of(&uuid).unwrap();
            state.state.battle_state.hand.remove(position)
        }
        CardReference::Draw(uuid) => {
            if let Some(position) = state.state.battle_state.draw_top_known.index_of(&uuid) {
                state.state.battle_state.draw_top_known.remove(position);
            } else if let Some(position) = state.state.battle_state.draw_bottom_known.index_of(&uuid) {
                state.state.battle_state.draw_bottom_known.remove(position);
            }
            
            state.state.battle_state.draw.remove(&uuid).unwrap()
        }
        CardReference::Exhaust(uuid) => {
            let position = state.state.deck.uuids.index_of(&uuid).unwrap();
            state.state.deck.remove(position)
        }
    }
}

pub fn add_card_to_location(card: Card, location: CardLocation, position: RelativePosition, state: &mut GamePossibilitySet) -> CardReference {
    let uuid = card.uuid;
    match location {
        CardLocation::DeckPile => {
            state.state.deck.add(card);
            CardReference::Deck(uuid)
        }
        CardLocation::DiscardPile => {
            state.state.battle_state.discard.add(card);
            CardReference::Discard(uuid)
        }
        CardLocation::DrawPile => {
            let uuid = card.uuid;
            state.state.battle_state.draw.insert(uuid, card);
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
            CardReference::Draw(uuid)
        }
        CardLocation::ExhaustPile => {
            state.state.battle_state.exhaust.add(card);
            CardReference::Exhaust(uuid)
        }
        CardLocation::PlayerHand => {
            if state.state.battle_state.hand.items.len() == 10 {
                state.state.battle_state.discard.add(card);
                CardReference::Discard(uuid)
            } else {
                state.state.battle_state.hand.add(card);
                CardReference::Hand(uuid)
            }
        }
    }
}

pub fn copy_card(card: &Card) -> Card {
    Card {
        base: card.base,
        cost: card.cost,
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
    effects: &'static Vec<Effect>,
    state: &mut GamePossibilitySet,
    binding: Binding,
    action: Option<GameAction>
){
    for effect in effects {
        eval_effect(effect, state, binding, action);
    }
}

pub fn eval_effect(
    effect: &'static Effect,
    state: &mut GamePossibilitySet,
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
            for creature in eval_targets(target, state.into(), binding, action)
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
            add_relic(name, state.into());
        }
        Effect::AddX(amount ) => {
            binding.get_mut_vars(state.into()).x += eval_amount(amount, state.into(), binding);
        }
        Effect::AttackDamage {amount, target, if_fatal} => {
            let attack_amount = eval_amount(amount, state.into(), binding);
            
            for creature in eval_targets(target, state, binding, action) {
                if damage(attack_amount as u16, creature, state.into()) {
                    eval_effects(if_fatal, state, binding, action);
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
            _rarity, 
            _class, 
            position, 
            then, 
            choices,
            exclude_healing,
        } => {
            let class = _class.unwrap_or(state.state.class);
            let amount = eval_amount(choices, &state.state, binding);

            let cards = models::cards::available_cards_by_class(class).iter().filter(|card| {
                (*_type == CardType::All || card._type == *_type) &&
                (*_rarity == None || _rarity.unwrap() == card.rarity) &&
                (!exclude_healing || match card.name.as_str() {
                    "Feed" | "Reaper" | "Lesson Learned" | "Alchemize" | "Wish" | "Bandage Up" | "Self Repair" => false,
                    _ => true
                })
            });

            let choice = state.choose_multiple(cards, amount as usize);

            let mut card_choices = UuidVector::new();
            for card in choice {
                card_choices.add(create_card(&card.name));
            }


            if state.state.battle_state.active {
                state.state.battle_state.card_choices = card_choices;
                state.state.battle_state.card_choice_type = CardChoiceType::AddToLocation(*location, *position, then.clone());
            } else {
                unimplemented!()
            }

        }
        _ => unimplemented!(),
    }
}

fn channel_orb(orb_type: OrbType, state: &mut GamePossibilitySet) {
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

fn add_block(amount: u16, target: CreatureReference, state: &mut GamePossibilitySet) {
    let mut_creature = target.get_mut(state.into());
    let new_block = std::cmp::min(mut_creature.block + amount, 999);
    mut_creature.block = new_block;
    eval_when(When::OnBlock, target, state)
}

fn eval_when(when: When, target: CreatureReference, state: &mut GamePossibilitySet) {
    unimplemented!();
}

fn evoke_orb(times: u8, state: &mut GamePossibilitySet) {
    if let Some(orb) = state.state.battle_state.orbs.pop_front() {
        match orb.base {
            OrbType::Any => panic!("Unexpected OrbType of any"),
            OrbType::Dark => {
                for _ in 0 .. times {
                    let lowest_monster =  
                        state.state.battle_state.monsters.iter()
                            .filter(|m| m.targetable)
                            .position_min_by_key(|m|m.creature.hp);
                    if let Some(position) = lowest_monster {
                        let mut orb_damage = orb.n as f64;
                        let creature = CreatureReference::Creature(position);
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
                        let monster_count = state.state.battle_state.monsters.iter().filter(|m|m.targetable).count();
                        for idx in 0 .. monster_count {
                            let creature = CreatureReference::Creature(idx);
                            let multiplier = if has_buff("Lock On", creature, &state.state) {
                                1.5
                            } else {
                                1.0
                            };
                            
                            damage((orb_damage * multiplier).floor() as u16, creature, &mut state.state);
                        }
                    } else {
                        let monsters: Vec<usize> = state.state.battle_state.monsters.iter().filter(|m|m.targetable).map(|m|m.position).collect();
                        if let Some(selected) = state.choose(&monsters) {
                            let creature = CreatureReference::Creature(*selected);
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
        CreatureReference::Creature(position) => {
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

pub fn add_relic(name: &str, state: &mut GamePossibilitySet) {
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
            } else {
                if enabled_at == disabled_at {
                    vec![activated_at, enabled_at]
                } else {
                    vec![activated_at, enabled_at, disabled_at]
                }
            }
        },
        Activation::Custom => vec![]
    };

    for when in whens {
        state.state.relic_whens.entry(*when).or_insert_with(||Vector::new()).push_back(relic.uuid)
    }
    state.state.relics.add(relic);
}

pub fn add_card_to_deck(name: &str, upgraded: bool, state: &mut GameState) {
    let mut card = create_card(name);
    if card.base._type == CardType::Curse {
        if let Some(relic) = find_relic_mut("Omamori", state) {
            if relic.vars.x > 0 {
                relic.vars.x -= 1;
                return;
            }
        }

        if state
            .relic_names
            .contains_key("Darkstone Periapt")
        {
            add_max_hp(6, state);
        }
    }

    let is_upgraded = upgraded
        || match card.base._type {
            CardType::Attack => state.relic_names.contains_key("Molten Egg"),
            CardType::Skill => state.relic_names.contains_key("Toxic Egg"),
            CardType::Power => state.relic_names.contains_key("Frozen Egg"),
            CardType::Curse => false,
            CardType::Status => false,
            CardType::All => panic!("Unexpected card type of All"),
        };

    if is_upgraded {
        card.upgrades = 1;
    }

    if state.relic_names.contains_key("Ceramic Fish") {
        add_gold(9, state);
    }

    state.deck.add(card);
}

pub fn find_relic_mut<'a>(name: &str, state: &'a mut GameState) -> Option<&'a mut Relic> {
    if let Some(uuid) = state.relic_names.get(name) {
        Some(state.relics.get_mut(*uuid))
    } else {
        None
    }
}

pub fn find_relic<'a>(name: &str, state: &'a GameState) -> Option<&'a Relic> {
    if let Some(uuid) = state.relic_names.get(name) {
        Some(state.relics.get(*uuid))
    } else {
        None
    }
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
        heal(5 as f64, state);
    }

    state.gold += amount;
}

fn add_buff(creature: &mut Creature, name: &str, amount: i16) {
    if let Some(uuid) = creature.buff_names.get(name) {
        let buff = creature.buffs.get_mut(*uuid);
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
            creature.buffs_when.entry(effects.when).or_insert_with(||Vector::new()).push_back(new_buff.uuid)
        }
        creature.buff_names.insert(name.to_string(), new_buff.uuid);
        creature.buffs.add(new_buff);
    }
}

fn empty_vars() -> Vars {
    Vars {
        n: 0,
        n_reset: 0,
        x: 0,
    }
}

pub fn create_card(name: &str) -> Card {
    let base = models::cards::by_name(name);
    let uuid = Uuid::new_v4();

    Card {
        base,
        uuid,
        cost: 0,
        vars: empty_vars(),
        upgrades: 0,
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

fn get_monster_count(state: &GameState) -> usize {
    state.battle_state.monsters.len()
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
                CreatureReference::Creature(idx) => matches!(
                    state.battle_state.monsters[idx].intent,
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
            state.battle_state.monsters.iter().any(|m|m.base.name == *name)
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
                    state.deck.items.values().any(|c|c.base._type == *card)
                }
                CardLocation::DiscardPile => {
                    state.battle_state.discard.items.values().any(|c|c.base._type == *card)
                }
                CardLocation::PlayerHand => {
                    state.battle_state.hand.items.values().any(|c|c.base._type == *card)
                }
                CardLocation::ExhaustPile => {
                    state.battle_state.exhaust.items.values().any(|c|c.base._type == *card)
                }
                CardLocation::DrawPile => {
                    state.battle_state.draw.values().any(|c|c.base._type == *card)
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
                .iter()
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
                .cards()
                .filter(|card| card_removable(*card, state) && card_types_match(*card, *card_type, state))
                .count()
                > *count as usize
        }
        Condition::HasUpgradableCard => state.cards().any(|card| card_upgradable(card, state)),
        Condition::InPosition(count) => {
            if let CreatureReference::Creature(position) = binding.get_creature() {
                position == *count
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

pub fn card_types_match(card_ref: CardReference, _type: CardType, state: &GameState) -> bool {
    let card = card_ref.get(state);
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

pub fn card_targeted(reference: CardReference, state: &GameState) -> bool {
    eval_condition(
        &reference.get(state).base.targeted,
        state,
        Binding::Card(reference),
        None,
    )
}
