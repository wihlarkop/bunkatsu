//! Heading-based chunking algorithm.
//!
//! Splits text at heading boundaries (# ## ### etc.)

use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;
use regex::Regex;

/// A parsed heading with its content.
#[derive(Debug)]
struct HeadingSection {
    /// The heading level (1-6)
    level: usize,
    /// The heading text
    title: String,
    /// Content under this heading
    content: String,
    /// Start byte position
    start: usize,
    /// End byte position
    end: usize,
}

/// Heading-based chunker that splits at heading boundaries.
pub struct HeadingChunker {
    /// Which heading levels to split at (e.g., [1, 2] for # and ##)
    pub levels: Vec<usize>,
}

impl Default for HeadingChunker {
    fn default() -> Self {
        Self {
            levels: vec![1, 2], // Default: split at h1 and h2
        }
    }
}

impl HeadingChunker {
    /// Create a new HeadingChunker with specified levels.
    pub fn new(levels: Vec<usize>) -> Self {
        Self { levels }
    }

    /// Parse text into sections based on headings.
    fn parse_sections(&self, text: &str) -> Vec<HeadingSection> {
        let heading_re = Regex::new(r"^(#{1,6})\s+(.+)$").unwrap();
        let mut sections = Vec::new();
        let mut current_section: Option<HeadingSection> = None;
        let mut current_pos = 0;

        for line in text.lines() {
            let line_start = current_pos;
            let line_end = current_pos + line.len();

            if let Some(caps) = heading_re.captures(line) {
                let level = caps.get(1).map(|m| m.as_str().len()).unwrap_or(1);
                let title = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string();

                // Check if this heading level should trigger a split
                if self.levels.contains(&level) {
                    // Save previous section
                    if let Some(mut section) = current_section.take() {
                        section.end = line_start;
                        section.content = section.content.trim().to_string();
                        sections.push(section);
                    }

                    // Start new section
                    current_section = Some(HeadingSection {
                        level,
                        title,
                        content: String::new(),
                        start: line_start,
                        end: 0,
                    });
                } else if let Some(ref mut section) = current_section {
                    // Add heading to current section content
                    section.content.push_str(line);
                    section.content.push('\n');
                }
            } else if let Some(ref mut section) = current_section {
                section.content.push_str(line);
                section.content.push('\n');
            } else {
                // Content before any heading - create implicit section
                if current_section.is_none() && !line.trim().is_empty() {
                    current_section = Some(HeadingSection {
                        level: 0,
                        title: String::new(),
                        content: format!("{}\n", line),
                        start: line_start,
                        end: 0,
                    });
                } else if let Some(ref mut section) = current_section {
                    section.content.push_str(line);
                    section.content.push('\n');
                }
            }

            current_pos = line_end + 1; // +1 for newline
        }

        // Save final section
        if let Some(mut section) = current_section {
            section.end = text.len();
            section.content = section.content.trim().to_string();
            sections.push(section);
        }

        sections
    }
}

impl ChunkAlgorithm for HeadingChunker {
    fn chunk(&self, text: &str, _config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let sections = self.parse_sections(text);
        let mut chunks = Vec::new();

        for section in sections {
            let section_name = if section.level > 0 {
                Some(format!("h{}: {}", section.level, section.title))
            } else {
                None
            };

            // Build chunk text with heading if present
            let chunk_text = if section.level > 0 && !section.title.is_empty() {
                format!(
                    "{} {}\n\n{}",
                    "#".repeat(section.level),
                    section.title,
                    section.content
                )
            } else {
                section.content.clone()
            };

            // If section exceeds max_size, we still keep it as one chunk
            // (recursive chunking would handle further splitting)
            if !chunk_text.trim().is_empty() {
                let metadata = ChunkMetadata {
                    method: self.name().to_string(),
                    section: section_name,
                    overlap_chars: None,
                    parent_chunk_id: None,
                };

                chunks.push(Chunk::with_uuid(
                    chunk_text.trim().to_string(),
                    section.start,
                    section.end,
                    metadata,
                ));
            }
        }

        chunks
    }

    fn name(&self) -> &str {
        "heading"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_basic() {
        let chunker = HeadingChunker::default();
        let config = ChunkConfig::new(1000);
        let text = r#"# First Section

Content 1.

# Second Section

Content 2.
"#;
        let chunks = chunker.chunk(text, &config);

        assert_eq!(chunks.len(), 2);
        assert!(chunks[0].text.contains("First Section"));
        assert!(chunks[1].text.contains("Second Section"));
    }

    #[test]
    fn test_heading_nested() {
        let chunker = HeadingChunker::new(vec![1]); // Only split at h1
        let config = ChunkConfig::new(1000);
        let text = r#"# Main Section

## Subsection 1

Content.

## Subsection 2

More content.
"#;
        let chunks = chunker.chunk(text, &config);

        // Only one chunk since we only split at h1
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].text.contains("Subsection 1"));
        assert!(chunks[0].text.contains("Subsection 2"));
    }

    #[test]
    fn test_heading_section_metadata() {
        let chunker = HeadingChunker::default();
        let config = ChunkConfig::new(1000);
        let text = "## My Section\n\nContent here.";
        let chunks = chunker.chunk(text, &config);

        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks[0].metadata.section,
            Some("h2: My Section".to_string())
        );
    }

    #[test]
    fn test_heading_empty() {
        let chunker = HeadingChunker::default();
        let config = ChunkConfig::new(100);
        let chunks = chunker.chunk("", &config);

        assert!(chunks.is_empty());
    }
}
