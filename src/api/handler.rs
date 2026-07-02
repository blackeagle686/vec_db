use axum::{
    Json, 
    extract::{Path, Query, State}, 
    body::Body,
};
use std::sync::{Arc, RwLock};

use crate::domain::entities::{CollectionError, Engine, EngineTrait};
use crate::api::models::*;
use crate::api::routers::AppState;

pub struct EngineHandler{}

impl EngineHandler {
    pub async fn create_collection(
        State(state): State<AppState>,
        Json(payload): Json<CreateCollectionRequest>,
    )-> Result<Json<DefaultSuccessCreationResponse>, CollectionError> 
    {
        
        match state.engine.create_collection(&payload.collection_name.to_string(), Some(&payload.index_type)) {
            Ok(_) => Ok(Json(DefaultSuccessCreationResponse {
                success: true,
                message: format!("Collection with name {} created successfully", payload.collection_name),
            })),
            Err(e) => Err(e),
        }
    }
}

pub struct CollectionHandler{}



