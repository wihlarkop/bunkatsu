use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;
use std::sync::LazyLock;

static SECTION_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"\\(part|chapter|section|subsection|subsubsection)\*?\{[^}]*\}").unwrap()
});

pub struct LatexChunker;

impl ChunkAlgorithm for LatexChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let mut chunks = Vec::new();
        let mut last_end = 0;
        let mut current_section: Option<String> = None;

        let matches: Vec<_> = SECTION_RE.find_iter(text).collect();

        if matches.is_empty() {
            chunks.push(Chunk::with_uuid(
                text.trim().to_string(),
                0,
                text.len(),
                ChunkMetadata {
                    method: "latex".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ));
            return chunks;
        }

        for mat in &matches {
            let segment = text[last_end..mat.start()].trim();
            if !segment.is_empty() && segment.len() <= config.max_size {
                chunks.push(Chunk::with_uuid(
                    segment.to_string(),
                    last_end,
                    mat.start(),
                    ChunkMetadata {
                        method: "latex".to_string(),
                        section: current_section.clone(),
                        overlap_chars: None,
                        parent_chunk_id: None,
                    },
                ));
            }
            current_section = Some(mat.as_str().to_string());
            last_end = mat.start();
        }

        // Final segment (from last section header to end)
        let segment = text[last_end..].trim();
        if !segment.is_empty() {
            chunks.push(Chunk::with_uuid(
                segment.to_string(),
                last_end,
                text.len(),
                ChunkMetadata {
                    method: "latex".to_string(),
                    section: current_section,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ));
        }

        chunks
    }

    fn name(&self) -> &str {
        "latex"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_splits_at_section() {
        let chunker = LatexChunker;
        let config = ChunkConfig::new(1000);
        let text = r"\section{Intro}\nHello.\n\section{Methods}\nWorld.";
        let chunks = chunker.chunk(text, &config);
        assert!(chunks.len() >= 2);
    }

    #[test]
    fn test_latex_empty() {
        let chunker = LatexChunker;
        let config = ChunkConfig::new(1000);
        assert!(chunker.chunk("", &config).is_empty());
    }

    #[test]
    fn test_latex_no_sections() {
        let chunker = LatexChunker;
        let config = ChunkConfig::new(1000);
        let chunks = chunker.chunk("Just plain text.", &config);
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn test_latex_name() {
        assert_eq!(LatexChunker.name(), "latex");
    }
}
