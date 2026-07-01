use axum::{
    Json, 
    extract::{Path, Query, State}, 
    body::Body,
};
use std::sync::{Arc, RwLock};

use crate::domain::entities::{Engine, CollectionError};
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
        match engine.create_collection(payload.id) {
            Ok(_) => Ok(Json(DefaultSuccessCreationResponse {
                success: true,
                message: format!("Collection with name {} created successfully", payload.id),
            })),
            Err(e) => Err(e),
        }
    }
}
