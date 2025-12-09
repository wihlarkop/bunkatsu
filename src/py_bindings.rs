//! Python bindings for the Bunkatsu chunking library.

use pyo3::prelude::*;

use crate::algorithms::{
    FixedSizeChunker, HeadingChunker, MarkdownChunker, ParagraphChunker, RecursiveChunker,
    SentenceChunker, SlidingWindowChunker,
};
use crate::chunk::Chunk;
use crate::config::{ChunkConfig, SentenceDetector};
use crate::traits::ChunkAlgorithm;

/// Main chunker class for Python.
#[pyclass]
pub struct Chunker {
    fixed_size: FixedSizeChunker,
    sliding_window: SlidingWindowChunker,
    sentence: SentenceChunker,
    paragraph: ParagraphChunker,
    markdown: MarkdownChunker,
    heading: HeadingChunker,
    recursive: RecursiveChunker,
}

#[pymethods]
impl Chunker {
    /// Create a new Chunker instance.
    #[new]
    pub fn new() -> Self {
        Self {
            fixed_size: FixedSizeChunker,
            sliding_window: SlidingWindowChunker,
            sentence: SentenceChunker,
            paragraph: ParagraphChunker,
            markdown: MarkdownChunker,
            heading: HeadingChunker::default(),
            recursive: RecursiveChunker::default(),
        }
    }

    /// Chunk text using fixed-size character-based chunking.
    #[pyo3(signature = (text, max_size=512))]
    pub fn chunk_fixed(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.fixed_size.chunk(text, &config)
    }

    /// Chunk text using sliding window with overlap.
    #[pyo3(signature = (text, max_size=512, overlap=64))]
    pub fn chunk_sliding(&self, text: &str, max_size: usize, overlap: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size).with_overlap(overlap);
        self.sliding_window.chunk(text, &config)
    }

    /// Chunk text by sentence boundaries.
    #[pyo3(signature = (text, max_size=512, detector=SentenceDetector::Regex))]
    pub fn chunk_sentences(
        &self,
        text: &str,
        max_size: usize,
        detector: SentenceDetector,
    ) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size).with_sentence_detector(detector);
        self.sentence.chunk(text, &config)
    }

    /// Chunk text by paragraph boundaries.
    #[pyo3(signature = (text, max_size=512))]
    pub fn chunk_paragraphs(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.paragraph.chunk(text, &config)
    }

    /// Chunk markdown text preserving code blocks and splitting at headings.
    #[pyo3(signature = (text, max_size=1000))]
    pub fn chunk_markdown(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.markdown.chunk(text, &config)
    }

    /// Chunk text by heading boundaries.
    #[pyo3(signature = (text, max_size=1000))]
    pub fn chunk_headings(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.heading.chunk(text, &config)
    }

    /// Chunk text recursively using multiple strategies.
    #[pyo3(signature = (text, max_size=512))]
    pub fn chunk_recursive(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.recursive.chunk(text, &config)
    }

    /// List available chunking methods.
    pub fn available_methods(&self) -> Vec<String> {
        vec![
            "fixed_size".to_string(),
            "sliding_window".to_string(),
            "sentence".to_string(),
            "paragraph".to_string(),
            "markdown".to_string(),
            "heading".to_string(),
            "recursive".to_string(),
        ]
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new()
    }
}
