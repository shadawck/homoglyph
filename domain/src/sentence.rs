use std::{fmt, slice::Iter, str::FromStr, num::ParseIntError};

use crate::{word::EncodedWord, hex_word::{Decodable, Encodable}};
use super::hex_word::HexWord;

// ############################################################### //

#[derive(Debug, PartialEq)]
pub struct Sentence(Vec<HexWord>);

impl Sentence {
    pub fn new(s: &str) -> Self {
        let split: Vec<HexWord> = s
            .split(" ")
            .into_iter()
            .map(|w| HexWord::encode(w).unwrap())
            .collect();
        Self(split)
    }

    pub fn iter(&mut self) -> Iter<HexWord> {
        self.0.iter()
    }
}

impl Encodable<EncodedSentence> for Sentence {
    fn encode(&self) -> Result<EncodedSentence, std::num::ParseIntError> {
        todo!()
    }
}

// ############################################################### //

#[derive(Debug, PartialEq)]
pub struct EncodedSentence(Vec<EncodedWord>);

impl EncodedSentence {}
impl Decodable<Sentence> for EncodedSentence {
    fn decode(&self) -> Result<Sentence, ParseIntError> {
        todo!()
    }
}




#[derive(Debug, Clone)]
pub enum SentenceError {
    InvalidSentence { details: String },
}

impl std::error::Error for SentenceError {}

impl fmt::Display for SentenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Sentence")
    }
}

impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl FromStr for Sentence {
    type Err = SentenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_sentence_when_new_then_create_list_od_encoded_hexword() {
        let mut hexwords_list: Vec<HexWord> = Vec::new();
        hexwords_list.push(HexWord::encode("rust").unwrap());
        hexwords_list.push(HexWord::encode("is").unwrap());
        hexwords_list.push(HexWord::encode("the").unwrap());
        hexwords_list.push(HexWord::encode("best").unwrap());

        let mut sentence = Sentence::new("rust is the best");
        for hex in sentence.iter().zip(hexwords_list.iter()) {
            let (hex1, hex2) = hex;
            assert_eq!(hex1, hex2)
        }
    }
}
