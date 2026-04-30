"""Sample text corpus for benchmarks."""

PLAIN_TEXT_SMALL = """\
The quick brown fox jumps over the lazy dog. This is a simple sentence for testing.
Natural language processing is a subfield of artificial intelligence. It deals with
the interaction between computers and human language. Text chunking is the process
of splitting text into meaningful segments. Each segment is called a chunk.

Modern NLP systems rely on transformer architectures. These models require input
within a fixed context window. Chunking ensures documents fit within these limits.
Good chunking preserves semantic coherence within each chunk.

There are many strategies for text chunking. Fixed-size chunking splits by character
count. Sentence-based chunking respects sentence boundaries. Recursive chunking
applies multiple separators hierarchically. Semantic chunking uses embeddings to
find natural topic boundaries.
"""

# ~10KB plain text
PLAIN_TEXT_MEDIUM = (PLAIN_TEXT_SMALL * 30)

# ~100KB plain text
PLAIN_TEXT_LARGE = (PLAIN_TEXT_SMALL * 300)

MARKDOWN_TEXT = """\
# Introduction to Text Chunking

Text chunking is a fundamental technique in NLP pipelines.

## Why Chunking Matters

Language models have fixed context windows. Documents often exceed these limits.
Chunking splits documents into processable segments.

### Token Limits

Most models support 4096 to 128000 tokens. GPT-4 supports up to 128k tokens.
Claude supports up to 200k tokens.

## Chunking Strategies

There are several approaches to chunking text effectively.

### Fixed-Size Chunking

The simplest approach: split every N characters. Fast but ignores semantic boundaries.

```python
def fixed_chunk(text, size=512):
    return [text[i:i+size] for i in range(0, len(text), size)]
```

### Sentence-Based Chunking

Respects sentence boundaries. Better semantic coherence than fixed-size.

### Recursive Chunking

Tries multiple separators: paragraphs → sentences → words. Falls back to smaller
units when chunks exceed the maximum size.

## Performance Considerations

- Fixed-size: O(n), fastest
- Sentence-based: O(n), fast
- Semantic: O(n²), slowest due to embedding computation

## Conclusion

Choose chunking strategy based on document type and downstream task requirements.
"""

MARKDOWN_TEXT_MEDIUM = (MARKDOWN_TEXT * 15)

CODE_TEXT = """\
use std::collections::HashMap;

/// A text chunker that splits documents into segments.
pub struct Chunker {
    max_size: usize,
    overlap: usize,
}

impl Chunker {
    /// Creates a new Chunker with the given max chunk size.
    pub fn new(max_size: usize) -> Self {
        Self { max_size, overlap: 0 }
    }

    /// Sets the overlap between consecutive chunks.
    pub fn with_overlap(mut self, overlap: usize) -> Self {
        self.overlap = overlap;
        self
    }

    /// Chunks text by fixed character count.
    pub fn chunk_fixed(&self, text: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let step = if self.overlap < self.max_size {
            self.max_size - self.overlap
        } else {
            1
        };
        let chars: Vec<char> = text.chars().collect();
        let mut start = 0;
        while start < chars.len() {
            let end = (start + self.max_size).min(chars.len());
            chunks.push(chars[start..end].iter().collect());
            start += step;
        }
        chunks
    }

    /// Chunks text by sentence boundaries.
    pub fn chunk_sentences(&self, text: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current = String::new();
        for sentence in text.split(". ") {
            if current.len() + sentence.len() > self.max_size {
                if !current.is_empty() {
                    chunks.push(current.trim().to_string());
                    current = String::new();
                }
            }
            current.push_str(sentence);
            current.push_str(". ");
        }
        if !current.is_empty() {
            chunks.push(current.trim().to_string());
        }
        chunks
    }
}

fn main() {
    let chunker = Chunker::new(512).with_overlap(64);
    let text = "Hello world. This is a test. Chunking is useful.";
    let chunks = chunker.chunk_fixed(text);
    for (i, chunk) in chunks.iter().enumerate() {
        println!("Chunk {}: {}", i, chunk);
    }
}
"""

CODE_TEXT_MEDIUM = (CODE_TEXT * 10)

HTML_TEXT = """\
<!DOCTYPE html>
<html>
<head><title>Text Chunking Guide</title></head>
<body>
<h1>Text Chunking for AI Applications</h1>
<p>Text chunking is the process of splitting large documents into smaller segments.
This is essential for working with language models that have token limits.</p>

<h2>Why Chunk Text?</h2>
<p>Language models like GPT-4 and Claude have maximum context windows.
Documents that exceed these limits must be split before processing.</p>

<h3>Common Approaches</h3>
<p>There are several strategies for chunking text effectively:</p>
<ul>
<li>Fixed-size chunking: Split every N characters</li>
<li>Sentence-based: Respect sentence boundaries</li>
<li>Semantic chunking: Use embeddings to find topic shifts</li>
</ul>

<h2>Implementation</h2>
<p>Modern chunking libraries like bunkatsu provide multiple strategies.
Choose the right one based on your document type and task.</p>

<h3>Fixed Size</h3>
<p>The simplest approach. Fast but may split sentences awkwardly.</p>

<h3>Semantic</h3>
<p>Uses cosine similarity between sentence embeddings to detect topic shifts.
Produces semantically coherent chunks but requires an embedding model.</p>

<h2>Benchmarks</h2>
<p>Performance varies by strategy. Fixed-size is fastest. Semantic is slowest.</p>
</body>
</html>
"""

HTML_TEXT_MEDIUM = (HTML_TEXT * 10)
