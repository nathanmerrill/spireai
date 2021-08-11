use im::{HashMap, Vector};
use uuid::Uuid;

use crate::comm::request as external;
use crate::models::core as internal_core;
use crate::models::monsters::Intent as NewIntent;
use crate::state as internal;

pub fn state_matches(
    external: &external::GameState,
    internal: &internal::game::GameState,
    uuid_map: &mut HashMap<String, Uuid>,
) -> bool {
    (if let Some(combat_state) = &external.combat_state {
        buffs_match(
            &combat_state.player.powers,
            &internal.player.buffs,
            uuid_map,
        ) && combat_state.player.block as u16 == internal.player.block
            && battle_state_matches(combat_state, &internal.battle_state, uuid_map)
    } else {
        !internal.battle_state.active
    }) && class_matches(&external.class, internal.class)
        && external.current_hp as u16 == internal.player.hp
        && external.max_hp as u16 == internal.player.max_hp
        && external.gold as u16 == internal.gold
        && external.floor as u8 == internal.floor
        && floor_state_matches(external, &internal.floor_state)
        && cards_match(&external.deck, &internal.deck, uuid_map)
        && potions_match(&external.potions, &internal.potions)
        && relics_match(&external.relics, &internal.relics, uuid_map)
        && external.act as u8 == internal.act
        && external.ascension_level as u8 == internal.asc
}
/*
pub fn convert_map(state: &external::GameState) -> internal::MapState {
    let mut nodes: HashMap<(i8, i8), internal::MapNode> = HashMap::new();
    for node in &state.map {
        let new_node = convert_node(node);
        nodes.insert((new_node.floor, new_node.x), new_node);
    }

    let (x, y) = match &state.screen_state {
        external::ScreenState::Map(current) => match &current.current_node {
            Some(node) => (node.x, node.y),
            None => (-1, state.floor),
        },
        _ => (-1, state.floor),
    };

    internal::MapState {
        nodes,
        x: x as i8,
        floor: y as i8,
    }
}
pub fn convert_node(node: &external::MapNode) -> internal::MapNode {
    internal::MapNode {
        floor: node.y as i8,
        icon: match node.symbol {
            'M' => internal::MapNodeIcon::Monster,
            '?' => internal::MapNodeIcon::Question,
            '$' => internal::MapNodeIcon::Shop,
            'R' => internal::MapNodeIcon::Campfire,
            'T' => internal::MapNodeIcon::Chest,
            'E' => internal::MapNodeIcon::Elite,
            _ => panic!("Unhandled node type: {}", node.symbol),
        },
        next: node.children.iter().map(|a| a.x as i8).collect(),
        x: node.x as i8,
    }
}
 */

pub fn relics_match(
    external: &[external::Relic],
    internal: &HashMap<Uuid, internal::core::Relic>,
    uuid_map: &mut HashMap<String, Uuid>,
) -> bool {
    sets_match(external, internal, uuid_map, relic_matches, |relic| {
        relic.id.to_string()
    })
}

pub fn relic_matches(external: &external::Relic, internal: &internal::core::Relic) -> bool {
    external.name == internal.base.name
}

pub fn battle_state_matches(
    external: &external::CombatState,
    internal: &internal::battle::BattleState,
    uuid_map: &mut HashMap<String, Uuid>,
) -> bool {
    internal.active
        && cards_match(
            &external.hand,
            &internal
                .hand
                .iter()
                .map(|u| (*u, internal.cards[u].clone()))
                .collect(),
            uuid_map,
        )
        && cards_match(
            &external.draw_pile,
            &internal
                .draw
                .iter()
                .map(|u| (*u, internal.cards[u].clone()))
                .collect(),
            uuid_map,
        )
        && cards_match(
            &external.discard_pile,
            &internal
                .discard
                .iter()
                .map(|u| (*u, internal.cards[u].clone()))
                .collect(),
            uuid_map,
        )
        && cards_match(
            &external.exhaust_pile,
            &internal
                .exhaust
                .iter()
                .map(|u| (*u, internal.cards[u].clone()))
                .collect(),
            uuid_map,
        )
        && external.cards_discarded_this_turn as u8 == internal.discard_count
        && monsters_match(&external.monsters, &internal.monsters, uuid_map)
        && external.player.energy as u8 == internal.energy
        && orbs_match(&external.player.orbs, &internal.orbs)
}

