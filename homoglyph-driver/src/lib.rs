use homoglyph_core::{domain::SentenceDomain, sentence::EncodedSentence};

pub mod tantivy;
pub trait SearchEngine {
    fn init() -> Self;
    fn index(&mut self);
    fn query(&mut self, sentence_enc: EncodedSentence);
    fn new() -> Self;
    fn search(&mut self) -> SentenceDomain;
}
