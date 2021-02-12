use std::error::Error;
use std::io;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

fn main() {
    ready();

}

#[derive(Deserialize)]
enum PlayerClass {
    Ironclad,
    Silent,
    Defect,
    Watcher,
    Other
}

#[derive(Deserialize)]
enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
}

#[derive(Deserialize)]
enum CardRarity {
    Basic,
    Common,
    Uncommon,
    Rare,
    Special,
    Curse,
}

#[derive(Deserialize)]
enum RoomPhase {
    Combat,
    Event,
    Complete,
    Incomplete,
}

#[derive(Deserialize)]
struct EventOption {
    text: String,
    label: String,
    disabled: bool,
    choice_index: Option<i32>,
}

#[derive(Deserialize)]
#[serde(tag = "screen_type", content = "screen_state")]
enum ScreenState {
    None,
    Event {
        event_name: String,
        event_id: String,
        body_text: String,
        options: List<EventOption>,
    },
    Chest {
        chest_type: ChestType,
        chest_open: bool,
    },
    Shop_Room,
    Rest {
        has_rested: bool,
        rest_options: List<RestOption>,
    },
    Card_Reward {
        cards: List<Card>,
        bowl_available: bool,
        skip_available: bool,
    },
    Combat_Reward {
        rewards: List<RewardType>
    },
    Map {
        current_node: Option<MapNode>,
        next_nodes: Option<List<MapNode>>,
        boss_available: bool,
    },
    Boss_Reward {
        relics: List<Relic>
    },
    Shop_Screen {
        cards: List<Card>,
        relics: List<Relic>,
        potions: List<Potion>,
        purge_available: bool,
        purge_cost: bool,
    },
    Grid {
        cards: List<Card>,
        selected_cards: List<Card>,
        num_cards: i32,
        #[serde(default = False)]
        any_number: bool,
        confirm_up: bool,
        for_upgrade: bool,
        for_transform: bool,
        for_purge: bool,
    },
    Hand_Select {
        cards: List<Card>,
        selected: List<Card>,
        num_cards: i32,
        can_pick_zero: bool,
    },
    Game_Over {
        score: i32,
        victory: bool,
    },
    Complete,
}

#[derive(Deserialize)]
enum ChestType {
    Small,
    Medium,
    Large,
    Boss,
    Unknown,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum RewardType {
    Card,
    Gold {
        gold: i32,
    },
    Relic {
        relic: Relic,
    },
    Potion {
        potion: Potion,
    },
    Stolen_Gold {
        gold: i32,
    },
    Emerald_Key,
    Sapphire_Key {
        link: Relic,
    }
}

#[derive(Deserialize)]
enum RestOption {
    Dig,
    Lift,
    Recall,
    Rest,
    Smith,
    Toke,
}

#[derive(Deserialize)]
struct Response {
    error: Option<String>,
    ready_for_command: bool,
    in_game: bool,
    game_state: GameState,
    #[serde(alias = "game_state")]
    screen_state: ScreenState,
    available_commands: Vector<String>,
}

#[derive(Deserialize)]
struct GameState {
    current_action: String,
    current_hp: i32,
    max_hp: i32,
    floor: i32,
    act: i32,
    gold: i32,
    seed: String,
    character: PlayerClass,
    ascension_level: i32,
    relics: List<Relic>,
    deck: List<Card>,
    map: List<MapNode>,
    potions: List<Potion>,
    act_boss: Option<String>,
    #[serde(default = False)]
    is_screen_up: bool,
    room_phase: RoomPhase,
    room_type: String,
    combat_state: CombatState,
}

struct CombatState {
    player: Player,
    monsters: List<Monster>,
    draw_pile: List<Card>,
    discard_pile: List<Card>,
    exhaust_pile: List<Card>,
    hand: List<Card>,
    limbo: List<Card>,
    card_in_play: Option<Card>,
    #[serde(default = 0)]
    turn: i32,
    #[serde(default = 0)]
    cards_discarded_this_turn: i32,
}

enum Intent {
    Attack,
    Attack_Buff,
    Attack_Debuff,
    Attack_Defend,
    Buff,
    Debuff,
    Strong_Debuff,
    Debug,
    Defend,
    Defend_Debuff,
    Defend_Buff,
    Escape,
    Magic,
    None,
    Sleep,
    Stun,
    Unknown,
}

struct Monster {
    max_hp: i32,
    current_hp: i32,
    block: i32,
    powers: List<Power>,
    name: String,
    id: String,
    intent: Intent,
    half_dead: bool,
    is_gone: bool,
    move_id: Option<i32>,
    last_move_id: Option<i32>,
    second_last_move_id: Option<i32>,
    #[serde(default = 0)]
    move_base_damage: i32,
    #[serde(default = 0)]
    move_adjusted_damage: i32,
    #[serde(default = 0)]
    move_hits: i32,
}

struct Player {
    max_hp: i32,
    current_hp: i32,
    block: i32,
    powers: List<Power>
    energy: i32,
    orbs: List<Orb>
}

struct Orb {
    name: String,
    orb_id: String,
    evoke_amount: i32,
    passive_amount: i32,
}

struct Power {
    id: String,
    name: String,
    amount: i32,
    damage: Option<i32>,
    misc: Option<i32>,
    #[serde(default = False)]
    just_applied: bool,
    card: Option<Card>,
}

#[derive(Deserialize)]
struct Relic {
    id: String,
    name: String,
    counter: i32,
    price: Option<i32>,
}

#[derive(Deserialize)]
struct Card {
    id: String,
    name: String,
    type: CardType,
    rarity: CardRarity,
    upgrades: i32,
    has_target: bool,
    cost: i32,
    uuid: String,
    misc: Option<i32>,
    price: Option<i32>,
    #[serde(default = False)]
    is_playable: bool,
    #[serde(default = False)]
    exhausts: bool,
}

#[derive(Deserialize)]
struct MapNode {
    x: i32,
    y: i32,
    symbol: String,
    children: List<MapNodeChild>,
}

#[derive(Deserialize)]
struct MapNodeChild {
    x: i32,
    y: i32,
}

struct Potion {
    id: String,
    name: String,
    #[serde(default = False)]
    can_use: bool,
    #[serde(default = False)]
    can_discard: bool,
    #[serde(default = False)]
    requires_target: bool,
    price: Option<i32>,
}


fn ready() {
    println!("ready");
}

fn read_response() -> Result<Response, Error> {
    let stdin = io::stdin();
    let input = &mut String::new();
    stdin.read_line(input)?;
    let response: Response = serde_json::from_str(input)?;
    
    return Ok(response)
}

