use std::fmt;

use crate::parser::{Parser, Pattern};

pub struct PatternIter {
    patterns: Vec<Pattern>,
    next_pattern: Option<Pattern>,
    pub index: usize,
}

impl fmt::Debug for PatternIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.patterns)
    }
}

impl PatternIter {
    pub fn new(pattern: &str) -> Result<Self, String> {
        let patterns = Parser::new(pattern).parse(0)?;

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

    pub fn peek(&self) -> Option<Pattern> {
        self.next_pattern.clone()
    }

    pub fn next(&mut self) -> Option<Pattern> {
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

    pub fn set_index(&mut self, index: usize) {
        self.index = index;

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
