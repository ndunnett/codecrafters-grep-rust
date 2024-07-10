use std::ops::Range;

use crate::iterators::{CharIter, PatternIter};
use crate::parser::{Anchor, Atom, Pattern, Set};

pub struct Matcher {
    patterns: PatternIter,
    chars: CharIter,
    match_start: Option<usize>,
    pub matches: Vec<Range<usize>>,
}

impl Matcher {
    pub fn new(pattern: &str, input: &str) -> Result<Self, String> {
        Ok(Self {
            patterns: PatternIter::new(pattern)?,
            chars: CharIter::new(input)?,
            match_start: None,
            matches: Vec::new(),
        })
    }

    fn start_match(&mut self) {
        self.match_start = Some(self.chars.index.saturating_sub(1));
    }

    fn end_match(&mut self) {
        if let Some(start) = self.match_start {
            self.matches.push(Range {
                start,
                end: self.chars.index,
            });

            self.match_start = None;
            self.patterns.reset();
        } else {
            self.start_match();
            self.end_match();
        }
    }

    fn reset_match(&mut self) {
        if let Some(start) = self.match_start {
            self.chars.set_index(start + 1);
            self.match_start = None;
            self.patterns.reset();
        }
    }

    pub fn matches(&mut self) {
        while self.chars.peek().is_some() {
            if let Some(pattern) = self.patterns.peek() {
                match (self.match_pattern(&pattern), self.match_start.is_some()) {
                    (true, false) => {
                        self.start_match();
                    }
                    (false, true) => {
                        self.reset_match();
                    }
                    (false, false) => {
                        self.chars.consume();
                    }
                    _ => {}
                }

                if self.match_start.is_some() {
                    self.patterns.consume();
                }
            } else {
                self.end_match();
            }
        }

        if matches!(
            self.patterns.peek(),
            None | Some(Pattern::Anchor(Anchor::End))
        ) {
            self.end_match();
        }
    }

    fn match_pattern(&mut self, pattern: &Pattern) -> bool {
        match pattern {
            Pattern::Atom(pat) => self.match_atom(pat, true),
            Pattern::Anchor(pat) => self.match_anchor(pat),
            Pattern::Set(pat) => self.match_set(pat),
            Pattern::Group(pat) => pat.iter().all(|p| self.match_pattern(p)),
            Pattern::OneOrMore(pat) => {
                let mut matched = false;

                while self.match_pattern(pat) {
                    matched = true;
                }

                matched
            }
            Pattern::ZeroOrMore(pat) => {
                while self.match_pattern(pat) {}
                true
            }
            Pattern::ZeroOrOne(pat) => {
                self.match_pattern(pat);
                true
            }
            Pattern::Alternation(lhs, rhs) => self.match_pattern(lhs) || self.match_pattern(rhs),
        }
    }

    fn match_atom(&mut self, pattern: &Atom, consuming: bool) -> bool {
        if let Some(c) = self.chars.peek() {
            let result = match pattern {
                Atom::Digit => c.is_ascii_digit(),
                Atom::Alphanumeric => c.is_alphanumeric(),
                Atom::Char(ch) => c == *ch,
                Atom::Wildcard => true,
            };

            if result && consuming {
                self.chars.consume();
            }

            result
        } else {
            false
        }
    }

    fn match_anchor(&mut self, pattern: &Anchor) -> bool {
        match pattern {
            Anchor::Start => self.chars.index == 0,
            Anchor::End => self.chars.peek().is_none(),
        }
    }

    fn match_set(&mut self, pattern: &Set) -> bool {
        let result = match pattern {
            Set::Positive(set) => set.iter().any(|atom| self.match_atom(atom, false)),
            Set::Negative(set) => !set.iter().any(|atom| self.match_atom(atom, false)),
        };

        if result {
            self.chars.consume();
        }

        result
    }
}
