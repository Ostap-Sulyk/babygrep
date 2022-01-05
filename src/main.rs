#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]

use std::{env, error::Error, fs, process};

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let context = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", context);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("\n\tNot enouth arguments.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("\nProblem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    run(config);
}
