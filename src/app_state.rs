use std::sync::{Arc, RwLock};

use crate::domain::entities::Engine;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<RwLock<Engine>>,
}

impl AppState {
    pub fn new(engine: Engine) -> Self {
        Self {
            engine: Arc::new(RwLock::new(engine)),
        }
    }
}