use crate::domain::entities::{Collection, Record, RecordError};

pub trait Indexing {
    fn insert(collection: &mut Collection, record: Record);
    fn insert_batch(collection: &mut Collection, records: Vec<Record>);
    fn search(collection: &Collection, query: &[f32]) -> Result<Option<(String, f32)>, RecordError>;
}
