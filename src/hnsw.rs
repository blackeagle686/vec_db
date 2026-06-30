use crate::entities::{Collection, Record, DistanceMetric};
use crate::search_teq::SearchAlgo; 
use std::marker::PhantomData;
use rand::Rng;

pub struct HnswIndex<'a, M: DistanceMetric> {
    pub collection: &'a mut Collection,
    _metric: PhantomData<M>, 
}

// 1. Struct-specific methods (Internal helpers)
impl<'a, M: DistanceMetric> HnswIndex<'a, M> {
    pub fn new(collection: &'a mut Collection) -> Self {
        Self {
            collection,
            _metric: PhantomData,
        }
    }

    fn search_layer(&self, query: &[f32], start_id: usize, layer: usize) -> (usize, f32) {
        let mut current = start_id;
        let mut curr_node = &self.collection.vectors[current];
        let mut best_dist = M::calculate(&curr_node.embeddings, query);

        loop {
            let mut moved = false;

            for neighbor_id in &curr_node.layers[layer] {
                if layer >= curr_node.layers.len() { continue; }
                let neighbor = &self.collection.vectors[*neighbor_id];
                let dist = M::calculate(&neighbor.embeddings, query);
                if dist < best_dist {
                    best_dist = dist;
                    current = *neighbor_id;
                    moved = true;
                }
            }

            if !moved { break; }
            curr_node = &self.collection.vectors[current];
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
impl<'a, M: DistanceMetric> SearchAlgo for HnswIndex<'a, M> {
    fn search(&self, query: &[f32]) -> Option<(String, f32)> {
        let mut current_node_id = match &self.collection.entry_point {
            Some(id) => *id,
            None => return None,
        };

        for layer in (1..=self.collection.max_layer).rev() {
            let closest = self.search_layer(query, current_node_id, layer);
            current_node_id = closest.0;
        }

        let result = self.search_layer(query, current_node_id, 0);
        Some((self.collection.vectors[result.0].id.clone(), result.1))
    }

    fn insert(&mut self, mut record: Record) {
        let node_max_layer = self.random_layer();
        record.mapped_id = self.collection.next_id;
        self.collection.id_map.insert(record.id.clone(), record.mapped_id);
        self.collection.next_id += 1;

        if self.collection.entry_point.is_none() {
            self.collection.entry_point = Some(record.mapped_id);
            self.collection.max_layer = node_max_layer;
            self.collection.vectors.push(record);
            return;
        }

        let mut current_node_id = self.collection.entry_point.as_ref().unwrap().clone();
        let mut current_layer = self.collection.max_layer;

        while current_layer > node_max_layer {
            let closest = self.search_layer(&record.embeddings, current_node_id, current_layer);
            current_node_id = closest.0;
            current_layer = current_layer.saturating_sub(1);
        }

        for layer in (0..=node_max_layer).rev() {
            let closest = self.search_layer(&record.embeddings, current_node_id, layer);
            let nearest_neighbor_id = closest.0;
            
            record.layers[layer].push(nearest_neighbor_id);
            self.collection.vectors[nearest_neighbor_id]
                .layers[layer]
                .push(record.mapped_id);
            current_node_id = nearest_neighbor_id;
        }

        self.collection.vectors.push(record);

        if node_max_layer > self.collection.max_layer {
            self.collection.max_layer = node_max_layer;
            self.collection.entry_point = Some(self.collection.next_id - 1);
        }
    }
}