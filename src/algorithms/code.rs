use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::{ChunkConfig, CodeLanguage};
use crate::traits::ChunkAlgorithm;
use std::sync::LazyLock;

static PYTHON_DEF_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?m)^(def |class |async def )").unwrap());
static JS_DEF_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?m)^(function |class |const .+ = \(|const .+ = async \(|export (default )?(function|class))").unwrap()
});
static RUST_DEF_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?m)^(pub )?(fn |struct |enum |impl |trait |mod )").unwrap()
});
static GO_DEF_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?m)^func ").unwrap());
static JAVA_DEF_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?m)^(\s*)(public|private|protected|static|void|class|interface) ").unwrap()
});
static GENERIC_DEF_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"(?m)^\s*\n").unwrap());
static C_DEF_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?m)^(\w[\w\s\*]+)\s+\w+\s*\([^)]*\)\s*\{").unwrap()
});
static CSHARP_DEF_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?m)^(\s*)(public|private|protected|internal|static|override|virtual|abstract|sealed|class|interface|struct|enum|namespace)\s").unwrap()
});
static PHP_DEF_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?m)^(\s*)(function |class |interface |trait |abstract class )").unwrap()
});
static RUBY_DEF_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?m)^(\s*)(def |class |module |attr_)").unwrap()
});
static SWIFT_DEF_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?m)^(\s*)(func |class |struct |enum |protocol |extension |var |let )").unwrap()
});

pub struct CodeChunker;

impl CodeChunker {
    fn get_regex(lang: CodeLanguage) -> &'static regex::Regex {
        match lang {
            CodeLanguage::Python => &PYTHON_DEF_RE,
            CodeLanguage::JavaScript | CodeLanguage::TypeScript => &JS_DEF_RE,
            CodeLanguage::Rust => &RUST_DEF_RE,
            CodeLanguage::Go => &GO_DEF_RE,
            CodeLanguage::Java => &JAVA_DEF_RE,
            CodeLanguage::C | CodeLanguage::Cpp => &C_DEF_RE,
            CodeLanguage::CSharp => &CSHARP_DEF_RE,
            CodeLanguage::PHP => &PHP_DEF_RE,
            CodeLanguage::Ruby => &RUBY_DEF_RE,
            CodeLanguage::Swift => &SWIFT_DEF_RE,
            CodeLanguage::Generic | CodeLanguage::Auto => &GENERIC_DEF_RE,
        }
    }

    fn detect_language(text: &str) -> CodeLanguage {
        if PYTHON_DEF_RE.is_match(text) {
            CodeLanguage::Python
        } else if RUST_DEF_RE.is_match(text) {
            CodeLanguage::Rust
        } else if GO_DEF_RE.is_match(text) {
            CodeLanguage::Go
        } else if JS_DEF_RE.is_match(text) {
            CodeLanguage::JavaScript
        } else {
            CodeLanguage::Generic
        }
    }
}

impl ChunkAlgorithm for CodeChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let lang = if config.code_language == CodeLanguage::Auto {
            Self::detect_language(text)
        } else {
            config.code_language
        };

        let re = Self::get_regex(lang);
        let mut chunks = Vec::new();
        let mut last_end = 0;

        let matches: Vec<_> = re.find_iter(text).collect();

        if matches.is_empty() {
            chunks.push(Chunk::with_uuid(
                text.trim().to_string(),
                0,
                text.len(),
                ChunkMetadata {
                    method: "code".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ));
            return chunks;
        }

        for mat in &matches {
            if mat.start() > last_end {
                let segment = text[last_end..mat.start()].trim();
                if !segment.is_empty() {
                    chunks.push(Chunk::with_uuid(
                        segment.to_string(),
                        last_end,
                        mat.start(),
                        ChunkMetadata {
                            method: "code".to_string(),
                            section: None,
                            overlap_chars: None,
                            parent_chunk_id: None,
                        },
                    ));
                }
            }
            last_end = mat.start();
        }

        // Final segment
        let segment = text[last_end..].trim();
        if !segment.is_empty() {
            chunks.push(Chunk::with_uuid(
                segment.to_string(),
                last_end,
                text.len(),
                ChunkMetadata {
                    method: "code".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ));
        }

        chunks
    }

    fn name(&self) -> &str {
        "code"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_python_split() {
        let chunker = CodeChunker;
        let config = ChunkConfig::new(1000).with_code_language(CodeLanguage::Python);
        let code = "def foo():\n    pass\n\ndef bar():\n    return 1";
        let chunks = chunker.chunk(code, &config);
        assert_eq!(chunks.len(), 2);
    }

    #[test]
    fn test_code_empty() {
        let chunker = CodeChunker;
        let config = ChunkConfig::new(1000);
        assert!(chunker.chunk("", &config).is_empty());
    }

    #[test]
    fn test_code_name() {
        assert_eq!(CodeChunker.name(), "code");
    }

    #[test]
    fn test_code_auto_detect_rust() {
        let chunker = CodeChunker;
        let config = ChunkConfig::new(1000);
        let code = "pub fn hello() {\n    println!(\"hi\");\n}\n\npub fn world() {}\n";
        let chunks = chunker.chunk(code, &config);
        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].metadata.method, "code");
    }
}
