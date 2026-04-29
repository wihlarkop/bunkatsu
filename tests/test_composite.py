"""Tests for hierarchical and hybrid chunking."""


class TestHierarchical:
    def test_parent_child_structure(self, chunker):
        text = "First paragraph. It has sentences.\n\nSecond paragraph. Also sentences."
        chunks = chunker.chunk_hierarchical(text, max_size=1000)

        parents = [c for c in chunks if c.metadata.method == "hierarchical_parent"]
        children = [c for c in chunks if c.metadata.method == "hierarchical_child"]

        assert len(parents) >= 1
        assert len(children) >= 1
        assert all(c.metadata.parent_chunk_id is not None for c in children)

    def test_empty(self, chunker):
        assert chunker.chunk_hierarchical("", max_size=1000) == []


class TestHybrid:
    def test_basic(self, chunker):
        text = "First paragraph.\n\nSecond paragraph that is a bit longer.\n\nThird."
        chunks = chunker.chunk_hybrid(text, max_size=50)
        assert len(chunks) >= 1
        assert all(c.metadata.method == "hybrid" for c in chunks)

    def test_empty(self, chunker):
        assert chunker.chunk_hybrid("", max_size=100) == []


class TestChunkByName:
    def test_dispatch_fixed(self, chunker):
        chunks = chunker.chunk_by_name("Hello world.", "fixed_size", max_size=5)
        assert len(chunks) >= 1

    def test_unknown_method(self, chunker):
        chunks = chunker.chunk_by_name("text", "nonexistent_method", max_size=512)
        assert chunks == []

    def test_all_methods_registered(self, chunker):
        methods = chunker.available_methods()
        assert "fixed_size" in methods
        assert "semantic" in methods
        assert "hierarchical" in methods
        assert len(methods) >= 15
