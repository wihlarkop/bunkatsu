//! Recursive chunking algorithm.
//!
//! Multi-level chunking with fallback strategies:
//! 1. Try paragraph boundaries
//! 2. Fall back to sentence boundaries
//! 3. Fall back to fixed-size

use crate::algorithms::{FixedSizeChunker, ParagraphChunker, SentenceChunker};
use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

/// Strategy for recursive chunking.
#[derive(Debug, Clone, Copy, Default)]
pub enum RecursiveStrategy {
    /// Try paragraph first, then sentence, then fixed
    #[default]
    ParagraphFirst,
    /// Try sentence first, then fixed
    SentenceFirst,
}

/// Recursive chunker that applies multiple strategies.
pub struct RecursiveChunker {
    strategy: RecursiveStrategy,
    paragraph_chunker: ParagraphChunker,
    sentence_chunker: SentenceChunker,
    fixed_chunker: FixedSizeChunker,
}

impl Default for RecursiveChunker {
    fn default() -> Self {
        Self::new(RecursiveStrategy::ParagraphFirst)
    }
}

impl RecursiveChunker {
    /// Create a new RecursiveChunker with the specified strategy.
    pub fn new(strategy: RecursiveStrategy) -> Self {
        Self {
            strategy,
            paragraph_chunker: ParagraphChunker,
            sentence_chunker: SentenceChunker,
            fixed_chunker: FixedSizeChunker,
        }
    }

    /// Recursively chunk a piece of text that exceeds max_size.
    fn chunk_recursive(
        &self,
        text: &str,
        config: &ChunkConfig,
        parent_id: Option<String>,
        level: usize,
    ) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        // If text fits, return as single chunk
        if text.len() <= config.max_size {
            let metadata = ChunkMetadata {
                method: format!("recursive_l{}", level),
                section: None,
                overlap_chars: None,
                parent_chunk_id: parent_id,
            };
            return vec![Chunk::with_uuid(text.to_string(), 0, text.len(), metadata)];
        }

        // Try chunking strategies based on strategy enum
        let initial_chunks = match self.strategy {
            RecursiveStrategy::ParagraphFirst if level == 0 => {
                self.paragraph_chunker.chunk(text, config)
            }
            RecursiveStrategy::ParagraphFirst if level == 1 => {
                self.sentence_chunker.chunk(text, config)
            }
            RecursiveStrategy::SentenceFirst if level == 0 => {
                self.sentence_chunker.chunk(text, config)
            }
            _ => {
                // Final fallback: fixed-size
                self.fixed_chunker.chunk(text, config)
            }
        };

        // If we only got one chunk and it's still too large, go deeper
        let mut result = Vec::new();

        for chunk in initial_chunks {
            if chunk.text.len() > config.max_size {
                let parent_start = chunk.start;
                let parent_chunk_id = chunk.id.clone();
                let mut sub_chunks =
                    self.chunk_recursive(&chunk.text, config, Some(parent_chunk_id), level + 1);
                for sub in &mut sub_chunks {
                    sub.start += parent_start;
                    sub.end += parent_start;
                }
                result.extend(sub_chunks);
            } else {
                // Chunk fits, add with proper metadata
                let mut new_metadata = chunk.metadata.clone();
                new_metadata.method = format!("recursive_l{}", level);
                new_metadata.parent_chunk_id = parent_id.clone();
                result.push(Chunk {
                    id: chunk.id,
                    text: chunk.text,
                    start: chunk.start,
                    end: chunk.end,
                    metadata: new_metadata,
                });
            }
        }

        result
    }
}

