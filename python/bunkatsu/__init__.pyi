"""Type stubs for bunkatsu."""

from typing import Callable, Optional


class ChunkMetadata:
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


class EmbeddedChunk:
    chunk: Chunk
    embedding: list[float]

    def __repr__(self) -> str: ...


class SentenceDetector:
    Regex: SentenceDetector
    Unicode: SentenceDetector


class CodeLanguage:
    Auto: CodeLanguage
    Python: CodeLanguage
    JavaScript: CodeLanguage
    TypeScript: CodeLanguage
    Rust: CodeLanguage
    Go: CodeLanguage
    Java: CodeLanguage
    C: CodeLanguage
    Cpp: CodeLanguage
    CSharp: CodeLanguage
    PHP: CodeLanguage
    Ruby: CodeLanguage
    Swift: CodeLanguage
    Generic: CodeLanguage


class Chunker:
    def __init__(self) -> None: ...

    # --- Pure-Rust algorithms ---

    def chunk_fixed(self, text: str, max_size: int = 512) -> list[Chunk]: ...

    def chunk_sliding(
        self, text: str, max_size: int = 512, overlap: int = 64
    ) -> list[Chunk]: ...

    def chunk_sentences(
        self,
        text: str,
        max_size: int = 512,
        overlap: int = 0,
        detector: SentenceDetector = ...,
    ) -> list[Chunk]: ...

    def chunk_paragraphs(
        self, text: str, max_size: int = 512, overlap: int = 0
    ) -> list[Chunk]: ...

    def chunk_markdown(self, text: str, max_size: int = 1000) -> list[Chunk]: ...

    def chunk_headings(
        self,
        text: str,
        max_size: int = 1000,
        levels: list[int] | None = None,
    ) -> list[Chunk]: ...

    def chunk_recursive(
        self, text: str, max_size: int = 512, overlap: int = 0
    ) -> list[Chunk]: ...

    def chunk_tokens(
        self,
        text: str,
        max_tokens: int = 512,
        tokenizer_fn: Callable[[str], int] | None = None,
    ) -> list[Chunk]: ...

    def chunk_html(self, text: str, max_size: int = 1000) -> list[Chunk]: ...

    def chunk_json(self, text: str, max_size: int = 1000) -> list[Chunk]: ...

    def chunk_csv(self, text: str, rows_per_chunk: int = 50) -> list[Chunk]: ...

    def chunk_latex(self, text: str, max_size: int = 1000) -> list[Chunk]: ...

    def chunk_code(
        self,
        text: str,
        max_size: int = 1000,
        language: CodeLanguage = ...,
    ) -> list[Chunk]: ...

    def chunk_hierarchical(self, text: str, max_size: int = 512) -> list[Chunk]: ...

    def chunk_hybrid(
        self,
        text: str,
        max_size: int = 512,
        strategies: list[str] | None = None,
    ) -> list[Chunk]: ...

    # --- Embedding-based algorithms ---

    def chunk_semantic(
        self,
        text: str,
        embedding_fn: Callable[[list[str]], list[list[float]]],
        threshold: float = 0.5,
    ) -> list[Chunk]: ...

    def chunk_kamradt(
        self,
        text: str,
        embedding_fn: Callable[[list[str]], list[list[float]]],
        percentile: float = 95.0,
    ) -> list[Chunk]: ...

    def chunk_proposition(
        self,
        text: str,
        llm_fn: Callable[[str], str],
    ) -> list[Chunk]: ...

    def chunk_late(
        self,
        text: str,
        embedding_fn: Callable[[list[str]], list[list[float]]],
        max_size: int = 512,
    ) -> list[EmbeddedChunk]: ...

    # --- Dynamic dispatch ---

    def chunk_by_name(
        self, text: str, method: str, max_size: int = 512, overlap: int = 0
    ) -> list[Chunk]:
        """Raises ValueError if method is not found."""
        ...

    def available_methods(self) -> list[str]: ...

    def available_methods(self) -> list[str]: ...
