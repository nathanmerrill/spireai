use im::{HashMap, Vector};
use uuid::Uuid;

use crate::{comm::request as old, models::state::UuidVector};
use crate::models::core as new_core;
use crate::models::monsters::Intent as NewIntent;
use crate::models::state as new;

pub fn convert_state(state: &old::GameState, uuid_map: &mut HashMap<String, Uuid>) -> new::GameState {
    let relics = convert_relics(&state.relics, uuid_map);
    let buffs = convert_buffs(&state.combat_state.as_ref().unwrap().player.powers, uuid_map);

    new::GameState {
        class: convert_class(&state.class),
        player: new::Creature {
            hp: state.current_hp as u16,
            max_hp: state.max_hp as u16,
            is_player: true,
            buff_names: buffs.items.iter().map(|(uuid, buff)| (buff.base.name.to_string(), *uuid)).collect(),
            buffs_when: HashMap::new(),
            buffs: buffs,
            block: state
                .combat_state
                .as_ref()
                .map(|a| a.player.block)
                .unwrap_or(0) as u16,
        },
        event_state: None,
        battle_state: state
            .combat_state
            .as_ref()
            .map(|a| convert_battle_state(a, state, uuid_map))
            .unwrap_or_else(|| new::BattleState::default()),
        gold: state.gold as u16,
        floor: state.floor as u8,
        floor_state: convert_floor_state(state, uuid_map),
        deck: convert_cards(&state.deck, uuid_map),
        map: convert_map(state),
        potions: convert_potions(&state.potions),
        act: state.act as u8,
        asc: state.ascension_level as u8,
        relic_whens: HashMap::new(),
        relic_names: relics.items.iter().map(|(uuid, r)|(r.base.name.to_string(), *uuid)).collect(),
        relics: relics,
        keys: None,
        won: None
    }
}

pub fn convert_map(state: &old::GameState) -> new::MapState {
    let mut nodes: HashMap<(i8, i8), new::MapNode> = HashMap::new();
    for node in &state.map {
        let new_node = convert_node(node);
        nodes.insert((new_node.floor, new_node.x), new_node);
    }

    let (x, y) = match &state.screen_state {
        old::ScreenState::Map(current) => match &current.current_node {
            Some(node) => (node.x, node.y),
            None => (-1, state.floor),
        },
        _ => (-1, state.floor),
    };

    new::MapState {
        nodes,
        x: x as i8,
        floor: y as i8,
    }
}
pub fn convert_node(node: &old::MapNode) -> new::MapNode {
    new::MapNode {
        floor: node.y as i8,
        icon: match node.symbol {
            'M' => new::MapNodeIcon::Monster,
            '?' => new::MapNodeIcon::Question,
            '$' => new::MapNodeIcon::Shop,
            'R' => new::MapNodeIcon::Campfire,
            'T' => new::MapNodeIcon::Chest,
            'E' => new::MapNodeIcon::Elite,
            _ => panic!("Unhandled node type: {}", node.symbol),
        },
        next: node.children.iter().map(|a| a.x as i8).collect(),
        x: node.x as i8,
    }
}

pub fn convert_relic(relic: &old::Relic, uuid_map: &mut HashMap<String, Uuid>) -> new::Relic {
    let uuid = uuid_map.entry(relic.id.to_string()).or_insert_with(Uuid::new_v4);

    new::Relic {
        base: crate::models::relics::by_name(relic.name.as_str()),
        vars: new::Vars {
            n: relic.counter as i16,
            x: 0,
            n_reset: 0,
        },
        uuid: *uuid,
        enabled: true,
    }
}

pub fn convert_relics(relics: &[old::Relic], uuid_map: &mut HashMap<String, Uuid>) -> UuidVector<new::Relic> {
    let mut vector = UuidVector::new();
    for relic in relics {
        let new_relic = convert_relic(relic, uuid_map);
        vector.add(new_relic);
    }
    vector
}

