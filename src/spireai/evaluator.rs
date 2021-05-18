use crate::models;
use crate::spireai::GamePossibilitySet;
use models::core::*;
use models::state::*;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardReference {
    Deck(usize),
    Discard(usize),
    Draw(usize),
    Hand(usize),
    Exhaust(usize),
}

impl BindingReference for CardReference {
    type Item = Card;
    fn get(self, state: &GameState) -> &Card {
        match self {
            CardReference::Deck(position) => &state.deck[position],
            CardReference::Discard(position) => {
                &state.battle_state.as_ref().unwrap().discard[position]
            }
            CardReference::Draw(position) => &state.battle_state.as_ref().unwrap().draw[position],
            CardReference::Hand(position) => &state.battle_state.as_ref().unwrap().hand[position],
            CardReference::Exhaust(position) => {
                &state.battle_state.as_ref().unwrap().exhaust[position]
            }
        }
    }

    fn get_mut(self, state: &mut GameState) -> &mut Card {
        match self {
            CardReference::Deck(position) => &mut state.deck[position],
            CardReference::Discard(position) => {
                &mut state.battle_state.as_mut().unwrap().discard[position]
            }
            CardReference::Draw(position) => {
                &mut state.battle_state.as_mut().unwrap().draw[position]
            }
            CardReference::Hand(position) => {
                &mut state.battle_state.as_mut().unwrap().hand[position]
            }
            CardReference::Exhaust(position) => {
                &mut state.battle_state.as_mut().unwrap().exhaust[position]
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreatureReference {
    Player,
    Creature(usize),
}

impl BindingReference for CreatureReference {
    type Item = Creature;
    fn get(self, state: &GameState) -> &Creature {
        match self {
            CreatureReference::Creature(position) => {
                &state.battle_state.as_ref().unwrap().monsters[position].creature
            }
            CreatureReference::Player => &state.player,
        }
    }

    fn get_mut(self, state: &mut GameState) -> &mut Creature {
        match self {
            CreatureReference::Creature(position) => {
                &mut state
                    .battle_state
                    .as_mut()
                    .unwrap()
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
                Some(&state.battle_state.as_ref().unwrap().monsters[position])
            }
            CreatureReference::Player => None,
        }
    }

    fn get_monster_mut(self, state: &mut GameState) -> Option<&mut Monster> {
        match self {
            CreatureReference::Creature(position) => state
                .battle_state
                .as_mut()
                .unwrap()
                .monsters
                .get_mut(position),
            CreatureReference::Player => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BuffReference {
    pub creature: CreatureReference,
    pub buff: &'static str,
}

impl BindingReference for BuffReference {
    type Item = Buff;
    fn get(self, state: &GameState) -> &Buff {
        let creature = self.creature.get(state);
        &creature.buffs[self.buff]
    }

    fn get_mut(self, state: &mut GameState) -> &mut Buff {
        let creature = self.creature.get_mut(state);
        creature.buffs.get_mut(self.buff).unwrap()
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
    pub relic: usize,
}

impl BindingReference for RelicReference {
    type Item = Relic;
    fn get(self, state: &GameState) -> &Relic {
        &state.relics[self.relic]
    }

    fn get_mut(self, state: &mut GameState) -> &mut Relic {
        state.relics.get_mut(self.relic).unwrap()
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
    fn get_creature(self, state: &GameState) -> &Creature {
        match self {
            Binding::Buff(buff) => buff.creature.get(state),
            Binding::Card(_) => &state.player,
            Binding::Potion(_) => &state.player,
            Binding::Relic(_) => &state.player,
            Binding::Creature(creature) => creature.get(state),
            Binding::Event(_) => &state.player,
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
                .contains(&String::from("Sacred Bark")),
            _ => panic!("Unexpected is_upgraded check on {:?}", self),
        }
    }
}

#[derive(Debug)]
pub enum ResolvedTarget {
    Player,
    Monster(usize),
    AllMonsters,
    RandomMonster(Vec<usize>),
    None,
}

impl ResolvedTarget {
    fn to_creature<'a>(&'a self, state: &'a GameState) -> &'a Creature {
        match self {
            ResolvedTarget::Player => &state.player,
            ResolvedTarget::Monster(idx) => {
                &state
                    .battle_state
                    .as_ref()
                    .expect("No battle state when resolving monster")
                    .monsters[*idx]
                    .creature
            }
            _ => panic!("Cannot resolve to a single creature: {:?}", self),
        }
    }

    fn to_creature_mut<'a>(&'a self, state: &'a mut GameState) -> &'a mut Creature {
        match self {
            ResolvedTarget::Player => &mut state.player,
            ResolvedTarget::Monster(idx) => {
                &mut state
                    .battle_state
                    .as_mut()
                    .expect("No battle state when resolving monster")
                    .monsters[*idx]
                    .creature
            }
            _ => panic!("Cannot resolve to a single creature: {:?}", self),
        }
    }

    fn to_creature_rand_mut<'a, R>(
        &'a self,
        state: &'a mut GamePossibilitySet,
        rng: &mut R,
    ) -> &'a mut Creature
    where
        R: Rng + ?Sized,
    {
        match self {
            ResolvedTarget::RandomMonster(choices) => {
                let choice = *choices.choose(rng).unwrap();
                state.1 /= choices.len() as f64;
                &mut state
                    .0
                    .battle_state
                    .as_mut()
                    .expect("No battle state when resolving monster")
                    .monsters[choice]
                    .creature
            }
            _ => self.to_creature_mut(&mut state.0),
        }
    }

    fn to_creatures_mut<'a>(&'a self, state: &'a mut GameState) -> Vec<&'a mut Creature> {
        match self {
            ResolvedTarget::AllMonsters => state
                .battle_state
                .as_mut()
                .expect("No battle state when resolving monster")
                .monsters
                .iter_mut()
                .map(|a| &mut a.creature)
                .collect(),
            _ => vec![self.to_creature_mut(state)],
        }
    }

    fn to_creatures_rand_mut<'a, R>(
        &'a self,
        state: &'a mut GamePossibilitySet,
        rng: &mut R,
    ) -> Vec<&'a mut Creature>
    where
        R: Rng + ?Sized,
    {
        match self {
            ResolvedTarget::RandomMonster(_) => {
                vec![self.to_creature_rand_mut(state, rng)]
            }
            _ => self.to_creatures_mut(&mut state.0),
        }
    }
}

pub fn eval_target(
    target: &Target,
    state: &GameState,
    binding: Binding,
    action: &Option<GameAction>,
) -> ResolvedTarget {
    match target {
        Target::_Self => match binding {
            Binding::Buff(BuffReference {
                creature: CreatureReference::Creature(position),
                ..
            })
            | Binding::Creature(CreatureReference::Creature(position)) => {
                ResolvedTarget::Monster(position)
            }
            _ => ResolvedTarget::Player,
        },
        Target::AllEnemies => match binding.get_monster(state) {
            Some(_) => ResolvedTarget::Player,
            None => ResolvedTarget::AllMonsters,
        },
        Target::AnyFriendly => match binding.get_monster(state) {
            Some(_) => ResolvedTarget::AllMonsters,
            None => ResolvedTarget::Player,
        },
        Target::Attacker => match action {
            Some(_action) => match _action.is_attack {
                true => match _action.creature.is_player {
                    true => ResolvedTarget::Player,
                    false => ResolvedTarget::Monster(_action.creature.position),
                },
                false => ResolvedTarget::None,
            },
            None => ResolvedTarget::None,
        },
        Target::Friendly(name) => {
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Battle state not found in Target::Friendly");
            match battle_state.monsters.iter().find(|m| &m.base.name == name) {
                Some(monster) => ResolvedTarget::Monster(monster.creature.position),
                None => ResolvedTarget::None,
            }
        }
        Target::RandomEnemy => match binding.get_monster(state) {
            Some(_) => ResolvedTarget::Player,
            None => ResolvedTarget::RandomMonster((0..get_monster_count(state)).collect()),
        },
        Target::RandomFriendly => {
            let creature_reference = match binding {
                Binding::Buff(buff) => buff.creature,
                Binding::Creature(creature) => creature,
                _ => return ResolvedTarget::Player,
            };
            match creature_reference {
                CreatureReference::Player => ResolvedTarget::Player,
                CreatureReference::Creature(position) => {
                    let monster_count = get_monster_count(state);
                    if monster_count == 1 {
                        ResolvedTarget::Monster(0)
                    } else {
                        let mut positions: Vec<usize> = (0..position).collect();
                        positions.extend(position + 1..monster_count);
                        ResolvedTarget::RandomMonster(positions)
                    }
                }
            }
        }
        Target::TargetEnemy => match action {
            Some(_action) => match _action.creature.is_player {
                true => ResolvedTarget::Monster(_action.creature.position),
                false => ResolvedTarget::Player,
            },
            None => ResolvedTarget::None,
        },
    }
}

pub fn eval_effects<R>(
    effects: &'static [Effect],
    state: &mut GamePossibilitySet,
    binding: Binding,
    action: &Option<GameAction>,
    rng: &mut R,
) where
    R: Rng + ?Sized,
{
    for effect in effects {
        eval_effect(effect, state, binding, action, rng);
    }
}

pub fn eval_effect<R>(
    effect: &'static Effect,
    state: &mut GamePossibilitySet,
    binding: Binding,
    action: &Option<GameAction>,
    rng: &mut R,
) where
    R: Rng + ?Sized,
{
    match effect {
        Effect::AddBuff {
            buff: buff_name,
            amount: buff_amount,
            target,
        } => {
            let immut_state: &GameState = &state.0;
            let amount = eval_amount(buff_amount, immut_state, binding);
            for creature in eval_target(target, immut_state, binding, action)
                .to_creatures_rand_mut(state, rng)
                .iter_mut()
            {
                add_buff(creature, buff_name, amount)
            }
        }
        Effect::AddEnergy(energy_amount) => {
            let amount = eval_amount(energy_amount, &state.0, binding) as u8;
            state
                .0
                .battle_state
                .as_mut()
                .expect("No battle state!")
                .energy += amount
        }
        Effect::AddGold(gold_amount) => {
            let amount = eval_amount(gold_amount, &state.0, binding) as u16;
            add_gold(amount, &mut state.0)
        }
        Effect::AddMaxHp(hp_amount) => {
            let amount = eval_amount(hp_amount, &state.0, binding) as u16;
            add_max_hp(amount, &mut state.0)
        }
        Effect::AddN(n_amount) => {
            let amount = eval_amount(n_amount, &state.0, binding);
            binding.get_mut_vars(&mut state.0).n += amount;
        }
        _ => unimplemented!(),
    }
}

pub fn add_card_to_deck(name: &str, upgraded: bool, state: &mut GameState) {
    let mut card = create_card(name);
    if card.base._type == CardType::Curse {
        if let Some(relic) = find_relic(&String::from("Omamori"), state) {
            if relic.vars.x > 0 {
                relic.vars.x -= 1;
                return;
            }
        }

        if state
            .relic_names
            .contains(&String::from("Darkstone Periapt"))
        {
            add_max_hp(6, state);
        }
    }

    let is_upgraded = upgraded
        || match card.base._type {
            CardType::Attack => state.relic_names.contains(&String::from("Molten Egg")),
            CardType::Skill => state.relic_names.contains(&String::from("Toxic Egg")),
            CardType::Power => state.relic_names.contains(&String::from("Frozen Egg")),
            CardType::Curse => false,
            CardType::Status => false,
            CardType::All => panic!("Unexpected card type of All"),
        };

    if is_upgraded {
        card.upgrades = 1;
    }

    if state.relic_names.contains(&String::from("Ceramic Fish")) {
        add_gold(9, state);
    }

    state.deck.push(card);
}

pub fn find_relic<'a>(name: &str, state: &'a mut GameState) -> Option<&'a mut Relic> {
    if state.relic_names.contains(name) {
        match state
            .relics
            .iter_mut()
            .find(|relic| relic.base.name == name)
        {
            Some(relic) => Some(relic),
            None => panic!("Expected to find {} in relics", name),
        }
    } else {
        None
    }
}

pub fn add_max_hp(amount: u16, state: &mut GameState) {
    state.player.max_hp += amount;
    heal(amount, state)
}

pub fn heal(mut amount: u16, state: &mut GameState) {
    if state
        .relic_names
        .contains(&String::from("Mark Of The Bloom"))
    {
        return;
    }

    if state.battle_state.is_some() && state.relic_names.contains(&String::from("Magic Flower")) {
        amount += div_up(amount, 2)
    }

    state.player.hp += amount;

    if state.player.hp > state.player.max_hp {
        state.player.hp = state.player.max_hp;
    }
}

fn div_up(a: u16, b: u16) -> u16 {
    (a + (b - 1)) / b
}

pub fn add_gold(amount: u16, state: &mut GameState) {
    if state.relic_names.contains(&String::from("Ectoplasm")) {
        return;
    }

    if state.relic_names.contains(&String::from("Bloody Idol")) {
        heal(5, state);
    }

    state.gold += amount;
}

fn add_buff(creature: &mut Creature, name: &str, amount: i16) {
    creature
        .buffs
        .entry(name.to_string())
        .and_modify(|buff| {
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
        })
        .or_insert_with(|| create_buff(name, amount));
}

fn empty_vars() -> Vars {
    Vars {
        n: 0,
        n_reset: 0,
        x: 0,
    }
}

pub fn create_card(name: &str) -> Card {
    let base_card = models::cards::by_name(name);

    Card {
        base: base_card,
        cost: 0,
        vars: empty_vars(),
        upgrades: 0,
        bottled: false,
    }
}

pub fn create_relic(name: &str) -> Relic {
    let base = models::relics::by_name(name);
    let mut relic = Relic {
        base,
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
            vars: empty_vars(),
            stacked_vars: vec![Vars {
                n: 0,
                n_reset: 0,
                x: amount,
            }],
        }
    }
}

pub fn card_upgradable(card: &Card) -> bool {
    match card.base._type {
        CardType::Attack | CardType::Skill | CardType::Power => {
            card.upgrades == 0 && card.base.name != "Searing Blow"
        }
        CardType::Status => false,
        CardType::Curse => false,
        CardType::All => panic!("Unexpected All on card type"),
    }
}

pub fn card_removable(card: &Card) -> bool {
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
            &None,
        )
}

fn get_monster_count(state: &GameState) -> usize {
    let battle_state = state
        .battle_state
        .as_ref()
        .expect("Battle state not found in get_monster_count");
    battle_state.monsters.len()
}

pub fn eval_amount(amount: &Amount, state: &GameState, binding: Binding) -> i16 {
    match amount {
        Amount::ByAsc { amount, low, high } => {
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Unable to read battle state when in Amount::ByAsc");
            match battle_state.battle_type {
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
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Unable to read battle state when in Amount::EnemyCount");
            battle_state.monsters.len() as i16
        }
        Amount::N => binding.get_vars(state).n as i16,
        Amount::NegX => -binding.get_vars(state).x as i16,
        Amount::OrbCount => {
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Unable to read battle state when in Amount::OrbCount");
            battle_state.orbs.len() as i16
        }
        Amount::MaxHp => binding.get_creature(state).max_hp as i16,
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
    action: &Option<GameAction>,
) -> bool {
    match condition {
        Condition::Act(act) => &state.act == act,
        Condition::Always => true,
        Condition::Asc(asc) => &state.asc >= asc,
        Condition::Attacking { target } => {
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Battle state not found in Condition::Attacking");
            match eval_target(target, state, binding, action) {
                ResolvedTarget::Monster(idx) => matches!(
                    battle_state.monsters[idx].intent,
                    Intent::Attack
                        | Intent::AttackBuff
                        | Intent::AttackDebuff
                        | Intent::AttackDefend
                ),
                _ => panic!("Unexpected target that is not a monster in Condition::Attacking"),
            }
        }
        Condition::Buff { target, buff } => {
            let target = eval_target(target, state, binding, action);
            let creature = target.to_creature(state);
            creature.buffs.contains_key(buff)
        }
        Condition::BuffX {
            target,
            buff,
            amount: x,
        } => {
            let val = eval_amount(x, state, binding);
            let target = eval_target(target, state, binding, action);
            let creature = target.to_creature(state);
            creature
                .buffs
                .get(buff)
                .map(|a| a.vars.x >= val)
                .unwrap_or(false)
        }
        Condition::Class(class) => state.class == *class,
        Condition::Custom => panic!("Unhandled custom condition: {:?}", binding),
        Condition::Dead { target } => {
            eval_target(target, state, binding, action)
                .to_creature(state)
                .hp
                == 0
        }
        Condition::Equals(amount1, amount2) => {
            eval_amount(amount1, state, binding) == eval_amount(amount2, state, binding)
        }
        Condition::HalfHp => {
            let target = match binding {
                Binding::Creature(CreatureReference::Creature(position)) => {
                    ResolvedTarget::Monster(position)
                }
                _ => ResolvedTarget::Player,
            };

            let creature = target.to_creature(state);
            creature.hp * 2 <= creature.max_hp
        }
        Condition::HasCard { location, card } => eval_card_location(location, state)
            .iter()
            .any(|c| c.base._type == *card),
        Condition::HasDiscarded => {
            state
                .battle_state
                .as_ref()
                .expect("No battle state!")
                .discard_count
                > 0
        }
        Condition::HasFriendlies(count) => {
            let creature = binding.get_monster(state).expect("Monster did not resolve");
            state
                .battle_state
                .as_ref()
                .expect("No battle state!")
                .monsters
                .iter()
                .filter(|a| a.targetable && a.creature != creature.creature)
                .count()
                >= *count as usize
        }
        Condition::HasGold(amount) => state.gold >= eval_amount(amount, state, binding) as u16,
        Condition::HasOrbSlot => {
            state
                .battle_state
                .as_ref()
                .expect("No battle state!")
                .orb_slots
                > 0
        }
        Condition::HasRelic(relic) => state.relic_names.contains(relic),
        Condition::HasRemoveableCards { count, card_type } => {
            state
                .deck
                .iter()
                .filter(|card| card_removable(card) && card_types_match(card, card_type))
                .count()
                > *count as usize
        }
        Condition::HasUpgradableCard => state.deck.iter().any(|card| card_upgradable(card)),
        Condition::InPosition(count) => binding.get_creature(state).position == *count,
        Condition::IsVariant(variant) => match binding {
            Binding::Event(event) => {
                event.get(state).variant.as_ref().expect("Expected variant") == variant
            }
            _ => panic!("Unexpected binding!"),
        },
        Condition::LastCard(_type) => {
            match state
                .battle_state
                .as_ref()
                .expect("No battle state!")
                .last_card_played
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
            let target_eval = eval_target(target, state, binding, action);
            let creature = target_eval.to_creature(state);
            let hp = eval_amount(amount, state, binding);
            creature.hp >= hp as u16
        }
        Condition::Stance(stance) => {
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Battle state not found in Condition::Stance");
            &battle_state.stance == stance
        }
        Condition::Status { target, status } => {
            let target_eval = eval_target(target, state, binding, action);
            let creature = target_eval.to_creature(state);
            creature.buffs.contains_key(status)
        }
        Condition::Upgraded => binding.is_upgraded(state),
    }
}

pub fn card_types_match(card: &Card, _type: &CardType) -> bool {
    *_type == CardType::All || card.base._type == *_type
}

pub fn eval_card_location<'a>(location: &CardLocation, state: &'a GameState) -> &'a Vec<Card> {
    match location {
        CardLocation::DeckPile => &state.deck,
        CardLocation::DiscardPile => {
            &state
                .battle_state
                .as_ref()
                .expect("No battle state!")
                .discard
        }
        CardLocation::DrawPile => &state.battle_state.as_ref().expect("No battle state!").draw,
        CardLocation::ExhaustPile => {
            &state
                .battle_state
                .as_ref()
                .expect("No battle state!")
                .exhaust
        }
        CardLocation::PlayerHand => &state.battle_state.as_ref().expect("No battle state!").hand,
    }
}

pub fn potion_targeted(reference: PotionReference, state: &GameState) -> bool {
    eval_condition(
        &reference.get(state).as_ref().unwrap().base.targeted,
        state,
        Binding::Potion(reference),
        &None,
    )
}

pub fn card_targeted(reference: CardReference, state: &GameState) -> bool {
    eval_condition(
        &reference.get(state).base.targeted,
        state,
        Binding::Card(reference),
        &None,
    )
}
