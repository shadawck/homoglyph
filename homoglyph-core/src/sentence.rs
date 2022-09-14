//! A sentence represent a sentence of word. A sentence can be encoded into unicode code or decoded from unicode code.

use crate::{
    word::{EncodedWord, Word},
    Decodable, Encodable,
};
use std::{fmt, num::ParseIntError, slice::Iter, str::FromStr};

/// A Sentence is a representation of the Input sentence which is a Vector of Word.
#[derive(Debug, PartialEq, Clone)]
pub struct Sentence(Vec<Word>);

impl Sentence {
    /// Create a new Sentence from a Vector of Word.
    pub fn new(words: Vec<Word>) -> Self {
        Self(words)
    }

    pub fn iter(&mut self) -> Iter<Word> {
        self.0.iter()
    }
}

impl From<String> for Sentence {
    fn from(string_dec: String) -> Self {
        Sentence::from_str(string_dec.as_str()).unwrap()
    }
}

impl FromStr for Sentence {
    type Err = ();

    fn from_str(s_dec: &str) -> Result<Self, Self::Err> {
        let sentence: Vec<Word> = s_dec
            .split(" ")
            .into_iter()
            .map(|w| Word::from_str(w).unwrap())
            .collect();

        Ok(Sentence::new(sentence))
    }
}

impl From<Sentence> for EncodedSentence {
    fn from(sentence_dec: Sentence) -> Self {
        sentence_dec.encode().unwrap()
    }
}

/// A whole sentence can be encoded into Unicode code.
impl Encodable<EncodedSentence> for Sentence {
    fn encode(&self) -> Result<EncodedSentence, std::num::ParseIntError> {
        let vec: Vec<EncodedWord> = self
            .0
            .iter()
            .map(|unencoded_w| unencoded_w.encode().unwrap())
            .collect();
        Ok(EncodedSentence::new(vec))
    }
}

/// A unicode encoded Sentence is an EncodedSentence
#[derive(Debug, PartialEq, Clone)]
pub struct EncodedSentence(Vec<EncodedWord>);

impl EncodedSentence {
    /// Create a new EncodedSentence from a Vector EncodedWord.
    pub fn new(sentence: Vec<EncodedWord>) -> Self {
        Self(sentence)
    }

    pub fn iter(&mut self) -> Iter<EncodedWord> {
        self.0.iter()
    }
}

/// A sentence can be decoded from a Unicode Encoded Sentence.
impl Decodable<Sentence> for EncodedSentence {
    fn decode(&self) -> Result<Sentence, ParseIntError> {
        let decoded_sentence = self
            .0
            .iter()
            .map(|encoded_w| encoded_w.decode().unwrap())
            .collect();

        Ok(Sentence::new(decoded_sentence))
    }
}

impl From<EncodedSentence> for Sentence {
    fn from(sentence_dec: EncodedSentence) -> Self {
        sentence_dec.decode().unwrap()
    }
}

impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_sentence_when_new_then_create_list_of_encoded_word() {
        let mut encoded_word_s: Vec<EncodedWord> = Vec::new();
        encoded_word_s.push(Word::from_str("rust").unwrap().encode().unwrap());
        encoded_word_s.push(Word::from_str("is").unwrap().encode().unwrap());
        encoded_word_s.push(Word::from_str("the").unwrap().encode().unwrap());
        encoded_word_s.push(Word::from_str("best").unwrap().encode().unwrap());

        let mut sentence_enc = Sentence::from_str("rust is the best")
            .unwrap()
            .encode()
            .unwrap();

        for (uni1, uni2) in sentence_enc.iter().zip(encoded_word_s.iter()) {
            assert_eq!(uni1, uni2)
        }

        let sentence_dec = sentence_enc.decode().unwrap();

        assert_eq!(
            sentence_dec,
            Sentence::from_str("rust is the best").unwrap()
        );
    }
}
