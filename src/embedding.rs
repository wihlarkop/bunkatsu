use pyo3::prelude::*;

pub trait EmbeddingProvider: Send + Sync {
    fn embed<'py>(&self, py: Python<'py>, texts: &[&str]) -> Result<Vec<Vec<f32>>, String>;
}

pub trait TextGenerator: Send + Sync {
    fn generate<'py>(&self, py: Python<'py>, prompt: &str) -> Result<String, String>;
}

/// Wraps a Python callable: `fn(list[str]) -> list[list[float]]`
pub struct PyEmbeddingProvider {
    pub callable: Py<PyAny>,
}

impl PyEmbeddingProvider {
    pub fn new(callable: Py<PyAny>) -> Self {
        Self { callable }
    }
}

impl EmbeddingProvider for PyEmbeddingProvider {
    fn embed<'py>(&self, py: Python<'py>, texts: &[&str]) -> Result<Vec<Vec<f32>>, String> {
        let py_texts: Vec<&str> = texts.to_vec();
        let result = self
            .callable
            .call1(py, (py_texts,))
            .map_err(|e| e.to_string())?;
        result
            .extract::<Vec<Vec<f32>>>(py)
            .map_err(|e| e.to_string())
    }
}

/// Wraps a Python callable: `fn(str) -> str`
pub struct PyTextGenerator {
    pub callable: Py<PyAny>,
}

impl PyTextGenerator {
    pub fn new(callable: Py<PyAny>) -> Self {
        Self { callable }
    }
}

impl TextGenerator for PyTextGenerator {
    fn generate<'py>(&self, py: Python<'py>, prompt: &str) -> Result<String, String> {
        let result = self
            .callable
            .call1(py, (prompt,))
            .map_err(|e| e.to_string())?;
        result.extract::<String>(py).map_err(|e| e.to_string())
    }
}
