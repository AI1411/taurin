mod csv_viewer;
mod image_compressor;

use csv_viewer::{get_csv_info, read_csv, save_csv, CsvData, CsvInfo};
use image_compressor::{
    compress_image, get_image_info, CompressionOptions, CompressionResult, ImageInfo,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn compress_image_cmd(
    input_path: String,
    output_path: String,
    quality: u8,
    width: Option<u32>,
    height: Option<u32>,
    output_format: String,
) -> CompressionResult {
    let options = CompressionOptions {
        quality,
        width,
        height,
        output_format,
    };
    compress_image(&input_path, &output_path, options)
}

#[tauri::command]
fn get_image_info_cmd(path: String) -> Result<ImageInfo, String> {
    get_image_info(&path)
}

#[tauri::command]
fn read_csv_cmd(path: String) -> Result<CsvData, String> {
    read_csv(&path)
}

#[tauri::command]
fn get_csv_info_cmd(path: String) -> Result<CsvInfo, String> {
    get_csv_info(&path)
}

#[tauri::command]
fn save_csv_cmd(path: String, headers: Vec<String>, rows: Vec<Vec<String>>) -> Result<(), String> {
    save_csv(&path, &headers, &rows)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            compress_image_cmd,
            get_image_info_cmd,
            read_csv_cmd,
            get_csv_info_cmd,
            save_csv_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
