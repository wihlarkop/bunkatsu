use crate::chunk::{Chunk, ChunkMetadata};
use crate::config::ChunkConfig;
use crate::traits::ChunkAlgorithm;

pub struct CsvChunker;

impl ChunkAlgorithm for CsvChunker {
    fn chunk(&self, text: &str, config: &ChunkConfig) -> Vec<Chunk> {
        if text.is_empty() {
            return Vec::new();
        }

        let rows_per_chunk = config.rows_per_chunk.max(1);
        let mut lines = text.lines();

        // Extract header row
        let header = match lines.next() {
            Some(h) => h,
            None => return Vec::new(),
        };

        let mut chunks = Vec::new();
        let mut current_rows: Vec<&str> = Vec::new();
        let mut current_pos = header.len() + 1; // skip past header + newline

        for line in lines {
            let line_start = current_pos;
            current_pos += line.len() + 1;

            if line.trim().is_empty() {
                continue;
            }

            current_rows.push(line);

            if current_rows.len() >= rows_per_chunk {
                let chunk_text = format!("{}\n{}", header, current_rows.join("\n"));
                let start = line_start
                    .saturating_sub(current_rows.iter().map(|r| r.len() + 1).sum::<usize>());
                chunks.push(Chunk::with_uuid(
                    chunk_text.clone(),
                    start,
                    start + chunk_text.len(),
                    ChunkMetadata {
                        method: "csv".to_string(),
                        section: None,
                        overlap_chars: None,
                        parent_chunk_id: None,
                    },
                ));
                current_rows.clear();
            }
        }

        if !current_rows.is_empty() {
            let chunk_text = format!("{}\n{}", header, current_rows.join("\n"));
            chunks.push(Chunk::with_uuid(
                chunk_text.clone(),
                current_pos.saturating_sub(chunk_text.len()),
                current_pos,
                ChunkMetadata {
                    method: "csv".to_string(),
                    section: None,
                    overlap_chars: None,
                    parent_chunk_id: None,
                },
            ));
        }

        chunks
    }

    fn name(&self) -> &str {
        "csv"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_basic() {
        let chunker = CsvChunker;
        let config = ChunkConfig::new(1000).with_rows_per_chunk(2);
        let csv = "name,age\nAlice,30\nBob,25\nCarol,35";
        let chunks = chunker.chunk(csv, &config);
        assert_eq!(chunks.len(), 2);
        // Each chunk should contain the header
        assert!(chunks[0].text.starts_with("name,age"));
        assert!(chunks[1].text.starts_with("name,age"));
    }

    #[test]
    fn test_csv_preserves_header() {
        let chunker = CsvChunker;
        let config = ChunkConfig::new(1000).with_rows_per_chunk(1);
        let csv = "id,value\n1,a\n2,b\n3,c";
        let chunks = chunker.chunk(csv, &config);
        assert_eq!(chunks.len(), 3);
        for chunk in &chunks {
            assert!(chunk.text.starts_with("id,value"));
        }
    }

    #[test]
    fn test_csv_empty() {
        let chunker = CsvChunker;
        let config = ChunkConfig::new(1000);
        assert!(chunker.chunk("", &config).is_empty());
    }

    #[test]
    fn test_csv_name() {
        assert_eq!(CsvChunker.name(), "csv");
    }
}