pub fn floor_state_matches(
    external: &external::GameState,
    internal: &internal::game::FloorState,
) -> bool {
    match &external.screen_state {
        external::ScreenState::None {} => match &external.room_phase {
            external::RoomPhase::Combat => internal == &internal::game::FloorState::Battle,
            _ => false,
        },
        external::ScreenState::Event(_) => internal == &internal::game::FloorState::Event,
        external::ScreenState::Map(_) => internal == &internal::game::FloorState::Map,
        external::ScreenState::CombatReward(external_rewards) => {
            if let internal::game::FloorState::Rewards(internal_rewards) = internal {
                rewards_match(external_rewards, internal_rewards)
            } else {
                false
            }
        }
        external::ScreenState::CardReward(external_rewards) => {
            if let internal::game::FloorState::CardReward(internal_rewards) = internal {
                external_rewards.cards.iter().all(|card| {
                    internal_rewards.iter().any(|(name, upgraded)| {
                        &card.name == name && (card.upgrades > 0) == *upgraded
                    })
                })
            } else {
                false
            }
        }
        external::ScreenState::ShopRoom {} => internal == &internal::game::FloorState::ShopEntrance,
        external::ScreenState::ShopScreen(_) => true, // Shops statistically will never match
        external::ScreenState::Rest(_) => internal == &internal::game::FloorState::Rest,
        external::ScreenState::Grid(grid) => match &external.room_phase {
            external::RoomPhase::Combat => internal == &internal::game::FloorState::Battle,
            external::RoomPhase::Event => {
                if grid.for_purge {
                    internal == &internal::game::FloorState::EventRemove(grid.num_cards as u8)
                } else if grid.for_transform {
                    internal
                        == &internal::game::FloorState::EventTransform(grid.num_cards as u8, false)
                } else if grid.for_upgrade {
                    internal == &internal::game::FloorState::EventUpgrade(grid.num_cards as u8)
                } else {
                    panic!("Unexpected grid in event")
                }
            }
            _ => panic!("Unexpected room phase in grid choice"),
        },
        external::ScreenState::Chest(chest) => {
            if let internal::game::FloorState::Chest(chest_type) = internal {
                chest_matches(chest, *chest_type)
            } else {
                false
            }
        }
        // ScreenState::CardReward(CardReward),
        // ScreenState::BossReward(Vec<Relic>),
        // ScreenState::Grid(Grid) => internal::game::FloorState::CardSelect,
        // ScreenState::HandSelect(HandSelect),
        external::ScreenState::GameOver(_) => internal == &internal::game::FloorState::GameOver,
        // ScreenState::Complete,
        _ => panic!("Unhandled screen state"),
    }
}

fn chest_matches(external: &external::Chest, internal: internal_core::ChestType) -> bool {
    internal
        == match external.chest_type {
            external::ChestType::SmallChest => internal_core::ChestType::Small,
            external::ChestType::MediumChest => internal_core::ChestType::Medium,
            external::ChestType::LargeChest => internal_core::ChestType::Large,
            external::ChestType::BossChest => internal_core::ChestType::Boss,
            _ => panic!("Unexpected type of chest"),
        }
}

fn rewards_match(
    external: &external::CombatRewards,
    internal: &Vector<internal::game::Reward>,
) -> bool {
    external.rewards.iter().all(|a| {
        internal.iter().any(|b| match a {
            external::RewardType::Card => b == &internal::game::Reward::CardChoice,
            external::RewardType::EmeraldKey => b == &internal::game::Reward::EmeraldKey,
            external::RewardType::Gold { gold } => b == &internal::game::Reward::Gold(*gold as u8),
            external::RewardType::Potion { potion } => {
                if let internal::game::Reward::Potion(p) = b {
                    potion.name == p.base.name
                } else {
                    false
                }
            }
            external::RewardType::Relic { relic } => {
                if let internal::game::Reward::Relic(r) = b {
                    relic.name == r.base.name
                } else {
                    false
                }
            }
            external::RewardType::StolenGold { gold } => {
                b == &internal::game::Reward::Gold(*gold as u8)
            }
            external::RewardType::SapphireKey { link } => {
                if let internal::game::Reward::SapphireKey(r) = b {
                    link.name == r.base.name
                } else {
                    false
                }
            }
        })
    })
}

fn events_match(external: &external::Event, internal: &internal::core::Event) -> bool {
    external.event_name == internal.base.name
        && external.options.iter().all(|option| {
            internal
                .available_choices
                .iter()
                .any(|a| a == &option.label)
        })
}

fn orbs_match(
    external_map: &[external::OrbType],
    internal_map: &Vector<internal::core::Orb>,
) -> bool {
    if external_map.len() != internal_map.len() {
        return false;
    }

    for (idx, external) in external_map.iter().enumerate() {
        let internal = &internal_map[idx];
        if !(internal.base
            == match external.name.as_str() {
                "Lightning" => internal_core::OrbType::Lightning,
                "Dark" => internal_core::OrbType::Dark,
                "Frost" => internal_core::OrbType::Frost,
                "Plasma" => internal_core::OrbType::Plasma,
                _ => panic!("Unrecognized orb type"),
            }
            && external.evoke_amount as u16 == internal.n)
        {
            return false;
        }
    }

    true
}

