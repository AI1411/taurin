use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub total_rows: usize,
    pub total_columns: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvInfo {
    pub file_name: String,
    pub file_size: u64,
    pub row_count: usize,
    pub column_count: usize,
    pub headers: Vec<String>,
}

pub fn read_csv(path: &str) -> Result<CsvData, String> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        return Err("File not found".to_string());
    }

    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(content.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| format!("Failed to read headers: {}", e))?
        .iter()
        .map(|s| s.to_string())
        .collect();

    let mut rows: Vec<Vec<String>> = Vec::new();

    for result in reader.records() {
        let record = result.map_err(|e| format!("Failed to read row: {}", e))?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        rows.push(row);
    }

    let total_rows = rows.len();
    let total_columns = headers.len();

    Ok(CsvData {
        headers,
        rows,
        total_rows,
        total_columns,
    })
}

pub fn get_csv_info(path: &str) -> Result<CsvInfo, String> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        return Err("File not found".to_string());
    }

    let metadata =
        fs::metadata(file_path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
    let file_size = metadata.len();

    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(content.as_bytes());

    let headers: Vec<String> = reader
        .headers()
        .map_err(|e| format!("Failed to read headers: {}", e))?
        .iter()
        .map(|s| s.to_string())
        .collect();

    let row_count = reader.records().count();
    let column_count = headers.len();

    Ok(CsvInfo {
        file_name,
        file_size,
        row_count,
        column_count,
        headers,
    })
}

pub fn save_csv(path: &str, headers: &[String], rows: &[Vec<String>]) -> Result<(), String> {
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_path(path)
        .map_err(|e| format!("Failed to create file: {}", e))?;

    writer
        .write_record(headers)
        .map_err(|e| format!("Failed to write headers: {}", e))?;

    for row in rows {
        writer
            .write_record(row)
            .map_err(|e| format!("Failed to write row: {}", e))?;
    }

    writer
        .flush()
        .map_err(|e| format!("Failed to flush: {}", e))?;

    Ok(())
}
