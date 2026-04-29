//! Algorithm registry for managing chunking strategies.

use crate::algorithms::{
    CodeChunker, CsvChunker, FixedSizeChunker, HeadingChunker, HierarchicalChunker, HtmlChunker,
    HybridChunker, JsonChunker, LatexChunker, MarkdownChunker, ParagraphChunker, RecursiveChunker,
    SentenceChunker, SlidingWindowChunker, TokenChunker,
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
    pub fn new() -> Self {
        let mut registry = Self { algorithms: HashMap::new() };

        registry.register(Arc::new(FixedSizeChunker));
        registry.register(Arc::new(SlidingWindowChunker));
        registry.register(Arc::new(SentenceChunker));
        registry.register(Arc::new(ParagraphChunker));
        registry.register(Arc::new(MarkdownChunker));
        registry.register(Arc::new(HeadingChunker::default()));
        registry.register(Arc::new(RecursiveChunker::default()));
        registry.register(Arc::new(TokenChunker));
        registry.register(Arc::new(HtmlChunker));
        registry.register(Arc::new(JsonChunker));
        registry.register(Arc::new(CsvChunker));
        registry.register(Arc::new(LatexChunker));
        registry.register(Arc::new(CodeChunker));
        registry.register(Arc::new(HierarchicalChunker));
        registry.register(Arc::new(HybridChunker));

        registry
    }

    pub fn register(&mut self, algorithm: Arc<dyn ChunkAlgorithm>) {
        self.algorithms.insert(algorithm.name().to_string(), algorithm);
    }

    pub fn get(&self, name: &str) -> Option<Arc<dyn ChunkAlgorithm>> {
        self.algorithms.get(name).cloned()
    }

    pub fn list(&self) -> Vec<String> {
        let mut names: Vec<String> = self.algorithms.keys().cloned().collect();
        names.sort();
        names
    }
}
