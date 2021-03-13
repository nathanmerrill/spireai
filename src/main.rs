use log::{error, info, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::error::Error;
use std::io::stdin;

mod models;
mod serialization;
mod spireai;

#[macro_use]
extern crate lazy_static;

const DESIRED_CLASS: models::core::Class = models::core::Class::Ironclad;

fn main() {
    init_logger().unwrap();
    println!("ready");

    let mut ai = spireai::SpireAi::new();

    loop {
        let response: serialization::Response = read_state();
        let choice = match response.game_state {
            Some(state) => ai.choose(&serialization::to_model(&state)),
            None => {
                if response.available_commands.contains(&String::from("start")) {
                    spireai::Choice::Start {
                        player_class: DESIRED_CLASS,
                        ascension: None,
                    }
                } else {
                    spireai::Choice::State
                }
            }
        };

        send_choice(choice);
    }
}

fn read_state() -> serialization::Response {
    match read_response() {
        Ok(a) => {
            match &a.error {
                Some(error) => {
                    error!("Error recieved: {}", error);
                }
                None => {}
            }

            return a;
        }
        Err(a) => {
            error!("Failure! {}", a);
            panic!();
        }
    }
}

fn fmt_opt_i(i: Option<u8>) -> String {
    i.map(|a| a.to_string()).unwrap_or(String::default())
}

fn serialize_choice(choice: spireai::Choice) -> String {
    match choice {
        spireai::Choice::Start {
            player_class,
            ascension,
        } => {
            format!("START {} {}", player_class, fmt_opt_i(ascension))
        }
        spireai::Choice::Potion {
            should_use,
            slot,
            target_index,
        } => {
            let action = match should_use {
                true => "Use",
                false => "Discard",
            };

            format!("POTION {} {} {}", action, slot, fmt_opt_i(target_index))
        }
        spireai::Choice::Play {
            card_index,
            target_index,
        } => {
            format!("PLAY {} {}", card_index, fmt_opt_i(target_index))
        }
        spireai::Choice::End => {
            format!("END")
        }
        spireai::Choice::Choose(choice_index) => {
            format!("CHOOSE {}", choice_index)
        }
        spireai::Choice::Proceed => {
            format!("PROCEED")
        }
        spireai::Choice::Return => {
            format!("RETURN")
        }
        spireai::Choice::State => {
            format!("STATE")
        }
    }
}

fn send_choice(choice: spireai::Choice) {
    let response = serialize_choice(choice);
    info!("Sending: {}", response);
    println!("{}", response);
}

fn init_logger() -> Result<(), Box<dyn Error>> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    return Ok(());
}

fn read_response() -> Result<serialization::Response, Box<dyn Error>> {
    let stdin = stdin();
    let input = &mut String::new();
    stdin.read_line(input)?;
    info!("{}", input);
    let response: serialization::Response = serde_json::from_str(input)?;

    return Ok(response);
}
