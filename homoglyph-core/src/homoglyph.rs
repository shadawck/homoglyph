//! Homoglyph definition : <https://en.wikipedia.org/wiki/Homoglyph>

use crate::{glyph::Glyph, word::Word};
use std::{fmt::Display, slice::Iter};

/// An homoglyph is composed of one Word.
#[derive(Debug, PartialEq, Clone)]
pub struct Homoglyph(pub Word);

impl Homoglyph {
    // Create a new Homoglyph from a computed Word.
    pub fn new(word: Word) -> Homoglyph {
        Self(word)
    }

    pub fn iter(&self) -> Iter<Glyph> {
        self.0 .0.iter()
    }
}

impl Display for Homoglyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
