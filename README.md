# Bunkatsu (分割)

Universal High-Performance Text Chunking Library

A Rust-core, Python-first text chunking library designed for RAG, NLP, and Document AI systems.

## Features

- 🚀 **High performance** - Rust core with PyO3 bindings
- 🐍 **Python-first API** - Clean, intuitive interface
- 📝 **Multiple strategies** - 7 chunking algorithms
- 🔓 **No embedding coupling** - Pure chunking, no dependencies

## Supported Algorithms

### Basic Chunking (v0.1) ✅
- [x] **Fixed Size** - `chunk_fixed()` - Split by character count
- [x] **Sliding Window** - `chunk_sliding()` - Overlapping chunks
- [x] **Sentence** - `chunk_sentences()` - Split at sentence boundaries
- [x] **Paragraph** - `chunk_paragraphs()` - Split at paragraph boundaries

### Structural Chunking (v0.2) ✅
- [x] **Markdown** - `chunk_markdown()` - Preserve code blocks, split at headings
- [x] **Heading** - `chunk_headings()` - Split by heading levels (#, ##, ###)
- [x] **Recursive** - `chunk_recursive()` - Multi-level: paragraph → sentence → fixed

### Advanced Chunking (v0.3) 🚧
- [ ] **Token-based** - `chunk_tokens()` - Split by token count with callback
- [ ] **Semantic** - Split by topic/meaning changes
- [ ] **Hybrid** - Combine multiple strategies

## Installation

```bash
# Development
maturin develop

# Build wheel
maturin build --release
```

## Quick Start

```python
from bunkatsu import Chunker, SentenceDetector

chunker = Chunker()

# Fixed-size chunking
chunks = chunker.chunk_fixed("Your long text...", max_size=512)

# Sliding window with overlap
chunks = chunker.chunk_sliding("Your text...", max_size=512, overlap=64)

# Sentence-based
chunks = chunker.chunk_sentences("Hello world. How are you?", max_size=512)

# Markdown-aware (preserves code blocks)
chunks = chunker.chunk_markdown(markdown_text, max_size=1000)

# Recursive (paragraph → sentence → fixed fallback)
chunks = chunker.chunk_recursive(text, max_size=500)

# Each chunk has:
for chunk in chunks:
    print(chunk.id)        # Unique UUID
    print(chunk.text)      # Chunk content
    print(chunk.start)     # Start position
    print(chunk.end)       # End position
    print(chunk.metadata)  # Method, section, overlap info
```

## License

MIT