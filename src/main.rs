#[macro_use]
extern crate tantivy;
use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;
use tantivy::collector::TopDocs;
use tantivy::directory::{MmapDirectory};
use tantivy::query::{QueryParser, Query};
use tantivy::{schema::{*, self}, IndexSettings, IndexWriter, ReloadPolicy};
use tantivy::Index;
//use unicode_segmentation::UnicodeSegmentation;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("glyph", TEXT | STORED);
    schema_builder.build()
}

fn create_index(directory: &str, schema : Schema)-> Index {
    let mmap = MmapDirectory::open(directory).unwrap();
    Index::create(mmap.to_owned(), schema.to_owned(), IndexSettings::default()).unwrap()
}

fn create_index_writer(index: Index) -> IndexWriter {
    index.writer(50_000_000).unwrap()
    
}

fn provision_index_writer(
    mut index_writer: IndexWriter,
    schema: Schema,
    glyph_file: PathBuf,
) -> IndexWriter {
    let glyph = schema.get_field("glyph").unwrap();

    if let Ok(lines) = read_lines(glyph_file) {
        for line in lines {
            if let Ok(ip) = line {
                index_writer.add_document(doc!(glyph => ip)).unwrap();
            }
        }
    }
    index_writer.commit().unwrap();
    index_writer
}


fn to_unicode_utf16_encoding(string : &str) -> String{
    let mut s = "".to_string();
    for c in string.chars() {
        let format = format!("{:04x?}, ", c as u32);
        s.push_str(format.as_str());
    }

    s
}

fn build_query(search_sentence: String, index: Index, schema : Schema) -> Box<dyn Query> {
    let glyph = schema.get_field("glyph").unwrap();
    let query_parser = QueryParser::for_index(&index, vec![glyph]);
    query_parser.parse_query(search_sentence.as_str()).unwrap()
}
    
fn search(index: Index, query: Box<dyn Query>, schema : Schema) {
    let reader = index.reader_builder().reload_policy(ReloadPolicy::OnCommit).try_into().unwrap();
    let searcher = reader.searcher();
    
    let top_docs = searcher.search(&query, &TopDocs::with_limit(5)).unwrap();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        println!("{}", schema.to_owned().to_json(&retrieved_doc));
    }
}

fn main() -> tantivy::Result<()> {
    let search_sentence = to_unicode_utf16_encoding("test");
    println!("{}", search_sentence);

    let glyph_file = Path::new("./src/data/homoglyphs.txt").to_path_buf();

    let schema = &create_schema();
    let index = &create_index("temp", schema.to_owned());
    let index_writer = create_index_writer(index.to_owned());    

    provision_index_writer(index_writer, schema.to_owned(), glyph_file);
    let query = build_query(search_sentence, index.to_owned(), schema.to_owned());
    search(index.to_owned(), query, schema.to_owned());

    Ok(())
}