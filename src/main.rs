#![allow(dead_code, unused_variables, unused_imports)]
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error|{
    println!("Problem reading arguments: {}", error);
    process::exit(-1);
    });
    println!("Searching for: {}", &config.query);
    println!("In file: {}", &config.filename);
}

struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
