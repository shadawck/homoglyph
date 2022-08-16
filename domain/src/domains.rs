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


    fn take(mut self, n : usize) -> Self {
        if self.domain.len() >= n {
            print!("test");
            let new_domain = self.domain[0..n].to_vec();
            
            
            
            Self {
                domain : new_domain
            }
        }
        else {
            Self {
                domain: self.domain
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let r = HexWord(["0072", "1d597", "1d4fb"].iter().map(|x| x.to_string()).collect());
        let u = HexWord(["0075", "1d462", "104f6"].iter().map(|x| x.to_string()).collect());
        let mut v = Vec::new();
        v.push(r); v.push(u);

        let d = Domains::new(v);
        let new_domain = d.take(1);
        println!("{}", new_domain);
    }


}

impl Display for Domains {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.domain)
    }
}

