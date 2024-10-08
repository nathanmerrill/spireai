use im::{HashMap, HashSet, Vector};
use uuid::Uuid;

use crate::comm::request as external;
use crate::models::monsters::Intent as NewIntent;
use crate::models::{self, core as internal_core};
use crate::state as internal;

pub fn state_matches(
    external: &Option<external::GameState>,
    internal: &internal::floor::FloorState,
    uuid_map: &mut HashMap<String, Uuid>,
) -> bool {
    if let Some(external) = external {
        if let Some(combat_state) = &external.combat_state {
            if let internal::floor::FloorState::Battle(battle_state) = &internal {
                battle_state_matches(combat_state, battle_state, uuid_map)
            } else {
                false
            }
        } else {
            let game = internal.game_state();
            class_matches(&external.class, game.class)
                && external.current_hp as u16 == game.hp.amount
                && external.max_hp as u16 == game.hp.max
                && external.gold as u16 == game.gold
                && external.floor as i8 == game.map.floor
                && floor_state_matches(external, internal, uuid_map)
                && cards_match(&external.deck, &game.deck, uuid_map)
                && potions_match(&external.potions, &game.potions)
                && relics_match(&external.relics, &game.relics, uuid_map)
                && external.act as u8 == game.act
                && external.ascension_level as u8 == game.asc
        }
    } else {
        matches!(internal, internal::floor::FloorState::Menu)
    }
}

pub fn update_state(
    external: &Option<external::GameState>,
    internal: &mut internal::floor::FloorState,
) {
    if let Some(external) = external {
        match external.floor {
            0 | 18 | 35 | 52 => {
                if external.combat_state.is_none() {
                    internal.game_state_mut().map = convert_map(external);
                }
            }
            _ => {}
        }

        if let external::ScreenState::ShopScreen(state) = &external.screen_state {
            if let internal::floor::FloorState::Shop(shop_state) = internal {
                if !shop_state.updated {
                    convert_shop(state, shop_state);
                }
            } else {
                panic!("Expected shop state!");
            }
        }
    }
}

pub fn convert_shop(state: &external::ShopScreen, shop: &mut internal::shop::ShopState) {
    shop.generated = true;
    shop.updated = true;
    shop.cards = state
        .cards
        .iter()
        .map(|a| {
            (
                internal::core::CardOffer {
                    base: models::cards::by_name(&a.name),
                    upgraded: a.upgrades > 0,
                },
                a.price.unwrap() as u16,
            )
        })
        .collect();
    shop.potions = state
        .potions
        .iter()
        .map(|a| (models::potions::by_name(&a.name), a.price.unwrap() as u16))
        .collect();
    shop.relics = state
        .relics
        .iter()
        .map(|a| (models::relics::by_name(&a.name), a.price.unwrap() as u16))
        .collect();
    shop.can_purge = state.purge_available;
}

pub fn convert_map(state: &external::GameState) -> internal::map::MapState {
    let mut nodes = [None; 105];
    for node in &state.map {
        let new_node = convert_node(node);
        nodes[new_node.index()] = Some(new_node)
    }

    let (x, y) = match &state.screen_state {
        external::ScreenState::Map(current) => match &current.current_node {
            Some(node) => (node.x, node.y),
            None => (-1, state.floor),
        },
        _ => (-1, state.floor),
    };

    internal::map::MapState {
        nodes,
        index: if y >= 0 {
            Some((y * 7 + x) as usize)
        } else {
            None
        },
        floor: y as i8,
        boss: state.act_boss.as_ref().unwrap().to_string(),
        history: internal::map::MapHistory {
            last_elite: None,
            last_normal: None,
            easy_fight_count: 0,
            unknown_normal_count: 0,
            unknown_shop_count: 0,
            unknown_treasure_count: 0,
            event_history: HashSet::new(),
            last_shop: false,
        },
    }
}

pub fn convert_node(node: &external::MapNode) -> internal::map::MapNode {
    internal::map::MapNode {
        x: node.x as u8,
        y: node.y as u8,
        left: node.children.iter().any(|a| a.x + 1 == node.x),
        up: node.children.iter().any(|a| a.x == node.x),
        right: node.children.iter().any(|a| a.x == node.x + 1),
        icon: match node.symbol {
            'M' => internal::map::MapNodeIcon::Monster,
            '?' => internal::map::MapNodeIcon::Question,
            '$' => internal::map::MapNodeIcon::Shop,
            'R' => internal::map::MapNodeIcon::Campfire,
            'T' => internal::map::MapNodeIcon::Chest,
            'E' => internal::map::MapNodeIcon::Elite,
            _ => panic!("Unhandled node type: {}", node.symbol),
        },
    }
}

