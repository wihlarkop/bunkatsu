//! Fixed-size character-based chunking algorithm.

use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

/// Fixed-size chunker that splits text into chunks of a specified maximum character count.
pub struct FixedSizeChunker;

impl ChunkAlgorithm for FixedSizeChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() || config.max_size == 0 {
            return Vec::new();
        }

        let mut chunks = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut start_char_idx = 0;

        while start_char_idx < chars.len() {
            let end_char_idx = (start_char_idx + config.max_size).min(chars.len());
            let chunk_text: String = chars[start_char_idx..end_char_idx].iter().collect();

            // Calculate byte positions for start/end
            let start_byte = chars[..start_char_idx].iter().map(|c| c.len_utf8()).sum();
            let end_byte = start_byte + chunk_text.len();

            let metadata = ChunkMetadata {
                method: self.name().to_string(),
                section: None,
                overlap_chars: None,
                parent_chunk_id: None,
            };

            chunks.push(Chunk::with_uuid(chunk_text, start_byte, end_byte, metadata));

            start_char_idx = end_char_idx;
        }

        chunks
    }

    fn name(&self) -> &str {
        "fixed_size"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_size_basic() {
        let chunker = FixedSizeChunker;
        let config = ChunkConfig::new(5);
        let chunks = chunker.chunk("hello world", &config);

        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].text, "hello");
        assert_eq!(chunks[1].text, " worl");
        assert_eq!(chunks[2].text, "d");
    }

    #[test]
    fn test_fixed_size_empty() {
        let chunker = FixedSizeChunker;
        let config = ChunkConfig::new(5);
        let chunks = chunker.chunk("", &config);

        assert!(chunks.is_empty());
    }

    #[test]
    fn test_fixed_size_unicode() {
        let chunker = FixedSizeChunker;
        let config = ChunkConfig::new(3);
        let chunks = chunker.chunk("日本語テスト", &config);

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].text, "日本語");
        assert_eq!(chunks[1].text, "テスト");
    }

    #[test]
    fn test_fixed_size_positions() {
        let chunker = FixedSizeChunker;
        let config = ChunkConfig::new(5);
        let chunks = chunker.chunk("hello world", &config);

        assert_eq!(chunks[0].start, 0);
        assert_eq!(chunks[0].end, 5);
        assert_eq!(chunks[1].start, 5);
        assert_eq!(chunks[1].end, 10);
    }
}
