use crate::domain::entities::DistanceMetric;

// ------------------------------ Cosine Distance ------------------------------

pub struct CosineDistance;

impl DistanceMetric for CosineDistance {
    #[inline(always)]
    fn calculate(a: &[f32], b: &[f32]) -> f32 {
        let len = a.len().min(b.len());
        let a = &a[..len];
        let b = &b[..len];

        let mut dot = 0.0;
        let mut norm_a_sq = 0.0;
        let mut norm_b_sq = 0.0;
        
        for i in 0..len {
            let x = a[i];
            let y = b[i];
            dot += x * y;
            norm_a_sq += x * x;
            norm_b_sq += y * y;
        }
        
        if norm_a_sq == 0.0 || norm_b_sq == 0.0 { return 1.0; }
        1.0 - (dot / (norm_a_sq.sqrt() * norm_b_sq.sqrt()))
    }
}

// ------------------------------ Euclidean Distance ------------------------------

pub struct EuclideanDistance;

impl DistanceMetric for EuclideanDistance {
    #[inline(always)]
    fn calculate(a: &[f32], b: &[f32]) -> f32 {
        let len = a.len().min(b.len());
        let a = &a[..len];
        let b = &b[..len];
        
        let mut sum = 0.0;
        for i in 0..len {
            let diff = a[i] - b[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }
}

// ------------------------------ Manhattan Distance ------------------------------

pub struct ManhattanDistance;

impl DistanceMetric for ManhattanDistance {
    fn calculate(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum::<f32>()
    }
}