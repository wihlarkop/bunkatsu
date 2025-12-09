//! Markdown-aware chunking algorithm.
//!
//! Parses markdown structure and preserves:
//! - Code blocks (fenced with ```) as atomic units
//! - Headings for section boundaries
//! - Lists and block quotes

use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;
use regex::Regex;

/// Represents a parsed markdown block.
#[derive(Debug, Clone)]
enum MarkdownBlock {
    /// A fenced code block (``` or ~~~)
    CodeBlock {
        content: String,
        #[allow(dead_code)]
        language: Option<String>,
        start: usize,
        end: usize,
    },
    /// A heading (# ## ### etc.)
    Heading {
        content: String,
        level: usize,
        start: usize,
        #[allow(dead_code)]
        end: usize,
    },
    /// Regular text content
    Text {
        content: String,
        start: usize,
        #[allow(dead_code)]
        end: usize,
    },
}

/// Markdown-aware chunker that preserves code blocks and splits at headings.
pub struct MarkdownChunker;

impl MarkdownChunker {
    /// Parse markdown text into blocks.
    fn parse_blocks(text: &str) -> Vec<MarkdownBlock> {
        let mut blocks = Vec::new();
        let mut current_pos = 0;
        let mut in_code_block = false;
        let mut code_block_start = 0;
        let mut code_block_lang: Option<String> = None;
        let mut pending_text_start: Option<usize> = None;
        let mut pending_text = String::new();

        let code_fence_re = Regex::new(r"^(`{3,}|~{3,})(\w*)\s*$").unwrap();
        let heading_re = Regex::new(r"^(#{1,6})\s+(.+)$").unwrap();

        for line in text.lines() {
            let line_start = current_pos;
            let line_end = current_pos + line.len();

            if let Some(caps) = code_fence_re.captures(line) {
                if !in_code_block {
                    // Start of code block - flush pending text first
                    if !pending_text.is_empty() {
                        blocks.push(MarkdownBlock::Text {
                            content: pending_text.clone(),
                            start: pending_text_start.unwrap_or(line_start),
                            end: line_start,
                        });
                        pending_text.clear();
                        pending_text_start = None;
                    }

                    in_code_block = true;
                    code_block_start = line_start;
                    code_block_lang = caps
                        .get(2)
                        .map(|m| m.as_str().to_string())
                        .filter(|s| !s.is_empty());
                } else {
                    // End of code block
                    in_code_block = false;
                    let code_content = &text[code_block_start..line_end];
                    blocks.push(MarkdownBlock::CodeBlock {
                        content: code_content.to_string(),
                        language: code_block_lang.take(),
                        start: code_block_start,
                        end: line_end,
                    });
                }
            } else if in_code_block {
                // Inside code block, continue
            } else if let Some(caps) = heading_re.captures(line) {
                // Flush pending text
                if !pending_text.is_empty() {
                    blocks.push(MarkdownBlock::Text {
                        content: pending_text.clone(),
                        start: pending_text_start.unwrap_or(line_start),
                        end: line_start,
                    });
                    pending_text.clear();
                    pending_text_start = None;
                }

                let level = caps.get(1).map(|m| m.as_str().len()).unwrap_or(1);
                let heading_text = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                blocks.push(MarkdownBlock::Heading {
                    content: heading_text.to_string(),
                    level,
                    start: line_start,
                    end: line_end,
                });
            } else {
                // Regular text
                if pending_text_start.is_none() {
                    pending_text_start = Some(line_start);
                }
                if !pending_text.is_empty() {
                    pending_text.push('\n');
                }
                pending_text.push_str(line);
            }

            // Move past line + newline character
            current_pos = line_end + 1; // +1 for \n
        }

        // Handle unclosed code block
        if in_code_block {
            let code_content = &text[code_block_start..];
            blocks.push(MarkdownBlock::CodeBlock {
                content: code_content.to_string(),
                language: code_block_lang,
                start: code_block_start,
                end: text.len(),
            });
        } else if !pending_text.is_empty() {
            // Flush remaining text
            blocks.push(MarkdownBlock::Text {
                content: pending_text,
                start: pending_text_start.unwrap_or(0),
                end: text.len(),
            });
        }

        blocks
    }
}

