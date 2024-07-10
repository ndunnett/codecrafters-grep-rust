use crate::parser::{FlatPattern, Parser};

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

    pub fn next(&mut self) -> Option<FlatPattern> {
        if self.next_pattern.is_some() {
            let pat = self.next_pattern.clone();

            self.next_pattern = if self.index + 1 < self.patterns.len() {
                Some(self.patterns[self.index + 1].clone())
            } else {
                None
            };

            self.index += 1;
            pat
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<FlatPattern> {
        self.next_pattern.clone()
    }

    pub fn reset(&mut self) {
        self.next_pattern = if !self.patterns.is_empty() {
            Some(self.patterns[0].clone())
        } else {
            None
        };

        self.index = 0;
    }
}

pub fn matches(pattern: &str, input: &str) -> Result<bool, String> {
    let mut patterns = PatternIter::new(pattern)?;
    let chars = input.chars().collect::<Vec<_>>();
    let mut char_index = 0;
    let mut in_match = false;

    while let Some(c) = chars.get(char_index) {
        if let Some(pattern) = patterns.peek() {
            match (pattern.matches(*c), in_match) {
                (true, false) => in_match = true,
                (false, true) => {
                    in_match = false;
                    patterns.reset()
                }
                _ => {}
            }

            if in_match {
                let _ = patterns.next();
            }
        } else {
            return Ok(true);
        }

        char_index += 1;
    }

    Ok(patterns.peek().is_none())
}
