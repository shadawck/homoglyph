use serde::{Deserialize, Serialize};

use crate::{
    glyph::{EncodedGlyph, Glyph},
    Decodable, Encodable,
};
use std::{num::ParseIntError, slice::Iter, str::FromStr};
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Word(pub Vec<Glyph>);

impl Word {
    pub fn new(glyphs: Vec<Glyph>) -> Self {
        Self(glyphs)
    }

    pub fn iter(&self) -> Iter<Glyph> {
        self.0.iter()
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for glyph_dec in self.0.iter() {
            string.push_str(glyph_dec.0.to_string().as_str());
        }
        string
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

impl From<Word> for String {
    fn from(word: Word) -> Self {
        let mut string = String::new();
        for glyph in word.0.iter() {
            string.push_str(glyph.0.to_string().as_str())
        }
        string
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

#[derive(Debug, PartialEq, Clone)]
pub struct EncodedWord(pub Vec<EncodedGlyph>);

impl EncodedWord {
    pub fn new(encoded_glyphs: Vec<EncodedGlyph>) -> Self {
        EncodedWord(encoded_glyphs)
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for glyph_enc in self.0.iter() {
            string.push_str(glyph_enc.0.as_str())
        }
        string
    }

    pub fn iter(&self) -> Iter<EncodedGlyph> {
        self.0.iter()
    }
}

impl From<&str> for EncodedWord {
    fn from(s: &str) -> Self {
        // Check of encoded_str
        let mut word: Vec<EncodedGlyph> = Vec::new();
        for c in s.chars().into_iter() {
            word.push(EncodedGlyph::from(c));
        }

        EncodedWord::new(word)
    }
}

impl From<EncodedWord> for String {
    fn from(word_enc: EncodedWord) -> Self {
        let mut string = String::new();
        for glyph_enc in word_enc.0.iter() {
            string.push_str(glyph_enc.0.as_str())
        }
        string
    }
}

impl FromStr for EncodedWord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check of encoded_str
        let mut word: Vec<EncodedGlyph> = Vec::new();
        for c in s.chars().into_iter() {
            word.push(EncodedGlyph::from(c));
        }

        Ok(EncodedWord::new(word))
    }
}

impl Decodable<Word> for EncodedWord {
    fn decode(&self) -> Result<Word, ParseIntError> {
        let decoded_word = self.0.iter().map(|enc_g| enc_g.decode().unwrap()).collect();
        Ok(Word::new(decoded_word))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encodable_then_decodable() {
        let w = Word::from_str("rust").unwrap();
        let w_enc = w.encode().unwrap();
        let w_dec = w_enc.decode().unwrap();
        assert_eq!(w_dec, w);
    }
}
