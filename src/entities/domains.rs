use std::fmt::Display;

use super::hex_word::HexWord;

/// Contain result of SearchEngine. Will be consume by a service
#[derive(Debug)]
pub struct Domains {
    pub domain : Vec<HexWord>
}

impl Domains {
    pub fn new(v: Vec<HexWord>) -> Self {
        Self{
            domain: v
        }
    }
}

impl Display for Domains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.domain)
    }
}
