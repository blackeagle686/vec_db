use crate::entities::{Collection, Engine, EngineTrait, EngineError, CollectionError};
use std::collections::HashMap;



impl EngineTrait for Engine {
    // Creates and returns a new Engine instance
    fn new(id: &str) -> Self {
        Engine {   
            id: id.to_string(),
            collections: HashMap::new(),
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

    fn create_collection(&mut self, name: &str) -> Result<(), CollectionError> {
        // Inserts a placeholder Collection if it doesn't exist
        if self.check_collection_found(name) {
            Err(CollectionError::CollectionAlreadyExists(name.to_string()))
        } else {
            let collection = Collection::new(name);
            self.collections.insert(name.to_string(), collection);
            Ok(())
        }
    }

    fn get_collection(&self, name: &str) -> Result<&Collection, CollectionError> {
        let collection = self.collections.get(name); // Returns Option<&Collection>
        if collection.is_none() {
            Err(CollectionError::CollectionNotFound(name.to_string()))
        } else {
            Ok(collection.unwrap())
        }
    }

    fn delete_collection(&mut self, name: &str) -> Result<(), CollectionError> {
        if !self.check_collection_found(name) {
            Err(CollectionError::CollectionNotFound(name.to_string()))  
        } else {
            let deleted = self.collections.remove(name);
            if deleted.is_none() {
                Err(CollectionError::CollectionDeleteFailed(name.to_string()))
            } else {
                Ok(())
            }
        }
    }
}


