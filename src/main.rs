use std::env;
use std::io;
use std::process;

enum Token {
    Digit,
    Char(char),
}

fn parse_pattern(pattern: &str) -> Result<Vec<Token>, String> {
    let mut it = pattern.chars();
    let mut tokens = Vec::new();

    while let Some(c) = it.next() {
        if c == '\\' {
            if let Some(c) = it.next() {
                if c == 'd' {
                    tokens.push(Token::Digit);
                } else {
                    return Err(format!("Unhandled escape pattern: \\{}", c));
                }
            } else {
                return Err(format!("Unhandled escape pattern: \\{}", c));
            }
        } else {
            tokens.push(Token::Char(c));
        }
    }

    Ok(tokens)
}

fn match_pattern(input_line: &str, pattern: &str) -> Result<bool, String> {
    let pat = parse_pattern(pattern)?;
    let token = pat.first().unwrap();

    match token {
        Token::Digit => {
            Ok(input_line.contains(|c: char| c.is_ascii_digit()))
        }
        Token::Char(c) => {
            Ok(input_line.contains(*c))
        }
    }
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) == Ok(true) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
