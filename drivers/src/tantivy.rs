use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::query::{Query, QueryParser};
use tantivy::schema::{STORED, TEXT};
use tantivy::{doc, IndexSettings, IndexWriter, ReloadPolicy};
use tantivy::{schema::Schema, Index};

use domain::domains::Domains;
use domain::hex_word::HexWord;
use domain::sentence::Sentence;
use domain::confusable;
use tempfile::TempDir;

pub struct Tantivy {
    index: Index,
    schema: Schema,
    queries: Vec<Box<dyn Query>>,
    index_writer: IndexWriter,
}

impl Tantivy {
    const HEX_GLYPH_FILE: &'static str = "./src/data/homoglyphs.txt";
    //const DIRECTORY: &'static str = "./temp";

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

    fn create_index(directory: PathBuf, schema: &Schema) -> Index {
        let mmap = MmapDirectory::open(directory).unwrap();
        Index::create(mmap.to_owned(), schema.to_owned(), IndexSettings::default()).unwrap()
    }
    fn create_index_writer(index: &Index) -> IndexWriter {
        index.writer(50_000_000).unwrap()
    }

    //}
    //
    //impl SearchEngine<'_> for Tantivy{
    fn init() -> Self {
        let schema = Tantivy::create_schema();
        let index_path = TempDir::new().unwrap().into_path();
        let index = Tantivy::create_index(index_path, &schema);
        let index_writer = Tantivy::create_index_writer(&index);
        Self {
            index,
            schema,
            queries: Vec::<Box<dyn Query>>::new(),
            index_writer,
        }
    }

    pub fn index(&mut self) {
        let confusable = confusable::confusable::HEX_FILE;

        let glyph = self.schema.get_field("glyph").unwrap();

        //if let Ok(lines) = Tantivy::read_lines(Tantivy::HEX_GLYPH_FILE) {
        for line in confusable.lines() {
            //if let Ok(ip) = line {
            self.index_writer.add_document(doc!(glyph => line)).unwrap();
            //}
        }
        //}
        self.index_writer.commit().unwrap();
    }

    pub fn start_tantivy() -> Self {
        let mut se = Self::init();
        se.index();
        se
    }

    pub fn query(&mut self, mut sentence: Sentence) {
        let glyph = self.schema.get_field("glyph").unwrap();
        let query_parser = QueryParser::for_index(&self.index, vec![glyph]);

        for c in sentence.iter() {
            for cc in c.0.iter() {
                let query = query_parser.parse_query(cc.to_string().as_str()).unwrap();
                self.queries.push(query);
            }
        }
    }

    pub fn search(&mut self) -> Domains {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();
        let searcher = reader.searcher();

        let mut domains: Vec<HexWord> = Vec::new();

        for query in &self.queries {
            let top_docs = searcher.search(query, &TopDocs::with_limit(1)).unwrap();
            let glyph = self.schema.get_field("glyph").unwrap();

            for (_score, doc_address) in top_docs {
                let retrieved_doc = searcher.doc(doc_address).unwrap();
                let value = retrieved_doc.get_all(glyph).into_iter().next().unwrap();
                let slice = value.as_text().unwrap().split_terminator(",");
                let mut glyphs: HexWord = HexWord::new();

                for s in slice {
                    glyphs.add(String::from(s.trim()));
                }
                domains.push(glyphs);
            }
        }

        Domains::new(domains)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_init_then_create_resource() {
        let mut tantivy = Tantivy::init();
        let sentence = Sentence::new("ru");
        tantivy.index();
        tantivy.query(sentence);
        let domains: Domains = tantivy.search();
        println!("{}", domains);
    }
}
