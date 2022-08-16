use domain::{sentence::{Sentence}, domains::Domains, homoglyphs::Homoglyphs};
use clap::Parser;
use drivers::tantivy::Tantivy;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    sentence: Sentence,
}

fn main() {
    let mut search_engine = Tantivy::start_tantivy();
    let args = Cli::parse();
    

    search_engine.query(args.sentence);
    let domains : Domains = search_engine.search();
    let mut homoglyphs = Homoglyphs::new(domains);
    homoglyphs.generate();

    println!("{:#?}", homoglyphs.homoglyphs);
}
