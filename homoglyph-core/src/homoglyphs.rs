//!

use crate::{homoglyph::Homoglyph, word::Word};
use std::{fmt::Display, str::FromStr};

/// Homoglyphs is a Vector of Homoglyph representing all the permutation computed for the input sequence.
#[derive(Debug, PartialEq, Clone)]
pub struct Homoglyphs(pub Vec<Homoglyph>);

impl Homoglyphs {
    /// Create an Homoglyphs from a Vector of Homoglyph
    pub fn new(homoglyphs: Vec<Homoglyph>) -> Homoglyphs {
        Self(homoglyphs)
    }
}

impl From<Vec<String>> for Homoglyphs {
    fn from(homoglyphs: Vec<String>) -> Self {
        let mut h: Vec<Homoglyph> = Vec::new();
        for s in homoglyphs {
            let word = Word::from_str(s.as_str()).unwrap();
            h.push(Homoglyph::new(word));
        }

        Homoglyphs::new(h)
    }
}

impl Display for Homoglyphs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a: Vec<String> = self
            .0
            .iter()
            .map(|p| p.iter().map(|g| g.0.to_string()).collect())
            .collect();
        write!(f, "{:?}", a)
    }
}
