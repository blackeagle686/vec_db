use axum::{extract::State, Json, http::StatusCode};
use crate::api::models::{CreateCollectionRequest, InsertRecordRequest, DefaultSuccessCreationResponse};
use crate::api::routers::AppState; 
use crate::domain::entities::{CollectionError, EngineTrait};

pub async fn create_collection_handler(
    State(state): State<AppState>, 
    Json(payload): Json<CreateCollectionRequest>,
) -> Result<Json<DefaultSuccessCreationResponse>, (StatusCode, String)> {
    
    // 1. We want to CREATE a collection, so we need a WRITE lock!
    // This will pause if someone else is currently writing.
    let mut engine = state.engine.write().unwrap();

    // 2. Do the database work
    let index_type_str = payload.index_type.as_deref();
    
    match engine.create_collection(&payload.collection_name, index_type_str) {
        Ok(_) => {
            Ok(Json(DefaultSuccessCreationResponse {
                success: true,
                message: format!("Collection {} created successfully", payload.collection_name),
            }))
        },
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())), 
    }
}

pub async fn get_collection_handler(
    State(state): State<AppState>, 
    Json(payload): Json<CreateCollectionRequest>,
) -> Result<Json<DefaultSuccessCreationResponse>, (StatusCode, String)> {
    
    // 1. We want to CREATE a collection, so we need a WRITE lock!
    // This will pause if someone else is currently writing.
    let mut engine = state.engine.write().unwrap();

    // 2. Do the database work
    let index_type_str = payload.index_type.as_deref();
    
    match engine.create_collection(&payload.collection_name, index_type_str) {
        Ok(_) => {
            Ok(Json(DefaultSuccessCreationResponse {
                success: true,
                message: format!("Collection {} created successfully", payload.collection_name),
            }))
        },
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())), 
    }
}



pub async fn insert_record_handler(
    State(state): State<AppState>,
    Json(payload): Json<InsertRecordRequest>, 
) -> Result<Json<DefaultSuccessCreationResponse>, (StatusCode, String)>{
    let mut engine = state.engine.write().unwrap();



    
}