"""
Benchmark bunkatsu against comparable chunking libraries.
Only tests algorithms present in both bunkatsu and the competitor.

Usage:
    pip install -r requirements.txt
    python bench.py
"""

import timeit
import sys
from dataclasses import dataclass
from typing import Callable

from corpus import (
    PLAIN_TEXT_SMALL,
    PLAIN_TEXT_MEDIUM,
    PLAIN_TEXT_LARGE,
    MARKDOWN_TEXT,
    MARKDOWN_TEXT_MEDIUM,
    CODE_TEXT,
    CODE_TEXT_MEDIUM,
    HTML_TEXT,
    HTML_TEXT_MEDIUM,
)

# ---------------------------------------------------------------------------
# Result types
# ---------------------------------------------------------------------------

@dataclass
class BenchResult:
    library: str
    group: str      # shared algorithm group (e.g. "fixed_size", "sentence")
    algorithm: str  # display name for this library's implementation
    text_size: str
    chars: int
    time_ms: float
    chunk_count: int

    @property
    def throughput_mb_s(self) -> float:
        if self.time_ms == 0:
            return 0.0
        return (self.chars / 1_000_000) / (self.time_ms / 1000)


RESULTS: list[BenchResult] = []

# ---------------------------------------------------------------------------
# Benchmark runner
# ---------------------------------------------------------------------------

NUMBER = 10  # timeit inner loops


def bench(library: str, group: str, algorithm: str, text_size: str, text: str, fn: Callable) -> None:
    try:
        result = fn()
        chunk_count = len(result) if result is not None else 0

        elapsed = timeit.timeit(fn, number=NUMBER, globals={}) / NUMBER
        time_ms = elapsed * 1000

        RESULTS.append(BenchResult(
            library=library,
            group=group,
            algorithm=algorithm,
            text_size=text_size,
            chars=len(text),
            time_ms=time_ms,
            chunk_count=chunk_count,
        ))
        print(f"  [{library:28s}] {algorithm:32s} {text_size:8s}  {time_ms:8.2f}ms  {chunk_count} chunks")
    except Exception as e:
        print(f"  [{library:28s}] {algorithm:32s} {text_size:8s}  ERROR: {e}")


# ---------------------------------------------------------------------------
# Import libraries (with availability checks)
# ---------------------------------------------------------------------------

def import_bunkatsu():
    import bunkatsu
    return bunkatsu.Chunker()

def import_langchain():
    try:
        from langchain_text_splitters import (
            CharacterTextSplitter,
            RecursiveCharacterTextSplitter,
            MarkdownTextSplitter,
            HTMLHeaderTextSplitter,
        )
        return True
    except ImportError:
        return False

def import_chonkie():
    try:
        import chonkie
        return True
    except ImportError:
        return False

def import_semantic_text_splitter():
    try:
        import semantic_text_splitter
        return True
    except ImportError:
        return False

def import_semchunk():
    try:
        import semchunk
        return True
    except ImportError:
        return False

# ---------------------------------------------------------------------------
# Benchmark sections
# ---------------------------------------------------------------------------

def run_fixed_size(chunker, available: dict):
    """Fixed/character chunking."""
    print("\n=== Fixed-size / Character chunking ===")
    MAX_SIZE = 512

    for label, text in [("small", PLAIN_TEXT_SMALL), ("medium", PLAIN_TEXT_MEDIUM), ("large", PLAIN_TEXT_LARGE)]:
        bench("bunkatsu", "fixed_size", "bunkatsu.chunk_fixed", label, text,
              lambda t=text: chunker.chunk_fixed(t, MAX_SIZE))

        if available.get("langchain"):
            from langchain_text_splitters import CharacterTextSplitter
            splitter = CharacterTextSplitter(chunk_size=MAX_SIZE, chunk_overlap=0, separator="\n")
            bench("langchain", "fixed_size", "CharacterTextSplitter", label, text,
                  lambda t=text, s=splitter: s.split_text(t))


def run_sentence(chunker, available: dict):
    """Sentence-based chunking."""
    print("\n=== Sentence chunking ===")
    MAX_SIZE = 512

    for label, text in [("small", PLAIN_TEXT_SMALL), ("medium", PLAIN_TEXT_MEDIUM), ("large", PLAIN_TEXT_LARGE)]:
        bench("bunkatsu", "sentence", "bunkatsu.chunk_sentences", label, text,
              lambda t=text: chunker.chunk_sentences(t, MAX_SIZE))

        if available.get("chonkie"):
            from chonkie import SentenceChunker
            ck = SentenceChunker(chunk_size=MAX_SIZE)
            bench("chonkie", "sentence", "SentenceChunker", label, text,
                  lambda t=text, c=ck: c.chunk(t))


