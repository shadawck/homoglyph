// homoglyphs -n 10 -c 5 "rust is best"
// homoglyphs -n 10 "rust is best"
// homoglyphs -c 5 "rust is best"
// homoglyphs "rust is best"

use clap::{value_parser, App, Arg};
use domain::homoglyphs::Homoglyphs;
use drivers::tantivy::Tantivy;
use services::{self, ComputeHomoglyphs};
use tabled::object::Segment;
use tabled::{builder::Builder, Style};
use tabled::{Alignment, ModifyObject};

fn validate_sentence(sentence: &str) -> Result<(), String> {
    todo!()
}

fn validate_homoglyphs_limit(homoglyphs_limit: &str) -> Result<(), String> {
    let h_lim = u32::from_str_radix(homoglyphs_limit, 10).unwrap();
    if h_lim == 0 {
        Err(String::from("homoglyphs_limit need to be greater than 0."))
    } else {
        Ok(())
    }
}
fn validate_confusable_limit(confusable_limit: &str) -> Result<(), String> {
    todo!()
}

pub fn construct_output(compute_homoglyphs_result: Vec<Homoglyphs>) {
    let mut builder = Builder::default();
    let results_len = &compute_homoglyphs_result[..].len();

    for homoglyphs in compute_homoglyphs_result {
        let homoglyphs_str: Vec<String> = homoglyphs.0.iter().map(|f| f.0.to_string()).collect();
        builder.add_record(homoglyphs_str);
    }

    let mut builder = builder.index();
    builder.transpose();
    builder.hide_index();
    let table = builder
        .build()
        .with(Style::rounded())
        .with(Segment::all().modify().with(Alignment::center()));

    println!("{}", table);
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

    // TODO: Do it only one time by creating a MANAGED DIRECTORY
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
            ch.compute_with_limit(sentence_clone.trim(), homoglyphs_limit, confusable_limit);

        construct_output(results)
    } else if matches.is_present("confusable_limit") {
        println!("Confuable  present");
        let confusable_limit: usize = *matches
            .get_one("confusable_limit")
            .expect("confusable_limit is required in this case");

        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();

        let results = ch.compute_with_n_confusable(sentence_clone.trim(), confusable_limit);

        construct_output(results)
    } else if matches.is_present("homoglyphs_limit") {
        println!("homoglyphs present");

        let homoglyphs_limit: usize = *matches
            .get_one("homoglyphs_limit")
            .expect("homoglyphs_limit is required in this case");

        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();

        let results = ch.compute_with_n_permutation(sentence_clone.trim(), homoglyphs_limit);

        construct_output(results)
    } else if matches.is_present("all") {
        println!("all");
        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();
        let results = ch.compute(sentence_clone.trim());

        //// TODO: NEED SPECIAL HANDLING to not crash / take all mem
        construct_output(results)
    } else {
        println!("DEFAULT");
        let DEFAULT_CONFUSABLE_LIMIT = 8;
        let DEFAULT_HOMOGLYPHS_LIMIT = 100;
        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();
        let results = ch.compute_with_limit(
            sentence_clone.trim(),
            DEFAULT_HOMOGLYPHS_LIMIT,
            DEFAULT_CONFUSABLE_LIMIT,
        );

        construct_output(results)
    }
}
