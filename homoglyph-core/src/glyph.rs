//use serde::Serialize;

use crate::{Decodable, Encodable};
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct EncodedGlyph(pub String);

impl EncodedGlyph {
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

impl Decodable<Glyph> for EncodedGlyph {
    fn decode(&self) -> Result<Glyph, ParseIntError> {
        let decimal = u32::from_str_radix(&self.0, 16).unwrap();
        Ok(Glyph::new(char::from_u32(decimal).unwrap()))
    }
}
