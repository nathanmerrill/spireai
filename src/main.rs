use std::error::Error;
use std::io;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

fn main() {
    ready();

}


#[derive(Deserialize)]
struct Response {

}

fn ready() {
    println!("ready");
}

fn read_response() -> Result<Response, Error> {
    let stdin = io::stdin();
    let input = &mut String::new();
    stdin.read_line(input)?;

    return Ok(Response::new())
    // let response: Response = serde_json::from_str(input)?;
}

