use crate::comm::request as old;
use crate::models::state as new;
use crate::models::core as new_core;
use std::collections::HashMap;

pub fn convert_state(state: &old::GameState) -> new::GameState {
    let relics = convert_relics(&state.relics);
    new::GameState {
        class: convert_class(&state.class),
        player: new::Creature {
            hp: state.current_hp as u16,
            max_hp: state.max_hp as u16,
            is_player: true,
            position: 0,
            buffs: convert_buffs(
                &state
                    .combat_state
                    .as_ref()
                    .map(|a| &a.player.powers)
                    .unwrap_or(&Vec::new()),
            ),
            block: state.combat_state.as_ref().map(|a| a.player.block).unwrap_or(0) as u16
        },
        battle_state: state.combat_state.as_ref().map(|a| convert_battle_state(a, state)),
        gold: state.gold as u16,
        floor_state: convert_floor_state(state),
        deck: convert_cards(&state.deck),
        map: convert_map(state),
        potions: convert_potions(&state.potions),
        act: state.act as u8,
        asc: state.ascension_level as u8,
        relic_names: relics.iter().map(|a| a.base.name).collect(), 
        relics: relics,
        keys: None,
    }
}

pub fn convert_map(state: &old::GameState) -> new::MapState {
    let mut nodes: HashMap<(i8, i8), new::MapNode> = HashMap::new();
    for node in &state.map {
        let new_node = convert_node(node);
        nodes.insert((new_node.floor, new_node.x), new_node);
    }

    let (x, y) = match &state.screen_state {
        old::ScreenState::Map(current) => {
            match &current.current_node {
                Some(node) => {
                    (node.x, node.y)
                }
                None => {
                    (-1, state.floor)
                }
            }
        }
        _ => {
            (-1, state.floor)
        }
    };

    new::MapState {
        nodes: nodes,
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
            _ => panic!("Unhandled node type: {}", node.symbol)
        },
        next: node.children.iter().map(|a| a.x as i8).collect(),
        x: node.x as i8,
    }
}

pub fn convert_relic(relic: &old::Relic) -> new::Relic {
    new::Relic {
        base: crate::models::relics::by_name(relic.name.as_str()),
        vars: new::Vars {
            n: relic.counter as u8,
            x: 0,
            n_reset: 0,
        },
    }
}

pub fn convert_relics(relics: &Vec<old::Relic>) -> Vec<new::Relic> {
    relics
        .iter()
        .map(convert_relic)
        .collect()
}

pub fn convert_intent(intent: &old::Intent) -> new_core::Intent {
    match intent {
        old::Intent::Attack => new_core::Intent::Attack,
        old::Intent::AttackBuff => new_core::Intent::AttackBuff,
        old::Intent::AttackDebuff => new_core::Intent::AttackDebuff,
        old::Intent::AttackDefend => new_core::Intent::AttackDefend,
        old::Intent::Buff => new_core::Intent::Buff,
        old::Intent::Debuff => new_core::Intent::Debuff,
        old::Intent::StrongDebuff => new_core::Intent::StrongDebuff,
        old::Intent::Defend => new_core::Intent::Defend,
        old::Intent::DefendDebuff => new_core::Intent::DefendDebuff,
        old::Intent::DefendBuff => new_core::Intent::DefendBuff,
        old::Intent::Escape => new_core::Intent::Escape,
        old::Intent::None => new_core::Intent::None,
        old::Intent::Sleep => new_core::Intent::Sleep,
        old::Intent::Stun => new_core::Intent::Stun,
        old::Intent::Unknown => new_core::Intent::Unknown,
        old::Intent::Debug | old::Intent::Magic => panic!("Unrecognized intent: {:?}", intent),
    }
}

pub fn convert_battle_state(state: &old::CombatState, game_state: &old::GameState) -> new::BattleState {
    new::BattleState {
        draw: convert_cards(&state.draw_pile),
        discard: convert_cards(&state.discard_pile),
        exhaust: convert_cards(&state.exhaust_pile),
        hand: convert_cards(&state.hand),
        monsters: convert_monsters(&state.monsters),
        energy: state.player.energy as u8,
        orbs: convert_orbs(&state.player.orbs),
        stance: new_core::Stance::None,
        battle_type: new::BattleType::Common,
        card_choices: convert_card_choices(state, game_state),
        card_choice_type: convert_card_choice_type(state, game_state),
    }
}

pub fn convert_card_choices(state: &old::CombatState, game_state: &old::GameState) -> Vec<new::Card> {
    match &game_state.screen_state {
        old::ScreenState::Grid(grid) => {
            convert_cards(&grid.cards)
        },
        _ => Vec::new()
    }
}

pub fn convert_card_choice_type(state: &old::CombatState, game_state: &old::GameState) -> new::CardChoiceType {
    match &game_state.current_action {
        Some(s) => match s.as_str() {
            "ScryAction" => new::CardChoiceType::Scry,
            "DamageAction" => new::CardChoiceType::None,
            _ => panic!("Unexpected action type: {}", s)
        }
        None => new::CardChoiceType::None
    }
}

