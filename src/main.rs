#![allow(dead_code, unused_imports, unused_variables)]

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let context =
        fs::read_to_string(config.filename).expect("\n\nSomething went wrong reading file\n\n");
    println!("With text:\n{}", context)
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}

