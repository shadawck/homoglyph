use std::time::Instant;

use itertools::Itertools;
use permutator::CartesianProductIterator;

use crate::{
    glyph::{EncodedGlyph, Glyph},
    homoglyph::Homoglyph,
};

pub struct Homoglyphs(Vec<Homoglyph>);

impl Homoglyphs {
    pub fn new(homoglyphs: Vec<Homoglyph>) -> Homoglyphs {
        Self(homoglyphs)
    }
}
