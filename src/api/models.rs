use crate::domain::entities::{Record, Engine, Collection};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CreateEngineRequest {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    pub engine_id: String,
    pub collection_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteCollectionRequest {
    pub engine_name: String,
    pub collection_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCollectionRequest {
    pub engine_name: String,
    pub collection_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCollectionRequest {
    pub engine_name: String,
    pub collection_name: String,
    pub record: Record,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionInsertRequest{
    pub engine_name: String,
    pub collection_name: String,
    pub records: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionSearchRequest {
    pub engine_name: String,
    pub collection_name: String,
    pub query_vector: Vec<f32>,
    pub top_k: Option<usize>,
}