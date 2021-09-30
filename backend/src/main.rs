mod reader;
mod server;

use crate::reader::parse;
use crate::server::serve;
use std::env;
use std::error::Error;
use std::process;

enum Action {
    Parse,
    Serve,
}

pub struct Config {
    action: Action,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, Box<dyn Error>> {
        let name = args.next().ok_or("Cannot get program name.")?;

        let action = match args.next() {
            Some(val) => val,
            None => return Result::Err(format!("Usage: {} [parse|serve]", name).into()),
        };

        let action = match action.as_str() {
            "parse" => Action::Parse,
            "serve" => Action::Serve,
            _ => return Result::Err("Unknown subcommand.".into()),
        };

        Ok(Config { action })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.action {
        Action::Parse => parse(),
        Action::Serve => serve(),
    }
}

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing config: {}", err.to_string());
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("{}", err.to_string());
        process::exit(1);
    }
}
