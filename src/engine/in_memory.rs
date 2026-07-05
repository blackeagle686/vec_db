use crate::{domain::{entities::{Collection, CollectionError, CollectionTrait, Engine, EngineError, EngineTrait, Record, RecordError}, metrics::CosineDistance}, indexing_algos::{hnsw::HnswIndex, indexing::Indexing}};
use std::collections::HashMap;
use bincode;
use std::fs; 

impl CollectionTrait for Collection{
    fn insert(
        &mut self, // MUST be &mut self, not mut self
        embeddings: Vec<f32>,
        max_layer: usize,
        metadata: Option<HashMap<String, String>>
    ) -> Result<(), RecordError> {
        let mut id = self.generate_vector_id();
        let record = Record::new(id.to_string(), embeddings, metadata, max_layer);
        
        // On-the-fly Strategy Pattern!
        match self.indexing_type.to_uppercase().as_str() {
            "HNSW" => {
                let mut index = HnswIndex::<CosineDistance>::new(self);
                index.insert(record);
            },
            _ => return Err(RecordError::IndexingTypeNotFound(self.indexing_type.clone())),
        }
        
        Ok(())
    }
    
    fn query(&self, query_vector: Vec<f32>) -> Result<Option<(String, f32)>, RecordError>{
        let mut index = HnswIndex::<CosineDistance>::new(&mut self);
        let res = index.search(&query_vector)?;
        Ok(res)
    }

    fn get(&self, id: &str) -> Result<&Record, RecordError>{
        unimplemented!("get is not implemented yet");
    }
    fn delete(&mut self, id: &str) -> Result<(), RecordError>{
        unimplemented!("delete is not implemented yet");
    }
    fn update(&mut self, id: &str, embeddings: Vec<f32>) -> Result<(), RecordError>{
        unimplemented!("update is not implemented yet");
    }

    fn generate_vector_id(&mut self) -> String{
        let id = self.next_id;
        self.next_id += 1;
        id.to_string()
    }
}

impl EngineTrait for Engine {
    // Creates and returns a new Engine instance
    fn new(id: &str) -> Self {
        Engine {   
            id: id.to_string(),
            collections: HashMap::new(),
            save_path: None,  
        }
    }

    // Takes ownership of self. When this block ends, Engine is dropped.
    fn destroy(self) {
        println!("Engine is being destroyed explicitly.");
        // Optional: Manual cleanup code goes here
    }   

    fn health_check(&self) -> Result<(), EngineError> {
        println!("Health check");
        Ok(())
    }

    fn create_collection(&mut self, name: &str, index_type: Option<&str>) -> Result<(), CollectionError> {
        if self.check_collection_found(name) {
            return Err(CollectionError::CollectionAlreadyExists(name.to_string()));
        }
        
        // Just pass the string into the new Collection!
        let collection = Collection::new(name, index_type);
        self.collections.insert(name.to_string(), collection);
        Ok(())
    }

    fn get_collection(&self, name: &str) -> Result<&Collection, CollectionError> {
        self.collections
        .get(name)
        .ok_or_else(|| CollectionError::CollectionNotFound(name.to_string()))
    }

    fn get_collection_mut(&mut self, name: &str) -> Result<&mut Collection, CollectionError> {
        self.collections
        .get_mut(name)
        .ok_or_else(|| CollectionError::CollectionNotFound(name.to_string()))
    }

    fn delete_collection(&mut self, name: &str) -> Result<(), CollectionError> {
        self.collections
        .remove(name)
        .map(|_| ())  // Maps the removed Collection to ()
        .ok_or_else(|| CollectionError::CollectionDeleteFailed(name.to_string()))
    }

    fn save_to_disk(&self) -> Result<(), EngineError> {
        let path = self.save_path.as_ref().ok_or_else(|| EngineError::EngineSaveFailed("Save path not set".to_string()))?;
        let bytes = bincode::serialize(&self)
        .map_err(|e| EngineError::EngineSaveFailed(e.to_string()))?;
        fs::write(path, &bytes)
        .map_err(|e| EngineError::EngineSaveFailed(e.to_string()))
    }

    fn load_from_disk(path: &str) -> Result<Self, EngineError> {
        if path.is_empty() {
            return Err(EngineError::EngineLoadFilePathNotFound("Save path not set".to_string()));
        }
        
        let bytes = fs::read(path)
        .map_err(|e| EngineError::EngineLoadFailed(e.to_string()))?;

        bincode::deserialize(&bytes)
        .map_err(|e| EngineError::EngineLoadFailed(e.to_string()))
    }
}


