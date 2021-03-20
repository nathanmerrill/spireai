use crate::models::core::*;
use crate::models::state::*;
use crate::models;

#[derive(Debug, Copy, Clone)]
pub enum Binding<'a> {
    Buff(&'a Creature, &'a Buff),
    Card(&'a Card),
    Monster(&'a Monster),
    Potion(&'a Potion),
    Relic(&'a Relic),
    Event(&'a EventState),
}

impl<'a> Binding<'a> {
    fn get_creature(self, game_state: &'a GameState) -> &'a Creature {
        match self {
            Binding::Buff(creature, _) => creature,
            Binding::Card(_) => &game_state.player,
            Binding::Potion(_) => &game_state.player,
            Binding::Relic(_) => &game_state.player,
            Binding::Monster(monster) => &monster.creature,
            Binding::Event(event) => {
                panic!("Unexpected get_creature call on an event: {}", event.base.name)
            }
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
            Binding::Event(event) =>  {
                panic!("Unexpected get_monster call on event: {}", event.base.name)
            }
        }
    }

    fn get_name(self) -> &'static str {
        match self {
            Binding::Buff(_, buff) => buff.base.name,
            Binding::Card(card) => card.base.name,
            Binding::Monster(monster) => monster.base.name,
            Binding::Potion(potion) => potion.base.name,
            Binding::Relic(relic) => relic.base.name,
            Binding::Event(event) => event.base.name,
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
            Binding::Event(event) => &event.vars,
        }
    }

    fn is_upgraded(self, game_state: &'a GameState) -> bool {
        match self {
            Binding::Buff(_, buff) => {
                panic!("Unexpected is_upgraded check on buff: {}", buff.base.name)
            }
            Binding::Card(card) => card.upgrades > 0,
            Binding::Monster(monster) => {
                panic!("Unexpected is_upgraded check on monster: {}", monster.base.name)
            },
            Binding::Potion(potion) => {
                game_state.relic_names.contains(models::relics::SACRED_BARK)
            }
            Binding::Relic(relic) => {
                panic!("Unexpected is_upgraded check on relic: {}", relic.base.name)
            },
            Binding::Event(event) => {
                game_state.asc >= 15
            }
        }
    }
}

#[derive(Debug)]
pub enum ResolvedTarget {
    Player,
    Monster(u8),
    AllMonsters,
    RandomMonster(Vec<u8>),
    None,
}

impl ResolvedTarget {
    fn to_creature<'a>(
        self,
        battle_state: &'a BattleState,
        game_state: &'a GameState,
    ) -> &'a Creature {
        match self {
            ResolvedTarget::Player => &game_state.player,
            ResolvedTarget::Monster(idx) => &battle_state.monsters[idx as usize].creature,
            _ => panic!("Cannot resolve to a single creature: {:?}", self),
        }
    }
}

pub fn eval_target(
    target: &Target,
    battle_state: &BattleState,
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
            match battle_state.monsters.iter().find(|m| &m.base.name == name) {
                Some(monster) => ResolvedTarget::Monster(monster.creature.position),
                None => ResolvedTarget::None,
            }
        }
        Target::RandomEnemy => match binding.get_monster() {
            Some(_) => ResolvedTarget::Player,
            None => ResolvedTarget::RandomMonster((0..battle_state.monsters.len() as u8).collect()),
        },
        Target::RandomFriendly => match binding.get_monster() {
            Some(creature) => {
                if battle_state.monsters.len() == 1 {
                    ResolvedTarget::Monster(0)
                } else {
                    let mut positions: Vec<u8> = (0..creature.position).collect();
                    positions.extend(creature.position + 1..battle_state.monsters.len() as u8);
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

pub fn eval_amount(
    amount: &Amount,
    game_state: &GameState,
    battle_state: &BattleState,
    binding: &Binding,
) -> i16 {
    match amount {
        Amount::ByAsc(low, mid, high) => match battle_state.battle_type {
            BattleType::Common | BattleType::Event => {
                if game_state.asc >= 17 {
                    *high
                } else if game_state.asc >= 2 {
                    *mid
                } else {
                    *low
                }
            }
            BattleType::Elite => {
                if game_state.asc >= 18 {
                    *high
                } else if game_state.asc >= 3 {
                    *mid
                } else {
                    *low
                }
            }
            BattleType::Boss => {
                if game_state.asc >= 19 {
                    *high
                } else if game_state.asc >= 4 {
                    *mid
                } else {
                    *low
                }
            }
        },
        Amount::Custom => match binding.get_name() {
            _ => panic!("Unhandled custom amount: {:?}", binding),
        },
        Amount::EnemyCount => battle_state.monsters.len() as i16,
        Amount::Fixed(amount) => *amount,
        Amount::Mult(amounts) => {
            let mut product = 1;
            for amount in amounts {
                product = product * eval_amount(amount, game_state, battle_state, binding);
            }
            product
        }
        Amount::N => binding.get_vars().n as i16,
        Amount::NegX => binding.get_vars().x as i16 * -1,
        Amount::OrbCount => battle_state.orbs.len() as i16,
        Amount::Sum(amounts) => {
            let mut sum = 0;
            for amount in amounts {
                sum = sum + eval_amount(amount, game_state, battle_state, binding);
            }
            sum
        },
        Amount::Upgradable(low, high) => {
            match binding.is_upgraded(game_state) {
                true => *high,
                false => *low
            }
        },
        Amount::MaxHp => binding.get_creature(game_state).max_hp as i16,
        Amount::X => binding.get_vars().x as i16,
        Amount::PlayerBlock => {
            game_state.player.block as i16
        },
        Amount::Any => {
            panic!("Any does not resolve to a fixed number")
        }
    }
}

pub fn eval_static_condition(condition: &StaticCondition, game_state: &GameState, binding: &Binding) -> bool{
    match condition {
        StaticCondition::DeckSize(size) => game_state.deck.len() as u8 >= *size,
        StaticCondition::False => false,
        StaticCondition::MinGold(gold) => game_state.gold as u16 >= *gold,
        StaticCondition::True => true,
        StaticCondition::WhenUnupgraded => !binding.is_upgraded(game_state),
        StaticCondition::WhenUpgraded => binding.is_upgraded(game_state),
    }
}

pub fn eval_condition(
    condition: &Condition,
    battle_state: &BattleState,
    game_state: &GameState,
    binding: &Binding,
    action: &Option<GameAction>,
) -> bool {
    match condition {
        Condition::Act(act) => &game_state.act == act,
        Condition::Always => true,
        Condition::Asc(asc) => &game_state.asc >= asc,
        Condition::Attacking(target) => match eval_target(target, battle_state, binding, action) {
            ResolvedTarget::Monster(idx) => match battle_state.monsters[idx as usize].intent {
                Intent::Attack => true,
                Intent::AttackBuff => true,
                Intent::AttackDebuff => true,
                Intent::AttackDefend => true,
                _ => false,
            },
            _ => panic!("Unexpected target that is not a monster in Condition::Attacking")
        },
        Condition::Buff(target, buff) => {
            let creature = eval_target(target, battle_state, binding, action)
                .to_creature(battle_state, game_state);
            creature.buffs.contains_key(buff)
        }
        Condition::BuffX(target, buff, x) => {
            let val = eval_amount(x, game_state, battle_state, binding) as u8;
            let creature = eval_target(target, battle_state, binding, action)
                .to_creature(battle_state, game_state);
            creature
                .buffs
                .get(buff)
                .map(|a| a.vars.x >= val)
                .unwrap_or(false)
        }
        Condition::Custom => match binding.get_name() {
            _ => panic!("Unhandled custom condition: {:?}", binding),
        },
        Condition::Dead(target) => {
            eval_target(target, battle_state, binding, action)
                .to_creature(battle_state, game_state)
                .hp
                == 0
        }
        Condition::Equals(amount1, amount2) => {
            eval_amount(amount1, game_state, battle_state, binding)
                == eval_amount(amount2, game_state, battle_state, binding)
        }
        Condition::HalfHp(target) => {
            let creature = eval_target(target, battle_state, binding, action)
                .to_creature(battle_state, game_state);
            creature.hp * 2 <= creature.max_hp
        }
        Condition::Stance(stance) => &battle_state.stance == stance,
        
        Condition::Never => false,
        _ => panic!("Unhandled condition")
    }
}
