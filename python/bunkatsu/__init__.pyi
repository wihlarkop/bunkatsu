"""Type stubs for bunkatsu."""

from typing import Optional

class ChunkMetadata:
    """Metadata associated with a chunk."""
    method: str
    section: Optional[str]
    overlap_chars: Optional[int]
    parent_chunk_id: Optional[str]
    
    def __init__(
        self,
        method: str,
        section: Optional[str] = None,
        overlap_chars: Optional[int] = None,
        parent_chunk_id: Optional[str] = None,
    ) -> None: ...
    
    def to_dict(self) -> dict: ...

class Chunk:
    """A text chunk with position and metadata."""
    id: str
    text: str
    start: int
    end: int
    metadata: ChunkMetadata
    
    def __init__(
        self,
        id: str,
        text: str,
        start: int,
        end: int,
        metadata: ChunkMetadata,
    ) -> None: ...
    
    @property
    def len(self) -> int: ...
    
    def __len__(self) -> int: ...

class SentenceDetector:
    """Sentence detection method."""
    Regex: "SentenceDetector"
    Unicode: "SentenceDetector"

class Chunker:
    """Main chunker class for text chunking operations."""
    
    def __init__(self) -> None: ...
    
    def chunk_fixed(self, text: str, max_size: int = 512) -> list[Chunk]:
        """Chunk text using fixed-size character-based chunking."""
        ...
    
    def chunk_sliding(
        self, text: str, max_size: int = 512, overlap: int = 64
    ) -> list[Chunk]:
        """Chunk text using sliding window with overlap."""
        ...
    
    def chunk_sentences(
        self,
        text: str,
        max_size: int = 512,
        detector: SentenceDetector = ...,
    ) -> list[Chunk]:
        """Chunk text by sentence boundaries."""
        ...
    
    def chunk_paragraphs(self, text: str, max_size: int = 512) -> list[Chunk]:
        """Chunk text by paragraph boundaries."""
        ...
    
    def available_methods(self) -> list[str]:
        """List available chunking methods."""
        ...
