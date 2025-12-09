//! Trait definitions for chunking algorithms.

use crate::chunk::Chunk;
use crate::config::ChunkConfig;

/// Trait for implementing chunking algorithms.
pub trait ChunkAlgorithm: Send + Sync {
    /// Chunk the given text according to the algorithm's strategy.
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk>;

    /// Get the name of this algorithm.
    fn name(&self) -> &str;
}
