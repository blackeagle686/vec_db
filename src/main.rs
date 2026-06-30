pub mod entities;
pub mod metrics;
pub mod in_memory;
pub mod search_teq;    
pub mod hnsw;

use entities::{Engine, EngineTrait};

fn main() {
    let mut engine = Engine::new("engine1");
}