pub fn convert_intent(intent: &old::Intent) -> NewIntent {
    match intent {
        old::Intent::Attack => NewIntent::Attack,
        old::Intent::AttackBuff => NewIntent::AttackBuff,
        old::Intent::AttackDebuff => NewIntent::AttackDebuff,
        old::Intent::AttackDefend => NewIntent::AttackDefend,
        old::Intent::Buff => NewIntent::Buff,
        old::Intent::Debuff => NewIntent::Debuff,
        old::Intent::StrongDebuff => NewIntent::StrongDebuff,
        old::Intent::Defend => NewIntent::Defend,
        old::Intent::DefendDebuff => NewIntent::DefendDebuff,
        old::Intent::DefendBuff => NewIntent::DefendBuff,
        old::Intent::Escape => NewIntent::Escape,
        old::Intent::None => NewIntent::None,
        old::Intent::Sleep => NewIntent::Sleep,
        old::Intent::Stun => NewIntent::Stun,
        old::Intent::Unknown => NewIntent::Unknown,
        old::Intent::Debug | old::Intent::Magic => panic!("Unrecognized intent: {:?}", intent),
    }
}

pub fn convert_battle_state(
    state: &old::CombatState,
    game_state: &old::GameState,
    uuid_map: &mut HashMap<String, Uuid>
) -> new::BattleState {
    new::BattleState {
        active: true,
        draw: convert_cards(&state.draw_pile, uuid_map).items,
        discard_count: state.cards_discarded_this_turn as u8,
        draw_bottom_known: Vector::new(),
        draw_top_known: Vector::new(),
        play_count: 0,
        discard: convert_cards(&state.discard_pile, uuid_map),
        exhaust: convert_cards(&state.exhaust_pile, uuid_map),
        hand: convert_cards(&state.hand, uuid_map),
        last_card_played: None,
        orb_slots: 0,
        monsters: convert_monsters(&state.monsters, uuid_map),
        energy: state.player.energy as u8,
        orbs: convert_orbs(&state.player.orbs),
        stance: new_core::Stance::None,
        battle_type: new::BattleType::Common,
        card_choices: convert_card_choices(game_state, uuid_map),
        card_choice_type: convert_card_choice_type(game_state)
    }
}

pub fn convert_card_choices(game_state: &old::GameState, uuid_map: &mut HashMap<String, Uuid>) -> UuidVector<new::Card> {
    match &game_state.screen_state {
        old::ScreenState::Grid(grid) => convert_cards(&grid.cards, uuid_map),
        _ => UuidVector::new(),
    }
}

pub fn convert_card_choice_type(game_state: &old::GameState) -> new::CardChoiceType {
    match &game_state.current_action {
        Some(s) => match s.as_str() {
            "ScryAction" => new::CardChoiceType::Scry,
            "DamageAction" => new::CardChoiceType::None,
            _ => panic!("Unexpected action type: {}", s),
        },
        None => new::CardChoiceType::None,
    }
}

pub fn convert_floor_state(state: &old::GameState, uuid_map: &mut HashMap<String, Uuid>) -> new::FloorState {
    match &state.screen_state {
        old::ScreenState::None {} => match &state.room_phase {
            old::RoomPhase::Combat => new::FloorState::Battle,
            _ => panic!("Expected Battle in None state"),
        },
        old::ScreenState::Event(_) => new::FloorState::Event,
        old::ScreenState::Map(_) => new::FloorState::Map,
        old::ScreenState::CombatReward(rewards) => {
            new::FloorState::Rewards(convert_rewards(rewards, uuid_map))
        }
        old::ScreenState::CardReward(reward) => new::FloorState::CardReward(
            reward
                .cards
                .iter()
                .map(|a| (a.name.to_string(), a.upgrades > 0))
                .collect(),
        ),
        old::ScreenState::ShopRoom {} => new::FloorState::ShopEntrance,
        old::ScreenState::ShopScreen(screen) => convert_shop(screen),
        old::ScreenState::Rest(_) => new::FloorState::Rest,
        old::ScreenState::Grid(grid) => match &state.room_phase {
            old::RoomPhase::Combat => new::FloorState::Battle,
            old::RoomPhase::Event => {
                if grid.for_purge {
                    new::FloorState::EventRemove(grid.num_cards as u8)
                } else if grid.for_transform {
                    new::FloorState::EventTransform(grid.num_cards as u8, false)
                } else if grid.for_upgrade {
                    new::FloorState::EventUpgrade(grid.num_cards as u8)
                } else {
                    panic!("Unexpected grid in event")
                }
            }
            _ => panic!("Unexpected room phase in grid choice"),
        },
        old::ScreenState::Chest(chest) => convert_chest(chest),
        // ScreenState::CardReward(CardReward),
        // ScreenState::BossReward(Vec<Relic>),
        // ScreenState::Grid(Grid) => new::FloorState::CardSelect,
        // ScreenState::HandSelect(HandSelect),
        old::ScreenState::GameOver(_) => new::FloorState::GameOver,
        // ScreenState::Complete,
        _ => panic!("Unhandled screen state"),
    }
}

