<div align="center">
  
<h1 align="center" style="background: linear-gradient(to right, #6A0DAD, #008080); -webkit-background-clip: text; color: transparent; font-size: 3em; font-weight: bold;">
  Phoenix-Vector
</h1>


**A Multi-Threaded Vector Database Engine in Pure Rust.**

[![Rust](https://img.shields.io/badge/Built_with-Rust-6A0DAD.svg?style=for-the-badge&logo=rust)]()
[![Index](https://img.shields.io/badge/Index-HNSW-008080.svg?style=for-the-badge)]()
[![Concurrency](https://img.shields.io/badge/Concurrency-Rayon-6A0DAD.svg?style=for-the-badge)]()
[![Status](https://img.shields.io/badge/Status-Ingestion_Layer_Active-008080.svg?style=for-the-badge)]()

<hr style="border: 1px solid #6A0DAD; width: 50%;" />
</div>

##  Overview

Phoenix-Vector is a highly optimized vector database engine and AI-native ingestion platform built entirely from scratch in Rust. Designed with strict systems architecture principles, it focuses on memory efficiency, zero-cost abstractions, multi-threaded CPU saturation, and blazingly fast approximate nearest neighbor (ANN) search capabilities.

Unlike standard vector databases that only accept floating-point arrays, **Phoenix-Vector is an end-to-end platform**. It can ingest raw files (`.pdf`, `.docx`, `.xlsx`, `.txt`), automatically chunk them, embed them using neural networks (like OpenAI or local ONNX models), and build a highly concurrent HNSW graph index in seconds.

## Core Features

- **Blazing Fast HNSW Architecture**: Navigates multi-layered undirected graphs for highly optimized sub-linear $O(\log N)$ search complexity.
- **Rayon-Powered Parallel Construction**: Batch ingestion operations are heavily parallelized across all CPU cores using `rayon` and granular `parking_lot::RwLock` interior mutability.
- **End-to-End Ingestion Pipeline**: Native support for parsing, chunking, and embedding PDF, DOCX, XLSX, and TXT files directly into the graph.
- **Cache-Optimized Memory Layout**: Converts String IDs into sequential `usize` indices internally, allowing for $O(1)$ memory access during graph jumps with zero heap-allocations in the hot loop.
- **High-Concurrency API**: The engine is wrapped in an `Arc<RwLock>` and served via a lightning-fast `axum` and `tokio` network layer.
- **Durable Persistence**: Native `bincode` binary serialization via `serde` ensures the entire engine state is safely saved and loaded from disk in milliseconds.

##  System Architecture

```mermaid
%%{init: {'theme': 'dark', 'themeVariables': { 'primaryColor': '#4B0082', 'primaryBorderColor': '#008080', 'lineColor': '#008080', 'tertiaryColor': '#1A1A1A'}}}%%
graph TD
    A[Client Request] -->|Files / Text / Vectors| B[Axum API Gateway]
    B --> C(Phoenix Engine Arc RwLock)
    
    subgraph Ingestion Layer
        C --> D[File Parser]
        D -->|Text| E[Chunker]
        E -->|Chunks| F[Embedding Model]
        F -->|OpenAI / ONNX| G[f32 Vectors]
    end
    
    subgraph Core Storage
        G --> H{Collection}
        H -->|ID Mapper| I[HashMap String -> usize]
        H -->|Contiguous Memory| J[Vec Record]
    end
    
    subgraph HNSW Graph Index
        K((HnswIndex)) -.->|Rayon Parallel Linking| H
        K -.->|Zero Cost Math| L[DistanceMetrics]
    end
```

##  Benchmarks

Our custom-built, Rayon-parallelized HNSW algorithm achieves production-grade performance in pure Rust without external C++ bindings. 

**Testing Hardware Specifications:**
- **CPU**: Intel Core i5 (10th Gen)
- **RAM**: 12 GB
- **GPU**: None (Pure CPU SIMD Execution)

Tested with `ef_construction=100`, `M=16`, and `ef_search=50` (using `target-cpu=native` for SIMD auto-vectorization):

### 100K Dataset (Heavy Dimensions: 763)
- **Massive Batch Insertion**: 100,000 vectors inserted and fully graph-linked in **~2.6 minutes** (avg **1.58ms** per vector).
- **Sub-Millisecond Search**: 100 queries executed sequentially in **~448ms** (avg **4.48ms** per query) with high recall.

### 1 MILLION Dataset (Standard Dimensions: 384)
- **Extreme Scale Insertion**: 1,000,000 vectors generated in-memory and graph-linked in **~18.8 minutes** (avg **1.13ms** per vector) across CPU cores.
- **Logarithmic Search Magic**: 100 queries executed in **~327ms** (avg **3.27ms** per query). Despite the dataset being 10x larger, the search time stayed identical due to the $O(\log N)$ HNSW graph traversal!

##  License
<img src="phx-vector.png" alt="Phoenix Vector Logo" width="100%" style="margin-bottom: 20px;" />
This project is licensed under the MIT License.
