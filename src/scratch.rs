use crate::domain::entities::{Collection, DistanceMetric, Record, RecordError, Index};
use crate::indexing_algos::indexing::Indexing; 
use rand::Rng;
use std::sync::Arc;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::{Reverse, Ordering};

const M: usize = 16;
const M_MAX_0: usize = 32;
const EF_CONSTRUCTION: usize = 100;
const EF_SEARCH: usize = 50;

#[derive(Clone, Copy, Debug)]
struct OrderedFloat(f32);
impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for OrderedFloat {}
impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub struct HnswIndex<M: DistanceMetric> {
    pub index: Arc<Index<M>>
}

impl<M: DistanceMetric> HnswIndex<M> {
    pub fn new(index: Arc<Index<M>>) -> Self {
        Self { index }
    }

    fn search_layer(
        &self,
        collection: &Collection,
        query: &[f32],
        entry_points: &[usize],
        layer: usize,
        ef: usize,
    ) -> BinaryHeap<(OrderedFloat, usize)> {
        let mut visited = HashSet::new();
        let mut candidates = BinaryHeap::new(); // min-heap using Reverse
        let mut results = BinaryHeap::new(); // max-heap

        for &ep in entry_points {
            visited.insert(ep);
            let dist = M::calculate(&collection.vectors[ep].embeddings, query);
            candidates.push(Reverse((OrderedFloat(dist), ep)));
            results.push((OrderedFloat(dist), ep));
        }

        while let Some(Reverse((c_dist, c_id))) = candidates.pop() {
            let f_dist = results.peek().unwrap().0;

            if c_dist > f_dist {
                break;
            }

            let curr_node = &collection.vectors[c_id];
            if layer >= curr_node.layers.len() {
                continue;
            }

            for &neighbor_id in &curr_node.layers[layer] {
                if visited.insert(neighbor_id) {
                    let neighbor = &collection.vectors[neighbor_id];
                    let dist = M::calculate(&neighbor.embeddings, query);
                    let f_dist = results.peek().unwrap().0;
                    
                    if results.len() < ef || dist < f_dist.0 {
                        candidates.push(Reverse((OrderedFloat(dist), neighbor_id)));
                        results.push((OrderedFloat(dist), neighbor_id));
                        if results.len() > ef {
                            results.pop();
                        }
                    }
                }
            }
        }

        results
    }

    fn random_layer(&self) -> usize {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen_range(0.00001..1.0);
        let m_l = 1.0 / (M as f32).ln();
        (-r.ln() * m_l).floor() as usize
    }
}

impl<M: DistanceMetric> Indexing for HnswIndex<M> {
    fn search(&self, query: &[f32]) -> Result<Option<(String, f32)>, RecordError> {
        let index = &self.index;
        let collection = index.collection_ptr.read().unwrap();
        
        let mut current_node_id = match collection.entry_point {
            Some(id) => id,
            None => return Err(RecordError::RecordNotFound("No entry point found, the collection is empty.".to_string())),
        };

        let mut current_layer = collection.max_layer;
        while current_layer > 0 {
            let mut changed = true;
            while changed {
                changed = false;
                let curr_node = &collection.vectors[current_node_id];
                let mut best_dist = M::calculate(&curr_node.embeddings, query);
                
                if current_layer < curr_node.layers.len() {
                    for &neighbor_id in &curr_node.layers[current_layer] {
                        let neighbor = &collection.vectors[neighbor_id];
                        let dist = M::calculate(&neighbor.embeddings, query);
                        if dist < best_dist {
                            best_dist = dist;
                            current_node_id = neighbor_id;
                            changed = true;
                        }
                    }
                }
            }
            current_layer -= 1;
        }

        let results = self.search_layer(&collection, query, &[current_node_id], 0, EF_SEARCH);
        let closest = results.into_iter().min_by(|a, b| a.0.cmp(&b.0));
        
        match closest {
            Some((dist, id)) => Ok(Some((collection.vectors[id].id.clone(), dist.0))),
            None => Ok(None)
        }
    }

    fn insert(&mut self, mut record: Record) {
        let node_max_layer = self.random_layer();
        let index = &self.index;
        let mut collection = index.collection_ptr.write().unwrap();

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

        let mut current_node_id = collection.entry_point.unwrap();
        let mut current_layer = collection.max_layer;

        while current_layer > node_max_layer {
            let mut changed = true;
            while changed {
                changed = false;
                let curr_node = &collection.vectors[current_node_id];
                let mut best_dist = M::calculate(&curr_node.embeddings, &record.embeddings);
                
                if current_layer < curr_node.layers.len() {
                    for &neighbor_id in &curr_node.layers[current_layer] {
                        let neighbor = &collection.vectors[neighbor_id];
                        let dist = M::calculate(&neighbor.embeddings, &record.embeddings);
                        if dist < best_dist {
                            best_dist = dist;
                            current_node_id = neighbor_id;
                            changed = true;
                        }
                    }
                }
            }
            if current_layer == 0 { break; }
            current_layer -= 1;
        }

        let new_id = record.mapped_id;
        collection.vectors.push(record);
        
        let mut ep = vec![current_node_id];
        let max_layer_to_link = std::cmp::min(collection.max_layer, node_max_layer);

        for layer in (0..=max_layer_to_link).rev() {
            let results = self.search_layer(&collection, &collection.vectors[new_id].embeddings, &ep, layer, EF_CONSTRUCTION);
            
            let m_max = if layer == 0 { M_MAX_0 } else { M };
            
            let mut neighbors: Vec<(OrderedFloat, usize)> = results.into_iter().collect();
            neighbors.sort_by(|a, b| a.0.cmp(&b.0));
            
            ep = neighbors.iter().map(|&(_, id)| id).collect();

            neighbors.truncate(M);
            let neighbor_ids: Vec<usize> = neighbors.into_iter().map(|(_, id)| id).collect();

            collection.vectors[new_id].layers[layer] = neighbor_ids.clone();
            
            let mut shrink_tasks = vec![];
            for &n_id in &neighbor_ids {
                collection.vectors[n_id].layers[layer].push(new_id);
                if collection.vectors[n_id].layers[layer].len() > m_max {
                    shrink_tasks.push(n_id);
                }
            }

            for n_id in shrink_tasks {
                let n_emb = collection.vectors[n_id].embeddings.clone();
                let mut connections: Vec<(OrderedFloat, usize)> = collection.vectors[n_id].layers[layer]
                    .iter()
                    .map(|&id| {
                        let dist = M::calculate(&collection.vectors[id].embeddings, &n_emb);
                        (OrderedFloat(dist), id)
                    })
                    .collect();
                connections.sort(); 
                connections.truncate(m_max);
                collection.vectors[n_id].layers[layer] = connections.into_iter().map(|(_, id)| id).collect();
            }
        }

        if node_max_layer > collection.max_layer {
            collection.max_layer = node_max_layer;
            collection.entry_point = Some(new_id);
        }
    }
}
