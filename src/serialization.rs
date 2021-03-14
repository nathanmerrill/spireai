use crate::models;
use im::Vector;
use serde::Deserialize;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerClass {
    Ironclad,
    Silent,
    Defect,
    Watcher,
    Other,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
    Curse,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoomPhase {
    Combat,
    Event,
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct EventOption {
    pub text: String,
    pub label: String,
    pub disabled: bool,
    pub choice_index: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Event {
    pub event_name: String,
    pub event_id: String,
    pub body_text: String,
    pub options: Vec<EventOption>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Chest {
    pub chest_type: ChestType,
    pub chest_open: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Rest {
    pub has_rested: bool,
    pub rest_options: Vec<RestOption>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CardReward {
    pub cards: Vec<Card>,
    pub bowl_available: bool,
    pub skip_available: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MapChoice {
    pub current_node: Option<MapNode>,
    pub next_nodes: Option<Vec<MapNode>>,
    pub boss_available: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ShopScreen {
    pub cards: Vec<Card>,
    pub relics: Vec<Relic>,
    pub potions: Vec<Potion>,
    pub purge_available: bool,
    pub purge_cost: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Grid {
    pub cards: Vec<Card>,
    pub selected_cards: Vec<Card>,
    pub num_cards: i32,
    #[serde(default)]
    pub any_number: bool,
    pub confirm_up: bool,
    pub for_upgrade: bool,
    pub for_transform: bool,
    pub for_purge: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct HandSelect {
    pub cards: Vec<Card>,
    pub selected: Vec<Card>,
    pub num_cards: i32,
    pub can_pick_zero: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GameOver {
    pub score: i32,
    pub victory: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(
    tag = "screen_type",
    content = "screen_state",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ScreenState {
    None,
    Event(Event),
    Chest(Chest),
    ShopRoom,
    Rest(Rest),
    CardReward(CardReward),
    CombatReward(Vec<RewardType>),
    Map(MapChoice),
    BossReward(Vec<Relic>),
    ShopScreen(ShopScreen),
    Grid(Grid),
    HandSelect(HandSelect),
    GameOver(GameOver),
    Complete,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChestType {
    Small,
    Medium,
    Large,
    Boss,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RewardType {
    Card,
    Gold { gold: i32 },
    Relic { relic: Relic },
    Potion { potion: Potion },
    StolenGold { gold: i32 },
    EmeraldKey,
    SapphireKey { link: Relic },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RestOption {
    Dig,
    Lift,
    Recall,
    Rest,
    Smith,
    Toke,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Response {
    pub error: Option<String>,
    pub ready_for_command: bool,
    #[serde(default)]
    pub in_game: bool,
    pub game_state: Option<GameState>,
    #[serde(default)]
    pub available_commands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GameState {
    pub current_action: Option<String>,
    pub current_hp: i32,
    pub max_hp: i32,
    pub floor: i32,
    pub act: i32,
    pub gold: i32,
    pub seed: i64,
    pub class: PlayerClass,
    pub ascension_level: i32,
    pub relics: Vec<Relic>,
    pub deck: Vec<Card>,
    pub map: Vec<MapNode>,
    pub potions: Vec<Potion>,
    pub act_boss: Option<String>,
    #[serde(default)]
    pub is_screen_up: bool,
    pub room_phase: RoomPhase,
    pub room_type: String,
    pub combat_state: Option<CombatState>,
    #[serde(flatten)]
    pub screen_state: ScreenState,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CombatState {
    pub player: Player,
    pub monsters: Vec<Monster>,
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub exhaust_pile: Vec<Card>,
    pub hand: Vec<Card>,
    pub limbo: Vec<Card>,
    pub card_in_play: Option<Card>,
    #[serde(default)]
    pub turn: i32,
    #[serde(default)]
    pub cards_discarded_this_turn: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Intent {
    Attack,
    AttackBuff,
    AttackDebuff,
    AttackDefend,
    Buff,
    Debuff,
    StrongDebuff,
    Debug,
    Defend,
    DefendDebuff,
    DefendBuff,
    Escape,
    Magic,
    None,
    Sleep,
    Stun,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Monster {
    pub max_hp: i32,
    pub current_hp: i32,
    pub block: i32,
    pub powers: Vec<Power>,
    pub name: String,
    pub id: String,
    pub intent: Intent,
    pub half_dead: bool,
    pub is_gone: bool,
    pub move_id: Option<i32>,
    pub last_move_id: Option<i32>,
    pub second_last_move_id: Option<i32>,
    #[serde(default)]
    pub move_base_damage: i32,
    #[serde(default)]
    pub move_adjusted_damage: i32,
    #[serde(default)]
    pub move_hits: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Player {
    pub max_hp: i32,
    pub current_hp: i32,
    pub block: i32,
    pub powers: Vec<Power>,
    pub energy: i32,
    pub orbs: Vec<OrbType>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OrbType {
    pub name: String,
    pub orb_id: String,
    pub evoke_amount: i32,
    pub passive_amount: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Power {
    pub id: String,
    pub name: String,
    pub amount: i32,
    pub damage: Option<i32>,
    pub misc: Option<i32>,
    #[serde(default)]
    pub just_applied: bool,
    pub card: Option<Card>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Relic {
    pub id: String,
    pub name: String,
    pub counter: i32,
    pub price: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub rarity: CardRarity,
    pub upgrades: i32,
    pub has_target: bool,
    pub cost: i32,
    pub uuid: String,
    pub misc: Option<i32>,
    pub price: Option<i32>,
    #[serde(default)]
    pub is_playable: bool,
    #[serde(default)]
    pub exhausts: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MapNode {
    pub x: i32,
    pub y: i32,
    #[serde(default)]
    pub symbol: char,
    #[serde(default)]
    pub children: Vec<MapNode>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Potion {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub can_use: bool,
    #[serde(default)]
    pub can_discard: bool,
    #[serde(default)]
    pub requires_target: bool,
    pub price: Option<i32>,
}

pub fn to_model(state: &GameState) -> models::state::GameState {
    models::state::GameState {
        class: convert_class(&state.class),
        player: models::state::Creature {
            hp: state.current_hp as u16,
            max_hp: state.max_hp as u16,
            is_player: true,
            position: 0,
            buffs: convert_buffs(&state.combat_state.as_ref().map(|a| &a.player.powers).unwrap_or(&Vec::new()))
        },
        floor: state.floor as u8,
        deck: convert_cards(&state.deck),
        screen: convert_state(&state),
        potions: convert_potions(&state.potions),
        act: state.act as u8,
        asc: state.ascension_level as u8,
        relics: convert_relics(&state.relics),
        room: convert_room(&state.room_type)
    }
}

pub fn convert_room(room_type: &String) -> models::core::RoomType {
    match room_type.as_str() {
        "Rest" => models::core::RoomType::Rest,
        "Shop" => models::core::RoomType::Shop,
        "Question" => models::core::RoomType::Question,
        "Battle" => models::core::RoomType::Battle,
        "HallwayFight" => models::core::RoomType::HallwayFight,
        "Event" => models::core::RoomType::Event,
        "Elite" => models::core::RoomType::Elite,
        "Boss" => models::core::RoomType::Boss,
        "Treasure" => models::core::RoomType::Treasure,
        _ => panic!("Unrecognized room type: {}", room_type)
    }
}

pub fn convert_relics(relics: &Vec<Relic>) -> HashMap<&'static str, models::state::Relic> {
    relics.iter().map(|relic| {
        models::state::Relic {
            base: models::relics::by_name(relic.name.as_str()),
            vars: models::state::Vars {
                n: relic.counter as u8,
                x: 0,
                n_reset: 0,
            },
        }
    }).map(|relic| {
        (relic.base.name, relic)
    }).collect()
}

pub fn convert_intent(intent: &Intent) -> models::core::Intent {
    match intent {
        Intent::Attack => models::core::Intent::Attack,
        Intent::AttackBuff => models::core::Intent::AttackBuff,
        Intent::AttackDebuff => models::core::Intent::AttackDebuff,
        Intent::AttackDefend => models::core::Intent::AttackDefend,
        Intent::Buff => models::core::Intent::Buff,
        Intent::Debuff => models::core::Intent::Debuff,
        Intent::StrongDebuff => models::core::Intent::StrongDebuff,
        Intent::Defend => models::core::Intent::Defend,
        Intent::DefendDebuff => models::core::Intent::DefendDebuff,
        Intent::DefendBuff => models::core::Intent::DefendBuff,
        Intent::Escape => models::core::Intent::Escape,
        Intent::None => models::core::Intent::None,
        Intent::Sleep => models::core::Intent::Sleep,
        Intent::Stun => models::core::Intent::Stun,
        Intent::Unknown => models::core::Intent::Unknown,
        Intent::Debug | Intent::Magic => panic!("Unrecognized intent: {:?}", intent),
    }
}

pub fn convert_state(state: &GameState) -> models::state::ScreenState {
    match &state.room_phase {
        RoomPhase::Combat => {
            let combat_state = (state.combat_state).as_ref().unwrap();
            models::state::ScreenState::Battle(models::state::BattleState {
                draw: convert_cards(&combat_state.draw_pile),
                discard: convert_cards(&combat_state.discard_pile),
                exhaust: convert_cards(&combat_state.exhaust_pile),
                hand: convert_cards(&combat_state.hand),
                monsters: convert_monsters(&combat_state.monsters),
                energy: combat_state.player.energy as u8,
                orbs: convert_orbs(&combat_state.player.orbs)
            })
        },
        _ => models::state::ScreenState::None
    }
}

fn convert_orbs(orbs: &Vec<OrbType>) -> Vec<models::state::Orb> {
    orbs.iter().map(|orb| {
        models::state::Orb {
            base: match orb.name.as_str() {
                "Lightning" => models::core::OrbType::Lightning,
                "Dark" => models::core::OrbType::Dark,
                "Frost" => models::core::OrbType::Frost,
                "Plasma" => models::core::OrbType::Plasma,
                _ => panic!("Unrecognized orb type")
            },
            n: orb.evoke_amount as u16
        }
    }).collect()
}

fn convert_monsters(monsters: &Vec<Monster>) -> Vec<models::state::Monster> {
    monsters.iter().enumerate().map(|(index, monster)| models::state::Monster {
        base: models::monsters::by_name(monster.name.as_str()),
        creature: models::state::Creature {
            hp: monster.current_hp as u16,
            max_hp: monster.max_hp as u16,
            is_player: false,
            position: index as u8,
            buffs: convert_buffs(&monster.powers)
        },
        vars: models::state::Vars {
            n: 0,
            x: 0,
            n_reset: 0
        },
        targetable: !monster.is_gone,
        intent: convert_intent(&monster.intent)
    }).collect()
}

fn convert_buffs(buffs: &Vec<Power>) -> HashMap<&'static str, models::state::Buff> {
    buffs.iter().map(|buff| {
        models::state::Buff {
            base: models::buffs::by_name(buff.name.as_str()),
            vars: models::state::Vars {
                n: buff.amount as u8,
                x: 0,
                n_reset: 0,
            },
        }
    }).map(|buff| {
        (buff.base.name, buff)
    }).collect()
}

fn convert_potions(potions: &Vec<Potion>) -> Vec<models::state::Potion> {
    let mut vec = Vec::new();
    vec.extend(potions.iter().map(|potion| models::state::Potion {
        base: models::potions::by_name(potion.name.as_str())
    }));
    vec
}

fn convert_cards(cards: &Vec<Card>) -> Vector<Rc<models::state::Card>> {
    cards.iter().map(|card| Rc::new(convert_card(card))).collect()
}

fn convert_card(card: &Card) -> models::state::Card {
    models::state::Card {
        base: models::cards::by_name(card.name.as_str()),
        vars: models::state::Vars {
            n: 0,
            n_reset: 0,
            x: 0,
        },
        upgrades: card.upgrades as u8,
        cost: card.cost as u8,
    }
}

fn convert_class(class: &PlayerClass) -> models::core::Class {
    match class {
        PlayerClass::Ironclad => models::core::Class::Ironclad,
        PlayerClass::Silent => models::core::Class::Silent,
        PlayerClass::Defect => models::core::Class::Defect,
        PlayerClass::Watcher => models::core::Class::Watcher,
        PlayerClass::Other => panic!("Unrecognized class"),
    }
}
