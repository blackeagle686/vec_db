pub mod domain;
pub mod api;
pub mod indexing_algos;
pub mod engine;
pub mod app_state;

use axum::{
    routing::{
        get, post
    }, 
    Router
}; 
use std::sync::{Arc, RwLock}; 
use tokio::net::TcpListener; 
use domain::


#[tokio::main]
async fn main() {



}
