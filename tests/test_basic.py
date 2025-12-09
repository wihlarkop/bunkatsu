"""Tests for basic chunking algorithms (v0.1)."""

from bunkatsu import SentenceDetector


class TestFixedSize:
    def test_basic(self, chunker):
        chunks = chunker.chunk_fixed("hello world", 5)
        assert len(chunks) == 3
        assert chunks[0].text == "hello"

    def test_empty(self, chunker):
        assert len(chunker.chunk_fixed("", 10)) == 0

    def test_metadata(self, chunker):
        chunks = chunker.chunk_fixed("hello", 10)
        assert chunks[0].metadata.method == "fixed_size"


class TestSlidingWindow:
    def test_overlap(self, chunker):
        chunks = chunker.chunk_sliding("hello world!", 5, 2)
        assert chunks[0].metadata.overlap_chars is None
        assert chunks[1].metadata.overlap_chars == 2


class TestSentence:
    def test_split(self, chunker):
        chunks = chunker.chunk_sentences("Hello. World.", 10)
        assert len(chunks) >= 1

    def test_unicode_detector(self, chunker):
        chunks = chunker.chunk_sentences("Hi.", 100, SentenceDetector.Unicode)
        assert len(chunks) >= 1


class TestParagraph:
    def test_split(self, chunker):
        chunks = chunker.chunk_paragraphs("A.\n\nB.", 10)
        assert len(chunks) >= 1
