//! Configuration types for chunking operations.

use pyo3::prelude::*;

/// Sentence detection method.
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SentenceDetector {
    /// Fast regex-based detection (handles common cases: . ! ?)
    #[default]
    Regex,
    /// Accurate Unicode-aware segmentation
    Unicode,
}

/// Configuration for chunking operations.
#[derive(Debug, Clone)]
pub struct ChunkConfig {
    /// Maximum size of each chunk in characters.
    pub max_size: usize,
    /// Number of overlapping characters between chunks (for sliding window).
    pub overlap: usize,
    /// Sentence detection method.
    pub sentence_detector: SentenceDetector,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            max_size: 512,
            overlap: 0,
            sentence_detector: SentenceDetector::Regex,
        }
    }
}

impl ChunkConfig {
    /// Create a new configuration with the specified max size.
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            ..Default::default()
        }
    }

    /// Set the overlap for sliding window chunking.
    pub fn with_overlap(mut self, overlap: usize) -> Self {
        self.overlap = overlap;
        self
    }

    /// Set the sentence detector method.
    pub fn with_sentence_detector(mut self, detector: SentenceDetector) -> Self {
        self.sentence_detector = detector;
        self
    }
}
