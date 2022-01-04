#![allow(dead_code, unused_imports, unused_variables)]

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("searching for {}", query);
    println!("in {}", file_name);
}
