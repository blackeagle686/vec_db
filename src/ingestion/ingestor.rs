use crate::domain::entities::CollectionTrait;
use crate::ingestion::embedder::EmbeddingModel;
use crate::ingestion::parser::{parse_file, chunk_text};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Ingestor<'a, C: CollectionTrait> {
    collection: &'a mut C,
    embedder: Box<dyn EmbeddingModel>,
}

impl<'a, C: CollectionTrait> Ingestor<'a, C> {
    pub fn new(collection: &'a mut C, embedder: Box<dyn EmbeddingModel>) -> Self {
        Self { collection, embedder }
    }

    /// Ingest raw text strings directly
    pub fn ingest_raw(&mut self, texts: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if texts.is_empty() { return Ok(()); }

        let embeddings = self.embedder.embed_batch(&texts)?;
        let mut records = Vec::with_capacity(texts.len());
        
        for (i, emb) in embeddings.into_iter().enumerate() {
            let mut meta = HashMap::new();
            meta.insert("text".to_string(), texts[i].clone());
            
            let id = format!("doc_{}", Uuid::new_v4());
            records.push((id, emb, Some(meta)));
        }
        
        self.collection.insert_batch(records)?;
        Ok(())
    }

    /// Open a file, parse the text, chunk it, and ingest it
    pub fn ingest_doc(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let text = parse_file(path)?;
        
        // Chunk the document into ~500 character chunks
        let chunks = chunk_text(&text, 500);
        
        self.ingest_raw(chunks)
    }

    /// Batch ingest multiple files
    pub fn ingest_docs(&mut self, paths: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        for path in paths {
            self.ingest_doc(path)?;
        }
        Ok(())
    }
}
