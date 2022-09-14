//! A glyph represent a unicode character.
use crate::{Decodable, Encodable};
use std::{num::ParseIntError, str::FromStr};

/// A glyph is composed of one rust character.
#[derive(Debug, PartialEq, Clone)]
pub struct Glyph(pub char);

impl Glyph {
    /// Create a glyph from a char.
    pub fn new(glyph: char) -> Self {
        Self(glyph)
    }
}

/// A glyph can be encoded into Unicode code.
impl Encodable<EncodedGlyph> for Glyph {
    fn encode(&self) -> Result<EncodedGlyph, ParseIntError> {
        Ok(EncodedGlyph::new(format!("{:04x}", self.0 as u32)))
    }
}

/// A unicode encoded glyph is an EncodedGlyph.
#[derive(Debug, PartialEq, Clone)]
pub struct EncodedGlyph(pub String);

impl EncodedGlyph {
    /// Create an EncodedGlyph from a unicode code formatted string.
    pub fn new(encoded_glyph: String) -> Self {
        Self(encoded_glyph)
    }
}

impl FromStr for EncodedGlyph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EncodedGlyph::new(s.to_string()))
    }
}

impl From<char> for EncodedGlyph {
    fn from(c_enc: char) -> Self {
        Self::new(c_enc.to_string())
    }
}

impl From<String> for EncodedGlyph {
    fn from(s_enc: String) -> Self {
        EncodedGlyph::from_str(s_enc.as_str()).unwrap()
    }
}

/// A glyph can be decoded from a Unicode code.
impl Decodable<Glyph> for EncodedGlyph {
    fn decode(&self) -> Result<Glyph, ParseIntError> {
        let decimal = u32::from_str_radix(&self.0, 16).unwrap();
        Ok(Glyph::new(char::from_u32(decimal).unwrap()))
    }
}
