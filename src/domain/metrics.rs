use crate::domain::entities::DistanceMetric;

// ------------------------------ Cosine Distance ------------------------------

pub struct CosineDistance;

impl DistanceMetric for CosineDistance {
    fn calculate(a: &[f32], b: &[f32]) -> f32 {
        let dot_product: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 { return 1.0; }
        1.0 - (dot_product / (norm_a * norm_b))
    }
}

// ------------------------------ Euclidean Distance ------------------------------

pub struct EuclideanDistance;

impl DistanceMetric for EuclideanDistance {
    fn calculate(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f32>().sqrt()
    }
}

// ------------------------------ Manhattan Distance ------------------------------

pub struct ManhattanDistance;

impl DistanceMetric for ManhattanDistance {
    fn calculate(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum::<f32>()
    }
}