//! Python bindings for the Bunkatsu chunking library.

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::algorithms::{
    CodeChunker, CsvChunker, EmbeddedChunk, FixedSizeChunker, HeadingChunker, HierarchicalChunker,
    HtmlChunker, HybridChunker, JsonChunker, KamradtChunker, LateChunker, LatexChunker,
    MarkdownChunker, ParagraphChunker, PropositionChunker, RecursiveChunker, SemanticChunker,
    SentenceChunker, SlidingWindowChunker, TokenChunker,
};
use crate::chunk::Chunk;
use crate::config::{ChunkConfig, CodeLanguage, SentenceDetector};
use crate::embedding::{PyEmbeddingProvider, PyTextGenerator};
use crate::registry::AlgorithmRegistry;
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
    token: TokenChunker,
    html: HtmlChunker,
    json: JsonChunker,
    csv: CsvChunker,
    latex: LatexChunker,
    code: CodeChunker,
    hierarchical: HierarchicalChunker,
    hybrid: HybridChunker,
    registry: AlgorithmRegistry,
}

#[pymethods]
impl Chunker {
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
            token: TokenChunker,
            html: HtmlChunker,
            json: JsonChunker,
            csv: CsvChunker,
            latex: LatexChunker,
            code: CodeChunker,
            hierarchical: HierarchicalChunker,
            hybrid: HybridChunker,
            registry: AlgorithmRegistry::new(),
        }
    }

    #[pyo3(signature = (text, max_size=512))]
    pub fn chunk_fixed(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.fixed_size.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=512, overlap=64))]
    pub fn chunk_sliding(&self, text: &str, max_size: usize, overlap: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size).with_overlap(overlap);
        self.sliding_window.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=512, overlap=0, detector=SentenceDetector::Regex))]
    pub fn chunk_sentences(
        &self,
        text: &str,
        max_size: usize,
        overlap: usize,
        detector: SentenceDetector,
    ) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size)
            .with_overlap(overlap)
            .with_sentence_detector(detector);
        self.sentence.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=512, overlap=0))]
    pub fn chunk_paragraphs(&self, text: &str, max_size: usize, overlap: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size).with_overlap(overlap);
        self.paragraph.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=1000))]
    pub fn chunk_markdown(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.markdown.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=1000, levels=None))]
    pub fn chunk_headings(
        &self,
        text: &str,
        max_size: usize,
        levels: Option<Vec<usize>>,
    ) -> Vec<Chunk> {
        let mut config = ChunkConfig::new(max_size);
        if let Some(lvls) = levels {
            config = config.with_heading_levels(lvls);
        }
        self.heading.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=512, overlap=0))]
    pub fn chunk_recursive(&self, text: &str, max_size: usize, overlap: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size).with_overlap(overlap);
        self.recursive.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_tokens=512, tokenizer_fn=None))]
    pub fn chunk_tokens(
        &self,
        text: &str,
        max_tokens: usize,
        tokenizer_fn: Option<Py<PyAny>>,
        py: Python<'_>,
    ) -> Vec<Chunk> {
        self.token.chunk_with_tokenizer(text, max_tokens, |s| {
            if let Some(ref f) = tokenizer_fn {
                f.call1(py, (s,))
                    .and_then(|r| r.extract::<usize>(py))
                    .unwrap_or_else(|_| s.split_whitespace().count())
            } else {
                s.split_whitespace().count()
            }
        })
    }

    #[pyo3(signature = (text, max_size=1000))]
    pub fn chunk_html(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.html.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=1000))]
    pub fn chunk_json(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.json.chunk(text, &config)
    }

    #[pyo3(signature = (text, rows_per_chunk=50))]
    pub fn chunk_csv(&self, text: &str, rows_per_chunk: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(usize::MAX).with_rows_per_chunk(rows_per_chunk);
        self.csv.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=1000))]
    pub fn chunk_latex(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.latex.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=1000, language=CodeLanguage::Auto))]
    pub fn chunk_code(&self, text: &str, max_size: usize, language: CodeLanguage) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size).with_code_language(language);
        self.code.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=512))]
    pub fn chunk_hierarchical(&self, text: &str, max_size: usize) -> Vec<Chunk> {
        let config = ChunkConfig::new(max_size);
        self.hierarchical.chunk(text, &config)
    }

    #[pyo3(signature = (text, max_size=512, strategies=None))]
    pub fn chunk_hybrid(
        &self,
        text: &str,
        max_size: usize,
        strategies: Option<Vec<String>>,
    ) -> Vec<Chunk> {
        let mut config = ChunkConfig::new(max_size);
        if let Some(s) = strategies {
            config = config.with_hybrid_strategies(s);
        }
        self.hybrid.chunk(text, &config)
    }

    #[pyo3(signature = (text, embedding_fn, threshold=0.5))]
    pub fn chunk_semantic(
        &self,
        text: &str,
        embedding_fn: Py<PyAny>,
        threshold: f32,
        py: Python<'_>,
    ) -> Vec<Chunk> {
        let embedder = PyEmbeddingProvider::new(embedding_fn);
        let chunker = SemanticChunker::new(threshold);
        chunker.chunk_with_embedder(py, text, &embedder)
    }

    #[pyo3(signature = (text, embedding_fn, percentile=95.0))]
    pub fn chunk_kamradt(
        &self,
        text: &str,
        embedding_fn: Py<PyAny>,
        percentile: f32,
        py: Python<'_>,
    ) -> Vec<Chunk> {
        let embedder = PyEmbeddingProvider::new(embedding_fn);
        let chunker = KamradtChunker::new(percentile);
        chunker.chunk_with_embedder(py, text, &embedder)
    }

    #[pyo3(signature = (text, llm_fn))]
    pub fn chunk_proposition(&self, text: &str, llm_fn: Py<PyAny>, py: Python<'_>) -> Vec<Chunk> {
        let generator = PyTextGenerator::new(llm_fn);
        PropositionChunker.chunk_with_generator(py, text, &generator)
    }

    #[pyo3(signature = (text, embedding_fn, max_size=512))]
    pub fn chunk_late(
        &self,
        text: &str,
        embedding_fn: Py<PyAny>,
        max_size: usize,
        py: Python<'_>,
    ) -> Vec<EmbeddedChunk> {
        let embedder = PyEmbeddingProvider::new(embedding_fn);
        LateChunker.chunk_with_embedder(py, text, max_size, &embedder)
    }

    /// Chunk text using a named algorithm from the registry.
    #[pyo3(signature = (text, method, max_size=512, overlap=0))]
    pub fn chunk_by_name(
        &self,
        text: &str,
        method: &str,
        max_size: usize,
        overlap: usize,
    ) -> PyResult<Vec<Chunk>> {
        let config = ChunkConfig::new(max_size).with_overlap(overlap);
        match self.registry.get(method) {
            Some(algo) => Ok(algo.chunk(text, &config)),
            None => Err(PyValueError::new_err(format!(
                "Unknown chunking method '{}'. Available: {}",
                method,
                self.registry.list().join(", ")
            ))),
        }
    }

    pub fn available_methods(&self) -> Vec<String> {
        self.registry.list()
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new()
    }
}
