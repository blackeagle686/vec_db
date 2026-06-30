use crate::entities::{Collection, Record, DistanceMetric};
use crate::search_teq::SearchAlgo; 
use std::collections::HashMap;
use std::marker::PhantomData;
use rand::Rng;

pub struct HnswIndex<M: DistanceMetric> {
    pub collection: Collection,
    _metric: PhantomData<M>, 
}

// 1. Struct-specific methods (Internal helpers)
impl<M: DistanceMetric> HnswIndex<M> {
    pub fn new(collection: Collection) -> Self {
        Self {
            collection,
            _metric: PhantomData,
        }
    }

    fn search_layer(&self, query: &[f32], start_id: &str, layer: usize) -> (String, f32) {
        let mut current = start_id.to_string();
        let mut curr_node = self.collection.vectors.get(&current).unwrap();
        let mut best_dist = M::calculate(&curr_node.embeddings, query);

        loop {
            let mut moved = false;

            for neighbor_id in &curr_node.layers[layer] {
                if let Some(neighbor) = self.collection.vectors.get(neighbor_id) {
                    let dist = M::calculate(&neighbor.embeddings, query);
                    if dist < best_dist {
                        best_dist = dist;
                        current = neighbor_id.clone();
                        moved = true;
                    }
                }
            }

            if !moved { break; }
            curr_node = self.collection.vectors.get(&current).unwrap();
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
impl<M: DistanceMetric> SearchAlgo for HnswIndex<M> {
    fn search(&self, query: &[f32]) -> Option<(String, f32)> {
        let mut current_node_id = match &self.collection.entry_point {
            Some(id) => id.clone(),
            None => return None,
        };

        for layer in (1..=self.collection.max_layer).rev() {
            let closest = self.search_layer(query, &current_node_id, layer);
            current_node_id = closest.0;
        }

        Some(self.search_layer(query, &current_node_id, 0))
    }

    fn insert(&mut self, id: String, embeddings: Vec<f32>, metadata: Option<HashMap<String, String>>) {
        let node_max_layer = self.random_layer();
        let mut new_record = Record::new(id.clone(), embeddings.clone(), metadata, node_max_layer);

        if self.collection.entry_point.is_none() {
            self.collection.entry_point = Some(id.clone());
            self.collection.max_layer = node_max_layer;
            self.collection.vectors.insert(id, new_record);
            return;
        }

        let mut current_node_id = self.collection.entry_point.as_ref().unwrap().clone();
        let mut current_layer = self.collection.max_layer;

        while current_layer > node_max_layer {
            let closest = self.search_layer(&embeddings, &current_node_id, current_layer);
            current_node_id = closest.0;
            current_layer = current_layer.saturating_sub(1);
        }

        for layer in (0..=node_max_layer).rev() {
            let closest = self.search_layer(&embeddings, &current_node_id, layer);
            let nearest_neighbor_id = closest.0;
            
            new_record.layers[layer].push(nearest_neighbor_id.clone());
            current_node_id = nearest_neighbor_id;
        }

        self.collection.vectors.insert(id.clone(), new_record);

        if node_max_layer > self.collection.max_layer {
            self.collection.max_layer = node_max_layer;
            self.collection.entry_point = Some(id);
        }
    }
}