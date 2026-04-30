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
        // (start_byte, owned_text) for each paragraph in the current window
        let mut current_paras: Vec<(usize, String)> = Vec::new();
        let mut current_text = String::new();
        let mut current_start = 0usize;
        // How many chars at the start of current_text are carry-over overlap
        let mut window_overlap: Option<usize> = None;
        let mut byte_offset = 0usize;

        for part in text.split("\n\n") {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                byte_offset += part.len() + 2;
                continue;
            }

            let para_start = byte_offset + part.find(trimmed).unwrap_or(0);

            let potential_len = if current_text.is_empty() {
                trimmed.len()
            } else {
                current_text.len() + 2 + trimmed.len()
            };

            if potential_len > config.max_size && !current_text.is_empty() {
                // Flush
                chunks.push(Chunk::with_uuid(
                    current_text.clone(),
                    current_start,
                    current_start + current_text.len(),
                    ChunkMetadata {
                        method: self.name().to_string(),
                        section: None,
                        overlap_chars: window_overlap,
                        parent_chunk_id: None,
                    },
                ));

                // Carry overlap tail into next window
                if config.overlap > 0 {
                    let mut tail_len = 0usize;
                    let mut tail_start = current_paras.len();
                    for (i, (_, p)) in current_paras.iter().enumerate().rev() {
                        let cost = if tail_len == 0 { p.len() } else { p.len() + 2 };
                        if tail_len + cost > config.overlap {
                            break;
                        }
                        tail_len += cost;
                        tail_start = i;
                    }
                    let tail = &current_paras[tail_start..];
                    if !tail.is_empty() {
                        let carried = tail
                            .iter()
                            .map(|(_, p)| p.as_str())
                            .collect::<Vec<_>>()
                            .join("\n\n");
                        let carried_len = carried.len();
                        current_start = tail[0].0;
                        current_text = carried;
                        current_paras = tail.to_vec();
                        window_overlap = Some(carried_len);
                    } else {
                        current_text.clear();
                        current_paras.clear();
                        window_overlap = None;
                    }
                } else {
                    current_text.clear();
                    current_paras.clear();
                    window_overlap = None;
                }
            }

            if current_text.is_empty() {
                current_start = para_start;
                current_text = trimmed.to_string();
            } else {
                current_text.push_str("\n\n");
                current_text.push_str(trimmed);
            }
            current_paras.push((para_start, trimmed.to_string()));

            byte_offset += part.len() + 2;
        }

        if !current_text.is_empty() {
            chunks.push(Chunk::with_uuid(
                current_text.clone(),
                current_start,
                current_start + current_text.len(),
                ChunkMetadata {
                    method: self.name().to_string(),
                    section: None,
                    overlap_chars: window_overlap,
                    parent_chunk_id: None,
                },
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