impl ChunkAlgorithm for RecursiveChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        let mut chunks = self.chunk_recursive(text, config, None, 0);

        if config.overlap == 0 || chunks.len() <= 1 {
            return chunks;
        }

        // Post-processing: prepend the tail of each preceding chunk as overlap.
        // We clone the original texts first so mutations don't affect the source.
        let original_texts: Vec<String> = chunks.iter().map(|c| c.text.clone()).collect();

        for i in 1..chunks.len() {
            let prev = &original_texts[i - 1];
            // Take the last `overlap` bytes, snapping back to a char boundary.
            let overlap = config.overlap.min(prev.len());
            let snap = prev.len() - overlap;
            let snap = prev
                .char_indices()
                .map(|(b, _)| b)
                .filter(|&b| b >= snap)
                .next()
                .unwrap_or(prev.len());
            let overlap_text = &prev[snap..];
            let actual_overlap = overlap_text.len();

            chunks[i].text = format!("{} {}", overlap_text, chunks[i].text);
            chunks[i].start = chunks[i].start.saturating_sub(actual_overlap + 1);
            chunks[i].metadata.overlap_chars = Some(actual_overlap);
        }

        chunks
    }

    fn name(&self) -> &str {
        "recursive"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_small_text() {
        let chunker = RecursiveChunker::default();
        let config = ChunkConfig::new(100);
        let chunks = chunker.chunk("Small text", &config);

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "Small text");
    }

    #[test]
    fn test_recursive_paragraph_split() {
        let chunker = RecursiveChunker::new(RecursiveStrategy::ParagraphFirst);
        let config = ChunkConfig::new(30);
        let text = "First paragraph here.\n\nSecond paragraph here.";
        let chunks = chunker.chunk(text, &config);

        assert!(chunks.len() >= 2);
    }

    #[test]
    fn test_recursive_fallback_to_fixed() {
        let chunker = RecursiveChunker::default();
        let config = ChunkConfig::new(10);
        let text = "This is a long sentence without any paragraph breaks.";
        let chunks = chunker.chunk(text, &config);

        // Should fall back to fixed-size chunking
        assert!(chunks.len() > 1);
        // Most chunks should be within size limit
        let within_limit = chunks.iter().filter(|c| c.text.len() <= 10).count();
        assert!(within_limit > 0);
    }

    #[test]
    fn test_recursive_empty() {
        let chunker = RecursiveChunker::default();
        let config = ChunkConfig::new(100);
        let chunks = chunker.chunk("", &config);

        assert!(chunks.is_empty());
    }

    #[test]
    fn test_recursive_positions_in_original_doc() {
        let chunker = RecursiveChunker::default();
        // max_size=10 forces each paragraph to be further split (both paragraphs are >10 chars)
        let config = ChunkConfig::new(10);
        // "Second paragraph here." starts at byte 23 in this string
        let text = "First paragraph here.\n\nSecond paragraph here.";
        let chunks = chunker.chunk(text, &config);

        assert!(chunks.len() >= 2);
        // At least one chunk must start at or after byte 23 (second paragraph offset)
        let has_correct_offset = chunks.iter().any(|c| c.start >= 23);
        assert!(
            has_correct_offset,
            "No chunk reflects second paragraph offset (23+); positions not offset into original doc"
        );
    }

    #[test]
    fn test_recursive_level_tracking() {
        let chunker = RecursiveChunker::default();
        let config = ChunkConfig::new(20);
        let text = "Para one.\n\nPara two which is a bit longer.";
        let chunks = chunker.chunk(text, &config);

        assert!(chunks[0].metadata.method.starts_with("recursive_l"));
    }

    #[test]
    fn test_recursive_overlap_applied() {
        let chunker = RecursiveChunker::default();
        let config = ChunkConfig::new(30).with_overlap(10);
        let text = "First paragraph here.\n\nSecond paragraph here.\n\nThird paragraph here.";
        let chunks = chunker.chunk(text, &config);

        assert!(chunks.len() >= 2);
        // All chunks after the first should have overlap metadata
        for chunk in chunks.iter().skip(1) {
            assert!(chunk.metadata.overlap_chars.is_some());
        }
    }

    #[test]
    fn test_recursive_overlap_zero_unchanged() {
        let chunker = RecursiveChunker::default();
        let config = ChunkConfig::new(30);
        let text = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let chunks = chunker.chunk(text, &config);
        for chunk in &chunks {
            assert_eq!(chunk.metadata.overlap_chars, None);
        }
    }
}
