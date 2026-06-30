<div align="center">
  
# VecDB Engine

[![Rust](https://img.shields.io/badge/Built_with-Rust-20B2AA.svg?style=for-the-badge&logo=rust)]()
[![Index](https://img.shields.io/badge/Index-HNSW-20B2AA.svg?style=for-the-badge)]()
[![Status](https://img.shields.io/badge/Status-Active_Development-20B2AA.svg?style=for-the-badge)]()

**A high-performance, structurally modular Vector Database Engine.**

</div>

<br>

## Overview

VecDB is a fast, purely in-memory vector database engine built entirely from scratch in Rust. Designed with strict systems architecture principles, it focuses on memory efficiency, zero-cost abstractions, and blazing-fast approximate nearest neighbor (ANN) search capabilities.

The engine implements a custom Hierarchical Navigable Small World (HNSW) graph algorithm to traverse highly dimensional embedding spaces efficiently, making it suitable for modern AI and machine learning workloads.

## Key Features

- **HNSW Architecture**: Navigates multi-layered graphs for highly optimized sub-linear search complexity.
- **Zero-Cost Distance Metrics**: Built around static trait dispatch (`PhantomData` and generics). Distance operations (Cosine, Euclidean, Manhattan) are perfectly inlined by the compiler, avoiding the overhead of dynamic dispatch (`Box<dyn>`).
- **Domain-Driven Design**: Clean separation of concerns across `Engine`, `Collection`, and `Record` entities.
- **Strict Error Handling**: Full integration with `thiserror` for deterministic, robust error domains (`EngineError`, `CollectionError`).

## Architecture

VecDB operates on three primary tiers:

1. **Engine**: The top-level manager responsible for lifecycle events and maintaining multiple distinct `Collections`.
2. **Collection**: A distinct namespace holding vectors of a similar embedding space. It manages the entry points and the maximum layer height for the HNSW index.
3. **Index / Metrics**: The underlying algorithms (`HnswIndex<M>`) and mathematical traits (`DistanceMetric`) that process spatial similarities. 

## Quick Start

```rust
use vec_db::entities::{Engine, EngineTrait, CollectionTrait};
use vec_db::metrics::CosineDistance;
use vec_db::hnsw::HnswIndex;

fn main() {
    // Initialize a new engine
    let mut engine = Engine::new("production_engine");
    
    // Create a new vector collection
    engine.create_collection("document_embeddings").unwrap();
    
    // Retrieve the collection
    let collection = engine.get_collection("document_embeddings").unwrap();
    
    // Example: Integrating the HNSW search algorithm (Implementation details vary based on active phase)
    // let mut index = HnswIndex::<CosineDistance>::new(collection);
    // index.insert("doc_1".to_string(), vec![0.1, 0.5, -0.2], None);
}
```

## Development Roadmap

- **Phase 1 (Complete)**: Core structs, static distance metrics, HNSW index foundation, custom error handling.
- **Phase 2 (In Progress)**: Disk persistence via `serde` and binary serialization formats to save and load the `Engine` state across restarts.
- **Phase 3**: Optimization of memory layout (reducing heap allocations for IDs), cache-line alignment, and SIMD vectorization.
- **Phase 4**: Concurrency and Network API wrapping via `tokio` and `axum`.

## License

This project is licensed under the MIT License.
