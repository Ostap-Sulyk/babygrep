use std::{env, process};
use babygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Problem reading arguments: {}", error);
        process::exit(-1);
    });

    if let Err(e) = babygrep::run(config){
        println!("Application error: {}", e);
        process::exit(-1);
    }
}
