use std::{fmt::Display, str::FromStr};

use clap::Parser;
use domain::{
    domain::{SentenceDomain, WordDomain},
    homoglyph::Homoglyph,
    sentence::{EncodedSentence, Sentence},
};
use drivers::tantivy::Tantivy;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    sentence: String,
}

fn main() {
    let mut search_engine = Tantivy::start_tantivy();
    let args = Cli::parse();
    let sentence_dec: Sentence = Sentence::from(args.sentence);
    let sentence_enc = EncodedSentence::from(sentence_dec);

    search_engine.query(sentence_enc);

    let new_sd : SentenceDomain = search_engine.search();

    println!("{:#?}", new_sd);
    println!("{:?}", new_sd.generate());
}
