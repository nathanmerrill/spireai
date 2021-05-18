use crate::comm::request::GameState;
use crate::models::choices::Choice;
use comm::request::Request;
use comm::response::Response;
use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::io::Write;
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

fn main() {
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

    loop {
        let request = process_queue(&mut queue, &game_state);
        let choice = handle_request(&request, &mut ai);

        game_state = request.game_state;
        queue = comm::response::decompose_choice(choice);
    }
}

fn handle_request(request: &Request, ai: &mut spireai::SpireAi) -> Choice {
    match &request.game_state {
        Some(state) => ai.choose(&comm::interop::convert_state(&state)),
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
