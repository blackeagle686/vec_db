pub mod domain;
pub mod api;
pub mod indexing_algos;
pub mod engine;
pub mod app_state;

use tokio::net::TcpListener;
use crate::domain::entities::{Engine, EngineTrait};
use crate::api::routers::App;

#[tokio::main]
async fn main() {
    // 1. Initialize the Engine
    let engine = Engine::new("production_db");
    
    // 2. Build the App (which sets up the Router and shared state)
    let app = App::new(engine);

    // 3. Bind to port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("[+] VecDB Server running on http://localhost:3000");
    
    // 4. Start serving requests!
    axum::serve(listener, app.app).await.unwrap();
}
