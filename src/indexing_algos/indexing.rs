use crate::domain::entities::Record;

pub trait Indexing {
    fn insert(&mut self, record: Record);
    fn search(&self, query: &[f32]) -> Result<Option<(String, f32)>, RecordError>;
}