fn monsters_match(
    external_map: &[external::Monster],
    internal_map: &HashMap<Uuid, internal::core::Monster>,
    uuid_map: &mut HashMap<String, Uuid>,
) -> bool {
    if external_map.len() != internal_map.len() {
        return false;
    }

    for internal in internal_map.values() {
        let external = &external_map[internal.position];
        if !(buffs_match(&external.powers, &internal.creature.buffs, uuid_map)
            && external.current_hp as u16 == internal.creature.hp
            && external.max_hp as u16 == internal.creature.hp
            && external.block as u16 == internal.creature.block
            && external.is_gone != internal.targetable
            && external.name == internal.base.name
            && intent_matches(&external.intent, internal.intent))
        {
            return false;
        }
    }
    true
}

fn intent_matches(external: &external::Intent, internal: NewIntent) -> bool {
    internal
        == match external {
            external::Intent::Attack => NewIntent::Attack,
            external::Intent::AttackBuff => NewIntent::AttackBuff,
            external::Intent::AttackDebuff => NewIntent::AttackDebuff,
            external::Intent::AttackDefend => NewIntent::AttackDefend,
            external::Intent::Buff => NewIntent::Buff,
            external::Intent::Debuff => NewIntent::Debuff,
            external::Intent::StrongDebuff => NewIntent::StrongDebuff,
            external::Intent::Defend => NewIntent::Defend,
            external::Intent::DefendDebuff => NewIntent::DefendDebuff,
            external::Intent::DefendBuff => NewIntent::DefendBuff,
            external::Intent::Escape => NewIntent::Escape,
            external::Intent::None => NewIntent::None,
            external::Intent::Sleep => NewIntent::Sleep,
            external::Intent::Stun => NewIntent::Stun,
            external::Intent::Unknown => NewIntent::Unknown,
            external::Intent::Debug | external::Intent::Magic => {
                panic!("Unrecognized intent: {:?}", external)
            }
        }
}

fn buffs_match(
    external: &[external::Power],
    internal: &HashMap<Uuid, internal::core::Buff>,
    uuid_map: &mut HashMap<String, Uuid>,
) -> bool {
    sets_match(
        external,
        internal,
        uuid_map,
        |a, b| a.name == b.base.name && a.amount as i16 == b.vars.n,
        |a| a.id.to_string(),
    )
}

fn potions_match(
    external: &[external::Potion],
    internal: &Vector<Option<internal::core::Potion>>,
) -> bool {
    if external.len() != internal.len() {
        return false;
    }

    for idx in 0..external.len() {
        let name = match &internal[idx] {
            None => "Potion Slot",
            Some(p) => p.base.name.as_str(),
        };

        if name != external[idx].name {
            return false;
        }
    }

    true
}

fn sets_match<A, B, F, T>(
    external: &[A],
    internal: &HashMap<Uuid, B>,
    uuid_map: &mut HashMap<String, Uuid>,
    matcher: F,
    id: T,
) -> bool
where
    B: Clone,
    F: Fn(&A, &B) -> bool,
    T: Fn(&A) -> String,
{
    if external.len() != internal.len() {
        return false;
    }

    let mut remaining = Vec::new();
    let mut used_uuids = HashMap::new();

    for external_item in external {
        if let Some(uuid) = uuid_map.get(&id(external_item)) {
            let internal_item = internal[uuid].clone();
            if !matcher(external_item, &internal_item) {
                return false;
            }
            used_uuids.insert(*uuid, internal_item);
        } else {
            remaining.push(external_item)
        }
    }

    let remaining_uuids = used_uuids.symmetric_difference(internal.clone());

    for external_item in remaining {
        let mut found_match = false;
        for (uuid, internal_item) in &remaining_uuids {
            if matcher(external_item, internal_item) {
                found_match = true;
                uuid_map.insert(id(external_item), *uuid);
                break;
            }
        }

        if !found_match {
            return false;
        }
    }

    true
}

fn cards_match(
    external: &[external::Card],
    internal: &HashMap<Uuid, internal::core::Card>,
    uuid_map: &mut HashMap<String, Uuid>,
) -> bool {
    sets_match(external, internal, uuid_map, card_matches, |a| {
        a.id.to_string()
    })
}

fn card_matches(external: &external::Card, internal: &internal::core::Card) -> bool {
    let name = if external.name.ends_with('+') {
        &external.name[0..external.name.len() - 1]
    } else {
        external.name.as_str()
    };
    name == internal.base.name
        && external.upgrades as u8 == internal.upgrades
        && external.cost as u8 == internal.cost
}

fn class_matches(external: &external::PlayerClass, internal: internal_core::Class) -> bool {
    internal
        == match external {
            external::PlayerClass::Ironclad => internal_core::Class::Ironclad,
            external::PlayerClass::Silent => internal_core::Class::Silent,
            external::PlayerClass::Defect => internal_core::Class::Defect,
            external::PlayerClass::Watcher => internal_core::Class::Watcher,
            external::PlayerClass::Other => panic!("Unrecognized class"),
        }
}
