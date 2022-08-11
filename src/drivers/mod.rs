use std::fmt::Error;

use crate::entities::sentence::Sentence;
use crate::entities::{domains::Domains};
use crate::entities::hex_word::HexWord;

mod tantivy;

pub trait SearchEngine {
    fn init(&mut self);
    fn index(&mut self);
    fn query(&mut self, search_sentence: Sentence);
    //fn search(&mut self);
}