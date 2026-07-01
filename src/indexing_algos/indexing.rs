use std::collections::HashMap;

use crate::domain::entities::{Record, RecordError};
use serde::{Serialize, Deserialize};

pub trait Indexing {
    fn insert(&mut self, record: Record);
    fn search(&self, query: &[f32]) -> Option<(String, f32)>;
}

#[derive(Serialize, Deserialize)]
pub struct IndexingFactory{
    pub indexies_registry: HashMap<String, Indexing>,
}

impl IndexingFactory {
    pub fn register(&mut self, indexing_type: &str, indexing: Box<dyn Indexing>) -> Result<(), RecordError> {
        if self.indexies_registry.contains_key(indexing_type) {
            return Err(RecordError::IndexingTypeAlreadyRegistered(indexing_type.to_string()));
        }
        self.indexies_registry.insert(indexing_type.to_string(), indexing);
        Ok(())
    }

    pub fn unregister(&mut self, indexing_type: &str) -> bool {
        self.indexies_registry.remove(indexing_type).is_some()
    }

    pub fn insert(&mut self, indexing_type: &str, record: Record) -> Result<(), RecordError> {
        if !self.indexies_registry.contains_key(indexing_type) {
            return Err(RecordError::IndexingTypeNotFound(indexing_type.to_string()));
        }
        self.indexies_registry.get_mut(indexing_type).unwrap().insert(record);
        Ok(())
    }

    pub fn search(&self, indexing_type: &str, query: &[f32]) -> Result<Option<(String, f32)>, RecordError> {
        if !self.indexies_registry.contains_key(indexing_type) {
            return Err(RecordError::IndexingTypeNotFound(indexing_type.to_string()));
        }
        Ok(self.indexies_registry.get(indexing_type).unwrap().search(query))
    }

    pub fn new() -> Self {
        IndexingFactory {
            indexies_registry: HashMap::new(),
        }
    }
}

