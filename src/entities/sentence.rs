use std::slice::Iter;

// Input sentence. useful for handling solo char, whitespace ...
use super::{hex_word::HexWord};


#[derive(Debug, PartialEq)]
pub struct Sentence(Vec<HexWord>);

impl Sentence {
    pub fn new(s : &str) -> Self {
        let split : Vec<HexWord> = s.split(" ").into_iter().map(|w| HexWord::encode(w).unwrap()).collect();
        Self(split)
    }

    pub fn iter(&mut self) -> Iter<HexWord>{
        self.0.iter()
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