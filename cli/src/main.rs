// homoglyphs -h 10 -c 5 "rust is best"
// homoglyphs -h 10 "rust is best"
// homoglyphs -c 5 "rust is best"
// homoglyphs "rust is best"

use clap::{value_parser, App, Arg, Parser};
use domain::sentence::{self, EncodedSentence, Sentence};
use drivers::tantivy::Tantivy;
use services::{self, ComputeHomoglyphs};

//#[derive(Parser, Debug)]
//#[clap(author, version, about, long_about = None)]
//pub struct Cli {
//    /// Sentence for which the possible homoglyphs are calculated
//    #[clap(forbid_empty_values = true)]
//    sentence: String,
//    /// Number of Homoglyphs to generate
//    #[clap(short, long, value_parser, validator = validate_homoglyphs_limit)]
//    homoglyphs_limit: usize,
//    /// Number of confusable to use for each letter of the Input sentence
//    #[clap(short, long, value_parser)]
//    confusable_limit: usize,
//}

fn validate_sentence(sentence: &str) -> Result<(), String> {
    todo!()
}

fn validate_homoglyphs_limit(homoglyphs_limit: &str) -> Result<(), String> {
    let h_lim = u8::from_str_radix(homoglyphs_limit, 10).unwrap();
    if h_lim == 0 {
        Err(String::from("homoglyphs_limit need to be greater than 0."))
    } else {
        Ok(())
    }
}
fn validate_confusable_limit(confusable_limit: &str) -> Result<(), String> {
    todo!()
}

fn main() {
    let matches = App::new("Homoglyphs")
        .version("0.1.0")
        .author("Shadawck <>")
        .about("Compute all possible homoglyphs from given sentence.")
        .arg(
            Arg::new("sentence")
                .value_name("SENTENCE")
                .help("Sentence for which the possible homoglyphs are calculated")
                .takes_value(true)
                .forbid_empty_values(true) //.validator(validate_sentence),
                .value_parser(value_parser!(String))
                .index(1)
                .required(true)
                ,
        ).arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Compute absolutely all possible homoglyphs. WARNING : Can take a long time or crash")
                .conflicts_with_all(&["homoglyphs_limit", "confusable_limit"]),
        ).arg(
            Arg::new("homoglyphs_limit")
                .short('n')

                .value_name("HOMOGLYPHS_LIMIT")
                .help("Number of Homoglyphs to generate")
                .takes_value(true)
                .validator(validate_homoglyphs_limit)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("confusable_limit")
                .short('c')
                .value_name("CONFUSABLE_LIMIT")
                .help("Number of confusable to use for each letter of the Input sentence")
                .takes_value(true)
                .value_parser(value_parser!(usize)), //.validator(validate_confusable_limit),
        )
        .get_matches();

    // TODO : Do it only one time by creating a MANAGED DIRECTORY
    let mut search_engine = Tantivy::init();
    search_engine.index();
    //////////////////////////////////////////////////////////////
    let mut ch = ComputeHomoglyphs::new();

    if matches.is_present("confusable_limit") && matches.is_present("homoglyphs_limit") {
        println!("Both present");
        let confusable_limit: usize = *matches
            .get_one("confusable_limit")
            .expect("confusable_limit is required in this case");

        let homoglyphs_limit: usize = *matches
            .get_one("homoglyphs_limit")
            .expect("homoglyphs_limit is required in this case");

        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();

        let results =
            ch.compute_with_limit(sentence_clone.as_str(), homoglyphs_limit, confusable_limit);
        //println!("{:#?}", results)
    } else if matches.is_present("confusable_limit") {
        println!("Confuable  present");
        let confusable_limit: usize = *matches
            .get_one("confusable_limit")
            .expect("confusable_limit is required in this case");

        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();

        let results = ch.compute_with_n_confusable(sentence_clone.as_str(), confusable_limit);
        //println!("{:#?}", results)
    } else if matches.is_present("homoglyphs_limit") {
        println!("homoglyphs present");

        let homoglyphs_limit: usize = *matches
            .get_one("homoglyphs_limit")
            .expect("homoglyphs_limit is required in this case");

        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();

        let results = ch.compute_with_n_permutation(sentence_clone.as_str(), homoglyphs_limit);
        //println!("{:#?}", results)
    } else if matches.is_present("all") {
        println!("all");
        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();
        let results = ch.compute(sentence_clone.as_str());

        //// NEED SPECIAL HANDLING to not crash / take all mem
    } else {
        println!("DEFAULT");
        let DEFAULT_CONFUSABLE_LIMIT = 8;
        let DEFAULT_HOMOGLYPHS_LIMIT = 100;
        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();
        let results = ch.compute_with_limit(
            sentence_clone.as_str(),
            DEFAULT_HOMOGLYPHS_LIMIT,
            DEFAULT_CONFUSABLE_LIMIT,
        );
    }
}
