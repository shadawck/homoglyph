use super::homoglyph::Homoglyph;

pub struct Homoglyphs {
    homoglyphs : Vec<Homoglyph>
}

impl Homoglyphs {
    pub fn new() -> Homoglyphs {
        Self { homoglyphs: Vec::<Homoglyph>::new() }
    }
}