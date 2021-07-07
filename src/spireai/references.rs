use uuid::Uuid;

use crate::{
    models::{
        self, buffs::BaseBuff, core::CardLocation, events::BaseEvent, monsters::BaseMonster,
        potions::BasePotion, relics::BaseRelic,
    },
    state::{
        core::{Buff, Card, Creature, Event, Monster, Potion, Relic, Vars},
        game::GameState,
    },
};

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub struct CardReference {
    pub location: CardLocation,
    pub uuid: Uuid,
    pub base: &'static models::cards::BaseCard,
}

impl BindingReference for CardReference {
    type Item = Card;
    fn get(self, state: &GameState) -> Option<&Card> {
        match self.location {
            CardLocation::DeckPile => state.deck.get(&self.uuid),
            _ => state.battle_state.cards.get(&self.uuid),
        }
    }

    fn get_mut(self, state: &mut GameState) -> Option<&mut Card> {
        match self.location {
            CardLocation::DeckPile => state.deck.get_mut(&self.uuid),
            _ => state.battle_state.cards.get_mut(&self.uuid),
        }
    }
}

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub struct MonsterReference {
    pub base: &'static BaseMonster,
    pub uuid: Uuid,
}

impl BindingReference for MonsterReference {
    type Item = Monster;
    fn get(self, state: &GameState) -> Option<&Monster> {
        state.battle_state.monsters.get(&self.uuid)
    }

    fn get_mut(self, state: &mut GameState) -> Option<&mut Monster> {
        state.battle_state.monsters.get_mut(&self.uuid)
    }
}

impl MonsterReference {
    pub fn creature_ref(self) -> CreatureReference {
        CreatureReference::Creature(self.uuid)
    }
}

#[derive(Eq, Debug, Clone, Copy, PartialEq)]
pub enum CreatureReference {
    Player,
    Creature(Uuid),
}

impl BindingReference for CreatureReference {
    type Item = Creature;
    fn get(self, state: &GameState) -> Option<&Creature> {
        match self {
            CreatureReference::Creature(uuid) => {
                state.battle_state.monsters.get(&uuid).map(|m| &m.creature)
            }
            CreatureReference::Player => Some(&state.player),
        }
    }

    fn get_mut(self, state: &mut GameState) -> Option<&mut Creature> {
        match self {
            CreatureReference::Creature(uuid) => state
                .battle_state
                .monsters
                .get_mut(&uuid)
                .map(|m| &mut m.creature),
            CreatureReference::Player => Some(&mut state.player),
        }
    }
}

impl CreatureReference {
    pub fn get_monster(self, state: &GameState) -> Option<&Monster> {
        match self {
            CreatureReference::Creature(uuid) => state.battle_state.monsters.get(&uuid),
            CreatureReference::Player => None,
        }
    }

    pub fn get_monster_mut(self, state: &mut GameState) -> Option<&mut Monster> {
        match self {
            CreatureReference::Creature(uuid) => state.battle_state.monsters.get_mut(&uuid),
            CreatureReference::Player => None,
        }
    }

