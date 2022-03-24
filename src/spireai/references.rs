use uuid::Uuid;

use ::std::hash::{Hash, Hasher};

use crate::{
    models::{
        self, buffs::BaseBuff, core::CardLocation, monsters::BaseMonster, potions::BasePotion,
        relics::BaseRelic,
    },
    state::{
        battle::BattleState,
        core::{Monster, Vars},
        game::GameState,
    },
};

#[derive(Eq, Debug, Clone, Copy)]
pub struct CardReference {
    pub location: CardLocation,
    pub uuid: Uuid,
    pub base: &'static models::cards::BaseCard,
}

impl PartialEq for CardReference {
    fn eq(&self, other: &CardReference) -> bool {
        self.uuid == other.uuid && self.location == other.location
    }
}

impl Hash for CardReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
        self.location.hash(state)
    }
}

#[derive(Eq, Debug, Clone, Copy)]
pub struct MonsterReference {
    pub base: &'static BaseMonster,
    pub uuid: Uuid,
}

impl PartialEq for MonsterReference {
    fn eq(&self, other: &MonsterReference) -> bool {
        self.uuid == other.uuid
    }
}

impl Hash for MonsterReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state)
    }
}

impl MonsterReference {
    pub fn creature_ref(self) -> CreatureReference {
        CreatureReference::Creature(self)
    }
}

#[derive(Eq, Debug, Clone, Copy, PartialEq, Hash)]
pub enum CreatureReference {
    Player,
    Creature(MonsterReference),
}

impl CreatureReference {
    pub fn monster_ref(self) -> Option<MonsterReference> {
        match self {
            CreatureReference::Player => None,
            CreatureReference::Creature(monster) => Some(monster),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BuffReference {
    pub base: &'static BaseBuff,
    pub creature: CreatureReference,
    pub buff: Uuid,
}

impl BuffReference {}

impl PartialEq for BuffReference {
    fn eq(&self, other: &BuffReference) -> bool {
        self.creature == other.creature && self.buff == other.buff
    }
}

impl Hash for BuffReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buff.hash(state);
        self.creature.hash(state)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PotionReference {
    pub base: &'static BasePotion,
    pub index: usize,
}

impl PartialEq for PotionReference {
    fn eq(&self, other: &PotionReference) -> bool {
        self.index == other.index
    }
}

impl Hash for PotionReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

#[derive(Debug, Eq, Clone, Copy)]
pub struct RelicReference {
    pub base: &'static BaseRelic,
    pub relic: Uuid,
}

impl PartialEq for RelicReference {
    fn eq(&self, other: &RelicReference) -> bool {
        self.relic == other.relic
    }
}

impl Hash for RelicReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.relic.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Binding {
    Buff(BuffReference),
    Card(CardReference),
    Creature(CreatureReference),
    Potion(PotionReference),
    Relic(RelicReference),
    CurrentEvent,
}

impl Binding {
    pub fn get_creature(self) -> CreatureReference {
        match self {
            Binding::Buff(buff) => buff.creature,
            Binding::Card(_) => CreatureReference::Player,
            Binding::Potion(_) => CreatureReference::Player,
            Binding::Relic(_) => CreatureReference::Player,
            Binding::Creature(creature) => creature,
            Binding::CurrentEvent => CreatureReference::Player,
        }
    }

    pub fn get_monster(self, state: &BattleState) -> Option<&Monster> {
        match self {
            Binding::Buff(buff) => buff
                .creature
                .monster_ref()
                .and_then(|m| state.get_monster(m)),
            Binding::Creature(creature) => {
                creature.monster_ref().and_then(|m| state.get_monster(m))
            }
            Binding::Card(_) | Binding::Potion(_) | Binding::Relic(_) | Binding::CurrentEvent => {
                None
            }
        }
    }

    pub fn get_vars(self, state: &GameState) -> &Vars {
        match self {
            Binding::Buff(buff) => &state.get_buff(buff).unwrap().vars,
            Binding::Card(card) => &state.floor_state.battle().get_card(card).vars,
            Binding::Creature(creature) => {
                &state
                    .floor_state
                    .battle()
                    .get_monster(creature.monster_ref().unwrap())
                    .unwrap()
                    .vars
            }
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.index)
            }
            Binding::CurrentEvent => &state.floor_state.event().vars,
            Binding::Relic(relic) => &state.relics.get(relic).vars,
        }
    }

    pub fn get_mut_vars(self, state: &mut GameState) -> &mut Vars {
        match self {
            Binding::Buff(buff) => &mut state.get_buff_mut(buff).unwrap().vars,
            Binding::Card(card) => &mut state.floor_state.battle_mut().get_card_mut(card).vars,
            Binding::Creature(creature) => {
                &mut state
                    .floor_state
                    .battle_mut()
                    .get_monster_mut(creature.monster_ref().unwrap())
                    .unwrap()
                    .vars
            }
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.index)
            }
            Binding::CurrentEvent => &mut state.floor_state.event_mut().vars,
            Binding::Relic(relic) => &mut state.relics.get_mut(relic).vars,
        }
    }

    pub fn is_upgraded(self, state: &GameState) -> bool {
        match self {
            Binding::Card(card) => state.floor_state.battle().get_card(card).upgrades > 0,
            Binding::Potion(_) => state.relics.contains("Sacred Bark"),
            _ => panic!("Unexpected is_upgraded check on {:?}", self),
        }
    }
}

impl Monster {
    pub fn monster_ref(&self) -> MonsterReference {
        MonsterReference {
            base: self.base,
            uuid: self.uuid,
        }
    }

    pub fn creature_ref(&self) -> CreatureReference {
        CreatureReference::Creature(self.monster_ref())
    }

    pub fn buffs(&self) -> impl Iterator<Item = BuffReference> + '_ {
        self.creature.buffs.values().map(move |b| BuffReference {
            base: b.base,
            creature: self.creature_ref(),
            buff: b.uuid,
        })
    }
}
