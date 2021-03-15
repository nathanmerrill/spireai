use crate::models::core::*;
use crate::models::state::*;

#[derive(Debug, Copy, Clone)]
pub enum Binding<'a> {
    Buff(&'a Creature, &'a Buff),
    Card(&'a Card),
    Monster(&'a Monster),
    Potion(&'a Potion),
    Relic(&'a Relic),
}

impl<'a> Binding<'a> {
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

    fn get_name(self) -> &'static str {
        match self {
            Binding::Buff(_, buff) => buff.base.name,
            Binding::Card(card) => card.base.name,
            Binding::Monster(monster) => monster.base.name,
            Binding::Potion(potion) => potion.base.name,
            Binding::Relic(relic) => relic.base.name,
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
        Amount::ByAsc(low, mid, high) => match game_state.room {
            RoomType::HallwayFight => {
                if game_state.asc >= 17 {
                    *high
                } else if game_state.asc >= 2 {
                    *mid
                } else {
                    *low
                }
            }
            RoomType::Elite => {
                if game_state.asc >= 18 {
                    *high
                } else if game_state.asc >= 3 {
                    *mid
                } else {
                    *low
                }
            }
            RoomType::Boss => {
                if game_state.asc >= 19 {
                    *high
                } else if game_state.asc >= 4 {
                    *mid
                } else {
                    *low
                }
            }
            _ => panic!("Unexpected room type in ByAsc: {:?}", amount),
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
        }
        Amount::Upgradable(low, high) => match binding {
            Binding::Buff(_, buff) => {
                panic!("Unexpected upgradeable check on buff: {}", buff.base.name)
            }
            Binding::Card(card) => {
                if card.upgrades == 0 {
                    *low
                } else {
                    *high
                }
            }
            Binding::Monster(monster) => panic!(
                "Unexpected upgradeable check on monster: {}",
                monster.base.name
            ),
            Binding::Potion(potion) => panic!(
                "Unexpected upgradeable check on potion: {}",
                potion.base.name
            ),
            Binding::Relic(_) => {
                if game_state
                    .relics
                    .contains_key(crate::models::relics::SACRED_BARK)
                {
                    *high
                } else {
                    *low
                }
            }
        },
        Amount::X => binding.get_vars().x as i16,
        Amount::Any => {
            panic!("Any does not resolve to a fixed number")
        }
    }
}

pub fn eval_condition<'a>(
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
            _ => panic!("Unexpected resolved target in condition: {:?}", condition),
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
        Condition::Never => false,
        _ => panic!("Unhandled condition: {:?}", condition),
    }
}
