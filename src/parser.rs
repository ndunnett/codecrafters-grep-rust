use std::str::Chars;

#[derive(Debug, Clone)]
pub enum Atom {
    Digit,
    Alphanumeric,
    Char(char),
}

#[derive(Debug, Clone)]
pub enum Anchor {
    Start,
    End,
}

#[derive(Debug, Clone)]
pub enum Set {
    Positive(Vec<Atom>),
    Negative(Vec<Atom>),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Atom(Atom),
    Anchor(Anchor),
    Set(Set),
    Group(Vec<Pattern>),
    OneOrMore(Box<Pattern>),
    ZeroOrMore(Box<Pattern>),
}

pub struct Parser<'a> {
    chars: Chars<'a>,
    next: Option<char>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let next = chars.next();
        Self { chars, next }
    }

    pub fn peek(&self) -> Option<char> {
        self.next
    }

    pub fn next(&mut self) -> Option<char> {
        if self.next.is_some() {
            let ch = self.next;
            self.next = self.chars.next();
            ch
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Pattern>, String> {
        let mut seq = Vec::new();

        while let Some(c) = self.next() {
            match c {
                '\\' => seq.push(self.parse_escape()?),
                '[' => seq.push(self.parse_set()?),
                '^' => seq.push(Pattern::Anchor(Anchor::Start)),
                '$' => seq.push(Pattern::Anchor(Anchor::End)),
                '+' => {
                    if let Some(last) = seq.pop() {
                        seq.push(Pattern::OneOrMore(Box::new(last)));
                    } else {
                        return Err("Failed to parse OneOrMore expression".into());
                    }
                }
                '*' => {
                    if let Some(last) = seq.pop() {
                        seq.push(Pattern::ZeroOrMore(Box::new(last)));
                    } else {
                        return Err("Failed to parse ZeroOrMore expression".into());
                    }
                }
                _ => seq.push(Pattern::Atom(Atom::Char(c))),
            }
        }

        Ok(seq)
    }

    fn parse_escape(&mut self) -> Result<Pattern, String> {
        if let Some(c) = self.next() {
            match c {
                'd' => Ok(Pattern::Atom(Atom::Digit)),
                'w' => Ok(Pattern::Atom(Atom::Alphanumeric)),
                '^' | '$' | '\\' | '+' | '*' => Ok(Pattern::Atom(Atom::Char(c))),
                _ => Err(format!("Unhandled escape pattern: \\{}", c)),
            }
        } else {
            Err("Unfinished escape pattern: \\".into())
        }
    }

    fn parse_set(&mut self) -> Result<Pattern, String> {
        let mut elements = Vec::new();

        let negative = if let Some('^') = self.peek() {
            let _ = self.next();
            true
        } else {
            false
        };

        while let Some(c) = self.next() {
            match c {
                ']' => {
                    return Ok(Pattern::Set(if negative {
                        Set::Negative(elements)
                    } else {
                        Set::Positive(elements)
                    }))
                }
                _ => elements.push(Atom::Char(c)),
            }
        }

        Err("Unclosed ]".into())
    }
}
