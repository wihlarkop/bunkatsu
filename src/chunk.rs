//! Core chunk data structures.

use pyo3::prelude::*;
use std::collections::HashMap;

/// Metadata associated with a chunk.
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct ChunkMetadata {
    /// The chunking method used.
    #[pyo3(get)]
    pub method: String,
    /// Section identifier (if applicable).
    #[pyo3(get)]
    pub section: Option<String>,
    /// Number of overlapping characters (for sliding window).
    #[pyo3(get)]
    pub overlap_chars: Option<usize>,
    /// Parent chunk ID (for recursive chunking).
    #[pyo3(get)]
    pub parent_chunk_id: Option<String>,
}

#[pymethods]
impl ChunkMetadata {
    /// Create a new ChunkMetadata.
    #[new]
    #[pyo3(signature = (method, section=None, overlap_chars=None, parent_chunk_id=None))]
    pub fn new(
        method: String,
        section: Option<String>,
        overlap_chars: Option<usize>,
        parent_chunk_id: Option<String>,
    ) -> Self {
        Self {
            method,
            section,
            overlap_chars,
            parent_chunk_id,
        }
    }

    /// Convert metadata to a Python dictionary.
    pub fn to_dict(&self, py: Python<'_>) -> HashMap<String, Py<PyAny>> {
        let mut map = HashMap::new();
        map.insert(
            "method".to_string(),
            self.method
                .clone()
                .into_pyobject(py)
                .unwrap()
                .into_any()
                .unbind(),
        );
        if let Some(ref section) = self.section {
            map.insert(
                "section".to_string(),
                section
                    .clone()
                    .into_pyobject(py)
                    .unwrap()
                    .into_any()
                    .unbind(),
            );
        }
        if let Some(overlap) = self.overlap_chars {
            map.insert(
                "overlap_chars".to_string(),
                overlap.into_pyobject(py).unwrap().into_any().unbind(),
            );
        }
        if let Some(ref parent_id) = self.parent_chunk_id {
            map.insert(
                "parent_chunk_id".to_string(),
                parent_id
                    .clone()
                    .into_pyobject(py)
                    .unwrap()
                    .into_any()
                    .unbind(),
            );
        }
        map
    }

    fn __repr__(&self) -> String {
        format!(
            "ChunkMetadata(method='{}', section={:?}, overlap_chars={:?}, parent_chunk_id={:?})",
            self.method, self.section, self.overlap_chars, self.parent_chunk_id
        )
    }
}

/// A text chunk with position and metadata.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Unique identifier for this chunk.
    #[pyo3(get)]
    pub id: String,
    /// The text content of this chunk.
    #[pyo3(get)]
    pub text: String,
    /// Start position (character index) in the original text.
    #[pyo3(get)]
    pub start: usize,
    /// End position (character index) in the original text.
    #[pyo3(get)]
    pub end: usize,
    /// Metadata associated with this chunk.
    #[pyo3(get)]
    pub metadata: ChunkMetadata,
}

#[pymethods]
impl Chunk {
    /// Create a new Chunk.
    #[new]
    #[pyo3(signature = (id, text, start, end, metadata))]
    pub fn new(
        id: String,
        text: String,
        start: usize,
        end: usize,
        metadata: ChunkMetadata,
    ) -> Self {
        Self {
            id,
            text,
            start,
            end,
            metadata,
        }
    }

    /// Get the length of the chunk text in characters.
    #[getter]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Check if the chunk text is empty.
    #[getter]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    fn __repr__(&self) -> String {
        let preview = if self.text.len() > 50 {
            format!("{}...", &self.text[..50])
        } else {
            self.text.clone()
        };
        format!(
            "Chunk(id='{}', text='{}', start={}, end={})",
            self.id, preview, self.start, self.end
        )
    }

    fn __len__(&self) -> usize {
        self.text.len()
    }
}

impl Chunk {
    /// Create a new chunk with auto-generated UUID.
    pub fn with_uuid(text: String, start: usize, end: usize, metadata: ChunkMetadata) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            text,
            start,
            end,
            metadata,
        }
    }
}
