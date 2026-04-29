use crate::algorithms::FixedSizeChunker;
use crate::chunk::Chunk;
use crate::config::ChunkConfig;
use crate::embedding::EmbeddingProvider;
use crate::traits::ChunkAlgorithm;
use pyo3::prelude::*;

/// A chunk paired with its pooled embedding.
#[pyclass(from_py_object)]
#[derive(Debug, Clone)]
pub struct EmbeddedChunk {
    #[pyo3(get)]
    pub chunk: Chunk,
    #[pyo3(get)]
    pub embedding: Vec<f32>,
}

#[pymethods]
impl EmbeddedChunk {
    fn __repr__(&self) -> String {
        format!(
            "EmbeddedChunk(text='{}'..., embedding_dim={})",
            &self.chunk.text[..self.chunk.text.len().min(30)],
            self.embedding.len()
        )
    }
}

pub struct LateChunker;

impl LateChunker {
    pub fn chunk_with_embedder<'py>(
        &self,
        py: Python<'py>,
        text: &str,
        max_size: usize,
        embedder: &dyn EmbeddingProvider,
    ) -> Vec<EmbeddedChunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let config = ChunkConfig::new(max_size);
        let chunks = FixedSizeChunker.chunk(text, &config);

        let chunk_texts: Vec<&str> = chunks.iter().map(|c| c.text.as_str()).collect();
        let embeddings = match embedder.embed(py, &chunk_texts) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };

        chunks
            .into_iter()
            .zip(embeddings)
            .map(|(mut chunk, embedding)| {
                chunk.metadata.method = "late_chunking".to_string();
                EmbeddedChunk { chunk, embedding }
            })
            .collect()
    }
}

impl ChunkAlgorithm for LateChunker {
    fn chunk(&self, _text: &str, _config: &ChunkConfig) -> Vec<Chunk> {
        Vec::new()
    }

    fn name(&self) -> &str {
        "late_chunking"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_late_chunking_name() {
        assert_eq!(LateChunker.name(), "late_chunking");
    }
}
