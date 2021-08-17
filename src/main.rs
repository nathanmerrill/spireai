#![allow(dead_code)]

use crate::comm::request::GameState;
use crate::models::choices::Choice;
use crate::models::core::Class;
use comm::request::Request;
use comm::response::Response;
use std::error::Error;
use std::io::stdin;

mod comm;
mod models;
mod spireai;
mod state;

#[macro_use]
extern crate lazy_static;

const DESIRED_CLASS: models::core::Class = models::core::Class::Watcher;

fn main() {
    run();
}

fn run() {
    let mut ai = spireai::SpireAi::new(crate::state::game::GameState::new(Class::Ironclad, 0));
    let mut game_state: Option<GameState> = None;
    let mut queue: Vec<Response> = vec![Response::Simple(String::from("ready"))];
    loop {
        let request = process_queue(&mut queue, &game_state);
        let choice = handle_request(&request, &mut ai);

        queue = comm::response::decompose_choice(choice, &request, &ai.uuid_map);
        game_state = request.game_state;
    }
}

fn handle_request(request: &Request, ai: &mut spireai::SpireAi) -> Choice {
    match &request.game_state {
        Some(state) => ai.choose(state),
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

    model
}

fn deserialize(state: &str) -> Result<comm::request::Request, Box<dyn Error>> {
    let deserialized = serde_json::from_str(state)?;
    Ok(deserialized)
}
