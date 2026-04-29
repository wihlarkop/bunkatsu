"""Tests for embedding-based and LLM-based chunking."""

from bunkatsu import EmbeddedChunk


def mock_embedder(texts):
    """Returns simple mock embeddings based on first character."""
    return [
        [ord(t[0]) / 128.0 if t else 0.0, 0.5]
        for t in texts
    ]


def mock_llm(prompt):
    """Returns two propositions per call."""
    return "Proposition one.\nProposition two."


class TestSemantic:
    def test_produces_chunks(self, chunker):
        text = "Hello world. How are you? Alpha beta gamma."
        chunks = chunker.chunk_semantic(text, embedding_fn=mock_embedder, threshold=0.99)
        assert len(chunks) >= 1
        assert all(c.metadata.method == "semantic" for c in chunks)

    def test_empty(self, chunker):
        assert chunker.chunk_semantic("", embedding_fn=mock_embedder) == []


class TestKamradt:
    def test_produces_chunks(self, chunker):
        text = "Hello world. How are you? Alpha beta gamma."
        chunks = chunker.chunk_kamradt(text, embedding_fn=mock_embedder, percentile=50.0)
        assert len(chunks) >= 1
        assert all(c.metadata.method == "kamradt" for c in chunks)

    def test_empty(self, chunker):
        assert chunker.chunk_kamradt("", embedding_fn=mock_embedder) == []


class TestProposition:
    def test_produces_propositions(self, chunker):
        text = "Hello world. How are you?"
        chunks = chunker.chunk_proposition(text, llm_fn=mock_llm)
        assert len(chunks) >= 2
        assert all(c.metadata.method == "proposition" for c in chunks)

    def test_empty(self, chunker):
        assert chunker.chunk_proposition("", llm_fn=mock_llm) == []


class TestLateChunking:
    def test_returns_embedded_chunks(self, chunker):
        text = "Hello world. This is a test sentence for late chunking."
        embedded = chunker.chunk_late(text, embedding_fn=mock_embedder, max_size=20)
        assert len(embedded) >= 1
        assert all(isinstance(e, EmbeddedChunk) for e in embedded)
        assert all(len(e.embedding) > 0 for e in embedded)

    def test_empty(self, chunker):
        assert chunker.chunk_late("", embedding_fn=mock_embedder) == []
