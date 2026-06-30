use crate::domain::entities::Record;

pub trait SearchAlgo {
    fn insert(&mut self, record: Record);
    fn search(&self, query: &[f32]) -> Option<(String, f32)>;
}