impl ChunkAlgorithm for MarkdownChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let blocks = Self::parse_blocks(text);
        let mut chunks = Vec::new();
        let mut current_section: Option<String> = None;
        let mut current_text = String::new();
        let mut current_start = 0;
        let mut chunk_start_set = false;

        for block in blocks {
            match block {
                MarkdownBlock::Heading {
                    content,
                    level,
                    start,
                    end: _,
                } => {
                    // Flush current chunk before new section
                    if !current_text.is_empty() {
                        let metadata = ChunkMetadata {
                            method: self.name().to_string(),
                            section: current_section.clone(),
                            overlap_chars: None,
                            parent_chunk_id: None,
                        };
                        chunks.push(Chunk::with_uuid(
                            current_text.trim().to_string(),
                            current_start,
                            start,
                            metadata,
                        ));
                        current_text.clear();
                        chunk_start_set = false;
                    }

                    // Update current section
                    current_section = Some(format!("h{}: {}", level, content));

                    // Add heading to next chunk
                    if !chunk_start_set {
                        current_start = start;
                        chunk_start_set = true;
                    }
                    current_text.push_str(&"#".repeat(level));
                    current_text.push(' ');
                    current_text.push_str(&content);
                    current_text.push('\n');
                }
                MarkdownBlock::CodeBlock {
                    content,
                    start,
                    end,
                    ..
                } => {
                    // Code blocks are atomic - check if we need to flush first
                    let potential_len = current_text.len() + content.len();

                    if potential_len > config.max_size && !current_text.is_empty() {
                        // Flush current chunk
                        let metadata = ChunkMetadata {
                            method: self.name().to_string(),
                            section: current_section.clone(),
                            overlap_chars: None,
                            parent_chunk_id: None,
                        };
                        chunks.push(Chunk::with_uuid(
                            current_text.trim().to_string(),
                            current_start,
                            start,
                            metadata,
                        ));
                        current_text.clear();
                        chunk_start_set = false;
                    }

                    if !chunk_start_set {
                        current_start = start;
                        chunk_start_set = true;
                    }

                    // If code block alone exceeds max_size, it becomes its own chunk
                    if content.len() > config.max_size {
                        if !current_text.is_empty() {
                            let metadata = ChunkMetadata {
                                method: self.name().to_string(),
                                section: current_section.clone(),
                                overlap_chars: None,
                                parent_chunk_id: None,
                            };
                            chunks.push(Chunk::with_uuid(
                                current_text.trim().to_string(),
                                current_start,
                                start,
                                metadata,
                            ));
                            current_text.clear();
                        }

                        let metadata = ChunkMetadata {
                            method: self.name().to_string(),
                            section: current_section.clone(),
                            overlap_chars: None,
                            parent_chunk_id: None,
                        };
                        chunks.push(Chunk::with_uuid(content, start, end, metadata));
                        chunk_start_set = false;
                    } else {
                        current_text.push_str(&content);
                        current_text.push('\n');
                    }
                }
                MarkdownBlock::Text {
                    content,
                    start,
                    end: _,
                } => {
                    let potential_len = current_text.len() + content.len();

                    if potential_len > config.max_size && !current_text.is_empty() {
                        // Flush current chunk
                        let metadata = ChunkMetadata {
                            method: self.name().to_string(),
                            section: current_section.clone(),
                            overlap_chars: None,
                            parent_chunk_id: None,
                        };
                        chunks.push(Chunk::with_uuid(
                            current_text.trim().to_string(),
                            current_start,
                            start,
                            metadata,
                        ));
                        current_text.clear();
                        chunk_start_set = false;
                    }

                    if !chunk_start_set {
                        current_start = start;
                        chunk_start_set = true;
                    }
                    current_text.push_str(&content);
                    current_text.push('\n');
                }
            }
        }

        // Flush remaining content
        if !current_text.is_empty() {
            let metadata = ChunkMetadata {
                method: self.name().to_string(),
                section: current_section,
                overlap_chars: None,
                parent_chunk_id: None,
            };
            chunks.push(Chunk::with_uuid(
                current_text.trim().to_string(),
                current_start,
                text.len(),
                metadata,
            ));
        }

        chunks
    }

    fn name(&self) -> &str {
        "markdown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_code_block_preserved() {
        let chunker = MarkdownChunker;
        let config = ChunkConfig::new(1000);
        let text = r#"# Introduction

Some text here.

```python
def hello():
    print("Hello, World!")
```

More text after code.
"#;
        let chunks = chunker.chunk(text, &config);

        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].text.contains("```python"));
        assert!(chunks[0].text.contains("def hello():"));
    }

    #[test]
    fn test_markdown_split_at_heading() {
        let chunker = MarkdownChunker;
        let config = ChunkConfig::new(50);
        let text = r#"# First Section

Content of first section.

# Second Section

Content of second section.
"#;
        let chunks = chunker.chunk(text, &config);

        assert!(chunks.len() >= 2);
        assert!(chunks[0]
            .metadata
            .section
            .as_ref()
            .unwrap()
            .contains("First"));
    }

    #[test]
    fn test_markdown_empty() {
        let chunker = MarkdownChunker;
        let config = ChunkConfig::new(100);
        let chunks = chunker.chunk("", &config);

        assert!(chunks.is_empty());
    }

    #[test]
    fn test_markdown_section_tracking() {
        let chunker = MarkdownChunker;
        let config = ChunkConfig::new(1000);
        let text = "## My Section\n\nSome content here.";
        let chunks = chunker.chunk(text, &config);

        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks[0].metadata.section,
            Some("h2: My Section".to_string())
        );
    }
}
