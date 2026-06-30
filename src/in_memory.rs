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
}


