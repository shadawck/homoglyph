use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone)]
pub struct HexWord(pub Vec<String>);

impl HexWord {
    pub fn new() -> Self {
        Self(Vec::<String>::new())
    }

    pub fn add(&mut self, s : String){
        self.0.push(s);
    }

    //fn to_string(&self) -> String {
    //    String::from(self.0)
    //}

    pub fn encode(s: &str) -> Result<Self, ParseIntError> {
        let vec: Vec<String> = s.chars().map(|c| format!("{:04x}", c as u32)).collect();

        Ok(Self(vec))
    }

    pub fn decode(&self) -> String {
        let decimal = self.0.iter().map(|h| u32::from_str_radix(h, 16).unwrap());
        decimal.map(|d| char::from_u32(d).unwrap()).collect()
    }

    pub fn decode_from_string(encoded_slice: String) -> String{
        char::from_u32(u32::from_str_radix(&encoded_slice, 16).unwrap()).unwrap().to_string()
    }


}

impl Display for HexWord {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:#?}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_str_when_encode_then_return_hex_encoded_value() {
        let s = "rust";
        let hex_of_rust_string = ["0072", "0075", "0073", "0074"].map(String::from).to_vec();
        assert_eq!(HexWord::encode(s).unwrap(), HexWord(hex_of_rust_string));
    }

    #[test]
    fn given_a_hex_object_when_decode_then_return_unicoded_string() {
        let h = HexWord::encode("rust").unwrap();
        assert_eq!("rust", h.decode());
    }
}
