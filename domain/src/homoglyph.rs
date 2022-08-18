use std::slice::Iter;

use crate::{glyph::Glyph, word::Word};

#[derive(Debug, PartialEq)]
pub struct Homoglyph(Word);

impl Homoglyph {
    pub fn new(word: Word) -> Homoglyph {
        Self(word)
    }

    pub fn iter(&self) -> Iter<Glyph> {
        self.0 .0.iter()
    }
}
