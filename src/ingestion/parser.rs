use std::path::Path;
use std::fs;
use std::io::Read;
use zip::ZipArchive;
use quick_xml::Reader;
use quick_xml::events::Event;
use calamine::{Reader as CalamineReader, open_workbook, Xlsx, DataType};

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

fn parse_pdf(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let text = pdf_extract::extract_text(path)?;
    Ok(text)
}

fn parse_docx(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    
    let mut document_xml = archive.by_name("word/document.xml")?;
    let mut xml_content = String::new();
    document_xml.read_to_string(&mut xml_content)?;
    
    let mut reader = Reader::from_str(&xml_content);
    reader.trim_text(true);
    
    let mut text = String::new();
    let mut is_in_text = false;
    
    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"w:t" => {
                is_in_text = true;
            },
            Ok(Event::End(ref e)) if e.name().as_ref() == b"w:t" => {
                is_in_text = false;
            },
            Ok(Event::Text(e)) if is_in_text => {
                text.push_str(&e.unescape()?.into_owned());
                text.push(' ');
            },
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"w:p" => {
                text.push('\n');
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
    }
    
    Ok(text)
}

fn parse_xlsx(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut text = String::new();
    
    let sheets = workbook.sheet_names().to_vec();
    
    for sheet_name in sheets {
        text.push_str(&format!("--- Sheet: {} ---\n", sheet_name));
        
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            for row in range.rows() {
                let mut row_texts = Vec::new();
                for cell in row {
                    match cell {
                        DataType::String(s) => row_texts.push(s.to_string()),
                        DataType::Float(f) => row_texts.push(f.to_string()),
                        DataType::Int(i) => row_texts.push(i.to_string()),
                        DataType::Bool(b) => row_texts.push(b.to_string()),
                        DataType::DateTime(d) => row_texts.push(d.to_string()),
                        _ => (),
                    }
                }
                if !row_texts.is_empty() {
                    text.push_str(&row_texts.join(" | "));
                    text.push('\n');
                }
            }
        }
    }
    
    Ok(text)
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
