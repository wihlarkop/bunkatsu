# Bunkatsu (分割)

Universal High-Performance Text Chunking Library

A Rust-core, Python-first text chunking library designed for RAG, NLP, and Document AI systems.

## Features

- **High performance** — Rust core with PyO3 bindings
- **Python-first API** — Clean, intuitive interface
- **19 chunking algorithms** — From fixed-size to semantic/LLM-based
- **Embedding/LLM callbacks** — Bring your own embedder or LLM
- **Dynamic dispatch** — `chunk_by_name()` for runtime algorithm selection

## Supported Algorithms

### Basic Chunking (v0.1) ✅
| Method | Function | Description |
|--------|----------|-------------|
| Fixed Size | `chunk_fixed()` | Split by character count |
| Sliding Window | `chunk_sliding()` | Overlapping chunks |
| Sentence | `chunk_sentences()` | Split at sentence boundaries |
| Paragraph | `chunk_paragraphs()` | Split at paragraph boundaries |

### Structural Chunking (v0.2) ✅
| Method | Function | Description |
|--------|----------|-------------|
| Markdown | `chunk_markdown()` | Preserve code blocks, split at headings |
| Heading | `chunk_headings()` | Split by heading levels (#, ##, ###) |
| Recursive | `chunk_recursive()` | Multi-level: paragraph → sentence → fixed |

### Advanced Pure-Rust Chunking (v0.3) ✅
| Method | Function | Description |
|--------|----------|-------------|
| Token | `chunk_tokens()` | Split by token count with Python callback |
| HTML | `chunk_html()` | Split at block-level HTML tags, strip markup |
| JSON | `chunk_json()` | Split JSON arrays/objects by size |
| CSV | `chunk_csv()` | N rows per chunk, preserve header |
| LaTeX | `chunk_latex()` | Split at `\section`, `\subsection`, etc. |
| Code | `chunk_code()` | Split at function/class boundaries per language |
| Hierarchical | `chunk_hierarchical()` | Paragraph parents + sentence children |
| Hybrid | `chunk_hybrid()` | Cascade strategies with oversized fallback |

### Embedding & LLM-Based Chunking (v0.3) ✅
| Method | Function | Description |
|--------|----------|-------------|
| Semantic | `chunk_semantic()` | Split where cosine similarity drops below threshold |
| Kamradt | `chunk_kamradt()` | Split at Nth percentile distance between embeddings |
| Proposition | `chunk_proposition()` | LLM decomposes sentences into atomic claims |
| Late Chunking | `chunk_late()` | Fixed chunks with pooled token-level embeddings |

## Installation

```bash
# Development (requires maturin)
maturin develop

# Build wheel
maturin build --release
```

## Quick Start

```python
from bunkatsu import Chunker, CodeLanguage, SentenceDetector

chunker = Chunker()

# Fixed-size
chunks = chunker.chunk_fixed("Your long text...", max_size=512)

# Sliding window with overlap
chunks = chunker.chunk_sliding("Your text...", max_size=512, overlap=64)

# Sentence-based
chunks = chunker.chunk_sentences("Hello world. How are you?", max_size=512)

# Markdown-aware (preserves code blocks)
chunks = chunker.chunk_markdown(markdown_text, max_size=1000)

# Token-based with custom tokenizer
import tiktoken
enc = tiktoken.get_encoding("cl100k_base")
chunks = chunker.chunk_tokens(text, tokenizer_fn=lambda s: len(enc.encode(s)), max_tokens=512)

# Code-aware
chunks = chunker.chunk_code(python_code, language=CodeLanguage.Python)

# Hierarchical (parent paragraphs + child sentences)
chunks = chunker.chunk_hierarchical(text, max_size=512)

# Semantic (embedding-based)
def embed(texts: list[str]) -> list[list[float]]:
    # your embedder here
    ...

chunks = chunker.chunk_semantic(text, embedding_fn=embed, threshold=0.5)

# Kamradt percentile chunking
chunks = chunker.chunk_kamradt(text, embedding_fn=embed, percentile=95.0)

# Proposition chunking (LLM-based)
def llm(prompt: str) -> str:
    # your LLM here
    ...

chunks = chunker.chunk_proposition(text, llm_fn=llm)

# Late chunking (fixed chunks + embeddings)
from bunkatsu import EmbeddedChunk
embedded: list[EmbeddedChunk] = chunker.chunk_late(text, embedding_fn=embed, max_size=512)
for ec in embedded:
    print(ec.chunk.text, ec.embedding[:3])

# Dynamic dispatch
chunks = chunker.chunk_by_name(text, "sentence", max_size=512)

# Each chunk has:
for chunk in chunks:
    print(chunk.id)        # Unique UUID
    print(chunk.text)      # Chunk content
    print(chunk.start)     # Start byte position in original text
    print(chunk.end)       # End byte position
    print(chunk.metadata)  # method, section, overlap_chars, parent_chunk_id
```

## License

MIT
