use crate::algorithms::{ParagraphChunker, SentenceChunker};
use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

pub struct HierarchicalChunker;

impl ChunkAlgorithm for HierarchicalChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let paragraphs = ParagraphChunker.chunk(text, config);
        let mut chunks = Vec::new();

        for para in paragraphs {
            let parent_id = para.id.clone();

            // Add the paragraph as parent chunk
            chunks.push(Chunk::with_uuid(
                para.text.clone(),
                para.start,
                para.end,
                ChunkMetadata {
                    method: "hierarchical_parent".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ));

            // Split paragraph into sentence children
            let sentence_config = ChunkConfig::new(1);
            let sentences = SentenceChunker.chunk(&para.text, &sentence_config);

            for sentence in sentences {
                chunks.push(Chunk::with_uuid(
                    sentence.text.clone(),
                    para.start + sentence.start,
                    para.start + sentence.end,
                    ChunkMetadata {
                        method: "hierarchical_child".to_string(),
                        section: None,
                        overlap_chars: None,
                        parent_chunk_id: Some(parent_id.clone()),
                    },
                ));
            }
        }

        chunks
    }

    fn name(&self) -> &str {
        "hierarchical"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchical_parent_child() {
        let chunker = HierarchicalChunker;
        let config = ChunkConfig::new(1000);
        let text = "First paragraph. It has sentences.\n\nSecond paragraph. Also sentences.";
        let chunks = chunker.chunk(text, &config);

        let parents: Vec<_> = chunks
            .iter()
            .filter(|c| c.metadata.method == "hierarchical_parent")
            .collect();
        let children: Vec<_> = chunks
            .iter()
            .filter(|c| c.metadata.method == "hierarchical_child")
            .collect();

        assert!(!parents.is_empty());
        assert!(!children.is_empty());
        // Children have parent IDs
        assert!(
            children
                .iter()
                .all(|c| c.metadata.parent_chunk_id.is_some())
        );
    }

    #[test]
    fn test_hierarchical_empty() {
        let chunker = HierarchicalChunker;
        let config = ChunkConfig::new(1000);
        assert!(chunker.chunk("", &config).is_empty());
    }

    #[test]
    fn test_hierarchical_name() {
        assert_eq!(HierarchicalChunker.name(), "hierarchical");
    }
}
