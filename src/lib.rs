#![allow(dead_code, unused_variables)]
use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("\n\tNot enouth arguments.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let context = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &context) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &'a str, context: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    for line in context.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
