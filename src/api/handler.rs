use axum::{extract::State, Json, http::StatusCode};
use crate::api::models::*;
use crate::api::routers::AppState; 
use crate::domain::entities::{EngineTrait, CollectionTrait, };
use std::collections::HashMap;

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
    Json(payload): Json<GetCollectionRequest>,
) -> Result<Json<DefaultSuccessCreationResponse>, (StatusCode, String)> {
    
    // 1. We want to CREATE a collection, so we need a WRITE lock!
    // This will pause if someone else is currently writing.
    let mut engine = state.engine.write().unwrap();
    
    match engine.get_collection(&payload.collection_name) {
        Ok(collection) => {  
            Ok(Json(DefaultSuccessCreationResponse {
                success: true,
                message: format!("Collection {} retrieved successfully", payload.collection_name),
            }))
        },
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())), 
    }
}



pub async fn insert_record_handler(
    State(state): State<AppState>,
    Json(payload): Json<InsertRecordRequest>, 
) -> Result<Json<DefaultSuccessCreationResponse>, (StatusCode, String)>{
    
    // Get the current engine form the app state
    let mut engine = state.engine.write().unwrap();

    // get the collection by name
    let mut collection = engine.get_collection_mut(&payload.collection_name).unwrap(); // unwrap will return the collection if it exists, or panic if it doesn't

    // get the max layer and metadata from the request
    let max_layer = payload.max_layer.unwrap_or(0);
    let metadata = payload.metadata.unwrap_or(HashMap::new());

    // insert the record into the collection
    collection.insert(payload.embeddings, max_layer, Some(metadata)).unwrap(); 
    Ok(Json(DefaultSuccessCreationResponse {
        success: true,
        message: format!("Record inserted successfully"),
    }))
}

pub async fn query_vector_handler(
    State(state): State<AppState>, 
    Json(payload): Json<CollectionQueryRequest>
) -> Result<Json<CollectionQueryResponse>, (StatusCode, String) >{
    let mut engine = state.engine.write().unwrap();
    let collection = engine.get_collection_mut(&payload.collection_name).unwrap();
    let res = collection.query(payload.query_vector).unwrap();
    Ok(Json(CollectionQueryResponse {
        id: res.unwrap().0,
        distance: *res.unwrap().1,
        record: Some(collection.get(&res.unwrap().0).unwrap().clone()), 
    }))
}