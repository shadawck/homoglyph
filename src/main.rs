#[macro_use]
extern crate tantivy;

use permutator::CartesianProductIterator;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::{Query, QueryParser};
use tantivy::Index;
use tantivy::{schema::*, IndexSettings, IndexWriter, ReloadPolicy};

mod drivers;
mod entities;


//fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//where
//    P: AsRef<Path>,
//{
//    let file = File::open(filename)?;
//    Ok(io::BufReader::new(file).lines())
//}
//
////fn create_schema() -> Schema {
////    let mut schema_builder = Schema::builder();
////
////    schema_builder.add_text_field("glyph", TEXT | STORED);
////    schema_builder.build()
////}
////
////fn create_index(directory: &str, schema: Schema) -> Index {
////    let mmap = MmapDirectory::open(directory).unwrap();
////    Index::create(mmap.to_owned(), schema.to_owned(), IndexSettings::default()).unwrap()
////}
////
////fn create_index_writer(index: Index) -> IndexWriter {
////    index.writer(50_000_000).unwrap()
////}
//
//fn provision_index_writer(
//    mut index_writer: IndexWriter,
//    schema: Schema,
//    glyph_file: PathBuf,
//) -> IndexWriter {
//    let glyph = schema.get_field("glyph").unwrap();
//
//    if let Ok(lines) = read_lines(glyph_file) {
//        for line in lines {
//            if let Ok(ip) = line {
//                index_writer.add_document(doc!(glyph => ip)).unwrap();
//            }
//        }
//    }
//    index_writer.commit().unwrap();
//    index_writer
//}
//
////use std::{fmt::Write, num::ParseIntError};
//
////pub fn decode_hex(s: &[&str]) -> Result<Vec<u32>, ParseIntError> {
////    let mut decimals = Vec::<u32>::new();
////    for hex_code in s {
////        let decimal = u32::from_str_radix(&hex_code, 16).unwrap();
////        decimals.push(decimal);
////    }
////
////    Ok(decimals)
////}
//
////pub fn decode_hex2(s: &str) -> Result<u32, ParseIntError> {
////    let decimal = u32::from_str_radix(s, 16).unwrap();
////    Ok(decimal)
////}
////
////pub fn encode_hex_from_u32(bytes: &str) -> String {
////    let mut s = String::with_capacity(bytes.len() * 2);
////    for b in bytes.chars() {
////        write!(&mut s, "{:04x}, ", b as u32).unwrap();
////    }
////    s
////}
//
////fn to_unicode_utf16_encoding(string: &str) -> String {
////    let mut s = "".to_string();
////    for c in string.chars() {
////        let format = format!("{:04x?}, ", c as u32);
////        s.push_str(format.as_str());
////    }
////    s
////}
//
//fn build_query(search_sentence: String, index: Index, schema: Schema) -> Vec<Box<dyn Query>> {
//    let glyph = schema.get_field("glyph").unwrap();    
//    
//    let mut queries : Vec<Box<dyn Query>> = Vec::new();
//  
//    let term = search_sentence.split_terminator(",");
//
//    for c in term {
//        let query_parser = QueryParser::for_index(&index, vec![glyph]);
//        let query = query_parser.parse_query(c.trim()).unwrap();
//        queries.push(query);
//    }
//
//    queries
//}
//
//fn search(index: Index, queries: Vec<Box<dyn Query>>, schema: Schema) -> Vec<Vec<String>> {
//    let reader = index
//        .reader_builder()
//        .reload_policy(ReloadPolicy::OnCommit)
//        .try_into()
//        .unwrap();
//    let searcher = reader.searcher();
//    let mut slsl: Vec<Vec<String>> = Vec::new();
//    
//    for query in queries {
//
//        let top_docs = searcher.search(&query, &TopDocs::with_limit(1)).unwrap();
//        let glyph = schema.get_field("glyph").unwrap();
//
//        for (_score, doc_address) in top_docs {
//            let retrieved_doc = searcher.doc(doc_address).unwrap();
//            let value = retrieved_doc.get_all(glyph).into_iter().next().unwrap();
//    
//            let slice = value.as_text().unwrap().split_terminator(",");
//            let mut homoglyph: Vec<String> = Vec::new();
//    
//            for s in slice {
//                homoglyph.push(String::from(s.trim()));
//            }
//    
//            slsl.push(homoglyph);
//        }
//    }
//
//    slsl
//}
//
//fn build_cartesian_domain(domains: &[&[&str]]) -> Vec<String> {
//    let cart = CartesianProductIterator::new(domains).into_iter();
//    
//    let mut collect_all = Vec::new();
//    for permutation in cart {
//        let p: String = permutation
//            .iter()
//            .map(|glyph| char::from_u32(decode_hex2(&glyph).unwrap()).unwrap())
//            .collect();
//        collect_all.push(p);
//    }
//
//    collect_all
//}

fn main() {
    //let search_sentences = "abcde";
    //
    //let search_sentence = encode_hex_from_u32(search_sentences);
    //let glyph_file = Path::new("./src/data/homoglyphs.txt").to_path_buf();
//
    //let schema = &create_schema();
    //let index = &create_index("temp", schema.to_owned());
    ////let index_writer = create_index_writer(index.to_owned());
    ////provision_index_writer(index_writer, schema.to_owned(), glyph_file);
//
    //let query = build_query(search_sentence, index.to_owned(), schema.to_owned());
    //let set = search(index.to_owned(), query, schema.to_owned());
//
    //let tmp: Vec<Vec<&str>> = set
    //    .iter()
    //    .map(|list| list.iter().map(AsRef::as_ref).collect::<Vec<&str>>())
    //    .collect();
//
    //let vector_of_arrays: Vec<&[&str]> = tmp.iter().map(AsRef::as_ref).collect();
    //let domains = &vector_of_arrays[..];
    ////println!("{:#?}", domains);
    //let _: Vec<String> = build_cartesian_domain(domains);
    ////println!("{:#?}", homoglyph);

}
