use std::collections::HashMap;
use thiserror::Error;
use serde::{Serialize, Deserialize};

use crate::indexing_algos::indexing::{self, Indexing, IndexingFactory};

#[derive(Error, Debug)]
pub enum CollectionError{
    #[error("Collection with name {0} already exists")]
    CollectionAlreadyExists(String),

    #[error("Collection with name {0} not found")]
    CollectionNotFound(String),

    #[error("Failed to delete collection with name {0}")]
    CollectionDeleteFailed(String),

    #[error("Failed to get collection with name {0}")]
    CollectionGetFailed(String),

    #[error("Failed to update collection with name {0}")]
    CollectionUpdateFailed(String),
}

#[derive(Error, Debug)]
pub enum RecordError{
    #[error("Record with id {0} already exists")]
    RecordAlreadyExists(String),

    #[error("Record with id {0} not found")]
    RecordNotFound(String),

    #[error("Failed to delete record with id {0}")]
    RecordDeleteFailed(String),

    #[error("Failed to get record with id {0}")]
    RecordGetFailed(String),

    #[error("Failed to update record with id {0}")]
    RecordUpdateFailed(String),

    #[error("Indexing type {0} not found")]
    IndexingTypeNotFound(String),
    
    #[error("Indexing type {0} already registered")]
    IndexingTypeAlreadyRegistered(String),
}

#[derive(Error, Debug)]
pub enum EngineError{
    #[error("Engine with id {0} already exists")]
    EngineAlreadyExists(String),

    #[error("Engine with id {0} not found")]
    EngineNotFound(String),

    #[error("Failed to delete engine with id {0}")]
    EngineDeleteFailed(String),

    #[error("Failed to get engine with id {0}")]
    EngineGetFailed(String),

    #[error("Failed to update engine with id {0}")]
    EngineUpdateFailed(String),

    #[error("Failed to save engine to disk")]
    EngineSaveFailed(String),

    #[error("Failed to load engine from disk")]
    EngineLoadFailed(String),

    #[error("Save Path Not Found to load from")]
    EngineLoadFilePathNotFound(String),
}

// ------------------------------ RECORD ------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub mapped_id: usize, 
    pub embeddings: Vec<f32>,
    pub metadata: Option<HashMap<String, String>>,
    pub layers: Vec<Vec<usize>>, 
}

impl Record {
    pub fn new(id: String, embeddings: Vec<f32>, metadata: Option<HashMap<String, String>>, max_layer: usize) -> Self {
        Record {
            id,
            embeddings,
            mapped_id: 0, // Placeholder, will be updated when inserted into collection
            metadata,
            layers: vec![vec![]; max_layer + 1],
        }
    }
}

// ------------------------------ COLLECTION ------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    pub vectors: Vec<Record>,
    pub id_map: HashMap<String, usize>,
    pub entry_point: Option<usize>,
    pub max_layer: usize,
    pub next_id: usize,
    pub indexing_type: String,
}

pub trait CollectionTrait {
    fn insert(&mut self, embeddings: Vec<f32>, max_layer: usize, metadata: Option<HashMap<String, String>>) -> Result<(), RecordError>; 
    fn get(&self, id: &str) -> Result<&Record, RecordError>;
    fn delete(&mut self, id: &str) -> Result<(), RecordError>;
    fn update(&mut self, id: &str, embeddings: Vec<f32>) -> Result<(), RecordError>;
}


impl Collection {
    pub fn new(name: &str, indexing_type: Option<&str> ) -> Self {
        let indexing_type = indexing_type.unwrap_or("HNSW");
        Collection {
            name: name.to_string(),
            vectors: Vec::new(),
            id_map: HashMap::new(),
            entry_point: None,
            max_layer: 0,
            next_id: 0,
            indexing_type: indexing_type.to_string(),
        }
    }
}

// ------------------------------ ENGINE ------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engine {
    pub id: String,
    pub collections: HashMap<String, Collection>,  
    pub save_path: Option<String>,
    pub indexing_factory: IndexingFactory,
}   

impl Engine {
    pub fn check_collection_found(&self, name: &str) -> bool {
        self.collections.contains_key(name)
    }
}

pub trait EngineTrait {
    // Associated function (Constructor)
    fn new(id: &str) -> Self;
    
    // Consumes 'self', completely destroying the object
    fn destroy(self); 

    // Takes a reference to read state
    fn health_check(&self) -> Result<(), EngineError>; 

    // Takes a mutable reference to change the HashMap
    fn create_collection(&mut self, name: &str, indexing_type: Option<&str>) -> Result<(), CollectionError>; 

    // Takes a reference to lookup a value
    fn get_collection(&self, name: &str) -> Result<&Collection, CollectionError>;

    // Takes a mutable reference to lookup a mutable value
    fn get_collection_mut(&mut self, name: &str) -> Result<&mut Collection, CollectionError>;

    // Takes a mutable reference to remove a value
    fn delete_collection(&mut self, name: &str) -> Result<(), CollectionError>;

    fn save_to_disk(&self) -> Result<(), EngineError>;

    fn load_from_disk(path: &str) -> Result<Self, EngineError> where Self: Sized;

}   

// ------------------------------ METRICS ------------------------------

pub trait DistanceMetric {
    fn calculate(a: &[f32], b: &[f32]) -> f32;
}

// ------------------------------ INDEXING ------------------------------

pub struct Indexing{
    
} 
