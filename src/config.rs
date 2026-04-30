//! Configuration types for chunking operations.

use pyo3::prelude::*;

/// Sentence detection method.
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SentenceDetector {
    /// Fast regex-based detection (handles common cases: . ! ?)
    #[default]
    Regex,
    /// Accurate Unicode-aware segmentation
    Unicode,
}

/// Programming language hint for code-aware chunking.
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CodeLanguage {
    #[default]
    Auto,
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Go,
    Java,
    C,
    Cpp,
    CSharp,
    PHP,
    Ruby,
    Swift,
    Generic,
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
    /// Number of CSV rows per chunk.
    pub rows_per_chunk: usize,
    /// Programming language hint for code chunking.
    pub code_language: CodeLanguage,
    /// Cosine similarity threshold for semantic chunking (0.0–1.0).
    pub breakpoint_threshold: f32,
    /// Percentile for Kamradt distance threshold (0.0–100.0).
    pub percentile: f32,
    /// Heading levels to split at for heading/markdown chunking.
    pub heading_levels: Vec<usize>,
    /// Strategy names for hybrid chunking (ordered fallback list).
    pub hybrid_strategies: Vec<String>,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            max_size: 512,
            overlap: 0,
            sentence_detector: SentenceDetector::Regex,
            rows_per_chunk: 50,
            code_language: CodeLanguage::Auto,
            breakpoint_threshold: 0.5,
            percentile: 95.0,
            heading_levels: vec![],
            hybrid_strategies: vec!["paragraph".into(), "sentence".into(), "fixed_size".into()],
        }
    }
}

impl ChunkConfig {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            ..Default::default()
        }
    }

    pub fn with_overlap(mut self, overlap: usize) -> Self {
        self.overlap = overlap;
        self
    }

    pub fn with_sentence_detector(mut self, detector: SentenceDetector) -> Self {
        self.sentence_detector = detector;
        self
    }

    pub fn with_rows_per_chunk(mut self, n: usize) -> Self {
        self.rows_per_chunk = n;
        self
    }

    pub fn with_code_language(mut self, lang: CodeLanguage) -> Self {
        self.code_language = lang;
        self
    }

    pub fn with_breakpoint_threshold(mut self, t: f32) -> Self {
        self.breakpoint_threshold = t;
        self
    }

    pub fn with_percentile(mut self, p: f32) -> Self {
        self.percentile = p;
        self
    }

    pub fn with_heading_levels(mut self, levels: Vec<usize>) -> Self {
        self.heading_levels = levels;
        self
    }

    pub fn with_hybrid_strategies(mut self, strategies: Vec<String>) -> Self {
        self.hybrid_strategies = strategies;
        self
    }
}
