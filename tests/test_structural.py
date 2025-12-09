"""Tests for structural chunking algorithms (v0.2)."""


class TestMarkdown:
    def test_code_block_preserved(self, chunker):
        md = "# Intro\n\n```python\ndef foo(): pass\n```"
        chunks = chunker.chunk_markdown(md, 1000)
        assert "```python" in chunks[0].text

    def test_section_tracking(self, chunker):
        chunks = chunker.chunk_markdown("## Section\n\nText.", 1000)
        assert chunks[0].metadata.section == "h2: Section"


class TestHeading:
    def test_split_at_headings(self, chunker):
        text = "# One\n\nA.\n\n# Two\n\nB."
        chunks = chunker.chunk_headings(text, 1000)
        assert len(chunks) == 2


class TestRecursive:
    def test_paragraph_fallback(self, chunker):
        text = "First paragraph here.\n\nSecond paragraph here.\n\nThird paragraph."
        chunks = chunker.chunk_recursive(text, 30)
        assert len(chunks) >= 2
        assert "recursive" in chunks[0].metadata.method
