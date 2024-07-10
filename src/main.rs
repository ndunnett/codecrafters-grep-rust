use std::env;
use std::io;
use std::process;

mod iterators;
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

    match matcher::Matcher::new(&pattern, &input) {
        Ok(mut matcher) => {
            matcher.matches();

            if matcher.matches.is_empty() {
                process::exit(1)
            } else {
                for range in matcher.matches.iter().rev() {
                    input.insert_str(range.end, "\x1B[0m");
                    input.insert_str(range.start, "\x1B[1;31m");
                }

                println!("{}", input);
                process::exit(0)
            }
        }
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
