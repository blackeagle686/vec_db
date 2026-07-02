use axum::{
    Json, 
    extract::{Path, Query, State}, 
    body::Body,
};
use std::sync::{Arc, RwLock};

use crate::domain::entities::{CollectionError, Engine, EngineTrait};
use crate::api::models::*; 

pub struct EngineHandler{
    pub engine: Arc<RwLock<Engine>>
}

impl EngineHandler {
    pub fn new(engine: Engine) -> Self {
        EngineHandler { 
            engine: Arc::new(RwLock::new(engine)) 
        }   
    }

    pub async fn create_collection(
        &mut self,
        Json(payload): Json<CreateCollectionRequest>,
    )-> Result<Json<DefaultSuccessCreationResponse>, CollectionError> 
    {
        let mut engine = self.engine.write().unwrap();
        match engine.create_collection(&payload.collection_name.to_string(), Some(&payload.index_type)) {
            Ok(_) => Ok(Json(DefaultSuccessCreationResponse {
                success: true,
                message: format!("Collection with name {} created successfully", payload.collection_name),
            })),
            Err(e) => Err(e),
        }
    }
}



