use crate::{
    word::{EncodedWord, Word},
    Decodable, Encodable,
};
use std::{fmt, num::ParseIntError, slice::Iter, str::FromStr};

// ############################################################### //

#[derive(Debug, PartialEq)]
pub struct Sentence(Vec<Word>);

impl Sentence {
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
            .map(|w: &str| Word::from_str(w).unwrap())
            .collect();
        Ok(Sentence::new(sentence))
    }
}

impl From<Sentence> for EncodedSentence {
    fn from(sentence_dec: Sentence) -> Self {
        sentence_dec.encode().unwrap()
    }
}

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

// ############################################################### //

#[derive(Debug, PartialEq)]
pub struct EncodedSentence(Vec<EncodedWord>);

impl EncodedSentence {
    pub fn new(sentence: Vec<EncodedWord>) -> Self {
        Self(sentence)
    }

    pub fn iter(&mut self) -> Iter<EncodedWord> {
        self.0.iter()
    }
}

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

impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_sentence_when_new_then_create_list_of_encoded_word() {
        let mut hexwords_list: Vec<EncodedWord> = Vec::new();
        hexwords_list.push(Word::from_str("rust").unwrap().encode().unwrap());
        hexwords_list.push(Word::from_str("is").unwrap().encode().unwrap());
        hexwords_list.push(Word::from_str("the").unwrap().encode().unwrap());
        hexwords_list.push(Word::from_str("best").unwrap().encode().unwrap());

        let mut sentence_enc = Sentence::from_str("rust is the best")
            .unwrap()
            .encode()
            .unwrap();

        for hex in sentence_enc.iter().zip(hexwords_list.iter()) {
            let (hex1, hex2) = hex;
            assert_eq!(hex1, hex2)
        }

        let sentence_dec = sentence_enc.decode().unwrap();

        assert_eq!(
            sentence_dec,
            Sentence::from_str("rust is the best").unwrap()
        );
    }
}
