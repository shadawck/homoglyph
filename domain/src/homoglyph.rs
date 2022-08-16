//// the list of homoglyph found for the input sentence after the permutation
//use super::hex_word::HexWord;
//
//#[derive(Debug, PartialEq)]
//pub struct Homoglyph(HexWord);
//pub struct Homoglyphs(Vec<Homoglyph>);
//
//impl Homoglyph {
//    fn new(hex_word: HexWord) -> Self {
//        Self(hex_word)
//    }
//
//    fn to_string(&self) -> String {
//        self.0.decode()
//    }
//}
//
//impl Homoglyphs {
//    pub fn new() -> Self {
//        Self(Vec::<Homoglyph>::new())
//    }
//
//    pub fn add(&mut self, homoglyph: Homoglyph) {
//        self.0.push(homoglyph);
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn given_hex_word_when_create_new_homoglyph_then_return_it() {
//        let hex_word = HexWord::encode("rust").unwrap();
//        let h = Homoglyph::new(hex_word);
//
//        assert_eq!(h, Homoglyph(HexWord::encode("rust").unwrap()));
//    }
//
//    #[test]
//    fn given_an_homoglyph_when_to_string_then_decode_homoglyph_as_string() {
//        let hex_word = HexWord::encode("rust").unwrap();
//        let h = Homoglyph::new(hex_word);
//        assert_eq!(h.to_string(), "rust");
//    }
//}
