use crate::domain::entities::{Collection, Record, RecordError, DistanceMetric};
use crate::indexing_algos::indexing::Indexing;
use std::marker::PhantomData;

// The radius step of our concentric spheres.
const SHELL_THICKNESS: f32 = 0.5;

pub struct ShellIndex<M: DistanceMetric> {
    _marker: PhantomData<M>,
}

impl<M: DistanceMetric> ShellIndex<M> {
    fn origin_distance(vec: &[f32]) -> f32 {
        let mut sum = 0.0;
        for &v in vec {
            sum += v * v;
        }
        sum.sqrt()
    }

    fn get_shell_id(vec: &[f32]) -> i32 {
        let dist = Self::origin_distance(vec);
        (dist / SHELL_THICKNESS).floor() as i32
    }
}

impl<M: DistanceMetric> Indexing for ShellIndex<M> {
    fn insert(collection: &mut Collection, mut record: Record) {
        let shell_id = Self::get_shell_id(&record.embeddings);
        let mapped_id = collection.next_id;
        
        record.mapped_id = mapped_id;
        collection.id_map.insert(record.id.clone(), mapped_id);
        
        collection.shells.entry(shell_id).or_insert_with(Vec::new).push(mapped_id);
        
        collection.vectors.push(record);
        collection.next_id += 1;
    }

    fn insert_batch(collection: &mut Collection, records: Vec<Record>) {
        for record in records {
            Self::insert(collection, record);
        }
    }

    fn search(collection: &Collection, query: &[f32]) -> Result<Option<(String, f32)>, RecordError> {
        let target_shell = Self::get_shell_id(query);
        
        let mut best_dist = f32::MAX;
        let mut best_id = None;
        
        // We search the target shell and its immediate neighbors (inner and outer shell)
        // because vectors on the boundary might be closer to a vector across the boundary.
        let shells_to_search = [target_shell - 1, target_shell, target_shell + 1];
        
        for shell_id in shells_to_search.iter() {
            if let Some(indices) = collection.shells.get(shell_id) {
                for &idx in indices {
                    let record = &collection.vectors[idx];
                    let dist = M::calculate(&record.embeddings, query);
                    if dist < best_dist {
                        best_dist = dist;
                        best_id = Some(record.id.clone());
                    }
                }
            }
        }
        
        // Fallback: If no vectors found in these shells, we just do a brute force scan on all vectors.
        if best_id.is_none() && !collection.vectors.is_empty() {
             for record in &collection.vectors {
                 let dist = M::calculate(&record.embeddings, query);
                 if dist < best_dist {
                     best_dist = dist;
                     best_id = Some(record.id.clone());
                 }
             }
        }

        Ok(best_id.map(|id| (id, best_dist)))
    }
}
