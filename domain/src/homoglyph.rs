use std::time::Instant;

use permutator::CartesianProductIterator;

use crate::{
    glyph::{EncodedGlyph, Glyph},
    word::Word,
    Decodable,
};

use super::domain::WordDomain;

pub struct Homoglyph {
    pub homoglyphs: Word,
}

impl Homoglyph {
    pub fn new() -> Homoglyph {
        Self {
            homoglyphs: Word::new(Vec::<Glyph>::new()),
        }
    }
}
