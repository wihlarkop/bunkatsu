"""Tests for format-specific chunking: HTML, JSON, CSV, LaTeX."""


class TestHTML:
    def test_strips_tags(self, chunker):
        chunks = chunker.chunk_html("<p>Hello world.</p>", max_size=1000)
        assert chunks
        assert "<" not in chunks[0].text

    def test_empty(self, chunker):
        assert chunker.chunk_html("", max_size=1000) == []

    def test_plain_text(self, chunker):
        chunks = chunker.chunk_html("No HTML here.", max_size=1000)
        assert len(chunks) == 1


class TestJSON:
    def test_array_split(self, chunker):
        json = '[{"a":1},{"b":2},{"c":3},{"d":4},{"e":5}]'
        chunks = chunker.chunk_json(json, max_size=30)
        assert len(chunks) >= 2
        assert all(c.metadata.method == "json" for c in chunks)

    def test_empty(self, chunker):
        assert chunker.chunk_json("", max_size=1000) == []

    def test_single_chunk_small(self, chunker):
        chunks = chunker.chunk_json('[{"a":1}]', max_size=1000)
        assert len(chunks) == 1


class TestCSV:
    def test_preserves_header(self, chunker):
        csv = "name,age\nAlice,30\nBob,25\nCarol,35"
        chunks = chunker.chunk_csv(csv, rows_per_chunk=2)
        assert len(chunks) == 2
        assert all(c.text.startswith("name,age") for c in chunks)

    def test_empty(self, chunker):
        assert chunker.chunk_csv("", rows_per_chunk=10) == []


class TestLaTeX:
    def test_splits_at_section(self, chunker):
        tex = r"\section{Intro}" + "\nHello.\n" + r"\section{Methods}" + "\nWorld."
        chunks = chunker.chunk_latex(tex, max_size=1000)
        assert len(chunks) >= 2

    def test_empty(self, chunker):
        assert chunker.chunk_latex("", max_size=1000) == []

    def test_no_sections(self, chunker):
        chunks = chunker.chunk_latex("Just plain text.", max_size=1000)
        assert len(chunks) == 1
