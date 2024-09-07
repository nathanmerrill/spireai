#![allow(dead_code)]

use crate::comm::request::GameState;
use crate::models::choices::Choice;
use comm::request::Request;
use comm::response::Response;
use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

mod comm;
mod models;
mod spireai;
mod state;

#[macro_use]
extern crate lazy_static;

fn main() {
    run(stdin().lock(), stdout());
}

fn run<R, W>(mut reader: R, mut writer: W)
where
    R: BufRead,
    W: Write,
{
    let mut ai = spireai::SpireAi::new(crate::state::floor::FloorState::Menu);
    let mut game_state: Option<GameState> = None;
    let mut queue: Vec<Response> = initial_queue();
    loop {
        let request = process_queue(&mut queue, &game_state, &mut reader, &mut writer);
        let choice = handle_request(&request, &mut ai);

        queue = comm::response::decompose_choice(choice, &request, &ai.uuid_map);
        game_state = request.game_state;
    }
}

fn initial_queue() -> Vec<Response> {
    vec![Response::Simple(String::from("ready"))]
}

fn handle_request(request: &Request, ai: &mut spireai::SpireAi) -> Choice {
    ai.choose(&request.game_state)
}

fn process_queue<R, W>(
    queue: &mut Vec<Response>,
    game_state: &Option<GameState>,
    reader: &mut R,
    writer: &mut W,
) -> Request
where
    R: BufRead,
    W: Write,
{
    loop {
        send_message(&queue[0], game_state, writer);
        let request = read_request(reader);
        match &request.error {
            Some(err) => {
                panic!("Game error: {}", err)
            }
            None => {}
        }

        if queue.len() > 1 {
            queue.remove(0);
        } else {
            return request;
        }
    }
}

fn send_message<W>(response: &Response, game: &Option<GameState>, writer: &mut W)
where
    W: Write,
{
    let serialized = comm::response::serialize_response(response, game);
    writeln!(writer, "{}", serialized).expect("Failed to write!");
}

fn read_request<R>(reader: &mut R) -> Request
where
    R: BufRead,
{
    let input = &mut String::new();
    let request = match reader.read_line(input) {
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

#[cfg(test)]
mod test {
    #[test]
    fn test_io() {
        let mut input = b"{\"available_commands\":[\"start\",\"state\"],\"ready_for_command\":true,\"in_game\":false}" as &[u8];
        let mut output = Vec::new();
        let mut queue = crate::initial_queue();

        let response = crate::process_queue(&mut queue, &None, &mut input, &mut output);

        assert!(response.error.is_none());
        assert!(response.game_state.is_none());
        assert_eq!(std::str::from_utf8(&output).unwrap(), "ready\n")
    }
}
