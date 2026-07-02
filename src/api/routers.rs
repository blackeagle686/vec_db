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
    pub 
}


