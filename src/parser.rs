use std::fmt;
use std::str::Chars;

#[derive(Clone)]
pub enum Atom {
    Digit,
    Alphanumeric,
    Wildcard,
    Char(char),
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Digit => write!(f, "\\d"),
            Self::Alphanumeric => write!(f, "\\w"),
            Self::Wildcard => write!(f, "."),
            Self::Char(c) => write!(f, "{c:?}"),
        }
    }
}

#[derive(Clone)]
pub enum Anchor {
    Start,
    End,
}

impl fmt::Debug for Anchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "^"),
            Self::End => write!(f, "$"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Set {
    Positive(Vec<Atom>),
    Negative(Vec<Atom>),
}

#[derive(Clone)]
pub enum Pattern {
    Atom(Atom),
    Anchor(Anchor),
    Set(Set),
    Group(Vec<Pattern>),
    OneOrMore(Box<Pattern>),
    ZeroOrMore(Box<Pattern>),
    ZeroOrOne(Box<Pattern>),
    Alternation(Box<Pattern>, Box<Pattern>),
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom(s) => write!(f, "{s:?}"),
            Self::Anchor(s) => write!(f, "{s:?}"),
            Self::Set(s) => write!(f, "set {s:#?}"),
            Self::Group(s) => write!(f, "{s:#?}"),
            Self::OneOrMore(s) => write!(f, "+ {s:#?}"),
            Self::ZeroOrMore(s) => write!(f, "* {s:#?}"),
            Self::ZeroOrOne(s) => write!(f, "? {s:#?}"),
            Self::Alternation(l, r) => write!(f, "Alternation {l:#?}, {r:#?}"),
        }
    }
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

    pub fn parse(&mut self, depth: usize) -> Result<Vec<Pattern>, String> {
        let mut group = Vec::new();

        while let Some(c) = self.next() {
            match c {
                '\\' => group.push(self.parse_escape()?),
                '[' => group.push(self.parse_set()?),
                ']' => return Err("Unopened [".into()),
                '^' => group.push(Pattern::Anchor(Anchor::Start)),
                '$' => group.push(Pattern::Anchor(Anchor::End)),
                '+' => {
                    if let Some(last) = group.pop() {
                        group.push(Pattern::OneOrMore(Box::new(last)));
                    } else {
                        return Err("Failed to parse OneOrMore expression".into());
                    }
                }
                '*' => {
                    if let Some(last) = group.pop() {
                        group.push(Pattern::ZeroOrMore(Box::new(last)));
                    } else {
                        return Err("Failed to parse ZeroOrMore expression".into());
                    }
                }
                '?' => {
                    if let Some(last) = group.pop() {
                        group.push(Pattern::ZeroOrOne(Box::new(last)));
                    } else {
                        return Err("Failed to parse ZeroOrOne expression".into());
                    }
                }
                '.' => group.push(Pattern::Atom(Atom::Wildcard)),
                '(' => {
                    group.push(Pattern::Group(self.parse(depth + 1)?));
                }
                ')' => {
                    if depth > 0 {
                        return Ok(group);
                    } else {
                        return Err("Unopened )".into());
                    }
                }
                '|' => {
                    let alt = Pattern::Alternation(
                        Box::new(Pattern::Group(group)),
                        Box::new(Pattern::Group(self.parse(depth)?)),
                    );

                    if depth > 0 {
                        return Ok(vec![alt]);
                    } else {
                        group = vec![alt];
                    }
                }
                _ => group.push(Pattern::Atom(Atom::Char(c))),
            }
        }

        if depth == 0 {
            Ok(group)
        } else {
            Err("Unclosed (".into())
        }
    }

    fn parse_escape(&mut self) -> Result<Pattern, String> {
        if let Some(c) = self.next() {
            match c {
                'd' => Ok(Pattern::Atom(Atom::Digit)),
                'w' => Ok(Pattern::Atom(Atom::Alphanumeric)),
                '^' | '$' | '\\' | '+' | '*' | '?' | '.' | '(' | ')' | '[' | ']' | '|' => {
                    Ok(Pattern::Atom(Atom::Char(c)))
                }
                _ => Err(format!("Unhandled escape pattern: \\{}", c)),
            }
        } else {
            Err("Unfinished escape pattern".into())
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

        Err("Unclosed [".into())
    }
}
