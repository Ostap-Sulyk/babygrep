#![allow(dead_code, unused_imports, unused_variables)]

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("in {}", file_name);

    let context = fs::read_to_string(file_name).expect("Something went wrong reading file");
    println!("With text:\n{}", context)
}
