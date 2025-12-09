//! Error types for Bunkatsu chunking library.

use pyo3::exceptions::PyValueError;
use pyo3::PyErr;
use thiserror::Error;

/// Errors that can occur during chunking operations.
#[derive(Debug, Error)]
pub enum ChunkError {
    /// Invalid configuration provided.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Error during text processing.
    #[error("Text processing error: {0}")]
    ProcessingError(String),

    /// Algorithm not found in registry.
    #[error("Algorithm not found: {0}")]
    AlgorithmNotFound(String),
}

impl From<ChunkError> for PyErr {
    fn from(err: ChunkError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}