fn convert_chest(chest: &old::Chest) -> new::FloorState {
    if chest.chest_open {
        panic!("Not sure how to handle open chest")
    } else {
        let chest_type = match chest.chest_type {
            old::ChestType::SmallChest => new_core::ChestType::Small,
            old::ChestType::MediumChest => new_core::ChestType::Medium,
            old::ChestType::LargeChest => new_core::ChestType::Large,
            old::ChestType::BossChest => new_core::ChestType::Boss,
            _ => panic!("Unexpected type of chest"),
        };

        new::FloorState::Chest(chest_type)
    }
}

fn convert_shop(shop: &old::ShopScreen) -> new::FloorState {
    let cards = shop
        .cards
        .iter()
        .map(|a| {
            (
                a.name.to_string(),
                a.price.expect("No price on card") as u16,
            )
        })
        .collect();
    let relics = shop
        .relics
        .iter()
        .map(|a| {
            (
                a.name.to_string(),
                a.price.expect("No price on relic") as u16,
            )
        })
        .collect();
    let potions = shop
        .potions
        .iter()
        .map(|a| {
            (
                a.name.to_string(),
                a.price.expect("No price on potion") as u16,
            )
        })
        .collect();
    let price = if shop.purge_available {
        shop.purge_cost as u16
    } else {
        0
    };

    new::FloorState::Shop {
        cards,
        relics,
        potions,
        purge_cost: price,
    }
}

fn convert_rewards(rewards: &old::CombatRewards, uuid_map: &mut HashMap<String, Uuid>) -> Vector<new::Reward> {
    rewards
        .rewards
        .iter()
        .map(|a| match a {
            old::RewardType::Card => new::Reward::CardChoice,
            old::RewardType::EmeraldKey => new::Reward::EmeraldKey,
            old::RewardType::Gold { gold } => new::Reward::Gold(*gold as u8),
            old::RewardType::Potion { potion } => new::Reward::Potion(convert_potion(potion)),
            old::RewardType::Relic { relic } => new::Reward::Relic(convert_relic(relic, uuid_map)),
            old::RewardType::StolenGold { gold } => new::Reward::Gold(*gold as u8),
            old::RewardType::SapphireKey { link } => new::Reward::SapphireKey(convert_relic(link, uuid_map)),
        })
        .collect()
}

fn convert_event(event: &old::Event) -> new::EventState {
    let base_event = crate::models::events::by_name(&event.event_name);

    new::EventState {
        base: base_event,
        variant: Option::None,
        variant_cards: vec![],
        variant_relic: Option::None,
        variant_amount: Option::None,
        vars: new::Vars {
            n: 0,
            x: 0,
            n_reset: 0,
        },
        available_choices: event
            .options
            .iter()
            .filter(|a| !a.disabled)
            .map(|option: &old::EventOption| {
                base_event
                    .choices
                    .iter()
                    .find(|a| a.name == option.label)
                    .unwrap_or_else(|| {
                        panic!("No option found that matches label: {}", option.label)
                    })
                    .name
                    .to_string()
            })
            .collect(),
    }
}

