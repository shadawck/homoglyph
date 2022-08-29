use crate::{glyph::Glyph, word::Word};
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Homoglyph(Word);

impl Homoglyph {
    pub fn new(word: Word) -> Homoglyph {
        Self(word)
    }

    pub fn iter(&self) -> Iter<Glyph> {
        self.0 .0.iter()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_homoglyph() {
        //let w = Word::from_str("rust").unwrap();
        //let res = Homoglyph::from(vec!["rust".to_string()]);
        //
        //print!("{:#?}", res);
    }
}
