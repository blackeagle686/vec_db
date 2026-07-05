use crate::domain::entities::{Record, Engine, Collection};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CreateEngineRequest {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DefaultSuccessCreationResponse{
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    pub collection_name: String,
    pub index_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteCollectionRequest {
    pub collection_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCollectionRequest {
    pub collection_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCollectionRequest {
    pub collection_name: String,
    pub record: Record,
}

#[derive(Serialize, Deserialize)]
pub struct InsertRecordRequest{
    pub collection_name: String,
    pub embeddings: Vec<f32>, 
    pub max_layer: Option<unsize>, 
    pub 
}

#[derive(Serialize, Deserialize)]
pub struct CollectionSearchRequest {
    pub collection_name: String,
    pub query_vector: Vec<f32>,
    pub top_k: Option<usize>,
}