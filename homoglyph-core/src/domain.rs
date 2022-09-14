//! A domain for each Word of the input sentence.

use crate::{glyph::EncodedGlyph, homoglyphs::Homoglyphs, word::EncodedWord, Decodable};
use permutator::CartesianProductIterator;
use std::{fmt::Display, slice::Iter, str::FromStr};

/// A WordDomain reprensent and encapsulate permutation of confusable for one Word of the input sentence.
#[derive(Debug, PartialEq, Clone)]
pub struct WordDomain(pub Vec<EncodedWord>);

impl WordDomain {
    /// Create a new WordDomain from a vector of EncodedWord
    pub fn new(word_domain: Vec<EncodedWord>) -> Self {
        Self(word_domain)
    }

    /// Build a WordDomain in where we take 'n' confusable glyph for each glyph of a Word from the input sentence.
    pub fn take(&mut self, n: usize) -> Self {
        let mut new_domain: Vec<EncodedWord> = Vec::new();
        let mut dest = vec![EncodedWord::from_str("").unwrap(); self.0.len()];
        dest.clone_from_slice(&self.0.as_slice());

        for enc_word in dest {
            if enc_word.0.len() >= n {
                let new_word = Vec::from_iter(enc_word.0.into_iter().take(n));
                new_domain.push(EncodedWord::new(new_word));
            } else {
                new_domain.push(enc_word)
            }
        }

        Self(new_domain)
    }

    /// Generate all possible or 'n' homoglyphs of a word (one) from a WordDomain.
    pub fn generate(&mut self, n: Option<usize>) -> Homoglyphs {
        let mut string = Vec::new();

        let mut vec: Vec<Vec<&str>> = Vec::new();
        for wd in self.0.iter() {
            let mut new = Vec::<&str>::new();
            for ew in wd.0.iter() {
                new.push(ew.0.as_str())
            }
            vec.push(new)
        }

        let vector_of_arrays: Vec<&[&str]> = vec.iter().map(AsRef::as_ref).collect();
        let str_domains: &[&[&str]] = &vector_of_arrays[..];

        let cart = CartesianProductIterator::new(str_domains).into_iter();

        let match_n = match n {
            Some(n) => n,
            None => cart.len(),
        };

        let mut cart_take = cart.take(match_n);

        for _ in 0..match_n {
            for permutation in cart_take.next() {
                let p: String = permutation
                    .iter()
                    .map(|string_glyph_enc| {
                        EncodedGlyph::from(string_glyph_enc.to_string())
                            .decode()
                            .unwrap()
                            .0
                    })
                    .collect();

                string.push(p);
            }
        }

        Homoglyphs::from(string)
    }

    pub fn iter(&self) -> Iter<EncodedWord> {
        self.0.iter()
    }
}

/// A SentenceDomain represent and encapsulate the permutation for all the WordDomain computed from the input sequence.
#[derive(Debug, Clone, PartialEq)]
pub struct SentenceDomain(pub Vec<WordDomain>);

impl SentenceDomain {
    /// Create a new SentenceDomain from a vector of WordDomain
    pub fn new(word_domains: Vec<WordDomain>) -> Self {
        Self(word_domains)
    }

    /// Generate all possible or 'n' homoglyphs computed in a WordDomain
    pub fn generate(mut self, n: Option<usize>) -> Vec<Homoglyphs> {
        let mut sentence_homoglyph = Vec::<Homoglyphs>::new();
        for wd in self.0.iter_mut() {
            sentence_homoglyph.push(wd.generate(n));
        }
        sentence_homoglyph
    }

    pub fn iter(&self) -> Iter<WordDomain> {
        self.0.iter()
    }
}

impl Display for WordDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
