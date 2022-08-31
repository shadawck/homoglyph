use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use domain::glyph::EncodedGlyph;
use domain::word::EncodedWord;
use tantivy::collector::TopDocs;
use tantivy::directory::{MmapDirectory, RamDirectory};
use tantivy::query::{Query, QueryParser};
use tantivy::schema::{STORED, TEXT};
use tantivy::{doc, IndexSettings, IndexWriter, Inventory, ReloadPolicy};
use tantivy::{schema::Schema, Index};

use domain::confusable;
use domain::domain::{SentenceDomain, WordDomain};
use domain::sentence::EncodedSentence;

use crate::SearchEngine;

#[allow(dead_code)]
pub struct Tantivy {
    index: Index,
    schema: Schema,
    queries_by_domain: Vec<Vec<Box<dyn Query>>>,
    index_writer: IndexWriter,
    inventory: Inventory<PathBuf>,
}

static HOMOGLYPHS_DIR: &'static str = "/tmp/homoglyphs";
static HOMOGLYPHS_STATE_FILE: &'static str = "/tmp/homoglyphs/.state";

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("glyph", TEXT | STORED);
    schema_builder.build()
}

fn build_mmap_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(HOMOGLYPHS_DIR);
    path
}

fn create_managed_index(schema: &Schema) -> Index {
    let homoglyphs_dir = build_mmap_path();
    if !homoglyphs_dir.exists() {
        create_dir_all(&homoglyphs_dir).unwrap();
    }

    let mmap = MmapDirectory::open(homoglyphs_dir).unwrap();
    //let managed_mmap = ManagedDirectory::wrap(Box::new(mmap.clone())).unwrap();

    let index = Index::open_or_create(mmap.clone(), schema.to_owned()).unwrap();
    index
}
fn create_in_ram_index(ram_directory: RamDirectory, schema: &Schema) -> Index {
    Index::create(
        ram_directory.to_owned(),
        schema.to_owned(),
        IndexSettings::default(),
    )
    .unwrap()
}

// TODO: Implement a Config Pattern and a State Pattern
// TODO: Add inventory tracking and hash checking on living file in new
// TODO: Add Garbage collect
#[allow(dead_code)]
impl SearchEngine for Tantivy {
    fn init() -> Self {
        let schema = create_schema();

        //let ramd = RamDirectory::create();
        //let index = Tantivy::create_in_ram_index(ramd, &schema);

        let index = create_managed_index(&schema);
        let index_writer = index.writer(50_000_000).unwrap();

        Self {
            index,
            schema,
            queries_by_domain: Vec::<Vec<Box<dyn Query>>>::new(),
            index_writer,
            inventory: Inventory::default(),
        }
    }
    fn new() -> Self {
        let mut tantivy = Self::init();

        let contents = read_to_string(HOMOGLYPHS_STATE_FILE);

        let indexed_state = match contents {
            Ok(p) => p,                // indexed
            Err(_) => "0".to_string(), // not indexed
        };

        if i32::from_str(indexed_state.as_str()).unwrap() == 0 {
            tantivy.index();
        }

        tantivy
    }

    fn index(&mut self) {
        let confusable = confusable::confusable::HEX_FILE;
        let glyph = self.schema.get_field("glyph").unwrap();

        for line in confusable.lines() {
            self.index_writer.add_document(doc!(glyph => line)).unwrap();
        }

        self.index_writer.commit().unwrap();

        let mut file = File::create("/tmp/homoglyphs/.state").unwrap();
        file.write_all(b"1").unwrap();
    }

    fn query(&mut self, mut sentence_enc: EncodedSentence) {
        let glyph = self.schema.get_field("glyph").unwrap();
        let query_parser = QueryParser::for_index(&self.index, vec![glyph]);

        for c in sentence_enc.iter() {
            let mut queries = Vec::new();
            for cc in c.iter() {
                let query = query_parser.parse_query(cc.0.as_str()).unwrap();
                queries.push(query);
            }
            self.queries_by_domain.push(queries)
        }
    }

    fn search(&mut self) -> SentenceDomain {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();
        let searcher = reader.searcher();

        let mut sentence_domain: Vec<WordDomain> = Vec::new();

        // vec of queries for each word
        for queries in &self.queries_by_domain {
            // queries of confusable in each word
            let mut world_domain: Vec<EncodedWord> = Vec::new();

            for query in queries {
                let mut confusable_word: Vec<EncodedWord> = Vec::new();
                let top_docs = searcher.search(query, &TopDocs::with_limit(1)).unwrap();
                let glyph = self.schema.get_field("glyph").unwrap();

                // Found confusable for each query of each word
                for (_score, doc_address) in top_docs {
                    let retrieved_doc = searcher.doc(doc_address).unwrap();
                    let value = retrieved_doc.get_all(glyph).into_iter().next().unwrap();
                    let slice = value.as_text().unwrap().split_terminator(",");

                    let mut domain_words_enc: Vec<EncodedGlyph> = Vec::new();

                    // Encoded each confusable found for each word
                    for s in slice {
                        let confusable_glyph_enc = EncodedGlyph::from_str(s.trim()).unwrap();
                        domain_words_enc.push(confusable_glyph_enc);
                    }
                    let encode_word = EncodedWord::new(domain_words_enc);
                    confusable_word.push(encode_word);
                }
                world_domain.append(&mut confusable_word);
            }
            sentence_domain.push(WordDomain::new(world_domain));
        }
        SentenceDomain::new(sentence_domain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::sentence::{EncodedSentence, Sentence};

    #[test]
    fn when_init_then_create_resource() {
        let mut tantivy = Tantivy::init();
        let sentence_dec = Sentence::from_str("ru best").unwrap();
        let sentence_dec: EncodedSentence = EncodedSentence::from(sentence_dec);
        tantivy.index();
        tantivy.query(sentence_dec);
        let domain: SentenceDomain = tantivy.search();
        println!("{:#?}", domain);
    }
}
