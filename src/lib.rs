//! Bunkatsu (分割) - Universal High-Performance Text Chunking Library
//!
//! Bunkatsu is a Rust-core, Python-first text chunking library designed for
//! RAG, NLP, and Document AI systems.
//!
//! # Features
//! - Multiple chunking strategies (fixed-size, sliding window, sentence, paragraph)
//! - High performance via Rust
//! - Clean Python API via PyO3
//! - No embedding coupling

use pyo3::prelude::*;

pub mod algorithms;
pub mod chunk;
pub mod config;
pub mod error;
pub mod py_bindings;
pub mod registry;
pub mod traits;

// Re-exports
pub use algorithms::{
    FixedSizeChunker, HeadingChunker, MarkdownChunker, ParagraphChunker, RecursiveChunker,
    RecursiveStrategy, SentenceChunker, SlidingWindowChunker,
};
pub use chunk::{Chunk, ChunkMetadata};
pub use config::{ChunkConfig, SentenceDetector};
pub use error::ChunkError;
pub use py_bindings::Chunker;
pub use registry::AlgorithmRegistry;
pub use traits::ChunkAlgorithm;

/// A Python module implemented in Rust.
#[pymodule]
fn _bunkatsu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Chunker>()?;
    m.add_class::<Chunk>()?;
    m.add_class::<ChunkMetadata>()?;
    m.add_class::<SentenceDetector>()?;
    Ok(())
}
