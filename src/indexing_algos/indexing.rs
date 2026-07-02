use crate::domain::entities::Record;

pub trait Indexing {
    fn insert(&mut self, record: Record);
    fn search(&self, query: &[f32]) -> Option<(String, f32)>;
}



