use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;
use std::sync::LazyLock;

static BLOCK_TAG_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"(?i)<(p|div|section|article|header|footer|h[1-6]|ul|ol|li|blockquote|pre|table|tr|td|th|form|nav|main|aside)[\s>]"
    ).unwrap()
});

static HTML_TAG_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<[^>]+>").unwrap());

pub struct HtmlChunker;

impl HtmlChunker {
    fn strip_tags(html: &str) -> String {
        HTML_TAG_RE.replace_all(html, "").to_string()
    }
}

impl ChunkAlgorithm for HtmlChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        // Split at block-level HTML tag boundaries
        let mut chunks = Vec::new();
        let mut last_end = 0;

        let matches: Vec<_> = BLOCK_TAG_RE.find_iter(text).collect();

        // If no block tags found, treat whole text as one chunk
        if matches.is_empty() {
            let stripped = Self::strip_tags(text).trim().to_string();
            if !stripped.is_empty() {
                chunks.push(Chunk::with_uuid(
                    stripped,
                    0,
                    text.len(),
                    ChunkMetadata {
                        method: "html".to_string(),
                        section: None,
                        overlap_chars: None,
                        parent_chunk_id: None,
                    },
                ));
            }
            return chunks;
        }

        for mat in &matches {
            let segment = &text[last_end..mat.start()];
            let stripped = Self::strip_tags(segment).trim().to_string();
            if !stripped.is_empty() && stripped.len() <= config.max_size {
                chunks.push(Chunk::with_uuid(
                    stripped,
                    last_end,
                    mat.start(),
                    ChunkMetadata {
                        method: "html".to_string(),
                        section: None,
                        overlap_chars: None,
                        parent_chunk_id: None,
                    },
                ));
            }
            last_end = mat.start();
        }

        // Final segment
        let segment = &text[last_end..];
        let stripped = Self::strip_tags(segment).trim().to_string();
        if !stripped.is_empty() {
            chunks.push(Chunk::with_uuid(
                stripped,
                last_end,
                text.len(),
                ChunkMetadata {
                    method: "html".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ));
        }

        chunks
    }

    fn name(&self) -> &str {
        "html"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_strips_tags() {
        let chunker = HtmlChunker;
        let config = ChunkConfig::new(1000);
        let html = "<p>Hello world.</p><p>Second paragraph.</p>";
        let chunks = chunker.chunk(html, &config);
        assert!(!chunks.is_empty());
        assert!(!chunks[0].text.contains('<'));
    }

    #[test]
    fn test_html_empty() {
        let chunker = HtmlChunker;
        let config = ChunkConfig::new(1000);
        assert!(chunker.chunk("", &config).is_empty());
    }

    #[test]
    fn test_html_name() {
        assert_eq!(HtmlChunker.name(), "html");
    }

    #[test]
    fn test_html_plain_text() {
        let chunker = HtmlChunker;
        let config = ChunkConfig::new(1000);
        let chunks = chunker.chunk("No HTML here.", &config);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "No HTML here.");
    }
}
