// homoglyphs -n 10 -c 5 "rust is best"
// homoglyphs -n 10 "rust is best"
// homoglyphs -c 5 "rust is best"
// homoglyphs "rust is best"
use clap::{value_parser, App, Arg};
use homoglyph_core::homoglyphs::Homoglyphs;
use homoglyph_service::{self, ComputeHomoglyphs};
use std::collections::HashMap;

use serde_json::json;
use tabled::builder::Builder;
use tabled::object::Segment;
use tabled::style::RawStyle;
use tabled::Style;
use tabled::{Alignment, ModifyObject};

fn validate_homoglyphs_limit(homoglyphs_limit: &str) -> Result<(), String> {
    let h_lim = u32::from_str_radix(homoglyphs_limit, 10).unwrap();
    if h_lim == 0 {
        Err(String::from("homoglyphs_limit need to be greater than 0."))
    } else {
        Ok(())
    }
}

fn validate_format(_format: &str) -> Result<(), String> {
    let possible_format = &[
        "ascii",
        "empty",
        "markdown",
        "modern",
        "psql",
        "re-structured-text",
        "raw",
        "json",
    ];
    if possible_format.contains(&_format) {
        Ok(())
    } else {
        Err(format!("{} is not a possible value for format.\n Possible values are: ascii, empty, markdown, modern, psql, re-structured-text, json, raw (default)", _format))
    }
}

pub fn construct_output(compute_homoglyphs_result: Vec<Homoglyphs>, formats: &String) {
    if formats == "raw" {
        let mut homoglyphs_str_vec = Vec::<Vec<String>>::new();
        for homoglyphs in compute_homoglyphs_result {
            let homoglyphs_str: Vec<String> =
                homoglyphs.0.iter().map(|f| f.0.to_string()).collect();

            for h in &homoglyphs_str {
                println!("{}", h);
            }

            homoglyphs_str_vec.push(homoglyphs_str);
        }
    } else if formats == "json" {
        let mut homoglyphs_map = HashMap::new();

        let mut i = 0;
        for homoglyphs in compute_homoglyphs_result {
            let homoglyphs_str: Vec<String> =
                homoglyphs.0.iter().map(|f| f.0.to_string()).collect();

            homoglyphs_map.insert(i, homoglyphs_str);
            i += 1;
        }

        println!("{}", json!(homoglyphs_map));
    } else {
        let f_to_style: RawStyle = match formats.to_lowercase().as_str() {
            "ascii" => Style::ascii().into(),
            "empty" => Style::empty().into(),
            "markdown" => Style::markdown().into(),
            "modern" => Style::modern().into(),
            "psql" => Style::psql().into(),
            "re-structured-text" => Style::re_structured_text().into(),
            "rounded" => Style::rounded().into(),
            _ => Style::empty().into(),
        };

        let mut builder = Builder::default();

        for homoglyphs in compute_homoglyphs_result {
            let homoglyphs_str: Vec<String> =
                homoglyphs.0.iter().map(|f| f.0.to_string()).collect();
            builder.add_record(homoglyphs_str);
        }

        let mut builder = builder.index();
        builder.transpose();
        builder.hide_index();
        let table = builder
            .build()
            .with(Segment::all().modify().with(Alignment::center()))
            .with(f_to_style);
        println!("{}", table);
    }
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
                .forbid_empty_values(true)
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
                .value_parser(value_parser!(usize)),
        ).arg(
            Arg::new("format")
            .short('f')
            .value_name("FORMAT")
            .help("Format can be : ascii, empty, markdown, modern, psql, re-structured-text, raw (default)")
            .takes_value(true)
            .validator(validate_format)
            .default_value("raw"),

        )
        .get_matches();

    let mut ch = ComputeHomoglyphs::new();
    let formats: &String = matches.get_one("format").unwrap();

    if matches.is_present("confusable_limit") && matches.is_present("homoglyphs_limit") {
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

        construct_output(results, formats)
    } else if matches.is_present("confusable_limit") {
        let confusable_limit: usize = *matches
            .get_one("confusable_limit")
            .expect("confusable_limit is required in this case");

        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();

        let results = ch.compute_with_n_confusable(sentence_clone.trim(), confusable_limit);

        construct_output(results, formats)
    } else if matches.is_present("homoglyphs_limit") {
        let homoglyphs_limit: usize = *matches
            .get_one("homoglyphs_limit")
            .expect("homoglyphs_limit is required in this case");

        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();

        let results = ch.compute_with_n_permutation(sentence_clone.trim(), homoglyphs_limit);

        construct_output(results, formats)
    } else if matches.is_present("all") {
        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();
        let results = ch.compute(sentence_clone.trim());

        //// TODO: NEED SPECIAL HANDLING to not crash / take all mem
        construct_output(results, formats)
    } else {
        let default_confusable_limit = 8;
        let default_homoglyphs_limit = 100;
        let sentence: &String = &*matches.get_one("sentence").unwrap();
        let sentence_clone = sentence.clone();
        let results = ch.compute_with_limit(
            sentence_clone.trim(),
            default_homoglyphs_limit,
            default_confusable_limit,
        );

        construct_output(results, formats)
    }
}
