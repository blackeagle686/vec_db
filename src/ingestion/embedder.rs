pub trait EmbeddingModel {
    /// Takes raw text documents and returns their floating point embeddings
    fn embed_batch(&self, documents: &[String]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>>;
    
    fn embed(&self, document: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let mut results = self.embed_batch(&[document.to_string()])?;
        Ok(results.pop().unwrap())
    }
}

pub struct OpenAiEmbedder {
    pub api_key: String,
    pub model: String,
}

impl EmbeddingModel for OpenAiEmbedder {
    fn embed_batch(&self, _documents: &[String]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        // TODO: Implement actual reqwest call to https://api.openai.com/v1/embeddings
        unimplemented!("OpenAI API integration coming soon")
    }
}

pub struct OllamaEmbedder {
    pub url: String,
    pub model: String,
}

impl EmbeddingModel for OllamaEmbedder {
    fn embed_batch(&self, _documents: &[String]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        // TODO: Implement actual reqwest call to http://localhost:11434/api/embeddings
        unimplemented!("Ollama API integration coming soon")
    }
}

pub struct LocalOnnxEmbedder {
    // Will hold the fastembed-rs model instance
}

impl EmbeddingModel for LocalOnnxEmbedder {
    fn embed_batch(&self, _documents: &[String]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        // TODO: Implement fastembed-rs local model call
        unimplemented!("Local ONNX model execution coming soon")
    }
}
