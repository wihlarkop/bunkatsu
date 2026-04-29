"""Tests for code-aware chunking."""

from bunkatsu import CodeLanguage


class TestCode:
    def test_python_split(self, chunker):
        code = "def foo():\n    pass\n\ndef bar():\n    return 1"
        chunks = chunker.chunk_code(code, max_size=1000, language=CodeLanguage.Python)
        assert len(chunks) == 2
        assert all(c.metadata.method == "code" for c in chunks)

    def test_auto_detect(self, chunker):
        code = "pub fn hello() {\n    println!(\"hi\");\n}\n\npub fn world() {}\n"
        chunks = chunker.chunk_code(code, max_size=1000)
        assert len(chunks) >= 1

    def test_empty(self, chunker):
        assert chunker.chunk_code("", max_size=1000) == []
