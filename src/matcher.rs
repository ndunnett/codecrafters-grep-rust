use crate::parser::*;

pub struct PatternIter {
    patterns: Vec<FlatPattern>,
    next_pattern: Option<FlatPattern>,
    pub index: usize,
}

impl PatternIter {
    pub fn new(pattern: &str) -> Result<Self, String> {
        let patterns = Parser::new(pattern).parse()?.flatten();

        let next_pattern = if !patterns.is_empty() {
            Some(patterns[0].clone())
        } else {
            None
        };

        Ok(Self {
            patterns,
            next_pattern,
            index: 0,
        })
    }

    pub fn consume(&mut self) {
        if self.next_pattern.is_some() {
            self.next_pattern = if self.index + 1 < self.patterns.len() {
                Some(self.patterns[self.index + 1].clone())
            } else {
                None
            };

            self.index += 1;
        }
    }

    pub fn peek(&self) -> Option<FlatPattern> {
        self.next_pattern.clone()
    }

    pub fn next(&mut self) -> Option<FlatPattern> {
        let pat = self.next_pattern.clone();
        self.consume();
        pat
    }

    pub fn reset(&mut self) {
        self.next_pattern = if !self.patterns.is_empty() {
            Some(self.patterns[0].clone())
        } else {
            None
        };

        self.index = 0;
    }

    pub fn len(&self) -> usize {
        self.patterns.len()
    }
}

pub struct CharIter {
    chars: Vec<char>,
    next_char: Option<char>,
    pub index: usize,
}

impl CharIter {
    pub fn new(input: &str) -> Result<Self, String> {
        let chars = input.chars().collect::<Vec<_>>();

        let next_char = if !chars.is_empty() {
            Some(chars[0])
        } else {
            None
        };

        Ok(Self {
            chars,
            next_char,
            index: 0,
        })
    }

    pub fn consume(&mut self) {
        if self.next_char.is_some() {
            self.next_char = if self.index + 1 < self.chars.len() {
                Some(self.chars[self.index + 1])
            } else {
                None
            };

            self.index += 1;
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.next_char
    }

    pub fn next(&mut self) -> Option<char> {
        let c = self.next_char;
        self.consume();
        c
    }

    pub fn wind_back(&mut self, offset: usize) {
        self.index = self.index.saturating_sub(offset);

        self.next_char = if !self.chars.is_empty() {
            Some(self.chars[self.index])
        } else {
            None
        };
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

pub fn matches(pattern: &str, input: &str) -> Result<bool, String> {
    let mut patterns = PatternIter::new(pattern)?;
    let mut chars = CharIter::new(input)?;
    let mut in_match = false;

    while let Some(c) = chars.peek() {
        match patterns.peek() {
            Some(FlatPattern::Atom(Atom::AnchorStart)) => {
                if chars.index == 0 {
                    in_match = true;
                    patterns.consume();
                } else {
                    return Ok(false);
                }
            }
            Some(FlatPattern::Atom(Atom::AnchorEnd)) => {
                in_match = false;
                chars.wind_back(patterns.index);
                chars.consume();
                patterns.reset();
            }
            Some(pattern) => {
                let matched = pattern.matches(c);

                match (matched, in_match) {
                    (true, false) => in_match = true,
                    (false, true) => {
                        in_match = false;
                        chars.wind_back(patterns.index);
                        patterns.reset();
                    }
                    _ => {}
                }

                if in_match {
                    patterns.consume();
                }

                chars.consume();
            }
            None => return Ok(true),
        }
    }

    Ok(matches!(
        patterns.peek(),
        Some(FlatPattern::Atom(Atom::AnchorEnd)) | None
    ))
}