pub fn convert_floor_state(state: &old::GameState) -> new::FloorState {
    match &state.screen_state {
        old::ScreenState::None{} => {
            match &state.room_phase {
                old::RoomPhase::Combat => {
                    new::FloorState::Battle
                },
                _ => panic!("Expected Battle in None state")
            }
        },
        old::ScreenState::Event(event) => new::FloorState::Event(convert_event(event)),
        old::ScreenState::Map(_) => new::FloorState::Map,
        old::ScreenState::CombatReward(rewards) => new::FloorState::Rewards(convert_rewards(rewards)),
        old::ScreenState::CardReward(reward) => new::FloorState::CardReward(reward.cards.iter().map(|a| convert_card(a)).collect()),
        old::ScreenState::ShopRoom{} => new::FloorState::ShopEntrance,
        old::ScreenState::ShopScreen(screen) => convert_shop(screen),
        old::ScreenState::Rest(_) => new::FloorState::Rest,
        old::ScreenState::Grid(grid) => {
            match &state.room_phase {
                old::RoomPhase::Combat => {
                    new::FloorState::Battle
                },
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
                _ => panic!("Unexpected room phase in grid choice")
            }
        }
        old::ScreenState::Chest(chest) => convert_chest(chest),
        // ScreenState::CardReward(CardReward),
        // ScreenState::BossReward(Vec<Relic>),
        // ScreenState::Grid(Grid) => new::FloorState::CardSelect,
        // ScreenState::HandSelect(HandSelect),
        old::ScreenState::GameOver(_) => {
            new::FloorState::GameOver
        },
        // ScreenState::Complete,
        _ => panic!("Unhandled screen state")
    }
}

fn convert_chest(chest: &old::Chest) -> new::FloorState {
    if chest.chest_open {
        panic!("Not sure how to handle open chest")
    } else {
        let chest_type = match chest.chest_type {
            old::ChestType::SmallChest => new::ChestType::Small,
            old::ChestType::MediumChest => new::ChestType::Medium,
            old::ChestType::LargeChest => new::ChestType::Large,
            old::ChestType::BossChest => new::ChestType::Boss,
            _ => panic!("Unexpected type of chest")
        };

        new::FloorState::Chest(chest_type)
    }
}

fn convert_shop(shop: &old::ShopScreen) -> new::FloorState {
    let cards = shop.cards.iter().map(|a|
        (convert_card(a), a.price.expect("No price on card") as u16)
    ).collect();
    let relics = shop.relics.iter().map(|a|
        (convert_relic(a), a.price.expect("No price on relic") as u16)
    ).collect();
    let potions = shop.potions.iter().map(|a|
        (convert_potion(a), a.price.expect("No price on potion") as u16)
    ).collect();
    let price = if shop.purge_available { shop.purge_cost as u16 } else { 0 };

    new::FloorState::Shop(cards, relics, potions, price)
}

fn convert_rewards(rewards: &old::CombatRewards) -> Vec<new::Reward> {
    rewards.rewards.iter().map(|a| match a {
        old::RewardType::Card => new::Reward::CardChoice,
        old::RewardType::EmeraldKey => new::Reward::EmeraldKey,
        old::RewardType::Gold{gold} => new::Reward::Gold(*gold as u8),
        old::RewardType::Potion{potion} => new::Reward::Potion(convert_potion(potion)),
        old::RewardType::Relic{relic} => new::Reward::Relic(convert_relic(relic)),
        old::RewardType::StolenGold{ gold} => new::Reward::Gold(*gold as u8),
        old::RewardType::SapphireKey{link} => new::Reward::SapphireKey(convert_relic(link)),
    }).collect()
}

fn convert_event(event: &old::Event) -> new::EventState {
    let base_event = crate::models::events::by_name(event.event_name.as_str());

    new::EventState{
        base: base_event,
        variant: Option::None,
        variant_cards: vec![],
        variant_relic: Option::None,
        variant_amount: Option::None,
        available_choices: event.options.iter()
        .filter(|a| !a.disabled)
        .map(|option: &old::EventOption| {
            base_event.choices.iter()
            .find(|a| a.name == option.label).expect(format!("No option found that matches label: {}", option.label).as_str())
            .name
        }).collect(),
    }
}

fn convert_orbs(orbs: &Vec<old::OrbType>) -> Vec<new::Orb> {
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

fn convert_monsters(monsters: &Vec<old::Monster>) -> Vec<new::Monster> {
    monsters
        .iter()
        .enumerate()
        .map(|(index, monster)| new::Monster {
            base: crate::models::monsters::by_name(monster.id.as_str()),
            creature: new::Creature {
                hp: monster.current_hp as u16,
                max_hp: monster.max_hp as u16,
                is_player: false,
                position: index as u8,
                buffs: convert_buffs(&monster.powers),
                block: monster.block as u16,
            },
            vars: new::Vars {
                n: 0,
                x: 0,
                n_reset: 0,
            },
            targetable: !monster.is_gone,
            intent: convert_intent(&monster.intent),
        })
        .collect()
}

fn convert_buffs(buffs: &Vec<old::Power>) -> HashMap<&'static str, new::Buff> {
    buffs
        .iter()
        .map(|buff| new::Buff {
            base: crate::models::buffs::by_name(buff.name.as_str()),
            vars: new::Vars {
                n: buff.amount as u8,
                x: 0,
                n_reset: 0,
            },
        })
        .map(|buff| (buff.base.name, buff))
        .collect()
}

fn convert_potion(potion: &old::Potion) -> new::Potion {
    new::Potion {
        base: crate::models::potions::by_name(potion.name.as_str()),
    }
}

fn convert_potions(potions: &Vec<old::Potion>) -> Vec<Option<new::Potion>> {
    potions.iter().map(|potion| 
        if potion.name == "Potion Slot" {
            None
        } else {
            Some(convert_potion(potion))
        }
    ).collect()
}

fn convert_cards(cards: &Vec<old::Card>) -> Vec<new::Card> {
    cards
        .iter()
        .map(|card| convert_card(card))
        .collect()
}

fn convert_card(card: &old::Card) -> new::Card {
    let name = if card.name.ends_with('+') {&card.name[0..card.name.len()-1]} else {card.name.as_str()};
    new::Card {
        base: crate::models::cards::by_name(name),
        vars: new::Vars {
            n: 0,
            n_reset: 0,
            x: 0,
        },
        id: card.id.to_string(),
        bottled: false,
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