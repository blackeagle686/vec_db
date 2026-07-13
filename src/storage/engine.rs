use crate::domain::entities::{Collection, CollectionError, Engine, EngineTrait, Record, RecordError};
use std::fs; 

pub struct StorageEngine{
    path: String,
}

