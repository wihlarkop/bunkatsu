use crate::algorithms::sentence::SentenceChunker;
use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

pub struct TokenChunker;

impl TokenChunker {
    pub fn chunk_with_tokenizer<F>(&self, text: &str, max_tokens: usize, tokenizer: F) -> Vec<Chunk>
    where
        F: Fn(&str) -> usize,
    {
        if text.is_empty() || max_tokens == 0 {
            return Vec::new();
        }

        // max_size=1 forces SentenceChunker to yield one sentence per chunk
        let config = ChunkConfig::new(1);
        let sentences = SentenceChunker.chunk(text, &config);

        let mut chunks = Vec::new();
        let mut current_text = String::new();
        let mut current_start = 0usize;
        let mut current_tokens = 0usize;

        for sentence in sentences {
            let sentence_tokens = tokenizer(&sentence.text);

            if !current_text.is_empty() && current_tokens + sentence_tokens > max_tokens {
                let end = current_start + current_text.len();
                chunks.push(Chunk::with_uuid(
                    current_text.clone(),
                    current_start,
                    end,
                    ChunkMetadata {
                        method: "token".to_string(),
                        section: None,
                        overlap_chars: None,
                        parent_chunk_id: None,
                    },
                ));
                current_text.clear();
                current_tokens = 0;
                current_start = sentence.start;
            }

            if current_text.is_empty() {
                current_start = sentence.start;
                current_text = sentence.text.clone();
            } else {
                current_text.push(' ');
                current_text.push_str(&sentence.text);
            }
            current_tokens += sentence_tokens;
        }

        if !current_text.is_empty() {
            let end = current_start + current_text.len();
            chunks.push(Chunk::with_uuid(
                current_text,
                current_start,
                end,
                ChunkMetadata {
                    method: "token".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ));
        }

        chunks
    }
}

impl ChunkAlgorithm for TokenChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        self.chunk_with_tokenizer(text, config.max_size, |s| s.split_whitespace().count())
    }

    fn name(&self) -> &str {
        "token"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word_count(s: &str) -> usize {
        s.split_whitespace().count()
    }

    #[test]
    fn test_token_basic() {
        let chunker = TokenChunker;
        let chunks = chunker.chunk_with_tokenizer(
            "Hello world. How are you? I am fine.",
            3,
            word_count,
        );
        assert!(chunks.len() >= 2);
    }

    #[test]
    fn test_token_empty() {
        let chunker = TokenChunker;
        let chunks = chunker.chunk_with_tokenizer("", 10, word_count);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_token_name() {
        assert_eq!(TokenChunker.name(), "token");
    }

    #[test]
    fn test_token_single_chunk_large_budget() {
        let chunker = TokenChunker;
        let chunks = chunker.chunk_with_tokenizer("Short text here.", 1000, word_count);
        assert_eq!(chunks.len(), 1);
    }
}
