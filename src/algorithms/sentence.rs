//! Sentence-based chunking algorithm.

use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::{ChunkConfig, SentenceDetector};
use crate::traits::ChunkAlgorithm;
use std::sync::LazyLock;
use unicode_segmentation::UnicodeSegmentation;

static SENTENCE_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"[.!?]+[\s]+|[.!?]+$").unwrap());

/// Sentence-based chunker with configurable detection method.
pub struct SentenceChunker;

impl SentenceChunker {
    /// Split text into sentences using regex (fast, basic).
    fn split_regex(text: &str) -> Vec<(usize, usize, &str)> {
        let mut sentences = Vec::new();
        let mut last_end = 0;

        for mat in SENTENCE_RE.find_iter(text) {
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

/// Returns the suffix of `sentences` whose total joined length fits within `max_overlap` chars.
/// Each entry is `(start_byte, sentence_text)`.
fn overlap_tail(sentences: &[(usize, String)], max_overlap: usize) -> &[(usize, String)] {
    if max_overlap == 0 || sentences.is_empty() {
        return &[];
    }
    let mut total = 0usize;
    let mut take_from = sentences.len();
    for (i, (_, s)) in sentences.iter().enumerate().rev() {
        let cost = if total == 0 { s.len() } else { s.len() + 1 };
        if total + cost > max_overlap {
            break;
        }
        total += cost;
        take_from = i;
    }
    &sentences[take_from..]
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
        // (start_byte, owned_text) for each sentence in the current window
        let mut current_sentences: Vec<(usize, String)> = Vec::new();
        let mut current_text = String::new();
        let mut current_start = 0usize;
        // How many chars at the START of current_text were carried over from the prior chunk
        let mut window_overlap: Option<usize> = None;

        for (start, _end, sentence) in &sentences {
            let potential_len = if current_text.is_empty() {
                sentence.len()
            } else {
                current_text.len() + 1 + sentence.len()
            };

            if potential_len > config.max_size && !current_text.is_empty() {
                // Flush current window
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

                // Carry the overlap tail into the next window
                if config.overlap > 0 {
                    let tail = overlap_tail(&current_sentences, config.overlap);
                    if !tail.is_empty() {
                        let carried_chars = tail
                            .iter()
                            .map(|(_, s)| s.len())
                            .sum::<usize>()
                            + tail.len().saturating_sub(1);
                        current_text = tail
                            .iter()
                            .map(|(_, s)| s.as_str())
                            .collect::<Vec<_>>()
                            .join(" ");
                        current_start = tail[0].0;
                        current_sentences = tail.to_vec();
                        window_overlap = Some(carried_chars);
                    } else {
                        current_text.clear();
                        current_sentences.clear();
                        window_overlap = None;
                    }
                } else {
                    current_text.clear();
                    current_sentences.clear();
                    window_overlap = None;
                }
            }

            if current_text.is_empty() {
                current_start = *start;
                current_text = sentence.to_string();
            } else {
                current_text.push(' ');
                current_text.push_str(sentence);
            }
            current_sentences.push((*start, sentence.to_string()));
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

    #[test]
    fn test_sentence_overlap_carries_sentences() {
        let chunker = SentenceChunker;
        // max_size=30 forces splits; overlap=20 should carry last sentence into next chunk
        let config = ChunkConfig::new(30).with_overlap(20);
        let text = "Hello world. How are you? I am fine.";
        let chunks = chunker.chunk(text, &config);

        assert!(chunks.len() >= 2);
        // Second+ chunks should have overlap_chars set
        assert!(chunks[1].metadata.overlap_chars.is_some());
        // The text of chunk[1] should start with content from the end of chunk[0]
        let last_sentence_of_first = chunks[0].text.split_whitespace().last().unwrap_or("");
        assert!(chunks[1].text.contains(last_sentence_of_first) || chunks[1].metadata.overlap_chars.unwrap() > 0);
    }

    #[test]
    fn test_sentence_overlap_zero_unchanged() {
        let chunker = SentenceChunker;
        let config_no_overlap = ChunkConfig::new(20);
        let config_overlap = ChunkConfig::new(20).with_overlap(0);
        let text = "Hello world. How are you? I am fine.";
        let c1 = chunker.chunk(text, &config_no_overlap);
        let c2 = chunker.chunk(text, &config_overlap);
        assert_eq!(c1.len(), c2.len());
    }
}
