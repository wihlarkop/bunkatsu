"""Tests for Chunk object properties."""


class TestChunk:
    def test_has_id(self, chunker):
        chunks = chunker.chunk_fixed("test", 10)
        assert chunks[0].id is not None

    def test_positions(self, chunker):
        chunks = chunker.chunk_fixed("hello", 10)
        assert chunks[0].start == 0
        assert chunks[0].end == 5

    def test_len(self, chunker):
        chunks = chunker.chunk_fixed("hello", 10)
        assert len(chunks[0]) == 5


class TestChunkerMethods:
    def test_available_methods(self, chunker):
        methods = chunker.available_methods()
        expected = [
            "fixed_size",
            "sliding_window",
            "sentence",
            "paragraph",
            "markdown",
            "heading",
            "recursive",
        ]
        for m in expected:
            assert m in methods