fn convert_orbs(orbs: &[old::OrbType]) -> Vector<new::Orb> {
    orbs.iter()
        .map(|orb| new::Orb {
            base: match orb.name.as_str() {
                "Lightning" => new_core::OrbType::Lightning,
                "Dark" => new_core::OrbType::Dark,
                "Frost" => new_core::OrbType::Frost,
                "Plasma" => new_core::OrbType::Plasma,
                _ => panic!("Unrecognized orb type"),
            },
            n: orb.evoke_amount as u16,
        })
        .collect()
}

fn convert_monsters(monsters: &[old::Monster], uuid_map: &mut HashMap<String, Uuid>) -> Vector<new::Monster> {

    monsters
        .iter()
        .enumerate()
        .map(|(index, monster)| {
            let buffs = convert_buffs(&monster.powers, uuid_map);
            new::Monster {
                base: crate::models::monsters::by_name(&monster.id),
                position: index,
                creature: new::Creature {
                    hp: monster.current_hp as u16,
                    max_hp: monster.max_hp as u16,
                    is_player: false,
                    buff_names: buffs.items.iter().map(|(uuid, buff)| (buff.base.name.to_string(), *uuid)).collect(),
                    buffs_when: HashMap::new(),
                    buffs: buffs,
                    block: monster.block as u16,
                },
                vars: new::Vars {
                    n: 0,
                    x: 0,
                    n_reset: 0,
                },
                targetable: !monster.is_gone,
                intent: convert_intent(&monster.intent),
            }
        })
        .collect()
}

fn convert_buffs(buffs: &[old::Power], uuid_map: &mut HashMap<String, Uuid>) -> UuidVector<new::Buff> {
    let mut vector = UuidVector::new();

    for buff in buffs {
        let uuid = uuid_map.entry(buff.id.to_string()).or_insert_with(Uuid::new_v4);

        vector.add(new::Buff {
            base: crate::models::buffs::by_name(&buff.name),
            uuid: *uuid,
            vars: new::Vars {
                n: buff.amount as i16,
                x: 0,
                n_reset: 0,
            },
            stacked_vars: vec![],
        })
    }

    vector
}

fn convert_potion(potion: &old::Potion) -> new::Potion {
    new::Potion {
        base: crate::models::potions::by_name(&potion.name),
    }
}

fn convert_potions(potions: &[old::Potion]) -> Vector<Option<new::Potion>> {
    potions
        .iter()
        .map(|potion| {
            if potion.name == "Potion Slot" {
                None
            } else {
                Some(convert_potion(potion))
            }
        })
        .collect()
}

fn convert_cards(cards: &[old::Card], uuid_map: &mut HashMap<String, Uuid>) -> UuidVector<new::Card> {
    let mut vector = UuidVector::new();
    for card in cards {
        let new_card = convert_card(card, uuid_map);
        vector.add(new_card);
    }
    vector
}

fn convert_card(card: &old::Card, uuid_map: &mut HashMap<String, Uuid>) -> new::Card {
    let uuid = uuid_map.entry(card.id.to_string()).or_insert_with(Uuid::new_v4);

    let name = if card.name.ends_with('+') {
        &card.name[0..card.name.len() - 1]
    } else {
        card.name.as_str()
    };
    new::Card {
        base: crate::models::cards::by_name(&String::from(name)),
        vars: new::Vars {
            n: 0,
            n_reset: 0,
            x: 0,
        },
        bottled: false,
        uuid: *uuid,
        upgrades: card.upgrades as u8,
        cost: card.cost as u8,
    }
}

fn convert_class(class: &old::PlayerClass) -> new_core::Class {
    match class {
        old::PlayerClass::Ironclad => new_core::Class::Ironclad,
        old::PlayerClass::Silent => new_core::Class::Silent,
        old::PlayerClass::Defect => new_core::Class::Defect,
        old::PlayerClass::Watcher => new_core::Class::Watcher,
        old::PlayerClass::Other => panic!("Unrecognized class"),
    }
}
