//! Sliding window chunking algorithm with overlap.

use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

/// Sliding window chunker that creates overlapping chunks.
pub struct SlidingWindowChunker;

impl ChunkAlgorithm for SlidingWindowChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() || config.max_size == 0 {
            return Vec::new();
        }

        let overlap = config.overlap.min(config.max_size.saturating_sub(1));
        let step = config.max_size.saturating_sub(overlap);

        if step == 0 {
            return Vec::new();
        }

        let mut chunks = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut start_char_idx = 0;

        while start_char_idx < chars.len() {
            let end_char_idx = (start_char_idx + config.max_size).min(chars.len());
            let chunk_text: String = chars[start_char_idx..end_char_idx].iter().collect();

            // Calculate byte positions
            let start_byte = chars[..start_char_idx].iter().map(|c| c.len_utf8()).sum();
            let end_byte = start_byte + chunk_text.len();

            // Calculate actual overlap for this chunk
            let actual_overlap = if start_char_idx > 0 {
                Some(overlap)
            } else {
                None
            };

            let metadata = ChunkMetadata {
                method: self.name().to_string(),
                section: None,
                overlap_chars: actual_overlap,
                parent_chunk_id: None,
            };

            chunks.push(Chunk::with_uuid(chunk_text, start_byte, end_byte, metadata));

            // Move to next position
            if end_char_idx >= chars.len() {
                break;
            }
            start_char_idx += step;
        }

        chunks
    }

    fn name(&self) -> &str {
        "sliding_window"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window_basic() {
        let chunker = SlidingWindowChunker;
        let config = ChunkConfig::new(5).with_overlap(2);
        let chunks = chunker.chunk("hello world!", &config);

        // With max_size=5. overlap=2, step=3
        // Chunks: "hello", "lo wo", "world", "rld!"
        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].text, "hello");
        assert_eq!(chunks[1].text, "lo wo");
        assert_eq!(chunks[2].text, "world");
        assert_eq!(chunks[3].text, "ld!");
    }

    #[test]
    fn test_sliding_window_no_overlap() {
        let chunker = SlidingWindowChunker;
        let config = ChunkConfig::new(5).with_overlap(0);
        let chunks = chunker.chunk("hello world", &config);

        // Same as fixed size
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].text, "hello");
        assert_eq!(chunks[1].text, " worl");
        assert_eq!(chunks[2].text, "d");
    }

    #[test]
    fn test_sliding_window_overlap_metadata() {
        let chunker = SlidingWindowChunker;
        let config = ChunkConfig::new(5).with_overlap(2);
        let chunks = chunker.chunk("hello world", &config);

        // First chunk should have no overlap
        assert_eq!(chunks[0].metadata.overlap_chars, None);
        // Subsequent chunks should have overlap
        assert_eq!(chunks[1].metadata.overlap_chars, Some(2));
    }

    #[test]
    fn test_sliding_window_empty() {
        let chunker = SlidingWindowChunker;
        let config = ChunkConfig::new(5).with_overlap(2);
        let chunks = chunker.chunk("", &config);

        assert!(chunks.is_empty());
    }
}
