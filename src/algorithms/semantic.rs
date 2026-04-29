use crate::algorithms::sentence::SentenceChunker;
use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::embedding::EmbeddingProvider;
use crate::traits::ChunkAlgorithm;
use pyo3::prelude::*;

pub struct SemanticChunker {
    threshold: f32,
}

impl SemanticChunker {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    pub fn chunk_with_embedder<'py>(
        &self,
        py: Python<'py>,
        text: &str,
        embedder: &dyn EmbeddingProvider,
    ) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let config = ChunkConfig::new(1);
        let sentences = SentenceChunker.chunk(text, &config);
        if sentences.is_empty() {
            return Vec::new();
        }
        if sentences.len() == 1 {
            let s = &sentences[0];
            return vec![Chunk::with_uuid(
                s.text.clone(),
                s.start,
                s.end,
                ChunkMetadata {
                    method: "semantic".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            )];
        }

        let texts: Vec<&str> = sentences.iter().map(|s| s.text.as_str()).collect();
        let embeddings = match embedder.embed(py, &texts) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };

        let mut break_indices: Vec<usize> = Vec::new();
        for i in 0..embeddings.len().saturating_sub(1) {
            let sim = cosine_similarity(&embeddings[i], &embeddings[i + 1]);
            if sim < self.threshold {
                break_indices.push(i + 1);
            }
        }

        let mut chunks = Vec::new();
        let mut group_start = 0usize;

        let build_chunk = |group: &[Chunk]| -> Option<Chunk> {
            if group.is_empty() {
                return None;
            }
            let text = group.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ");
            let start = group[0].start;
            let end = group[group.len() - 1].end;
            Some(Chunk::with_uuid(
                text,
                start,
                end,
                ChunkMetadata {
                    method: "semantic".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ))
        };

        for &break_at in &break_indices {
            if let Some(chunk) = build_chunk(&sentences[group_start..break_at]) {
                chunks.push(chunk);
            }
            group_start = break_at;
        }
        if let Some(chunk) = build_chunk(&sentences[group_start..]) {
            chunks.push(chunk);
        }

        chunks
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let dot: f32 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    (dot / (norm_a * norm_b)).clamp(-1.0, 1.0)
}

impl ChunkAlgorithm for SemanticChunker {
    fn chunk(&self, _text: &str, _config: &ChunkConfig) -> Vec<Chunk> {
        Vec::new()
    }

    fn name(&self) -> &str {
        "semantic"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEmbedder;
    impl EmbeddingProvider for MockEmbedder {
        fn embed<'py>(&self, _py: Python<'py>, texts: &[&str]) -> Result<Vec<Vec<f32>>, String> {
            Ok(texts.iter().map(|t| {
                let v = t.chars().next().map(|c| c as u8 as f32).unwrap_or(0.0);
                vec![v / 128.0, 1.0 - v / 128.0, 0.5]
            }).collect())
        }
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0f32, 0.0, 0.0];
        let b = vec![1.0f32, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);

        let c = vec![0.0f32, 1.0, 0.0];
        assert!(cosine_similarity(&a, &c).abs() < 1e-6);
    }

    #[test]
    fn test_semantic_name() {
        assert_eq!(SemanticChunker::new(0.5).name(), "semantic");
    }
}
