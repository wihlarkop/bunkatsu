//! Sentence-based chunking algorithm.

use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::{ChunkConfig, SentenceDetector};
use crate::traits::ChunkAlgorithm;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

/// Sentence-based chunker with configurable detection method.
pub struct SentenceChunker;

impl SentenceChunker {
    /// Split text into sentences using regex (fast, basic).
    fn split_regex(text: &str) -> Vec<(usize, usize, &str)> {
        // Match sentence-ending punctuation followed by whitespace or end of string
        let re = Regex::new(r"[.!?]+[\s]+|[.!?]+$").unwrap();

        let mut sentences = Vec::new();
        let mut last_end = 0;

        for mat in re.find_iter(text) {
            let sentence_end = mat.end();
            let sentence = &text[last_end..sentence_end];
            if !sentence.trim().is_empty() {
                sentences.push((last_end, sentence_end, sentence.trim_end()));
            }
            last_end = sentence_end;
        }

        // Handle remaining text (no ending punctuation)
        if last_end < text.len() {
            let remaining = &text[last_end..];
            if !remaining.trim().is_empty() {
                sentences.push((last_end, text.len(), remaining.trim()));
            }
        }

        sentences
    }

    /// Split text into sentences using Unicode segmentation (accurate).
    fn split_unicode(text: &str) -> Vec<(usize, usize, &str)> {
        let mut sentences = Vec::new();
        let mut byte_offset = 0;

        for sentence in text.split_sentence_bounds() {
            let trimmed = sentence.trim();
            if !trimmed.is_empty() {
                let start = byte_offset + sentence.find(trimmed).unwrap_or(0);
                let end = start + trimmed.len();
                sentences.push((start, end, trimmed));
            }
            byte_offset += sentence.len();
        }

        sentences
    }
}

impl ChunkAlgorithm for SentenceChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let sentences = match config.sentence_detector {
            SentenceDetector::Regex => Self::split_regex(text),
            SentenceDetector::Unicode => Self::split_unicode(text),
        };

        let mut chunks = Vec::new();
        let mut current_text = String::new();
        let mut current_start = 0;
        let mut chunk_start_set = false;

        for (start, _end, sentence) in sentences {
            // Check if adding this sentence would exceed max_size
            let potential_len = if current_text.is_empty() {
                sentence.len()
            } else {
                current_text.len() + 1 + sentence.len() // +1 for space
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
                current_text = sentence.to_string();
                current_start = start;
                chunk_start_set = true;
            } else {
                if !chunk_start_set {
                    current_start = start;
                    chunk_start_set = true;
                }
                if current_text.is_empty() {
                    current_text = sentence.to_string();
                } else {
                    current_text.push(' ');
                    current_text.push_str(sentence);
                }
            }
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
        "sentence"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sentence_single() {
        let chunker = SentenceChunker;
        let config = ChunkConfig::new(1000);
        let chunks = chunker.chunk("Hello world. How are you?", &config);

        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].text.contains("Hello world."));
        assert!(chunks[0].text.contains("How are you?"));
    }

    #[test]
    fn test_sentence_split_by_size() {
        let chunker = SentenceChunker;
        let config = ChunkConfig::new(20);
        let chunks = chunker.chunk("Hello world. How are you? I am fine.", &config);

        // Should split into multiple chunks due to size limit
        assert!(chunks.len() > 1);
    }

    #[test]
    fn test_sentence_unicode_detector() {
        let chunker = SentenceChunker;
        let config = ChunkConfig::new(1000).with_sentence_detector(SentenceDetector::Unicode);
        let chunks = chunker.chunk("Hello world. How are you?", &config);

        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_sentence_empty() {
        let chunker = SentenceChunker;
        let config = ChunkConfig::new(100);
        let chunks = chunker.chunk("", &config);

        assert!(chunks.is_empty());
    }
}
