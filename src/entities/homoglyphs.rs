use permutator::CartesianProductIterator;

use super::{homoglyph::Homoglyph, domains::Domains, hex_word::HexWord};

pub struct Homoglyphs {
    pub homoglyphs : Vec<String>,
    domains : Domains,
}

impl Homoglyphs {
    pub fn new(domains: Domains) -> Homoglyphs {
        Self { 
            homoglyphs: Vec::<String>::new(),
            domains: domains
        }
    }

    fn generate(&mut self){
        
        let tmp: Vec<Vec<&str>> = self.domains.domain
            .iter()
            .map(|list| list.0.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
            .collect();

        let vector_of_arrays: Vec<&[&str]> = tmp.iter().map(AsRef::as_ref).collect();
        let str_domains : &[&[&str]] = &vector_of_arrays[..];
        
        let cart = CartesianProductIterator::new(str_domains).into_iter();

        for permutation in cart {
            let p: String = permutation
                .iter()
                .map(|glyph| HexWord::decode_from_string(glyph.to_string()))
                .collect();
            self.homoglyphs.push(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_init_then_create_resource() {
        let r = HexWord(["0072", "1d597", "1d4fb"].iter().map(|x| x.to_string()).collect());
        let u = HexWord(["0075", "1d462", "104f6"].iter().map(|x| x.to_string()).collect());
        let mut v = Vec::new();
        v.push(r); v.push(u);

        let d = Domains::new(v);
        let mut homoglyphs = Homoglyphs::new(d);
        homoglyphs.generate();
        
        println!("{:#?}", homoglyphs.homoglyphs);
    }
}
