use crate::algorithms::sentence::SentenceChunker;
use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::embedding::TextGenerator;
use crate::traits::ChunkAlgorithm;
use pyo3::prelude::*;

pub struct PropositionChunker;

const PROPOSITION_PROMPT: &str = "Decompose the following sentence into atomic factual propositions. Output one proposition per line. Do not add explanations.\n\nSentence: {sentence}";

impl PropositionChunker {
    pub fn chunk_with_generator<'py>(
        &self,
        py: Python<'py>,
        text: &str,
        generator: &dyn TextGenerator,
    ) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let config = ChunkConfig::new(1);
        let sentences = SentenceChunker.chunk(text, &config);

        let mut chunks = Vec::new();
        let mut offset = 0usize;

        for sentence in &sentences {
            let prompt = PROPOSITION_PROMPT.replace("{sentence}", &sentence.text);
            let response = match generator.generate(py, &prompt) {
                Ok(r) => r,
                Err(_) => continue,
            };

            for line in response.lines() {
                let proposition = line.trim().to_string();
                if proposition.is_empty() {
                    continue;
                }
                let start = offset;
                let end = start + proposition.len();
                chunks.push(Chunk::with_uuid(
                    proposition,
                    start,
                    end,
                    ChunkMetadata {
                        method: "proposition".to_string(),
                        section: None,
                        overlap_chars: None,
                        parent_chunk_id: Some(sentence.id.clone()),
                    },
                ));
                offset = end + 1;
            }
        }

        chunks
    }
}

impl ChunkAlgorithm for PropositionChunker {
    fn chunk(&self, _text: &str, _config: &ChunkConfig) -> Vec<Chunk> {
        Vec::new()
    }

    fn name(&self) -> &str {
        "proposition"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposition_name() {
        assert_eq!(PropositionChunker.name(), "proposition");
    }
}