def run_recursive(chunker, available: dict):
    """Recursive chunking."""
    print("\n=== Recursive chunking ===")
    MAX_SIZE = 512

    for label, text in [("small", PLAIN_TEXT_SMALL), ("medium", PLAIN_TEXT_MEDIUM), ("large", PLAIN_TEXT_LARGE)]:
        bench("bunkatsu", "recursive", "bunkatsu.chunk_recursive", label, text,
              lambda t=text: chunker.chunk_recursive(t, MAX_SIZE))

        if available.get("langchain"):
            from langchain_text_splitters import RecursiveCharacterTextSplitter
            splitter = RecursiveCharacterTextSplitter(chunk_size=MAX_SIZE, chunk_overlap=0)
            bench("langchain", "recursive", "RecursiveCharacterTextSplitter", label, text,
                  lambda t=text, s=splitter: s.split_text(t))

        if available.get("chonkie"):
            from chonkie import RecursiveChunker
            ck = RecursiveChunker(chunk_size=MAX_SIZE)
            bench("chonkie", "recursive", "RecursiveChunker", label, text,
                  lambda t=text, c=ck: c.chunk(t))


def run_token(chunker, available: dict):
    """Token-based chunking."""
    print("\n=== Token chunking (max_tokens=128, tiktoken cl100k_base) ===")
    MAX_TOKENS = 128

    try:
        import tiktoken
        enc = tiktoken.get_encoding("cl100k_base")
        def tiktoken_count(s: str) -> int:
            return len(enc.encode(s))
    except ImportError:
        tiktoken_count = None

    for label, text in [("small", PLAIN_TEXT_SMALL), ("medium", PLAIN_TEXT_MEDIUM), ("large", PLAIN_TEXT_LARGE)]:
        if tiktoken_count:
            bench("bunkatsu", "token", "bunkatsu.chunk_tokens(tiktoken)", label, text,
                  lambda t=text: chunker.chunk_tokens(t, MAX_TOKENS, tokenizer_fn=tiktoken_count))
        bench("bunkatsu(words)", "token_words", "bunkatsu.chunk_tokens(words)", label, text,
              lambda t=text: chunker.chunk_tokens(t, MAX_TOKENS))

        if available.get("chonkie"):
            try:
                from chonkie import TokenChunker
                ck = TokenChunker(chunk_size=MAX_TOKENS)
                bench("chonkie", "token", "TokenChunker(tiktoken)", label, text,
                      lambda t=text, c=ck: c.chunk(t))
            except Exception as e:
                print(f"  chonkie TokenChunker {label}: SKIP: {e}")

        if available.get("semantic_text_splitter"):
            try:
                from semantic_text_splitter import TextSplitter
                splitter = TextSplitter.from_tiktoken_model("gpt-4", capacity=MAX_TOKENS)
                bench("semantic-text-splitter", "token", "TextSplitter(tiktoken)", label, text,
                      lambda t=text, s=splitter: s.chunks(t))
            except Exception as e:
                print(f"  semantic-text-splitter token {label}: SKIP: {e}")


def run_markdown(chunker, available: dict):
    """Markdown chunking."""
    print("\n=== Markdown chunking ===")
    MAX_SIZE = 512

    for label, text in [("small", MARKDOWN_TEXT), ("medium", MARKDOWN_TEXT_MEDIUM)]:
        bench("bunkatsu", "markdown", "bunkatsu.chunk_markdown", label, text,
              lambda t=text: chunker.chunk_markdown(t, MAX_SIZE))

        if available.get("langchain"):
            from langchain_text_splitters import MarkdownTextSplitter
            splitter = MarkdownTextSplitter(chunk_size=MAX_SIZE, chunk_overlap=0)
            bench("langchain", "markdown", "MarkdownTextSplitter", label, text,
                  lambda t=text, s=splitter: s.split_text(t))

        if available.get("semantic_text_splitter"):
            try:
                from semantic_text_splitter import MarkdownSplitter
                splitter = MarkdownSplitter(MAX_SIZE)
                bench("semantic-text-splitter", "markdown", "MarkdownSplitter", label, text,
                      lambda t=text, s=splitter: s.chunks(t))
            except Exception as e:
                print(f"  semantic-text-splitter MarkdownSplitter {label}: SKIP: {e}")


