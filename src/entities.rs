use std::collections::HashMap;
use thiserror::Error;


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
}

// ------------------------------ RECORD ------------------------------

#[derive(Debug, Clone)]
pub struct Record {
    pub id: String,
    pub embeddings: Vec<f32>,
    pub metadata: Option<HashMap<String, String>>,
    pub layers: Vec<Vec<String>>, 
}

impl Record {
    pub fn new(id: String, embeddings: Vec<f32>, metadata: Option<HashMap<String, String>>, max_layer: usize) -> Self {
        Record {
            id,
            embeddings,
            metadata,
            layers: vec![vec![]; max_layer + 1],
        }
    }
}

// ------------------------------ COLLECTION ------------------------------

pub struct Collection {
    pub name: String,
    pub vectors: HashMap<String, Record>,
    pub entry_point: Option<String>,
    pub max_layer: usize,
}

pub trait CollectionTrait {
    fn insert(&self, id: &str, embeddings: Vec<f32>); 
    fn get(&self, id: &str) -> Option<&Record>;
    fn delete(&mut self, id: &str) -> bool;
    fn update(&mut self, id: &str, embeddings: Vec<f32>);
}


impl Collection {
    pub fn new(name: &str) -> Self {
        Collection {
            name: name.to_string(),
            vectors: HashMap::new(),
            entry_point: None,
            max_layer: 0,
        }
    }
}

// ------------------------------ ENGINE ------------------------------

pub struct Engine {
    pub id: String,
    pub collections: HashMap<String, Collection>,  
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
    fn create_collection(&mut self, name: &str) -> Result<(), CollectionError>; 

    // Takes a reference to lookup a value
    fn get_collection(&self, name: &str) -> Result<&Collection, CollectionError>;

    // Takes a mutable reference to remove a value
    fn delete_collection(&mut self, name: &str) -> Result<(), CollectionError>;
}   

// ------------------------------ METRICS ------------------------------

pub trait DistanceMetric {
    fn calculate(a: &[f32], b: &[f32]) -> f32;
}