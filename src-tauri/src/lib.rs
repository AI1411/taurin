mod csv_viewer;
mod image_compressor;
mod image_editor;
mod kanban;
mod markdown_to_pdf;
mod password_generator;
mod pdf_tools;
mod uuid_generator;

use csv_viewer::{get_csv_info, read_csv, save_csv, CsvData, CsvInfo};
use image_compressor::{
    compress_image, get_image_info, CompressionOptions, CompressionResult, ImageInfo,
};
use image_editor::{
    adjust_brightness, adjust_contrast, apply_filter, crop_image, flip_horizontal, flip_vertical,
    get_editor_image_info, resize_image, rotate_image, EditResult, ImageEditorInfo, ImageFilter,
    RotationAngle,
};
use kanban::{
    create_task, delete_task, load_board, move_task, update_task, KanbanBoard, Task, TaskColumn,
    TaskPriority,
};
use markdown_to_pdf::{
    convert_markdown_to_pdf, markdown_to_html, read_markdown, MarkdownInfo, MarkdownToHtmlResult,
    MarkdownToPdfResult,
};
use password_generator::{
    generate_passphrases, generate_passwords, PassphraseOptions, PasswordGenerateResult,
    PasswordOptions,
};
use pdf_tools::{
    get_pdf_info, merge_pdfs, split_pdf_by_pages, split_pdf_by_range, PdfInfo, PdfMergeResult,
    PdfSplitResult,
};
use uuid_generator::{
    generate_uuids, validate_uuid, UuidFormat, UuidGenerateOptions, UuidGenerateResult,
    UuidValidateResult, UuidVersion,
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

#[tauri::command]
fn get_pdf_info_cmd(path: String) -> Result<PdfInfo, String> {
    get_pdf_info(&path)
}

#[tauri::command]
fn split_pdf_by_pages_cmd(input_path: String, output_dir: String) -> PdfSplitResult {
    split_pdf_by_pages(&input_path, &output_dir)
}

#[tauri::command]
fn split_pdf_by_range_cmd(
    input_path: String,
    output_path: String,
    start_page: u32,
    end_page: u32,
) -> PdfSplitResult {
    split_pdf_by_range(&input_path, &output_path, start_page, end_page)
}

#[tauri::command]
fn merge_pdfs_cmd(input_paths: Vec<String>, output_path: String) -> PdfMergeResult {
    merge_pdfs(&input_paths, &output_path)
}

#[tauri::command]
fn read_markdown_cmd(path: String) -> Result<MarkdownInfo, String> {
    read_markdown(&path)
}

#[tauri::command]
fn markdown_to_html_cmd(markdown: String) -> MarkdownToHtmlResult {
    markdown_to_html(&markdown)
}

#[tauri::command]
fn convert_markdown_to_pdf_cmd(
    markdown: String,
    output_path: String,
    source_path: Option<String>,
) -> MarkdownToPdfResult {
    convert_markdown_to_pdf(&markdown, &output_path, source_path.as_deref())
}

#[tauri::command]
fn load_kanban_board_cmd(app: tauri::AppHandle) -> Result<KanbanBoard, String> {
    load_board(&app)
}

#[tauri::command]
fn create_task_cmd(
    app: tauri::AppHandle,
    title: String,
    description: Option<String>,
    priority: TaskPriority,
    assignee: Option<String>,
    due_date: Option<String>,
) -> Result<Task, String> {
    create_task(&app, title, description, priority, assignee, due_date)
}

#[tauri::command]
fn update_task_cmd(
    app: tauri::AppHandle,
    task_id: String,
    title: Option<String>,
    description: Option<String>,
    column: Option<TaskColumn>,
    priority: Option<TaskPriority>,
    assignee: Option<String>,
    due_date: Option<String>,
) -> Result<Task, String> {
    update_task(
        &app,
        task_id,
        title,
        description,
        column,
        priority,
        assignee,
        due_date,
    )
}

#[tauri::command]
fn delete_task_cmd(app: tauri::AppHandle, task_id: String) -> Result<(), String> {
    delete_task(&app, task_id)
}

#[tauri::command]
fn move_task_cmd(
    app: tauri::AppHandle,
    task_id: String,
    column: TaskColumn,
) -> Result<Task, String> {
    move_task(&app, task_id, column)
}

#[tauri::command]
fn get_editor_image_info_cmd(path: String) -> Result<ImageEditorInfo, String> {
    get_editor_image_info(&path)
}

#[tauri::command]
fn resize_image_cmd(
    input_path: String,
    output_path: String,
    width: u32,
    height: u32,
    maintain_aspect: bool,
) -> EditResult {
    resize_image(&input_path, &output_path, width, height, maintain_aspect)
}

#[tauri::command]
fn rotate_image_cmd(input_path: String, output_path: String, angle: RotationAngle) -> EditResult {
    rotate_image(&input_path, &output_path, angle)
}

#[tauri::command]
fn crop_image_cmd(
    input_path: String,
    output_path: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> EditResult {
    crop_image(&input_path, &output_path, x, y, width, height)
}

#[tauri::command]
fn adjust_brightness_cmd(input_path: String, output_path: String, value: i32) -> EditResult {
    adjust_brightness(&input_path, &output_path, value)
}

#[tauri::command]
fn adjust_contrast_cmd(input_path: String, output_path: String, value: f32) -> EditResult {
    adjust_contrast(&input_path, &output_path, value)
}

#[tauri::command]
fn apply_filter_cmd(input_path: String, output_path: String, filter: ImageFilter) -> EditResult {
    apply_filter(&input_path, &output_path, filter)
}

#[tauri::command]
fn flip_horizontal_cmd(input_path: String, output_path: String) -> EditResult {
    flip_horizontal(&input_path, &output_path)
}

#[tauri::command]
fn flip_vertical_cmd(input_path: String, output_path: String) -> EditResult {
    flip_vertical(&input_path, &output_path)
}

#[tauri::command]
fn generate_uuids_cmd(version: UuidVersion, format: UuidFormat, count: u32) -> UuidGenerateResult {
    let options = UuidGenerateOptions {
        version,
        format,
        count,
    };
    generate_uuids(options)
}

#[tauri::command]
fn validate_uuid_cmd(input: String) -> UuidValidateResult {
    validate_uuid(&input)
}

#[tauri::command]
fn generate_passwords_cmd(options: PasswordOptions) -> PasswordGenerateResult {
    generate_passwords(options)
}

#[tauri::command]
fn generate_passphrases_cmd(options: PassphraseOptions) -> PasswordGenerateResult {
    generate_passphrases(options)
}

use tauri::{Emitter, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .on_window_event(|window, event| {
            if let WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event {
                let paths_str: Vec<String> = paths
                    .iter()
                    .filter_map(|p| p.to_str().map(|s| s.to_string()))
                    .collect();
                let _ = window.emit("file-drop", paths_str);
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            compress_image_cmd,
            get_image_info_cmd,
            read_csv_cmd,
            get_csv_info_cmd,
            save_csv_cmd,
            get_pdf_info_cmd,
            split_pdf_by_pages_cmd,
            split_pdf_by_range_cmd,
            merge_pdfs_cmd,
            load_kanban_board_cmd,
            create_task_cmd,
            update_task_cmd,
            delete_task_cmd,
            move_task_cmd,
            get_editor_image_info_cmd,
            resize_image_cmd,
            rotate_image_cmd,
            crop_image_cmd,
            adjust_brightness_cmd,
            adjust_contrast_cmd,
            apply_filter_cmd,
            flip_horizontal_cmd,
            flip_vertical_cmd,
            read_markdown_cmd,
            markdown_to_html_cmd,
            convert_markdown_to_pdf_cmd,
            generate_uuids_cmd,
            validate_uuid_cmd,
            generate_passwords_cmd,
            generate_passphrases_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
