use std::str::FromStr;

use domain::{
    domain::{SentenceDomain, WordDomain},
    homoglyphs::Homoglyphs,
    sentence::Sentence,
    Encodable,
};
use drivers::tantivy::*;
//use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

//#[wasm_bindgen]
pub struct ComputeHomoglyphs {
    search_engine: Tantivy,
}

//#[wasm_bindgen]
impl ComputeHomoglyphs {
    //#[wasm_bindgen(constructor)]
    pub fn new() -> ComputeHomoglyphs {
        let mut se = Tantivy::init();
        se.index();
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

    // Wasm impl
    //pub fn js_compute_all_homoglyphs(&mut self, js_sentence: &str) -> JsValue {
    //    let hgs = Self::compute_all_homoglyphs(self, js_sentence);
    //    JsValue::from_serde(&hgs).unwrap()
    //}

    //pub fn js_compute_n_homoglyphs(&mut self, js_sentence: &str, n_permutation: usize) -> JsValue {
    //    let hgs = Self::compute_n_homoglyphs(self, js_sentence, n_permutation);
    //    JsValue::from_serde(&hgs).unwrap()
    //}
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