pub fn relics_match(
    external: &[external::Relic],
    internal: &[internal::core::Relic],
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
    buffs_match(&external.player.powers, &internal.player.buffs, uuid_map)
        && external.player.block as u16 == internal.player.block
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
    internal: &internal::floor::FloorState,
    uuid_map: &mut HashMap<String, Uuid>,
) -> bool {
    match &external.screen_state {
        external::ScreenState::None {} => match &external.room_phase {
            external::RoomPhase::Combat => {
                if let internal::floor::FloorState::Battle(battle_state) = internal {
                    battle_state_matches(
                        external.combat_state.as_ref().unwrap(),
                        battle_state,
                        uuid_map,
                    )
                } else {
                    false
                }
            }
            _ => false,
        },
        external::ScreenState::Event(event) => {
            if let internal::floor::FloorState::Event(event_state) = internal {
                events_match(event, event_state)
            } else {
                false
            }
        }
        external::ScreenState::Map(_) => {
            matches!(internal, internal::floor::FloorState::Map(_))
        }
        external::ScreenState::CombatReward(external_rewards) => {
            if let internal::floor::FloorState::BattleRewards(battle_over) = internal {
                rewards_match(external_rewards, &battle_over.rewards.rewards)
            } else {
                false
            }
        }
        external::ScreenState::ShopRoom {} => {
            matches!(internal, internal::floor::FloorState::Shop(_))
        }
        external::ScreenState::ShopScreen(external) => {
            match internal {
                internal::floor::FloorState::Shop(internal) => {
                    internal.cards.iter().zip(&external.cards).all(
                        |((offer, price), external_card)| {
                            card_offer_matches(external_card, offer, *price)
                        },
                    ) && internal.relics.iter().zip(&external.relics).all(
                        |((offer, price), external_relic)| {
                            offer.name == external_relic.name
                                && external_relic.price.unwrap() as u16 == *price
                        },
                    ) && internal.potions.iter().zip(&external.potions).all(
                        |((offer, price), external_potion)| {
                            offer.name == external_potion.name
                                && external_potion.price.unwrap() as u16 == *price
                        },
                    ) && internal.can_purge == external.purge_available
                }
                _ => false,
            }
        }
        external::ScreenState::Rest(_) => matches!(internal, internal::floor::FloorState::Rest(_)),
        external::ScreenState::Chest(chest) => {
            if let internal::floor::FloorState::Chest(chest_type) = internal {
                chest_matches(chest, chest_type.chest)
            } else {
                false
            }
        }
        external::ScreenState::GameOver(_) => {
            matches!(internal, internal::floor::FloorState::GameOver(..))
        }
        _ => true,
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
    internal: &Vector<internal::core::Reward>,
) -> bool {
    external.rewards.iter().all(|a| {
        internal.iter().any(|b| match a {
            external::RewardType::Card => matches!(b, &internal::core::Reward::CardChoice(_, _, _)),
            external::RewardType::EmeraldKey => b == &internal::core::Reward::EmeraldKey,
            external::RewardType::Gold { gold } => b == &internal::core::Reward::Gold(*gold as u16),
            external::RewardType::Potion { potion } => {
                if let internal::core::Reward::Potion(p) = b {
                    potion.name == p.name
                } else {
                    false
                }
            }
            external::RewardType::Relic { relic } => {
                if let internal::core::Reward::Relic(r) = b {
                    relic.name == r.name
                } else {
                    false
                }
            }
            external::RewardType::StolenGold { gold } => {
                b == &internal::core::Reward::Gold(*gold as u16)
            }
            external::RewardType::SapphireKey { link } => {
                if let internal::core::Reward::SapphireLinkedRelic(r) = b {
                    link.name == r.name
                } else {
                    false
                }
            }
        })
    })
}

fn events_match(external: &external::Event, internal: &internal::event::EventState) -> bool {
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
            && external.current_hp as u16 == internal.creature.hp.amount
            && external.max_hp as u16 == internal.creature.hp.max
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
    internal: &Vector<Option<&'static models::potions::BasePotion>>,
) -> bool {
    if external.len() != internal.len() {
        return false;
    }

    for idx in 0..external.len() {
        let name = match &internal[idx] {
            None => "Potion Slot",
            Some(p) => p.name.as_str(),
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

fn card_offer_matches(
    external: &external::Card,
    internal: &internal::core::CardOffer,
    price: u16,
) -> bool {
    let name = if external.name.ends_with('+') {
        &external.name[0..external.name.len() - 1]
    } else {
        external.name.as_str()
    };
    name == internal.base.name
        && (external.upgrades > 0) == internal.upgraded
        && external.price.unwrap() as u16 == price
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
