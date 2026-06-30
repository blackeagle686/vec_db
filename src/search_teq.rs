use std::collections::HashMap;

pub trait SearchAlgo {
    // Defines how an algorithm inserts data into the index
    fn insert(
        &mut self, 
        id: String, 
        embeddings: Vec<f32>, 
        metadata: Option<HashMap<String, String>>
    );

    // Defines how an algorithm searches the index
    fn search(&self, query: &[f32]) -> Option<(String, f32)>;
}