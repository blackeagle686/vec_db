use crate::domain::entities::{Collection, CollectionError, Engine, EngineTrait, Record, RecordError};
use std::fs; 

pub struct StorageEngine{
    path: String,
}

impl StorageEngine{
    fn new(path: &str) -> Self {
        StorageEngine { path: path.to_string() }
    }

    fn save(&self, engine: &Engine) -> Result<(), EngineError> {
        let bytes = bincode::serialize(&engine)
        .map_err(|e| EngineError::EngineSaveFailed(e.to_string()))?;
        fs::write(&self.path, &bytes)
        .map_err(|e| EngineError::EngineSaveFailed(e.to_string()))
    }

    fn load(&self) -> Result<Engine, EngineError> {
        let bytes = fs::read(&self.path)
        .map_err(|e| EngineError::EngineLoadFailed(e.to_string()))?;
        bincode::deserialize(&bytes)
        .map_err(|e| EngineError::EngineLoadFailed(e.to_string()))
    }
}
