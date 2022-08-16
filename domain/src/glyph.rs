use std::num::ParseIntError;

use crate::hex_word::{Decodable, Encodable};

#[derive(Debug, PartialEq)]
pub struct Glyph(pub char);

impl Glyph {
    pub fn new(glyph: char) -> Self {
        Self(glyph)
    }
}

impl Encodable<EncodedGlyph> for Glyph {
    fn encode(&self) -> Result<EncodedGlyph, ParseIntError> {
        Ok(EncodedGlyph::new(format!("{:04x}", self.0 as u32)))
    }
}

#[derive(Debug, PartialEq)]
pub struct EncodedGlyph(pub String);

impl EncodedGlyph {
    pub fn new(encoded_glyph: String) -> Self {
        Self(encoded_glyph)
    }
}

impl Decodable<Glyph> for EncodedGlyph {
    fn decode(&self) -> Result<Glyph, ParseIntError> {
        let decimal = u32::from_str_radix(&self.0, 16).unwrap();
        Ok(Glyph::new(char::from_u32(decimal).unwrap()))
    }
}
