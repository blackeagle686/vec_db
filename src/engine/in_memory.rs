use crate::{
    domain::{entities::{Collection, CollectionError,
    CollectionTrait, Engine, EngineError, 
    EngineTrait, Record, RecordError},
    metrics::CosineDistance},
    indexing_algos::{hnsw::HnswIndex, indexing::Indexing}
};
use std::collections::HashMap;
use bincode;
use std::fs; 

impl CollectionTrait for Collection{
    fn insert(
        &mut self,
        embeddings: Vec<f32>,
        max_layer: usize,
        metadata: Option<HashMap<String, String>>
    ) -> Result<(), RecordError> {
        let id = format!("vec_{}", self.vectors.len());
        let record = Record::new(id, embeddings, metadata, max_layer);
        
        // On-the-fly Strategy Pattern!
        match self.indexing_type.to_uppercase().as_str() {
            "HNSW" => {
                HnswIndex::<CosineDistance>::insert(self, record);
            },
            _ => return Err(RecordError::IndexingTypeNotFound(self.indexing_type.clone())),
        }
        
        Ok(())
    }
    
    fn insert_batch(
        &mut self,
        records: Vec<(String, Vec<f32>, Option<HashMap<String, String>>)>
    ) -> Result<(), RecordError> {
        let mut to_insert = Vec::with_capacity(records.len());
        for (id, embeddings, metadata) in records {
            // max_layer 0 is a placeholder, HNSW will generate the correct one
            to_insert.push(Record::new(id, embeddings, metadata, 0)); 
        }

        match self.indexing_type.to_uppercase().as_str() {
            "HNSW" => {
                HnswIndex::<CosineDistance>::insert_batch(self, to_insert);
            },
            _ => return Err(RecordError::IndexingTypeNotFound(self.indexing_type.clone())),
        }
        
        Ok(())
    }
    
    fn query(&self, query_vector: Vec<f32>) -> Result<Option<(String, f32)>, RecordError>{
        let res = match self.indexing_type.to_uppercase().as_str() {
            "HNSW" => HnswIndex::<CosineDistance>::search(self, &query_vector)?,
            _ => return Err(RecordError::IndexingTypeNotFound(self.indexing_type.clone())),
        };
        Ok(res)
    }

    fn get(&self, id: &str) -> Result<&Record, RecordError>{
        self.vectors.get(*self.id_map.get(id).ok_or_else(|| RecordError::RecordNotFound(id.to_string()))?).ok_or_else(|| RecordError::RecordNotFound(id.to_string()))
    }
    fn delete(&mut self, _id: &str) -> Result<(), RecordError>{
        unimplemented!("delete is not implemented yet");
    }
    fn update(&mut self, _id: &str, _embeddings: Vec<f32>) -> Result<(), RecordError>{
        unimplemented!("update is not implemented yet");
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use rand::Rng;

    #[test]
    fn test_hnsw_benchmark() {
        println!(" Starting HNSW Vector DB benchmark...");

        let mut engine = Engine::new("benchmark_db");
        engine.create_collection("vectors", Some("HNSW")).unwrap();
        let collection = engine.get_collection_mut("vectors").unwrap();

        let num_vectors = 100_000;
        let dim = 763;
        let mut rng = rand::thread_rng();

        println!("Generating and inserting {} random vectors ({} dimensions)...", num_vectors, dim);
        
        let mut batch = Vec::with_capacity(num_vectors);
        for i in 0..num_vectors {
            let mut vector = Vec::with_capacity(dim);
            for _ in 0..dim {
                vector.push(rng.gen_range(-1.0..1.0));
            }
            batch.push((format!("vec_{}", i), vector, None));
        }
        
        let start_insert = Instant::now();
        collection.insert_batch(batch).unwrap();
        let duration_insert = start_insert.elapsed();
        println!("[+] Finished inserting {} vectors in {:?}", num_vectors, duration_insert);
        println!("   Average insert time: {:?}", duration_insert / num_vectors as u32);

        println!("\n[+] Running queries...");
        let num_queries = 100;
        let mut total_search_time = std::time::Duration::new(0, 0);

        for _ in 0..num_queries {
            let mut query = Vec::with_capacity(dim);
            for _ in 0..dim {
                query.push(rng.gen_range(-1.0..1.0));
            }

            let start_search = Instant::now();
            let _result = collection.query(query).unwrap();
            total_search_time += start_search.elapsed();
        }

        println!("[+] Executed {} queries.", num_queries);
        println!("   Average search time: {:?}", total_search_time / num_queries as u32);
    }

    #[test]
    fn test_hnsw_1m_benchmark() {
        println!(" Starting HNSW 1 Million Vector Benchmark...");

        let mut engine = Engine::new("benchmark_1m_db");
        engine.create_collection("vectors_1m", Some("HNSW")).unwrap();
        let collection = engine.get_collection_mut("vectors_1m").unwrap();

        let num_vectors = 1_000_000;
        let dim = 384; // Standard MiniLM embedding size
        let mut rng = rand::thread_rng();

        println!("Generating {} random vectors ({} dimensions) in memory...", num_vectors, dim);
        
        let mut batch = Vec::with_capacity(num_vectors);
        for i in 0..num_vectors {
            let mut vector = Vec::with_capacity(dim);
            for _ in 0..dim {
                vector.push(rng.gen_range(-1.0..1.0));
            }
            batch.push((format!("vec_{}", i), vector, None));
        }
        
        println!("Inserting batch...");
        let start_insert = Instant::now();
        collection.insert_batch(batch).unwrap();
        let duration_insert = start_insert.elapsed();
        
        println!("[+] Finished inserting 1,000,000 vectors in {:?}", duration_insert);
        println!("   Average insert time: {:?}", duration_insert / num_vectors as u32);

        println!("\n[+] Running queries on 1M dataset...");
        let num_queries = 100;
        let mut total_search_time = std::time::Duration::new(0, 0);

        for _ in 0..num_queries {
            let mut query = Vec::with_capacity(dim);
            for _ in 0..dim {
                query.push(rng.gen_range(-1.0..1.0));
            }

            let start_search = Instant::now();
            let _result = collection.query(query).unwrap();
            total_search_time += start_search.elapsed();
        }

        println!("[+] Executed {} queries.", num_queries);
        println!("   Average search time: {:?}", total_search_time / num_queries as u32);
    }
}
