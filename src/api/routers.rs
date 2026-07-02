use axum::{
    routing::{
        get, post
    }, 
    Router
}; 
use std::sync::{Arc, RwLock}; 
use tokio::net::TcpListener; 
use crate::domain::entities::Engine; 

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
            .route("/collection", post(create_collection))
            .with_state(app_state);
        Self { app, app_state }
    }
}


