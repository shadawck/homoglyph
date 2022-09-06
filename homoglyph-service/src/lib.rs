use homoglyph_core::{
    domain::{SentenceDomain, WordDomain},
    homoglyphs::Homoglyphs,
    sentence::Sentence,
    Encodable,
};
use homoglyph_driver::{tantivy::*, SearchEngine};
use std::str::FromStr;

pub fn homoglyphs_to_string(homoglyphs_vec: Vec<Homoglyphs>) -> Vec<Vec<String>> {
    let mut res_vec = Vec::<Vec<String>>::new();
    for hs in homoglyphs_vec {
        let mut h_vec = Vec::<String>::new();
        for h in hs.0.iter() {
            h_vec.push(h.to_string());
        }
        res_vec.push(h_vec);
    }
    res_vec
}

pub struct ComputeHomoglyphs {
    search_engine: Tantivy,
}

impl ComputeHomoglyphs {
    pub fn new() -> ComputeHomoglyphs {
        let se = Tantivy::new();
        ComputeHomoglyphs { search_engine: se }
    }

    pub fn compute(&mut self, js_sentence: &str) -> Vec<Homoglyphs> {
        let s = Sentence::from_str(js_sentence).unwrap();
        let enc_s = s.encode().unwrap();
        self.search_engine.query(enc_s);

        let sd: SentenceDomain = self.search_engine.search();
        sd.generate(None)
    }

    pub fn compute_with_n_permutation(
        &mut self,
        js_sentence: &str,
        n_permutation: usize,
    ) -> Vec<Homoglyphs> {
        let s = Sentence::from_str(js_sentence).unwrap();
        let enc_s = s.encode().unwrap();
        self.search_engine.query(enc_s);

        let sd: SentenceDomain = self.search_engine.search();
        sd.generate(Some(n_permutation))
    }

    pub fn compute_with_n_confusable(
        &mut self,
        js_sentence: &str,
        n_confusable: usize,
    ) -> Vec<Homoglyphs> {
        let s = Sentence::from_str(js_sentence).unwrap();
        let enc_s = s.encode().unwrap();
        self.search_engine.query(enc_s);

        let sd: SentenceDomain = self.search_engine.search();
        let wd_custom: Vec<WordDomain> =
            sd.0.iter()
                .map(|w| {
                    let ww = w.clone().take(n_confusable);
                    let new_word = WordDomain::new(ww.0);
                    new_word
                })
                .collect();
        let new_sd = SentenceDomain::new(wd_custom);
        new_sd.generate(None)
    }

    pub fn compute_with_limit(
        &mut self,
        js_sentence: &str,
        n_permutation: usize,
        n_confusable: usize,
    ) -> Vec<Homoglyphs> {
        let s = Sentence::from_str(js_sentence).unwrap();
        let enc_s = s.encode().unwrap();
        self.search_engine.query(enc_s);

        let sd: SentenceDomain = self.search_engine.search();
        let wd_custom: Vec<WordDomain> =
            sd.0.iter()
                .map(|w| {
                    let ww = w.clone().take(n_confusable);
                    let new_word = WordDomain::new(ww.0);
                    new_word
                })
                .collect();
        let new_sd = SentenceDomain::new(wd_custom);
        new_sd.generate(Some(n_permutation))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PERMUTATION_NUMBER: usize = 5;
    const CONFUSABLE_NUMBER: usize = 2;
    const JS_SENTENCE: &str = "rust";

    #[test]
    fn when_compute_then_compute_all_possible_homoglyphs() {
        let mut chgs = ComputeHomoglyphs::new();
        chgs.compute(JS_SENTENCE);
    }

    #[test]
    fn when_compute_with_n_permutation_then_compute_n_possible_homoglyphs() {
        let mut chgs = ComputeHomoglyphs::new();
        let homoglyphs = chgs.compute_with_n_permutation(JS_SENTENCE, PERMUTATION_NUMBER);

        let size = match homoglyphs.get(0) {
            Some(h) => h.0.len(),
            None => 0,
        };

        assert_eq!(PERMUTATION_NUMBER, size);
    }

    #[test]
    fn when_compute_with_n_confusable_then_compute_possible_homoglyphs() {
        let mut chgs = ComputeHomoglyphs::new();
        let homoglyphs = chgs.compute_with_n_confusable(JS_SENTENCE, CONFUSABLE_NUMBER);

        let size = match homoglyphs.get(0) {
            Some(h) => h.0.len(),
            None => 0,
        };

        assert_eq!(
            4_u8.pow(CONFUSABLE_NUMBER.try_into().unwrap()) as usize,
            size
        );
    }

    #[test]
    fn when_compute_with_limit_then_compute_possible_homoglyphs() {
        let mut chgs = ComputeHomoglyphs::new();
        let homoglyphs =
            chgs.compute_with_limit(JS_SENTENCE, PERMUTATION_NUMBER, CONFUSABLE_NUMBER);

        let size = match homoglyphs.get(0) {
            Some(h) => h.0.len(),
            None => 0,
        };

        assert_eq!(PERMUTATION_NUMBER, size);
    }
}
