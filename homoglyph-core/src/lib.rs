//!

use std::num::ParseIntError;

pub mod confusable;
pub mod domain;
pub mod homoglyph;
pub mod homoglyphs;

pub mod glyph;
pub mod sentence;
pub mod word;

pub trait Encodable<T> {
    fn encode(&self) -> Result<T, ParseIntError>;
}

pub trait Decodable<T> {
    fn decode(&self) -> Result<T, ParseIntError>;
}
