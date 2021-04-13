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

    fn is_upgraded(self, game_state: &'a GameState) -> bool {
        match self {
            Binding::Buff(_, buff) => {
                panic!("Unexpected is_upgraded check on buff: {}", buff.base.name)
            }
            Binding::Card(card) => card.upgrades > 0,
            Binding::Monster(monster) => {
                panic!("Unexpected is_upgraded check on monster: {}", monster.base.name)
            },
            Binding::Potion(_) => {
                game_state.relic_names.contains(models::relics::SACRED_BARK)
            }
            Binding::Relic(relic) => {
                panic!("Unexpected is_upgraded check on relic: {}", relic.base.name)
            },
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
        state: &'a GameState,
    ) -> &'a Creature {
        match self {
            ResolvedTarget::Player => &state.player,
            ResolvedTarget::Monster(idx) => {
                &state.battle_state.as_ref().expect("No battle state when resolving monster").monsters[idx as usize].creature
            }
            _ => panic!("Cannot resolve to a single creature: {:?}", self),
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
            let battle_state = state.battle_state.as_ref().expect("Battle state not found in Target::Friendly");
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
                    let mut positions: Vec<u8> = (0..creature.position).collect();
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

pub fn card_upgradable(card: &Card) -> bool {
    match card.base._type {
        CardType::Attack | CardType::Skill | CardType::Power => {
            card.upgrades == 0 && card.base.name != models::cards::SEARING_BLOW
        },
        CardType::Status => false,
        CardType::Curse => false,
        CardType::ByName(_) => panic!("Unexpected ByName on card type"),
        CardType::All => panic!("Unexpected All on card type")
    }
}

pub fn card_removable(card: &Card) -> bool {
    if card.bottled {
        return false
    }

    match card.base.name {
        models::cards::ASCENDERS_BANE => false,
        models::cards::CURSE_OF_THE_BELL => false,
        models::cards::NECRONOMICURSE => false,
        _ => true
    }
}

pub fn card_playable(card: &Card, battle_state: &BattleState, state: &GameState) -> bool {
    card.cost <= battle_state.energy && eval_condition(&card.base.playable_if, state, &Binding::Card(card), &None)
}

fn get_monster_count(state: &GameState) -> u8{
    let battle_state = state.battle_state.as_ref().expect("Battle state not found in get_monster_count");
    battle_state.monsters.len() as u8
}

pub fn eval_amount(
    amount: &Amount,
    state: &GameState,
    binding: &Binding,
) -> i16 {
    match amount {
        Amount::ByAsc(low, mid, high) => {
            let battle_state = state.battle_state.as_ref().expect("Unable to read battle state when in Amount::ByAsc");
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
        },
        Amount::Custom => match binding.get_name() {
            _ => panic!("Unhandled custom amount: {:?}", binding),
        },
        Amount::EnemyCount => {
            let battle_state = state.battle_state.as_ref().expect("Unable to read battle state when in Amount::EnemyCount");
            battle_state.monsters.len() as i16
        },
        Amount::Fixed(amount) => *amount,
        Amount::Mult(amounts) => {
            let mut product = 1;
            for amount in amounts {
                product = product * eval_amount(amount, state, binding);
            }
            product
        }
        Amount::N => binding.get_vars().n as i16,
        Amount::NegX => binding.get_vars().x as i16 * -1,
        Amount::OrbCount => {
            let battle_state = state.battle_state.as_ref().expect("Unable to read battle state when in Amount::OrbCount");
            battle_state.orbs.len() as i16
        },
        Amount::Sum(amounts) => {
            let mut sum = 0;
            for amount in amounts {
                sum = sum + eval_amount(amount, state, binding);
            }
            sum
        },
        Amount::Upgradable(low, high) => {
            match binding.is_upgraded(state) {
                true => *high,
                false => *low
            }
        },
        Amount::MaxHp => binding.get_creature(state).max_hp as i16,
        Amount::X => binding.get_vars().x as i16,
        Amount::PlayerBlock => {
            state.player.block as i16
        },
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
            let battle_state = state.battle_state.as_ref().expect("Battle state not found in Condition::Attacking");
            match eval_target(target, state, binding, action) {
                ResolvedTarget::Monster(idx) => match battle_state.monsters[idx as usize].intent {
                    Intent::Attack => true,
                    Intent::AttackBuff => true,
                    Intent::AttackDebuff => true,
                    Intent::AttackDefend => true,
                    _ => false,
                },
                _ => panic!("Unexpected target that is not a monster in Condition::Attacking")
            }
        },
        Condition::Buff(target, buff) => {
            let creature = eval_target(target, state, binding, action).to_creature(state);
            creature.buffs.contains_key(buff)
        }
        Condition::BuffX(target, buff, x) => {
            let battle_state = state.battle_state.as_ref().expect("Battle state not found in Condition::BuffX");
            let val = eval_amount(x, state, binding) as u8;
            let creature = eval_target(target, state, binding, action).to_creature(state);
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
            eval_target(target, state, binding, action).to_creature(state).hp == 0
        }
        Condition::Equals(amount1, amount2) => {
            eval_amount(amount1, state, binding) == eval_amount(amount2, state, binding)
        }
        Condition::HalfHp(target) => {
            let creature = eval_target(target, state, binding, action).to_creature(state);
            creature.hp * 2 <= creature.max_hp
        }
        Condition::Stance(stance) => {
            let battle_state = state.battle_state.as_ref().expect("Battle state not found in Condition::Stance");
            &battle_state.stance == stance
        },        
        Condition::Never => false,
        _ => panic!("Unhandled condition")
    }
}

pub fn potion_targeted(potion: &Potion, state: &GameState) -> bool {
    eval_condition(&potion.base.targeted, state, &Binding::Potion(potion), &None)
}

pub fn card_targeted(card: &Card, state: &GameState) -> bool {
    eval_condition(
        &card.base.targeted, 
        state, 
        &Binding::Card(card),
        &None,
    )
}
