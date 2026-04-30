# Bunkatsu (分割)

Rust-core text chunking library for Python — 3–98× faster than pure-Python chunkers, zero external dependencies for pure-text algorithms, 19 strategies in one API.

```bash
uv add bunkatsu
```

**Algorithms you won't find elsewhere in one package:** CSV chunking with header preservation, LaTeX section splitting, Hierarchical chunks with parent–child IDs, and a bring-your-own embedder/LLM pattern for Semantic, Kamradt, Proposition, and Late Chunking.

## Quick Start

```python
from bunkatsu import Chunker

chunker = Chunker()

# Fixed-size
chunks = chunker.chunk_fixed("Your long document...", max_size=512)

# Sentence-aware
chunks = chunker.chunk_sentences("Hello world. How are you?", max_size=512)

# Markdown (preserves code blocks, splits at headings)
chunks = chunker.chunk_markdown(markdown_text, max_size=1000)

# Token-based — bring your own tokenizer
import tiktoken
enc = tiktoken.get_encoding("cl100k_base")
chunks = chunker.chunk_tokens(text, max_tokens=512, tokenizer_fn=lambda s: len(enc.encode(s)))

# Code-aware (Python, Rust, JS, Go, …)
chunks = chunker.chunk_code(source_code, max_size=512)

# CSV — N rows per chunk, header preserved in every chunk
chunks = chunker.chunk_csv(csv_text, rows_per_chunk=50)

# Hierarchical — paragraph parents + sentence children with linked IDs
chunks = chunker.chunk_hierarchical(text, max_size=512)
for chunk in chunks:
    print(chunk.metadata.parent_chunk_id)  # links child → parent

# Semantic — bring your own embedder
def embed(texts: list[str]) -> list[list[float]]: ...
chunks = chunker.chunk_semantic(text, embedding_fn=embed, threshold=0.5)

# Kamradt percentile chunking
chunks = chunker.chunk_kamradt(text, embedding_fn=embed, percentile=95.0)

# Proposition — LLM decomposes text into atomic claims
def llm(prompt: str) -> str: ...
chunks = chunker.chunk_proposition(text, llm_fn=llm)

# Dynamic dispatch by name
chunks = chunker.chunk_by_name(text, "recursive", max_size=512)
```

Every chunk carries position and metadata:

```python
for chunk in chunks:
    chunk.id        # UUID
    chunk.text      # content
    chunk.start     # byte offset in original text
    chunk.end
    chunk.metadata  # method, section, overlap_chars, parent_chunk_id
```

## Algorithms

| Method | Function | Description |
|--------|----------|-------------|
| Fixed Size | `chunk_fixed(text, max_size)` | Split every N characters |
| Sliding Window | `chunk_sliding(text, max_size, overlap)` | Overlapping windows |
| Sentence | `chunk_sentences(text, max_size)` | Respect sentence boundaries |
| Paragraph | `chunk_paragraphs(text, max_size)` | Respect paragraph boundaries |
| Markdown | `chunk_markdown(text, max_size)` | Preserve code blocks, split at headings |
| Heading | `chunk_headings(text, max_size)` | Split at `#`, `##`, `###` markers |
| Recursive | `chunk_recursive(text, max_size)` | Cascade: paragraph → sentence → fixed |
| Token | `chunk_tokens(text, max_tokens, tokenizer_fn)` | Split by token count (BYO tokenizer) |
| HTML | `chunk_html(text, max_size)` | Split at block tags, strip markup |
| JSON | `chunk_json(text, max_size)` | Split JSON arrays/objects by size |
| CSV | `chunk_csv(text, rows_per_chunk)` | N rows per chunk, header preserved |
| LaTeX | `chunk_latex(text, max_size)` | Split at `\section`, `\subsection`, etc. |
| Code | `chunk_code(text, max_size)` | Split at function/class boundaries |
| Hierarchical | `chunk_hierarchical(text, max_size)` | Paragraph parents + sentence children with IDs |
| Hybrid | `chunk_hybrid(text, max_size)` | Cascade strategies with oversized fallback |
| Semantic | `chunk_semantic(text, embedding_fn, threshold)` | Split where cosine similarity drops |
| Kamradt | `chunk_kamradt(text, embedding_fn, percentile)` | Split at Nth percentile embedding distance |
| Proposition | `chunk_proposition(text, llm_fn)` | LLM decomposes text into atomic claims |
| Late Chunking | `chunk_late(text, embedding_fn, max_size)` | Fixed chunks with pooled embeddings |

All pure-Rust algorithms are also available via `chunk_by_name(text, name, max_size)`.

## Benchmark

Tested against langchain-text-splitters, chonkie, and semantic-text-splitter on 250 KB of plain text. Averaged over 10 runs, Python 3.11, release build.

| Algorithm | bunkatsu | Competitor | Speedup |
|-----------|----------|------------|---------|
| Fixed-size | 0.64 ms | langchain 2.1 ms | **3.2× faster** |
| Sentence | 0.75 ms | chonkie 73 ms | **98× faster** |
| Recursive | 2.1 ms | langchain 4.1 ms | **1.9× faster** |
| Token (tiktoken) | 43 ms | chonkie 40 ms | ~1× (parity) |
| Token (tiktoken) | 43 ms | semantic-text-splitter 122 ms | **2.9× faster** |
| Markdown | 0.31 ms | langchain 0.54 ms | **1.7× faster** |
| Code | 0.02 ms | langchain 0.63 ms | **26× faster** |
| Code | 0.02 ms | semantic-text-splitter 8.1 ms | **338× faster** |
| HTML | 0.15 ms | langchain 7.5 ms | **50× faster** |

Token chunking is dominated by the tokenizer call (tiktoken), so bunkatsu and chonkie are at parity. Without an external tokenizer, bunkatsu's word-count mode runs at ~93 MB/s.

## Installation

```bash
# From PyPI
uv add bunkatsu

# From source (requires Rust + maturin)
uv tool install maturin
maturin develop --release
```

## License

MIT
