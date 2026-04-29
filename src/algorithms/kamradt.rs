use crate::algorithms::semantic::cosine_similarity;
use crate::algorithms::sentence::SentenceChunker;
use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::embedding::EmbeddingProvider;
use crate::traits::ChunkAlgorithm;
use pyo3::prelude::*;

pub struct KamradtChunker {
    percentile: f32,
}

impl KamradtChunker {
    pub fn new(percentile: f32) -> Self {
        Self { percentile: percentile.clamp(0.0, 100.0) }
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
        if sentences.len() <= 1 {
            return sentences;
        }

        let texts: Vec<&str> = sentences.iter().map(|s| s.text.as_str()).collect();
        let embeddings = match embedder.embed(py, &texts) {
            Ok(e) => e,
            Err(_) => return Vec::new(),
        };

        let distances: Vec<f32> = embeddings
            .windows(2)
            .map(|w| 1.0 - cosine_similarity(&w[0], &w[1]))
            .collect();

        let threshold = percentile_val(&distances, self.percentile);

        let mut chunks = Vec::new();
        let mut group_start = 0usize;

        for (i, &dist) in distances.iter().enumerate() {
            if dist >= threshold {
                let group = &sentences[group_start..=i];
                let text = group.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ");
                let start = group[0].start;
                let end = group[group.len() - 1].end;
                chunks.push(Chunk::with_uuid(text, start, end, ChunkMetadata {
                    method: "kamradt".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                }));
                group_start = i + 1;
            }
        }

        if group_start < sentences.len() {
            let group = &sentences[group_start..];
            let text = group.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(" ");
            let start = group[0].start;
            let end = group[group.len() - 1].end;
            chunks.push(Chunk::with_uuid(text, start, end, ChunkMetadata {
                method: "kamradt".to_string(),
                section: None,
                overlap_chars: None,
                parent_chunk_id: None,
            }));
        }

        chunks
    }
}

pub fn percentile_val(values: &[f32], p: f32) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = ((p / 100.0) * (sorted.len() - 1) as f32).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

impl ChunkAlgorithm for KamradtChunker {
    fn chunk(&self, _text: &str, _config: &ChunkConfig) -> Vec<Chunk> {
        Vec::new()
    }

    fn name(&self) -> &str {
        "kamradt"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEmbedder;
    impl EmbeddingProvider for MockEmbedder {
        fn embed<'py>(&self, _py: Python<'py>, texts: &[&str]) -> Result<Vec<Vec<f32>>, String> {
            Ok(texts.iter().enumerate().map(|(i, _)| {
                if i % 2 == 0 { vec![1.0, 0.0] } else { vec![0.0, 1.0] }
            }).collect())
        }
    }

    #[test]
    fn test_percentile_calculation() {
        let distances = vec![0.1f32, 0.2, 0.8, 0.9, 0.5];
        let p95 = percentile_val(&distances, 95.0);
        assert!(p95 > 0.8);
    }

    #[test]
    fn test_kamradt_name() {
        assert_eq!(KamradtChunker::new(95.0).name(), "kamradt");
    }
}
