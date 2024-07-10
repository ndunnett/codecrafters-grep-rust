use std::str::Chars;

#[derive(Debug, Clone)]
pub enum Atom {
    Digit,
    Alphanumeric,
    Char(char),
}

impl Atom {
    pub fn matches(&self, c: char) -> bool {
        match self {
            Self::Digit => c.is_ascii_digit(),
            Self::Alphanumeric => c.is_alphanumeric(),
            Self::Char(ch) => c == *ch,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Set {
    Positive(Vec<Atom>),
    Negative(Vec<Atom>),
}

impl Set {
    pub fn matches(&self, c: char) -> bool {
        match self {
            Self::Positive(set) => set.iter().any(|atom| atom.matches(c)),
            Self::Negative(set) => !set.iter().any(|atom| atom.matches(c)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Atom(Atom),
    Set(Set),
    Sequence(Vec<Pattern>),
}

impl Pattern {
    pub fn flatten(&self) -> Vec<FlatPattern> {
        let mut flat = Vec::new();

        match self {
            Self::Atom(atom) => flat.push(FlatPattern::Atom(atom.clone())),
            Self::Set(set) => flat.push(FlatPattern::Set(set.clone())),
            Self::Sequence(seq) => seq.iter().for_each(|p| flat.extend(p.flatten())),
        }

        flat
    }
}

#[derive(Debug, Clone)]
pub enum FlatPattern {
    Atom(Atom),
    Set(Set),
}

impl FlatPattern {
    pub fn matches(&self, c: char) -> bool {
        match self {
            Self::Atom(atom) => atom.matches(c),
            Self::Set(set) => set.matches(c),
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

    pub fn parse(&mut self) -> Result<Pattern, String> {
        let mut sequence = Vec::new();

        while let Some(c) = self.next() {
            match c {
                '\\' => sequence.push(self.parse_escape()?),
                '[' => sequence.push(self.parse_set()?),
                _ => sequence.push(Pattern::Atom(Atom::Char(c))),
            }
        }

        Ok(Pattern::Sequence(sequence))
    }

    fn parse_escape(&mut self) -> Result<Pattern, String> {
        if let Some(c) = self.next() {
            match c {
                'd' => Ok(Pattern::Atom(Atom::Digit)),
                'w' => Ok(Pattern::Atom(Atom::Alphanumeric)),
                '\\' => Ok(Pattern::Atom(Atom::Char('\\'))),
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
