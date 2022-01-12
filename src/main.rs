use babygrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem reading arguments: {}", error);
        process::exit(-1);
    });

    if let Err(e) = babygrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(-1);
    }
}
