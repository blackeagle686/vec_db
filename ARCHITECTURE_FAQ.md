# TachyonDB: Architecture & Engineering FAQ

This document serves as an architectural deep-dive into the engineering decisions, performance optimizations, and libraries that power TachyonDB.

## 1. Core Architecture Decisions

### Q: Why build the engine in pure Rust?
**A:** Vector databases are extremely computationally heavy and memory-bound. Rust provides:
- **Zero-Cost Abstractions**: High-level ergonomics (like traits and closures) without runtime overhead.
- **Predictable Performance**: No Garbage Collector means no unexpected latency spikes during massive vector queries.
- **Fearless Concurrency**: The compiler guarantees thread safety, allowing us to saturate all CPU cores during graph construction without fear of data races.

### Q: Why use HNSW (Hierarchical Navigable Small World) instead of IVF or Flat Search?
**A:** Flat search ($O(N)$) is too slow for millions of records. IVF (Inverted File Index) requires periodic re-training of clusters. HNSW provides sub-linear $O(\log N)$ search speeds, allows incremental inserts without retraining, and provides the best balance of speed and recall for high-dimensional data.

### Q: Why map user `String` IDs to sequential `usize` integers internally?
**A:** This is our biggest optimization. In early versions, traversing the graph required hashing and cloning strings for every jump. By mapping `String -> usize` on ingestion, we store all vectors in a contiguous memory block (`Vec<Record>`). 
Graph jumps become $O(1)$ pointer math, maximizing CPU cache-locality and guaranteeing **zero heap-allocations** during the search hot-loop.

---

## 2. Concurrency & Performance Libraries

### Q: Why use `rayon` for Parallel Construction?
**A:** `rayon` is Rust's premier data-parallelism library. Instead of manually managing thread pools, `rayon` uses highly optimized work-stealing algorithms. It allows us to turn sequential `iter()` loops into `par_iter()` loops, instantly scaling our massive batch insertions across all available CPU cores.

### Q: Why use `parking_lot::RwLock` instead of the standard library `std::sync::RwLock`?
**A:** The standard library locks carry OS-level overhead and "poisoning" state tracking (which adds memory overhead). `parking_lot` locks are smaller, significantly faster under high contention, and avoid poisoning. Since we use thousands of granular locks (one on every single node's neighbor list), the memory and speed savings are massive.

### Q: Why use `rustc-hash` (`FxHashMap` / `FxHashSet`)?
**A:** Rust's default `HashSet` uses SipHash, which is cryptographically secure but computationally slow. During graph traversal, we need to track thousands of "visited" nodes per query. We switched to `rustc-hash` (used by the Rust compiler itself) which uses the lightning-fast FxHash algorithm, drastically reducing CPU cycles spent on hashing.

---

## 3. Network & Persistence Layers

### Q: Why use `axum` and `tokio` for the API?
**A:** `tokio` is the industry standard asynchronous runtime for Rust. `axum` is a web framework built directly by the `tokio` team on top of `hyper`. It provides zero-cost routing, excellent ergonomics, and handles massive concurrent network requests without blocking the underlying CPU threads.

### Q: How is data saved to disk, and why use `serde` + `bincode`?
**A:** We use `serde` to automatically derive serialization logic for our massive graph structures. Instead of saving data as JSON (which is bloated and slow to parse), we use `bincode`. Bincode encodes the structs into a highly compact binary format natively understood by Rust. This allows us to dump and load gigabytes of graph state to/from the disk in mere seconds.

---

## 4. The Ingestion Layer

### Q: How does TachyonDB natively read files without Python?
**A:** We built a decoupled parser module using specialized Rust crates:
- **`pdf-extract`**: Reads the binary structure of PDFs to pull out raw text strings.
- **`zip` + `quick-xml`**: A `.docx` file is actually just a zipped folder of XML files. We unzip it in memory and use `quick-xml` to stream through `word/document.xml`, extracting pure text lightning-fast.
- **`calamine`**: The absolute gold-standard Rust crate for Excel files. It iterates over sheets and rows to extract structured string representations of `.xlsx` data.

### Q: How are Embedding Models handled?
**A:** We utilize a Trait-based `EmbeddingModel` architecture. This allows the core engine to remain agnostic to the neural network being used. You can plug in a `reqwest` API connector (for OpenAI/Ollama) or use the `fastembed` crate to run local ONNX quantization models directly in Rust's memory space.
