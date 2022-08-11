use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use tantivy::directory::MmapDirectory;
use tantivy::query::{Query, QueryParser};
use tantivy::schema::{STORED, TEXT, self};
use tantivy::{schema::Schema, Index};
use tantivy::{IndexSettings, IndexWriter};

use crate::entities::domains::Domains;
use crate::entities::hex_word::HexWord;
use crate::entities::sentence::Sentence;

use super::SearchEngine;

struct Tantivy {
    index: Index,
    schema: Schema,
    queries: Vec<Box<dyn Query>>,
    index_writer: IndexWriter,
}

impl Tantivy {
    const HEX_GLYPH_FILE : &'static str = "./src/data/homoglyphs.txt";

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn create_schema() -> Schema {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("glyph", TEXT | STORED);
        schema_builder.build()
    }
    pub fn create_index(directory: &str, schema: &Schema) -> Index {
        let mmap = MmapDirectory::open(directory).unwrap();
        Index::create(mmap.to_owned(), schema.to_owned(), IndexSettings::default()).unwrap()
    }
    pub fn create_index_writer(index: &Index) -> IndexWriter {
        index.writer(50_000_000).unwrap()
    }
}

impl SearchEngine for Tantivy {
    fn init(&mut self){
        self.schema = Tantivy::create_schema();
        self.index = Tantivy::create_index(".", &self.schema);
        self.index_writer = Tantivy::create_index_writer(&self.index);
    }

    fn index(&mut self) {
        let glyph = self.schema.get_field("glyph").unwrap();
        let p = Path::new(Tantivy::HEX_GLYPH_FILE);
        if let Ok(lines) = Tantivy::read_lines(Tantivy::HEX_GLYPH_FILE) {
            for line in lines {
                if let Ok(ip) = line {
                    self.index_writer.add_document(doc!(glyph => ip)).unwrap();
                }
            }
        }
        self.index_writer.commit().unwrap();
    }

    fn query(&mut self, mut sentence : Sentence) {
        let glyph = self.schema.get_field("glyph").unwrap();

        let mut queries: Vec<Box<dyn Query>> = Vec::new();

        for c in sentence.iter() {
            let query_parser = QueryParser::for_index(&self.index, vec![glyph]);
            let query = query_parser.parse_query(c.to_string().as_str()).unwrap();
            queries.push(query);
        }
    }

    //fn search(&mut self) -> Result<Domains, Error> {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_sentence_when_new_then_create_list_od_encoded_hexword() {

    }


}