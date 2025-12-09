"""
Bunkatsu (分割) - Universal High-Performance Text Chunking Library

A Rust-core, Python-first text chunking library designed for RAG, NLP, 
and Document AI systems.
"""

from bunkatsu._bunkatsu import Chunk, ChunkMetadata, Chunker, SentenceDetector

__all__ = [
    "Chunker",
    "Chunk", 
    "ChunkMetadata",
    "SentenceDetector",
]

__version__ = "0.1.0"
