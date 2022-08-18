use std::{fmt::Display, time::Instant};

use permutator::CartesianProductIterator;

use crate::{glyph::EncodedGlyph, homoglyphs::Homoglyphs, word::EncodedWord, Decodable};

#[derive(Debug)]
pub struct WordDomain(pub Vec<EncodedWord>);

impl WordDomain {
    pub fn new(word_domain: Vec<EncodedWord>) -> Self {
        Self(word_domain)
    }

    /// Take only 'n' confusable for each glyph to build the domain of a word
    pub fn take(self, n: usize) -> Self {
        let mut new_domain: Vec<EncodedWord> = Vec::new();
        for enc_word in self.0 {
            if enc_word.0.len() >= n {
                let new_word = Vec::from_iter(enc_word.0.into_iter().take(n));
                new_domain.push(EncodedWord::new(new_word));
            }
        }
        Self(new_domain)
    }

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

        //let mut counter = 0;
        //let timer = Instant::now();
        let mut cart = CartesianProductIterator::new(str_domains).into_iter();

        for _ in 0..n.unwrap_or(cart.len()) {
            for permutation in cart.next() {
                //counter += 1;
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

        //println!("{:?}", string);
        //println!("Total {} products done in {:?}", counter, timer.elapsed());
        Homoglyphs::from(string)
    }
}

#[derive(Debug)]
pub struct SentenceDomain(pub Vec<WordDomain>);

impl SentenceDomain {
    pub fn new(word_domains: Vec<WordDomain>) -> Self {
        Self(word_domains)
    }

    pub fn generate(mut self, n: Option<usize>) -> Vec<Homoglyphs> {
        let mut sentence_homoglyph = Vec::<Homoglyphs>::new();
        for wd in self.0.iter_mut() {
            sentence_homoglyph.push(wd.generate(n));
        }
        sentence_homoglyph
    }
}

impl Display for WordDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