def run_code(chunker, available: dict):
    """Code chunking."""
    print("\n=== Code chunking (Rust) ===")
    MAX_SIZE = 512

    for label, text in [("small", CODE_TEXT), ("medium", CODE_TEXT_MEDIUM)]:
        bench("bunkatsu", "code", "bunkatsu.chunk_code(rust)", label, text,
              lambda t=text: chunker.chunk_code(t, MAX_SIZE))

        if available.get("semantic_text_splitter"):
            try:
                from semantic_text_splitter import CodeSplitter
                import tree_sitter_rust
                splitter = CodeSplitter(tree_sitter_rust.language(), MAX_SIZE)
                bench("semantic-text-splitter", "code", "CodeSplitter(rust)", label, text,
                      lambda t=text, s=splitter: s.chunks(t))
            except Exception as e:
                print(f"  semantic-text-splitter CodeSplitter(rust) {label}: SKIP: {e}")

        if available.get("langchain"):
            try:
                from langchain_text_splitters import RecursiveCharacterTextSplitter, Language
                splitter = RecursiveCharacterTextSplitter.from_language(Language.RUST, chunk_size=MAX_SIZE, chunk_overlap=0)
                bench("langchain", "code", "RecursiveCS(RUST)", label, text,
                      lambda t=text, s=splitter: s.split_text(t))
            except Exception as e:
                print(f"  langchain RecursiveCS(RUST) {label}: SKIP: {e}")


def run_html(chunker, available: dict):
    """HTML chunking."""
    print("\n=== HTML chunking ===")
    MAX_SIZE = 512

    for label, text in [("small", HTML_TEXT), ("medium", HTML_TEXT_MEDIUM)]:
        bench("bunkatsu", "html", "bunkatsu.chunk_html", label, text,
              lambda t=text: chunker.chunk_html(t, MAX_SIZE))

        if available.get("langchain"):
            try:
                from langchain_text_splitters import HTMLHeaderTextSplitter
                headers = [("h1", "h1"), ("h2", "h2"), ("h3", "h3")]
                splitter = HTMLHeaderTextSplitter(headers_to_split_on=headers)
                bench("langchain", "html", "HTMLHeaderTextSplitter", label, text,
                      lambda t=text, s=splitter: s.split_text(t))
            except Exception as e:
                print(f"  langchain HTMLHeaderTextSplitter {label}: SKIP: {e}")


# ---------------------------------------------------------------------------
# Summary table
# ---------------------------------------------------------------------------

def print_summary():
    SIZE_ORDER = {"small": 0, "medium": 1, "large": 2}

    # Group results by (group, text_size) for the comparison table
    by_group: dict[str, list[BenchResult]] = {}
    for r in RESULTS:
        by_group.setdefault(r.group, []).append(r)

    print("\n" + "=" * 110)
    print("SUMMARY  —  time in ms (lower is better)  |  MB/s throughput (higher is better)")
    print("=" * 110)

    for group, results in by_group.items():
        print(f"\n  [{group}]")
        print(f"  {'Implementation':40s} {'Size':8s} {'Time(ms)':>10s} {'MB/s':>10s} {'Chunks':>8s} {'vs bunkatsu':>14s}")
        print(f"  {'-'*40} {'-'*8} {'-'*10} {'-'*10} {'-'*8} {'-'*14}")

        bunkatsu_by_size = {r.text_size: r for r in results if r.library == "bunkatsu"}

        for r in sorted(results, key=lambda x: (SIZE_ORDER.get(x.text_size, 99), x.library)):
            bunk = bunkatsu_by_size.get(r.text_size)
            if r.library == "bunkatsu" or bunk is None or bunk.time_ms == 0:
                vs = ""
            else:
                ratio = r.time_ms / bunk.time_ms
                vs = f"{ratio:.1f}x slower" if ratio > 1 else f"{1/ratio:.1f}x faster"
            print(f"  {r.algorithm:40s} {r.text_size:8s} {r.time_ms:10.2f} {r.throughput_mb_s:10.2f} {r.chunk_count:8d} {vs:>14s}")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    print("Bunkatsu Chunking Benchmark")
    print("=" * 60)

    # Check availability
    available = {
        "langchain": import_langchain(),
        "chonkie": import_chonkie(),
        "semantic_text_splitter": import_semantic_text_splitter(),
        "semchunk": import_semchunk(),
    }
    print("Libraries available:")
    for name, ok in available.items():
        status = "OK" if ok else "MISSING (not installed)"
        print(f"  {name:30s} {status}")

    chunker = import_bunkatsu()

    run_fixed_size(chunker, available)
    run_sentence(chunker, available)
    run_recursive(chunker, available)
    run_token(chunker, available)
    run_markdown(chunker, available)
    run_code(chunker, available)
    run_html(chunker, available)

    print_summary()


if __name__ == "__main__":
    main()
