use std::path::Path;
use std::fs;

pub fn parse_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ext = Path::new(path)
        .extension()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();

    match ext.as_str() {
        "txt" => Ok(fs::read_to_string(path)?),
        "pdf" => parse_pdf(path),
        "docx" => parse_docx(path),
        "xlsx" => parse_xlsx(path),
        _ => Err(format!("Unsupported file format: {}", ext).into()),
    }
}

fn parse_pdf(_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Implement using lopdf or pdf-extract
    unimplemented!("PDF parsing coming soon")
}

fn parse_docx(_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Implement using docx-rs or quick-xml
    unimplemented!("DOCX parsing coming soon")
}

fn parse_xlsx(_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Implement using calamine
    unimplemented!("XLSX parsing coming soon")
}

/// Simple chunker to split long text into smaller pieces
pub fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    // Basic implementation: split by paragraphs, then combine up to max_chars
    let paragraphs: Vec<&str> = text.split("\n\n").collect();
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();

    for p in paragraphs {
        if current_chunk.len() + p.len() > max_chars && !current_chunk.is_empty() {
            chunks.push(current_chunk.clone());
            current_chunk.clear();
        }
        if !current_chunk.is_empty() {
            current_chunk.push_str("\n\n");
        }
        current_chunk.push_str(p);
    }
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }
    
    chunks
}
