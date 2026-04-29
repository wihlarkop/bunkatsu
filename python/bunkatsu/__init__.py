"""
Bunkatsu (分割) - Universal High-Performance Text Chunking Library

A Rust-core, Python-first text chunking library designed for RAG, NLP,
and Document AI systems.
"""

from bunkatsu._bunkatsu import (
    Chunk,
    Chunker,
    ChunkMetadata,
    CodeLanguage,
    EmbeddedChunk,
    SentenceDetector,
)

__all__ = [
    "Chunk",
    "ChunkMetadata",
    "Chunker",
    "CodeLanguage",
    "EmbeddedChunk",
    "SentenceDetector",
]

__version__ = "0.3.0"
