"""Shared fixtures for Bunkatsu tests."""

import pytest
from bunkatsu import Chunker


@pytest.fixture
def chunker():
    """Create a Chunker instance for tests."""
    return Chunker()
