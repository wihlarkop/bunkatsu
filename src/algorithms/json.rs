use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

pub struct JsonChunker;

impl ChunkAlgorithm for JsonChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let trimmed = text.trim();

        // Try to parse as JSON
        match serde_json::from_str::<serde_json::Value>(trimmed) {
            Ok(serde_json::Value::Array(arr)) => {
                // Split array elements into chunks
                let mut chunks = Vec::new();
                let mut current_items: Vec<serde_json::Value> = Vec::new();
                let mut current_size = 2usize; // brackets

                for item in arr {
                    let item_str = item.to_string();
                    let item_size = item_str.len() + 2; // +2 for comma + space

                    if !current_items.is_empty() && current_size + item_size > config.max_size {
                        let chunk_text = serde_json::to_string(&current_items).unwrap_or_default();
                        let start = text.find(&chunk_text).unwrap_or(0);
                        chunks.push(Chunk::with_uuid(
                            chunk_text.clone(),
                            start,
                            start + chunk_text.len(),
                            ChunkMetadata {
                                method: "json".to_string(),
                                section: None,
                                overlap_chars: None,
                                parent_chunk_id: None,
                            },
                        ));
                        current_items.clear();
                        current_size = 2;
                    }

                    current_size += item_size;
                    current_items.push(item);
                }

                if !current_items.is_empty() {
                    let chunk_text = serde_json::to_string(&current_items).unwrap_or_default();
                    let start = text.find(&chunk_text).unwrap_or(0);
                    chunks.push(Chunk::with_uuid(
                        chunk_text.clone(),
                        start,
                        start + chunk_text.len(),
                        ChunkMetadata {
                            method: "json".to_string(),
                            section: None,
                            overlap_chars: None,
                            parent_chunk_id: None,
                        },
                    ));
                }

                chunks
            }
            Ok(serde_json::Value::Object(map)) => {
                // Split object keys into chunks
                let mut chunks = Vec::new();
                let mut current_pairs: serde_json::Map<String, serde_json::Value> =
                    serde_json::Map::new();
                let mut current_size = 2usize;

                for (key, val) in map {
                    let pair_str = format!("\"{}\":{}", key, val);
                    let pair_size = pair_str.len() + 2;

                    if !current_pairs.is_empty() && current_size + pair_size > config.max_size {
                        let chunk_text = serde_json::to_string(&current_pairs).unwrap_or_default();
                        let start = text.find(&chunk_text).unwrap_or(0);
                        chunks.push(Chunk::with_uuid(
                            chunk_text.clone(),
                            start,
                            start + chunk_text.len(),
                            ChunkMetadata {
                                method: "json".to_string(),
                                section: None,
                                overlap_chars: None,
                                parent_chunk_id: None,
                            },
                        ));
                        current_pairs = serde_json::Map::new();
                        current_size = 2;
                    }

                    current_size += pair_size;
                    current_pairs.insert(key, val);
                }

                if !current_pairs.is_empty() {
                    let chunk_text = serde_json::to_string(&current_pairs).unwrap_or_default();
                    let start = text.find(&chunk_text).unwrap_or(0);
                    chunks.push(Chunk::with_uuid(
                        chunk_text.clone(),
                        start,
                        start + chunk_text.len(),
                        ChunkMetadata {
                            method: "json".to_string(),
                            section: None,
                            overlap_chars: None,
                            parent_chunk_id: None,
                        },
                    ));
                }

                chunks
            }
            _ => {
                // Not valid JSON or scalar — return as-is
                vec![Chunk::with_uuid(
                    trimmed.to_string(),
                    0,
                    text.len(),
                    ChunkMetadata {
                        method: "json".to_string(),
                        section: None,
                        overlap_chars: None,
                        parent_chunk_id: None,
                    },
                )]
            }
        }
    }

    fn name(&self) -> &str {
        "json"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_array_splits() {
        let chunker = JsonChunker;
        let config = ChunkConfig::new(30);
        let json = r#"[{"a":1},{"b":2},{"c":3},{"d":4}]"#;
        let chunks = chunker.chunk(json, &config);
        assert!(chunks.len() >= 2);
        assert!(chunks.iter().all(|c| c.metadata.method == "json"));
    }

    #[test]
    fn test_json_object_splits() {
        let chunker = JsonChunker;
        let config = ChunkConfig::new(20);
        let json = r#"{"key1":"val1","key2":"val2","key3":"val3"}"#;
        let chunks = chunker.chunk(json, &config);
        assert!(chunks.len() >= 2);
    }

    #[test]
    fn test_json_empty() {
        let chunker = JsonChunker;
        let config = ChunkConfig::new(1000);
        assert!(chunker.chunk("", &config).is_empty());
    }

    #[test]
    fn test_json_name() {
        assert_eq!(JsonChunker.name(), "json");
    }

    #[test]
    fn test_json_single_chunk_small() {
        let chunker = JsonChunker;
        let config = ChunkConfig::new(1000);
        let json = r#"[{"a":1},{"b":2}]"#;
        let chunks = chunker.chunk(json, &config);
        assert_eq!(chunks.len(), 1);
    }
}
