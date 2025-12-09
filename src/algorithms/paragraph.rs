//! Paragraph-based chunking algorithm.

use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

/// Paragraph-based chunker that splits on double newlines.
pub struct ParagraphChunker;

impl ChunkAlgorithm for ParagraphChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let mut chunks = Vec::new();
        let mut current_text = String::new();
        let mut current_start = 0;
        let mut byte_offset = 0;
        let mut chunk_start_set = false;

        // Split on double newlines (paragraph boundaries)
        for part in text.split("\n\n") {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                byte_offset += part.len() + 2; // +2 for the \n\n
                continue;
            }

            let para_start = byte_offset + part.find(trimmed).unwrap_or(0);

            // Check if adding this paragraph would exceed max_size
            let potential_len = if current_text.is_empty() {
                trimmed.len()
            } else {
                current_text.len() + 2 + trimmed.len() // +2 for paragraph separator
            };

            if potential_len > config.max_size && !current_text.is_empty() {
                // Flush current chunk
                let metadata = ChunkMetadata {
                    method: self.name().to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                };
                chunks.push(Chunk::with_uuid(
                    current_text.clone(),
                    current_start,
                    current_start + current_text.len(),
                    metadata,
                ));

                // Start new chunk
                current_text = trimmed.to_string();
                current_start = para_start;
                chunk_start_set = true;
            } else {
                if !chunk_start_set {
                    current_start = para_start;
                    chunk_start_set = true;
                }
                if current_text.is_empty() {
                    current_text = trimmed.to_string();
                } else {
                    current_text.push_str("\n\n");
                    current_text.push_str(trimmed);
                }
            }

            byte_offset += part.len() + 2; // +2 for the \n\n separator
        }

        // Flush remaining text
        if !current_text.is_empty() {
            let metadata = ChunkMetadata {
                method: self.name().to_string(),
                section: None,
                overlap_chars: None,
                parent_chunk_id: None,
            };
            chunks.push(Chunk::with_uuid(
                current_text.clone(),
                current_start,
                current_start + current_text.len(),
                metadata,
            ));
        }

        chunks
    }

    fn name(&self) -> &str {
        "paragraph"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragraph_basic() {
        let chunker = ParagraphChunker;
        let config = ChunkConfig::new(1000);
        let text = "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.";
        let chunks = chunker.chunk(text, &config);

        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].text.contains("First paragraph."));
        assert!(chunks[0].text.contains("Second paragraph."));
        assert!(chunks[0].text.contains("Third paragraph."));
    }

    #[test]
    fn test_paragraph_split_by_size() {
        let chunker = ParagraphChunker;
        let config = ChunkConfig::new(30);
        let text = "First paragraph here.\n\nSecond paragraph here.\n\nThird paragraph here.";
        let chunks = chunker.chunk(text, &config);

        // Should split due to size limit
        assert!(chunks.len() > 1);
    }

    #[test]
    fn test_paragraph_single() {
        let chunker = ParagraphChunker;
        let config = ChunkConfig::new(1000);
        let text = "Just one paragraph with no breaks.";
        let chunks = chunker.chunk(text, &config);

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "Just one paragraph with no breaks.");
    }

    #[test]
    fn test_paragraph_empty() {
        let chunker = ParagraphChunker;
        let config = ChunkConfig::new(100);
        let chunks = chunker.chunk("", &config);

        assert!(chunks.is_empty());
    }

    #[test]
    fn test_paragraph_only_whitespace() {
        let chunker = ParagraphChunker;
        let config = ChunkConfig::new(100);
        let chunks = chunker.chunk("\n\n\n\n", &config);

        assert!(chunks.is_empty());
    }
}
