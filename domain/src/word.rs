use std::{num::ParseIntError, str::FromStr};

use crate::{
    glyph::{EncodedGlyph, Glyph},
    hex_word::{Decodable, Encodable},
};

#[derive(Debug, PartialEq)]
pub struct Word(pub Vec<Glyph>);

impl Word {
    pub fn new(glyphs: Vec<Glyph>) -> Self {
        Self(glyphs)
    }
}

impl FromStr for Word {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut word: Vec<Glyph> = Vec::new();
        for c in s.chars().into_iter() {
            word.push(Glyph::new(c));
        }

        Ok(Word::new(word))
    }
}

impl Encodable<EncodedWord> for Word {
    fn encode(&self) -> Result<EncodedWord, ParseIntError> {
        let vec: Vec<EncodedGlyph> = self
            .0
            .iter()
            .map(|unencoded_g| unencoded_g.encode().unwrap())
            .collect();
        Ok(EncodedWord::new(vec))
    }
}

#[derive(Debug, PartialEq)]
pub struct EncodedWord(Vec<EncodedGlyph>);

impl EncodedWord {
    pub fn new(encoded_glyphs: Vec<EncodedGlyph>) -> Self {
        EncodedWord(encoded_glyphs)
    }
}

impl Decodable<Word> for EncodedWord {
    fn decode(&self) -> Result<Word, ParseIntError> {
        let decoded_word = self.0.iter().map(|enc_g| enc_g.decode().unwrap()).collect();
        Ok(Word::new(decoded_word))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_encodeable() {
        let w = Word::from_str("rust").unwrap();
        let w_enc = w.encode().unwrap();
        let w_dec = w_enc.decode().unwrap();
        assert_eq!(w_dec, w);
    }
}
