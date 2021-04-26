use crate::models;
use crate::models::core::*;
use crate::models::state::*;
use crate::spireai::GamePossibilitySet;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum Binding<'a> {
    Buff(&'a Creature, &'a Buff),
    Card(&'a Card),
    Monster(&'a Monster),
    Potion(&'a Potion),
    Relic(&'a Relic),
}

impl<'a> Binding<'a> {
    fn get_creature(self, game_state: &'a GameState) -> &'a Creature {
        match self {
            Binding::Buff(creature, _) => creature,
            Binding::Card(_) => &game_state.player,
            Binding::Potion(_) => &game_state.player,
            Binding::Relic(_) => &game_state.player,
            Binding::Monster(monster) => &monster.creature,
        }
    }

    fn get_monster(self) -> Option<&'a Creature> {
        match self {
            Binding::Buff(creature, _) => {
                if creature.is_player {
                    None
                } else {
                    Some(creature)
                }
            }
            Binding::Card(_) | Binding::Potion(_) | Binding::Relic(_) => None,
            Binding::Monster(monster) => Some(&monster.creature),
        }
    }

    fn get_vars(self) -> &'a Vars {
        match self {
            Binding::Buff(_, buff) => &buff.vars,
            Binding::Card(card) => &card.vars,
            Binding::Monster(monster) => &monster.vars,
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.base.name)
            }
            Binding::Relic(relic) => &relic.vars,
        }
    }

    fn is_upgraded(self, game_state: &'a GameState) -> bool {
        match self {
            Binding::Buff(_, buff) => {
                panic!("Unexpected is_upgraded check on buff: {}", buff.base.name)
            }
            Binding::Card(card) => card.upgrades > 0,
            Binding::Monster(monster) => {
                panic!(
                    "Unexpected is_upgraded check on monster: {}",
                    monster.base.name
                )
            }
            Binding::Potion(_) => game_state.relic_names.contains(models::relics::SACRED_BARK),
            Binding::Relic(relic) => {
                panic!("Unexpected is_upgraded check on relic: {}", relic.base.name)
            }
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
    binding: &Binding,
    action: &Option<GameAction>,
) -> ResolvedTarget {
    match target {
        Target::_Self => match binding.get_monster() {
            Some(creature) => ResolvedTarget::Monster(creature.position),
            None => ResolvedTarget::Player,
        },
        Target::AllEnemies => match binding.get_monster() {
            Some(_) => ResolvedTarget::Player,
            None => ResolvedTarget::AllMonsters,
        },
        Target::AnyFriendly => match binding.get_monster() {
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
        Target::RandomEnemy => match binding.get_monster() {
            Some(_) => ResolvedTarget::Player,
            None => ResolvedTarget::RandomMonster((0..get_monster_count(state)).collect()),
        },
        Target::RandomFriendly => match binding.get_monster() {
            Some(creature) => {
                let monster_count = get_monster_count(state);
                if monster_count == 1 {
                    ResolvedTarget::Monster(0)
                } else {
                    let mut positions: Vec<usize> = (0..creature.position).collect();
                    positions.extend(creature.position + 1..monster_count);
                    ResolvedTarget::RandomMonster(positions)
                }
            }
            None => ResolvedTarget::Player,
        },
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
    binding: &Binding,
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
    binding: &Binding,
    action: &Option<GameAction>,
    rng: &mut R,
) where
    R: Rng + ?Sized,
{
    match effect {
        Effect::AddBuff(buff_name, buff_amount, target) => {
            let immut_state: &GameState = &state.0;
            let amount = eval_amount(buff_amount, immut_state, binding);
            for creature in eval_target(target, immut_state, binding, action)
                .to_creatures_rand_mut(state, rng)
                .iter_mut()
            {
                add_buff(creature, buff_name, amount)
            }
        }
        _ => unimplemented!(),
    }
}

fn eval_card_reference(reference: &CardReference, state: &mut GamePossibilitySet) -> Card {
    unimplemented!()
}

fn add_buff(creature: &mut Creature, name: &'static str, amount: i16) {
    creature
        .buffs
        .entry(name)
        .and_modify(|buff| {
            if buff.base.stacks {
                if buff.base.is_additive {
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

pub fn create_buff(name: &'static str, amount: i16) -> Buff {
    let base = models::buffs::by_name(name);
    if base.stacks {
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
            card.upgrades == 0 && card.base.name != models::cards::SEARING_BLOW
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
    !matches!(
        card.base.name,
        models::cards::ASCENDERS_BANE
            | models::cards::CURSE_OF_THE_BELL
            | models::cards::NECRONOMICURSE
    )
}

pub fn card_playable(card: &Card, battle_state: &BattleState, state: &GameState) -> bool {
    card.cost <= battle_state.energy
        && eval_condition(&card.base.playable_if, state, &Binding::Card(card), &None)
}

fn get_monster_count(state: &GameState) -> usize {
    let battle_state = state
        .battle_state
        .as_ref()
        .expect("Battle state not found in get_monster_count");
    battle_state.monsters.len()
}

pub fn eval_amount(amount: &Amount, state: &GameState, binding: &Binding) -> i16 {
    match amount {
        Amount::ByAsc(low, mid, high) => {
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Unable to read battle state when in Amount::ByAsc");
            match battle_state.battle_type {
                BattleType::Common | BattleType::Event => {
                    if state.asc >= 17 {
                        *high
                    } else if state.asc >= 2 {
                        *mid
                    } else {
                        *low
                    }
                }
                BattleType::Elite => {
                    if state.asc >= 18 {
                        *high
                    } else if state.asc >= 3 {
                        *mid
                    } else {
                        *low
                    }
                }
                BattleType::Boss => {
                    if state.asc >= 19 {
                        *high
                    } else if state.asc >= 4 {
                        *mid
                    } else {
                        *low
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
        Amount::Fixed(amount) => *amount,
        Amount::Mult(amounts) => {
            let mut product = 1;
            for amount in amounts {
                product *= eval_amount(amount, state, binding);
            }
            product
        }
        Amount::N => binding.get_vars().n as i16,
        Amount::NegX => -binding.get_vars().x as i16,
        Amount::OrbCount => {
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Unable to read battle state when in Amount::OrbCount");
            battle_state.orbs.len() as i16
        }
        Amount::Sum(amounts) => {
            let mut sum = 0;
            for amount in amounts {
                sum += eval_amount(amount, state, binding);
            }
            sum
        }
        Amount::Upgradable(low, high) => match binding.is_upgraded(state) {
            true => *high,
            false => *low,
        },
        Amount::MaxHp => binding.get_creature(state).max_hp as i16,
        Amount::X => binding.get_vars().x as i16,
        Amount::PlayerBlock => state.player.block as i16,
        Amount::Any => {
            panic!("Any does not resolve to a fixed number")
        }
    }
}

pub fn eval_condition(
    condition: &Condition,
    state: &GameState,
    binding: &Binding,
    action: &Option<GameAction>,
) -> bool {
    match condition {
        Condition::Act(act) => &state.act == act,
        Condition::Always => true,
        Condition::Asc(asc) => &state.asc >= asc,
        Condition::Attacking(target) => {
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
        Condition::Buff(target, buff) => {
            let target = eval_target(target, state, binding, action);
            let creature = target.to_creature(state);
            creature.buffs.contains_key(buff)
        }
        Condition::BuffX(target, buff, x) => {
            let val = eval_amount(x, state, binding);
            let target = eval_target(target, state, binding, action);
            let creature = target.to_creature(state);
            creature
                .buffs
                .get(buff)
                .map(|a| a.vars.x >= val)
                .unwrap_or(false)
        }
        Condition::Custom => panic!("Unhandled custom condition: {:?}", binding),
        Condition::Dead(target) => {
            eval_target(target, state, binding, action)
                .to_creature(state)
                .hp
                == 0
        }
        Condition::Equals(amount1, amount2) => {
            eval_amount(amount1, state, binding) == eval_amount(amount2, state, binding)
        }
        Condition::HalfHp(target) => {
            let target = eval_target(target, state, binding, action);
            let creature = target.to_creature(state);
            creature.hp * 2 <= creature.max_hp
        }
        Condition::Stance(stance) => {
            let battle_state = state
                .battle_state
                .as_ref()
                .expect("Battle state not found in Condition::Stance");
            &battle_state.stance == stance
        }
        Condition::Never => false,
        _ => panic!("Unhandled condition"),
    }
}

pub fn potion_targeted(potion: &Potion, state: &GameState) -> bool {
    eval_condition(
        &potion.base.targeted,
        state,
        &Binding::Potion(potion),
        &None,
    )
}

pub fn card_targeted(card: &Card, state: &GameState) -> bool {
    eval_condition(&card.base.targeted, state, &Binding::Card(card), &None)
}
