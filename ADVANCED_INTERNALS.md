# TachyonDB: Advanced Code & Internals Deep Dive

This document explains the highly optimized, advanced Rust patterns used under the hood in TachyonDB. It is intended for developers looking to understand the complex systems programming concepts utilized to achieve million-vector scale.

---

## 1. Zero-Allocation Hot Loops via `thread_local!`

**File**: `src/indexing_algos/hnsw.rs`
**Concept**: Thread-Local Preallocated Memory Pools

During the HNSW graph traversal (`search_layer`), the algorithm constantly needs a `visited` set and multiple `BinaryHeap` priority queues to track neighbors. Allocating these structures on the heap for *every single jump* in a million-node graph would cause massive memory fragmentation and latency.

**The Advanced Implementation:**
```rust
thread_local! {
    static SEARCH_CTX: RefCell<SearchContext> = RefCell::new(SearchContext {
        visited: FxHashSet::default(),
        candidates: BinaryHeap::with_capacity(EF_CONSTRUCTION),
        results: BinaryHeap::with_capacity(EF_CONSTRUCTION),
    });
}
```
Instead of instantiating these collections inside the function, we use Rust's `thread_local!` macro combined with `RefCell`. This gives every thread (including `rayon` worker threads) its own pre-allocated memory pool. At the end of a search, we call `.clear()` on the collections instead of dropping them, completely eliminating heap allocations in the hot path.

---

## 2. Granular Lock Striping with Interior Mutability

**File**: `src/domain/entities.rs` & `src/indexing_algos/hnsw.rs`
**Concept**: Concurrent Graph Linking using `parking_lot::RwLock`

Building the graph sequentially is too slow. But wrapping the entire `Collection` in a single `Mutex` would bottleneck the parallel CPU cores. 

**The Advanced Implementation:**
```rust
pub struct Record {
    pub id: String,
    pub mapped_id: usize,
    pub embeddings: Vec<f32>,
    pub layers: Vec<parking_lot::RwLock<Vec<usize>>>, // <-- Granular Locking
}
```
Instead of locking the `Collection`, we utilize the **Interior Mutability** design pattern. The collection itself is borrowed immutably (`&Collection`), allowing `rayon::par_iter()` to spawn thousands of threads. When a thread needs to connect a new vector to an existing neighbor, it acquires a micro-lock strictly on that specific `layer` of that specific `Record`. This "lock striping" avoids deadlocks and allows massive concurrency.

---

## 3. Static Dispatch for Zero-Cost Distance Math

**File**: `src/domain/metrics.rs` & `src/indexing_algos/indexing.rs`
**Concept**: `PhantomData` and Monomorphization

Dynamic dispatch (using `Box<dyn DistanceMetric>`) requires a vtable lookup at runtime, which destroys branch prediction and prevents the compiler from using SIMD instructions. 

**The Advanced Implementation:**
```rust
pub struct HnswIndex<M: DistanceMetric> {
    _metric: std::marker::PhantomData<M>,
}

pub trait DistanceMetric {
    fn calculate(a: &[f32], b: &[f32]) -> f32;
}
```
By using generic type constraints `HnswIndex<CosineDistance>`, Rust utilizes monomorphization. At compile-time, the compiler generates a specific, hardcoded version of the index purely for `CosineDistance`. The `calculate` function is perfectly inlined, and the LLVM backend automatically auto-vectorizes the loop into AVX-512 / SIMD CPU instructions.

---

## 4. The Edge Shrinking Algorithm

**File**: `src/indexing_algos/hnsw.rs`
**Concept**: Bounded Degree Graph Pruning

When connecting nodes in HNSW, a highly popular "hub" node can easily exceed its maximum connection limit (`M_MAX_0`). We must prune its edges to maintain the logarithmic search speed.

**The Advanced Implementation:**
```rust
let mut shrink_tasks = vec![];
for &n_id in &neighbor_ids {
    let mut n_layer = collection_ref.vectors[n_id].layers[layer].write();
    n_layer.push(new_id);
    if n_layer.len() > m_max {
        shrink_tasks.push(n_id); // Mark for pruning
    }
}
// Pruning execution...
```
During parallel insertion, threads first blindly append reverse links to neighbors. If a neighbor overflows, it is pushed into a local `shrink_tasks` buffer. After the initial linkage is done, the thread iterates through its shrink tasks, sorting the neighbor's connections by distance, and cleanly truncating them back down to `m_max`. This prevents memory bloat while preserving the Shortest Path algorithms.

---

## 5. In-Memory ZIP Streaming for DOCX

**File**: `src/ingestion/parser.rs`
**Concept**: Event-Driven XML Parsing

A `.docx` file is technically a Zip archive containing XML files. Extracting it to disk and parsing the DOM tree would be very slow and I/O heavy.

**The Advanced Implementation:**
```rust
let mut archive = ZipArchive::new(file)?;
let mut document_xml = archive.by_name("word/document.xml")?;
let mut reader = Reader::from_str(&xml_content);

loop {
    match reader.read_event() {
        Ok(Event::Text(e)) if is_in_text => {
            if let Ok(s) = std::str::from_utf8(e.as_ref()) {
                text.push_str(s);
            }
        },
        // ...
    }
}
```
We read the raw bytes of the `word/document.xml` file directly from the zipped memory buffer. We then pass it into `quick-xml`, which uses a zero-allocation, event-driven state machine. It streams through the tokens, extracting `<w:t>` (Word Text) nodes natively. This bypasses the OS filesystem completely for extreme speed.
