#![allow(dead_code)]

use std::env;
use std::io;
use std::process;

mod matcher;
mod parser;

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("input: {input:?}");
    println!("pattern: {pattern:?}");

    let result = matcher::matches(&pattern, &input);

    println!("result: {result:?}");

    match result {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(1),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
