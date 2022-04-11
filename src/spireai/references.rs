use uuid::Uuid;

use ::std::hash::{Hash, Hasher};

use crate::{
    models::{
        self, buffs::BaseBuff, core::{CardLocation, Target}, monsters::BaseMonster, potions::BasePotion,
        relics::BaseRelic,
    },
    state::core::Monster
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
    Relic(RelicReference)
}

impl Binding {
    pub fn creature_ref(self) -> CreatureReference {
        match self {
            Binding::Buff(buff) => buff.creature,
            Binding::Card(_) => CreatureReference::Player,
            Binding::Potion(_) => CreatureReference::Player,
            Binding::Relic(_) => CreatureReference::Player,
            Binding::Creature(creature) => creature,
        }
    }

    pub fn monster_ref(self) -> Option<MonsterReference> {
        self.creature_ref().monster_ref()
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


impl Target {
    pub fn creature_ref(self, binding: Binding, action: Option<GameAction>) -> CreatureReference {
        match self {
            Target::_Self => binding.creature_ref(),
            Target::Attacker => {
                let action = action.expect("Expected action!");
                debug_assert!(action.is_attack, "Expected attack action!");
                action.creature
            }
            Target::TargetMonster => {
                action.expect("Expected action!").target.expect("Expected target!")
            }
            Target::Player => CreatureReference::Player,
            _ => panic!("Target does not resolve to a single creature! {:?}", self),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct GameAction {
    pub is_attack: bool,
    pub creature: CreatureReference,
    pub target: Option<CreatureReference>,
}