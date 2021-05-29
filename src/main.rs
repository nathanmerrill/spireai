use crate::comm::request::GameState;
use crate::models::choices::Choice;
use comm::request::Request;
use comm::response::Response;
use im::HashMap;
use itertools::Itertools;
use models::acts::Act;
use models::buffs::BaseBuff;
use models::cards::BaseCard;
use models::events::BaseEvent;
use models::monsters::BaseMonster;
use models::potions::BasePotion;
use models::relics::BaseRelic;
use serde::Serialize;
use uuid::Uuid;
use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;

mod comm;
mod models;
mod spireai;

#[macro_use]
extern crate lazy_static;

const DESIRED_CLASS: models::core::Class = models::core::Class::Watcher;

lazy_static! {
    static ref LAST_ACTION: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref LAST_STATE: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}



pub fn write<T>(cards: T, filename: &'static str) where T: Serialize {
    let filepath = Path::new("data").join(filename);
    let file = File::create(filepath).unwrap();
    let mut config = ron::ser::PrettyConfig::new();
    config.indentor = "  ".to_string();
    config.separate_tuple_members = false;
    ron::ser::to_writer_pretty(file, &cards, config).unwrap();
}

fn main() {
    let acts: Vec<Act> = models::acts::all_acts();
    let cards: Vec<BaseCard> = models::cards::ALL_CARDS.values().sorted_by_key(|f|&f.name).map(|card| card.clone()).collect();
    let relics: Vec<BaseRelic> = models::relics::RELICS.values().sorted_by_key(|f|&f.name).map(|relic| relic.clone()).collect();
    let potions: Vec<BasePotion> = models::potions::POTIONS.values().sorted_by_key(|f|&f.name).map(|potion| potion.clone()).collect();
    let monsters: Vec<BaseMonster> = models::monsters::MONSTERS.values().sorted_by_key(|f|&f.name).map(|monster| monster.clone()).collect();
    let events: Vec<BaseEvent> = models::events::EVENTS.values().sorted_by_key(|f|&f.name).map(|event| event.clone()).collect();
    let buffs: Vec<BaseBuff> = models::buffs::BUFFS.values().sorted_by_key(|f|&f.name).map(|buff| buff.clone()).collect();
    
    write(acts, "acts.ron");
    write(cards, "cards.ron");
    write(relics, "relics.ron");
    write(potions, "potions.ron");
    write(monsters, "monsters.ron");
    write(events, "events.ron");
    write(buffs, "buffs.ron");
    

    let last_action_clone = Arc::clone(&LAST_ACTION);
    let last_state_clone = Arc::clone(&LAST_STATE);
    std::panic::set_hook(Box::new(move |_info| {
        let filename = format!(
            "log/output-{}.log",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        let state = json::parse((*last_state_clone.lock().unwrap()).as_str()).unwrap();
        let message = format!(
            "{}\nLast action: {}\nLast state: {}",
            _info,
            last_action_clone.lock().unwrap(),
            state.pretty(2)
        );

        if let Ok(mut f) = File::create(filename) {
            f.write_all(message.as_bytes()).ok();
        }
    }));

    std::panic::catch_unwind(|| {
        run("ready");
    })
    .ok();

    loop {
        std::panic::catch_unwind(|| {
            run("state");
        })
        .ok();
    }
}

fn run(start_message: &str) {
    let mut ai = spireai::SpireAi::new();
    let mut game_state: Option<GameState> = None;
    let mut queue: Vec<Response> = vec![Response::Simple(String::from(start_message))];
    let mut uuid_map: HashMap<String, Uuid> = HashMap::new();

    loop {
        let request = process_queue(&mut queue, &game_state);
        let choice = handle_request(&request, &mut ai, &mut uuid_map);

        game_state = request.game_state;
        queue = comm::response::decompose_choice(choice);
    }
}

fn handle_request(request: &Request, ai: &mut spireai::SpireAi, uuid_map: &mut HashMap<String, Uuid>) -> Choice {
    match &request.game_state {
        Some(state) => ai.choose(&comm::interop::convert_state(&state, uuid_map)),
        None => {
            if request.available_commands.contains(&String::from("start")) {
                Choice::Start {
                    player_class: DESIRED_CLASS,
                    ascension: None,
                }
            } else {
                Choice::State
            }
        }
    }
}

fn process_queue(queue: &mut Vec<Response>, game_state: &Option<GameState>) -> Request {
    send_message(&queue[0], game_state);
    let request = read_request();
    match &request.error {
        Some(err) => {
            panic!("Game error: {}", err)
        }
        None => {}
    }

    if queue.len() > 1 {
        queue.remove(0);
        return process_queue(queue, &request.game_state);
    }

    request
}

fn send_message(response: &Response, game: &Option<GameState>) {
    let serialized = comm::response::serialize_response(response, game);
    println!("{}", serialized);
    *LAST_ACTION.lock().unwrap() = serialized;
}

fn read_request() -> Request {
    let stdin = stdin();
    let input = &mut String::new();
    let request = match stdin.read_line(input) {
        Ok(_) => input.to_string(),
        Err(err) => panic!("Communication failed! Error: {}", err),
    };

    let model = match deserialize(&request) {
        Ok(model) => model,
        Err(err) => panic!("Failed to deserialize game state: Error: {}", err),
    };

    *LAST_STATE.lock().unwrap() = request;

    model
}

fn deserialize(state: &str) -> Result<comm::request::Request, Box<dyn Error>> {
    let deserialized = serde_json::from_str(state)?;
    Ok(deserialized)
}
