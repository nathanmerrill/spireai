use crate::comm::request::GameState;
use std::sync::Mutex;
use std::sync::{Arc};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::error::Error;
use std::io::stdin;
use comm::request::Request;
use comm::response::Response;

mod models;
mod spireai;
mod comm;

#[macro_use]
extern crate lazy_static;

const DESIRED_CLASS: models::core::Class = models::core::Class::Watcher;

lazy_static! {
    static ref LAST_ACTION: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
    static ref LAST_STATE: Arc<Mutex<String>>  = Arc::new(Mutex::new(String::new()));
}

fn main() {
    init_logger().unwrap();

    let last_action_clone = Arc::clone(&LAST_ACTION);
    let last_state_clone = Arc::clone(&LAST_STATE);
    std::panic::set_hook(Box::new(move |_info| {
        let state = json::parse((*last_state_clone.lock().unwrap()).as_str()).unwrap();
        log::error!("{}\nLast action: {}\nLast state: {}", _info, last_action_clone.lock().unwrap(), state.pretty(2));

        std::process::exit(1);
    }));

    let mut ai = spireai::SpireAi::new();
    let mut game_state: Option<GameState> = None;
    let mut queue: Vec<Response> = vec![Response::Simple(String::from("ready"))];

    loop {
        let request = process_queue(&mut queue, &game_state);
        let choice = handle_request(&request, &mut ai);
        
        game_state = request.game_state;
        queue = comm::response::decompose_choice(choice);
    }
}

fn handle_request(request: &Request, ai: &mut spireai::SpireAi) -> spireai::Choice {
    match &request.game_state {
        Some(state) => ai.choose(&comm::interop::convert_state(&state)),
        None => {
            if request.available_commands.contains(&String::from("start")) {
                spireai::Choice::Start {
                    player_class: DESIRED_CLASS,
                    ascension: None,
                }
            } else {
                spireai::Choice::State
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
        },
        None => {}
    }

    if queue.len() > 1 {
        queue.remove(0);
        return process_queue(queue, &request.game_state);
    }

    return request;
}


fn send_message(response: &Response, game: &Option<GameState>) {
    let serialized = comm::response::serialize_response(response, game);
    println!("{}", serialized);
    *LAST_ACTION.lock().unwrap() = serialized;
}


fn init_logger() -> Result<(), Box<dyn Error>> {
    let filename = format!("log/output-{}.log", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(filename)?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    return Ok(());
}

fn read_request() -> Request {
    let stdin = stdin();
    let input = &mut String::new();
    let request = match stdin.read_line(input) {
        Ok(_) => input.to_string(),
        Err(err) => panic!("Communication failed! Error: {}", err)
    };

    let model = match deserialize(&request) {
        Ok(model) => model,
        Err(err) => panic!("Failed to deserialize game state: Error: {}", err)
    };

    *LAST_STATE.lock().unwrap() = request;

    model
}

fn deserialize(state: &String) -> Result<comm::request::Request, Box<dyn Error>> {
    let deserialized = serde_json::from_str(state)?;
    Ok(deserialized)
}
