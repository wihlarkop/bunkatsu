"""Tests for token-based chunking."""


def word_count(text):
    return len(text.split())


class TestToken:
    def test_splits_by_token_budget(self, chunker):
        text = "Hello world. How are you? I am fine today."
        chunks = chunker.chunk_tokens(text, tokenizer_fn=word_count, max_tokens=3)
        assert len(chunks) >= 2
        assert all(c.metadata.method == "token" for c in chunks)

    def test_empty_text(self, chunker):
        assert chunker.chunk_tokens("", tokenizer_fn=word_count, max_tokens=10) == []

    def test_single_chunk_when_budget_large(self, chunker):
        chunks = chunker.chunk_tokens("Short text here.", tokenizer_fn=word_count, max_tokens=1000)
        assert len(chunks) == 1
