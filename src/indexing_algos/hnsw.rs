use crate::domain::entities::{DistanceMetric, Record, RecordError, Index};
use crate::indexing_algos::indexing::Indexing; 
use rand::Rng;
use std::sync::{Arc, RwLock};

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