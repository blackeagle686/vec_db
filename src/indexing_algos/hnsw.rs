use crate::domain::entities::{Collection, DistanceMetric, Record, RecordError, Index, IndexTrait};
use crate::indexing_algos::indexing::Indexing; 
use std::marker::PhantomData;
use rand::Rng;
use std::sync::{Arc, RwLock};

/*
    each collection must have one index
    so for example: 
        let mut col_names = engine.create_collection("names", index_type="hnsw").unwrap();
        let mut col_images = engine.create_collection("images", index_type="ivfflat").unwrap();
        
        col_names.insert(record).unwrap();
        col_images.insert(record).unwrap();
        col_names.search(query).unwrap();
        col_images.search(query).unwrap();

    but the qustion is who must have the vectors inside is it the Index or the Collection
    i have a good idea on who should have the vectors 
    the Collection must have the vectors inside
    and the Index has only the indexing data (pointers)
    this will make the Indexing Operation safe memory we didn't need to copy the Collections every time to insert or search
    
    but we have a problem here we can't use the &self.collection.vectors[neighbor_id] because it will cause a re-borrowing issue
    so we need to use the indexes to get the vectors
    i think its better to use the raw vectors of the collection 
    and use the index map to get the vectors
    this way we can use the Arc<Mutex<Vec<Record>>> to share the vectors between the Index and the Collection

    we must re implement the Indexing trait to handle with the following

    1. insert a record into the index
    2. search for a record in the index
    3. delete a record from the index
    4. update a record in the index
    5. the index must able to WAL write-Ahead Log i think this will be useful we already has the indexies of the vectors but we need to save them to the disk 
    6. if the collection is empty then the entry_point must be None

    what do you think about the deployment bugs ??
    i noticed that vector dbs like chroma use sqlite2 to store data in disk 
    and wal mode to make it safe to use 
    what do you think about it 

    Wooow Qdrant don't use any sql db layer it build its own db engine
    i love that so we will build our own db engine to. 
    we must make it cover the filttering with metadata as well.
    the metadata must be stored in disk with WAL mode to make it safe to use 

    we must add caching layer
    we can use the redis to store the cache data

    we must provide way to connect embeddings models 
    handle with some thing like that example:
    engine.create_collection(
    "names", 
    embeddings_model:"name-embedding-v1" , 
    index_type: "hnsw",
    metric: "cosine",
    );

    i think we can make the embedding_model in the framework layer not now let's foucs on building the engine
    so we have those:
     - Records
     - Collections - contain many Records and one Indexing
     - Indexing - contain pointers to the Records
     - Engine - contain many Collections

     we need to refactor the Record & Collection & create Indexing entities
     and refactor the current hnsw module that only insert and search using the but stop here for a miniut
    i remember something from tensorflow when i train some image the meomry whas destroy so i used
    the tensorflow dataset to load the data in batches streams only when i need them 
    it was load the data that i need only from disk to memory and then destroy it 
    when i finished 
    so we can use something like this archticutre:
    Disk -> Memory -> Cache -> Indexing -> CPU/GPU -> API
    we will use the cache as the main data structure 
    but the collection must have the vectors inside
    the HnswIndex only point to the vectors

    

*/

pub struct HnswIndex<M: DistanceMetric> {
    pub index: Arc<RwLock<Index<M>>>
}

// 1. Struct-specific methods (Internal helpers)
impl<M: DistanceMetric> HnswIndex<M> {
    pub fn new(index: Arc<RwLock<Index<M>>>) -> Self {
        Self {
            index,
        }
    }

    fn search_layer(&self, query: &[f32], start_id: usize, layer: usize) -> (usize, f32) {
        let index = self.index.read().unwrap();
        let collection = index.collection_ptr.read().unwrap();
        let mut current = start_id;
        let mut curr_node = &collection.vectors[current];
        let mut best_dist = M::calculate(&curr_node.embeddings, query);

        loop {
            let mut moved = false;

            for neighbor_id in &curr_node.layers[layer] {
                if layer >= curr_node.layers.len() { continue; }
                let neighbor = &collection.vectors[*neighbor_id];
                let dist = M::calculate(&neighbor.embeddings, query);
                if dist < best_dist {   
                    best_dist = dist;
                    current = *neighbor_id;
                    moved = true;
                }
            }

            if !moved { break; }
            curr_node = &collection.vectors[current];
        }

        (current, best_dist)
    }

    fn random_layer(&self) -> usize {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen_range(0.00001..1.0);
        (-r.ln() * 0.5).floor() as usize
    }
}

// 2. Trait Implementation (The Public API)
impl<M: DistanceMetric> Indexing for HnswIndex<M> {
    fn search(&self, query: &[f32]) -> Result<Option<(String, f32)>, RecordError> {
        let index = self.index.read().unwrap();
        let collection = index.collection_ptr.read().unwrap();
        let mut current_node_id = match &collection.entry_point {
            Some(id) => *id,
            None => return Err(RecordError::RecordNotFound("No entry point found, the collection is empty.".to_string())), // No entry point, so no vectors
        };

        for layer in (1..=collection.max_layer).rev() {
            let closest = self.search_layer(query, current_node_id, layer);
            current_node_id = closest.0;
        }

        let result = self.search_layer(query, current_node_id, 0);
        Ok(Some((collection.vectors[result.0].id.clone(), result.1)))
    }

    fn insert(&mut self, mut record: Record) {
        let node_max_layer = self.random_layer();
        let mut index = self.index.write().unwrap();
        let mut collection = index.collection_ptr.write().unwrap();

        // CRITICAL: Expand the layers array if node_max_layer > original max_layer
        if record.layers.len() <= node_max_layer {
            record.layers.resize(node_max_layer + 1, vec![]);
        }
        
        record.mapped_id = collection.next_id;
        collection.id_map.insert(record.id.clone(), record.mapped_id);
        collection.next_id += 1;

        if collection.entry_point.is_none() {
            collection.entry_point = Some(record.mapped_id);
            collection.max_layer = node_max_layer;
            collection.vectors.push(record);
            return;
        }

        let mut current_node_id = collection.entry_point.as_ref().unwrap().clone();
        let mut current_layer = collection.max_layer;

        while current_layer > node_max_layer {
            let closest = self.search_layer(&record.embeddings, current_node_id, current_layer);
            current_node_id = closest.0;
            current_layer = current_layer.saturating_sub(1);
        }

        for layer in (0..=node_max_layer).rev() {
            let closest = self.search_layer(&record.embeddings, current_node_id, layer);
            let nearest_neighbor_id = closest.0;
            
            record.layers[layer].push(nearest_neighbor_id);
            collection.vectors[nearest_neighbor_id]
                .layers[layer]
                .push(record.mapped_id);
            current_node_id = nearest_neighbor_id;
        }

        collection.vectors.push(record);

        if node_max_layer > collection.max_layer {
            collection.max_layer = node_max_layer;
            collection.entry_point = Some(collection.next_id - 1);
        }
    }
}