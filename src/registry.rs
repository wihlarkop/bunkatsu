//! Algorithm registry for managing chunking strategies.

use crate::algorithms::{
    FixedSizeChunker, ParagraphChunker, SentenceChunker, SlidingWindowChunker,
};
use crate::traits::ChunkAlgorithm;
use std::collections::HashMap;
use std::sync::Arc;

/// Central registry for chunking algorithms.
pub struct AlgorithmRegistry {
    algorithms: HashMap<String, Arc<dyn ChunkAlgorithm>>,
}

impl Default for AlgorithmRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AlgorithmRegistry {
    /// Create a new registry with built-in algorithms.
    pub fn new() -> Self {
        let mut registry = Self {
            algorithms: HashMap::new(),
        };

        // Register built-in algorithms
        registry.register(Arc::new(FixedSizeChunker));
        registry.register(Arc::new(SlidingWindowChunker));
        registry.register(Arc::new(SentenceChunker));
        registry.register(Arc::new(ParagraphChunker));

        registry
    }

    /// Register a new algorithm.
    pub fn register(&mut self, algorithm: Arc<dyn ChunkAlgorithm>) {
        self.algorithms
            .insert(algorithm.name().to_string(), algorithm);
    }

    /// Get an algorithm by name.
    pub fn get(&self, name: &str) -> Option<Arc<dyn ChunkAlgorithm>> {
        self.algorithms.get(name).cloned()
    }

    /// List all registered algorithm names.
    pub fn list(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }
}
