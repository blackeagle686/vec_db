use crate::domain::entities::{Collection, CollectionError, Engine, EngineError, EngineTrait, Record, RecordError};
use std::fs; 

pub struct StorageEngine{
    path: String,
}

impl StorageEngine{
    pub fn new(path: &str) -> Self {
        StorageEngine { path: path.to_string() }
    }

    pub fn save(&self, engine: &Engine) -> Result<(), EngineError> {
        let bytes = bincode::serialize(&engine)
        .map_err(|e| EngineError::EngineSaveFailed(e.to_string()))?;
        fs::write(&self.path, &bytes)
        .map_err(|e| EngineError::EngineSaveFailed(e.to_string()))
    }

    pub fn load(&self) -> Result<Engine, EngineError> {
        let bytes = fs::read(&self.path)
        .map_err(|e| EngineError::EngineLoadFailed(e.to_string()))?;
        bincode::deserialize(&bytes)
        .map_err(|e| EngineError::EngineLoadFailed(e.to_string()))
    }
}
