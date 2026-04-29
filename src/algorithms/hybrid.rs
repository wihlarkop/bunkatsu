use crate::algorithms::{
    FixedSizeChunker, HeadingChunker, MarkdownChunker, ParagraphChunker, SentenceChunker,
};
use crate::chunk::Chunk;
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

pub struct HybridChunker;

impl HybridChunker {
    fn chunk_by_strategy(strategy: &str, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        match strategy {
            "paragraph" => ParagraphChunker.chunk(text, config),
            "sentence" => SentenceChunker.chunk(text, config),
            "fixed_size" | "fixed" => FixedSizeChunker.chunk(text, config),
            "markdown" => MarkdownChunker.chunk(text, config),
            "heading" => HeadingChunker::default().chunk(text, config),
            _ => FixedSizeChunker.chunk(text, config),
        }
    }
}

impl ChunkAlgorithm for HybridChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let strategies = &config.hybrid_strategies;

        if strategies.is_empty() {
            return FixedSizeChunker.chunk(text, config);
        }

        // Try strategies in order; re-chunk oversized results with next strategy
        let mut result = Vec::new();
        let first = &strategies[0];
        let initial = Self::chunk_by_strategy(first, text, config);

        for chunk in initial {
            if chunk.text.len() <= config.max_size {
                result.push(chunk);
            } else {
                // Try fallback strategies on the oversized chunk
                let mut resolved = false;
                for strategy in strategies.iter().skip(1) {
                    let sub = Self::chunk_by_strategy(strategy, &chunk.text, config);
                    if sub.iter().all(|c| c.text.len() <= config.max_size) || strategy == strategies.last().unwrap() {
                        result.extend(sub);
                        resolved = true;
                        break;
                    }
                }
                if !resolved {
                    result.push(chunk);
                }
            }
        }

        // Tag all chunks with "hybrid" method
        for chunk in &mut result {
            chunk.metadata.method = "hybrid".to_string();
        }

        result
    }

    fn name(&self) -> &str {
        "hybrid"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hybrid_basic() {
        let chunker = HybridChunker;
        let config = ChunkConfig::new(50);
        let text = "First paragraph.\n\nSecond paragraph that is a bit longer and may exceed limit.\n\nThird.";
        let chunks = chunker.chunk(text, &config);
        assert!(!chunks.is_empty());
        assert!(chunks.iter().all(|c| c.metadata.method == "hybrid"));
    }

    #[test]
    fn test_hybrid_empty() {
        let chunker = HybridChunker;
        let config = ChunkConfig::new(100);
        assert!(chunker.chunk("", &config).is_empty());
    }

    #[test]
    fn test_hybrid_name() {
        assert_eq!(HybridChunker.name(), "hybrid");
    }

    #[test]
    fn test_hybrid_custom_strategies() {
        let chunker = HybridChunker;
        let config = ChunkConfig::new(50).with_hybrid_strategies(vec![
            "sentence".to_string(),
            "fixed_size".to_string(),
        ]);
        let text = "Hello world. How are you? I am fine today.";
        let chunks = chunker.chunk(text, &config);
        assert!(!chunks.is_empty());
    }
}
