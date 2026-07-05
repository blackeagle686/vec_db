use axum::{
    routing::{
        get, post
    }, 
    Router
}; 
use std::sync::{Arc, RwLock}; 
use tokio::net::TcpListener; 
use crate::domain::entities::Engine; 
use crate::api::handler::*; 

#[derive(Clone)]
pub struct AppState{
    pub engine: Arc<RwLock<Engine>>,
}

pub struct App{
    pub app: Router,
    pub app_state: AppState,
}

impl App {
    pub fn new(engine: Engine) -> Self {
        let app_state = AppState {
            engine: Arc::new(RwLock::new(engine)),
        };
        let app = Router::new()
            .route("/collection", post(create_collection_handler))
            .route("/collection/:id", get(get_collection_handler))
            .route("/insert", post(insert_record_handler))
            .route("/query", post(query_vector_handler))
            .with_state(app_state.clone());
        Self { app, app_state }
    }
}