    pub fn monster_ref(self, state: &GameState) -> MonsterReference {
        match self {
            CreatureReference::Player => panic!("Cannot convert player to monster reference"),
            CreatureReference::Creature(uuid) => state.battle_state.monsters[&uuid].monster_ref(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BuffReference {
    pub base: &'static BaseBuff,
    pub creature: CreatureReference,
    pub buff: Uuid,
}

impl BindingReference for BuffReference {
    type Item = Buff;
    fn get(self, state: &GameState) -> Option<&Buff> {
        self.creature
            .get(state)
            .and_then(|c| c.buffs.get(&self.buff))
    }

    fn get_mut(self, state: &mut GameState) -> Option<&mut Buff> {
        self.creature
            .get_mut(state)
            .and_then(|c| c.buffs.get_mut(&self.buff))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PotionReference {
    pub base: &'static BasePotion,
    pub index: usize,
}

impl BindingReference for PotionReference {
    type Item = Potion;
    fn get(self, state: &GameState) -> Option<&Potion> {
        state.potions.get(self.index).unwrap().as_ref()
    }

    fn get_mut(self, state: &mut GameState) -> Option<&mut Potion> {
        state.potions.get_mut(self.index).unwrap().as_mut()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelicReference {
    pub base: &'static BaseRelic,
    pub relic: Uuid,
}

impl BindingReference for RelicReference {
    type Item = Relic;
    fn get(self, state: &GameState) -> Option<&Relic> {
        state.relics.get(&self.relic)
    }

    fn get_mut(self, state: &mut GameState) -> Option<&mut Relic> {
        state.relics.get_mut(&self.relic)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventReference {
    pub base: &'static BaseEvent,
}

impl BindingReference for EventReference {
    type Item = Event;
    fn get(self, state: &GameState) -> Option<&Event> {
        state.event_state.as_ref()
    }

    fn get_mut(self, state: &mut GameState) -> Option<&mut Event> {
        state.event_state.as_mut()
    }
}

pub trait BindingReference {
    type Item;

    fn get(self, state: &GameState) -> Option<&Self::Item>;
    fn get_mut(self, state: &mut GameState) -> Option<&mut Self::Item>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Binding {
    Buff(BuffReference),
    Card(CardReference),
    Creature(CreatureReference),
    Monster(MonsterReference),
    Potion(PotionReference),
    Relic(RelicReference),
    Event(EventReference),
}

impl Binding {
    pub fn get_creature(self) -> CreatureReference {
        match self {
            Binding::Buff(buff) => buff.creature,
            Binding::Card(_) => CreatureReference::Player,
            Binding::Potion(_) => CreatureReference::Player,
            Binding::Relic(_) => CreatureReference::Player,
            Binding::Creature(creature) => creature,
            Binding::Monster(monster) => monster.creature_ref(),
            Binding::Event(_) => CreatureReference::Player,
        }
    }

    pub fn get_monster(self, state: &GameState) -> Option<&Monster> {
        match self {
            Binding::Buff(buff) => buff.creature.get_monster(state),
            Binding::Creature(creature) => creature.get_monster(state),
            Binding::Monster(monster) => monster.creature_ref().get_monster(state),
            Binding::Card(_) | Binding::Potion(_) | Binding::Relic(_) | Binding::Event(_) => None,
        }
    }

    pub fn get_vars(self, state: &GameState) -> &Vars {
        match self {
            Binding::Buff(buff) => &state.get(buff).vars,
            Binding::Card(card) => &state.get(card).vars,
            Binding::Creature(creature) => &creature.get_monster(state).unwrap().vars,
            Binding::Monster(monster) => &state.get(monster).vars,
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.index)
            }
            Binding::Event(event) => &state.get(event).vars,
            Binding::Relic(relic) => &state.get(relic).vars,
        }
    }

    pub fn get_mut_vars(self, state: &mut GameState) -> &mut Vars {
        match self {
            Binding::Buff(buff) => &mut state.get_mut(buff).vars,
            Binding::Card(card) => &mut state.get_mut(card).vars,
            Binding::Creature(creature) => &mut creature.get_monster_mut(state).unwrap().vars,
            Binding::Monster(monster) => &mut state.get_mut(monster).vars,
            Binding::Potion(potion) => {
                panic!("Unexpected vars check on potion: {}", potion.index)
            }
            Binding::Event(event) => &mut state.get_mut(event).vars,
            Binding::Relic(relic) => &mut state.get_mut(relic).vars,
        }
    }

    pub fn is_upgraded(self, game_state: &GameState) -> bool {
        match self {
            Binding::Card(card) => game_state.get(card).upgrades > 0,
            Binding::Potion(_) => game_state.relic_names.contains_key("Sacred Bark"),
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
        CreatureReference::Creature(self.uuid)
    }

    pub fn buffs(&self) -> impl Iterator<Item = BuffReference> + '_ {
        self.creature.buffs.values().map(move |b| BuffReference {
            base: b.base,
            creature: self.creature_ref(),
            buff: b.uuid,
        })
    }
}
