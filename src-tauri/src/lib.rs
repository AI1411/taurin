mod base64_encoder;
mod char_counter;
mod csv_viewer;
mod image_compressor;
mod image_editor;
mod json_formatter;
mod kanban;
mod markdown_to_pdf;
mod password_generator;
mod pdf_tools;
mod regex_tester;
mod scratch_pad;
mod text_diff;
mod unit_converter;
mod unix_time_converter;
mod uuid_generator;

use base64_encoder::{
    decode_base64, decode_base64_image, encode_base64, encode_image_to_base64,
    Base64DecodeImageResult, Base64DecodeResult, Base64EncodeResult, Base64ImageResult,
};
use char_counter::{count_chars, CharCountResult};
use csv_viewer::{get_csv_info, read_csv, save_csv, CsvData, CsvInfo};
use image_compressor::{
    compress_image, get_image_info, CompressionOptions, CompressionResult, ImageInfo,
};
use image_editor::{
    adjust_brightness, adjust_contrast, apply_filter, crop_image, flip_horizontal, flip_vertical,
    get_editor_image_info, resize_image, rotate_image, EditResult, ImageEditorInfo, ImageFilter,
    RotationAngle,
};
use json_formatter::{
    format_json, minify_json, parse_to_tree, search_json, validate_json, JsonFormatResult,
    JsonMinifyResult, JsonParseResult, JsonSearchResult, JsonValidateResult,
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
use regex_tester::{replace_regex, test_regex, RegexFlags, RegexResult, ReplaceResult};
use scratch_pad::{
    create_note, delete_note, export_to_file, load_scratch_pad, set_active_note, update_note, Note,
    ScratchPadData,
};
use text_diff::{compute_diff, get_file_info, DiffMode, DiffResult, FileInfo};
use unit_converter::{
    convert_area, convert_data_size, convert_length, convert_temperature, convert_time,
    convert_volume, convert_weight, AreaUnit, ConversionResult, DataSizeUnit, LengthUnit,
    TemperatureUnit, TimeUnit, VolumeUnit, WeightUnit,
};
use unix_time_converter::{
    datetime_to_unix, get_current_unix_time, unix_to_datetime, CurrentUnixTimeResult,
    DateTimeToUnixResult, TimestampUnit, TimezoneOption, UnixToDateTimeResult,
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
#[allow(clippy::too_many_arguments)]
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

#[tauri::command]
fn convert_length_cmd(value: f64, from: LengthUnit, to: LengthUnit) -> ConversionResult {
    convert_length(value, from, to)
}

#[tauri::command]
fn convert_weight_cmd(value: f64, from: WeightUnit, to: WeightUnit) -> ConversionResult {
    convert_weight(value, from, to)
}

#[tauri::command]
fn convert_data_size_cmd(value: f64, from: DataSizeUnit, to: DataSizeUnit) -> ConversionResult {
    convert_data_size(value, from, to)
}

#[tauri::command]
fn convert_temperature_cmd(
    value: f64,
    from: TemperatureUnit,
    to: TemperatureUnit,
) -> ConversionResult {
    convert_temperature(value, from, to)
}

#[tauri::command]
fn convert_time_cmd(value: f64, from: TimeUnit, to: TimeUnit) -> ConversionResult {
    convert_time(value, from, to)
}

#[tauri::command]
fn convert_area_cmd(value: f64, from: AreaUnit, to: AreaUnit) -> ConversionResult {
    convert_area(value, from, to)
}

#[tauri::command]
fn convert_volume_cmd(value: f64, from: VolumeUnit, to: VolumeUnit) -> ConversionResult {
    convert_volume(value, from, to)
}

#[tauri::command]
fn compute_diff_cmd(old_text: String, new_text: String, mode: DiffMode) -> DiffResult {
    compute_diff(&old_text, &new_text, mode)
}

#[tauri::command]
fn test_regex_cmd(pattern: String, test_text: String, flags: RegexFlags) -> RegexResult {
    test_regex(&pattern, &test_text, flags)
}

#[tauri::command]
fn replace_regex_cmd(
    pattern: String,
    test_text: String,
    replacement: String,
    flags: RegexFlags,
) -> ReplaceResult {
    replace_regex(&pattern, &test_text, &replacement, flags)
}

#[tauri::command]
fn get_text_file_info_cmd(path: String) -> Result<FileInfo, String> {
    get_file_info(&path)
}

#[tauri::command]
fn load_scratch_pad_cmd(app: tauri::AppHandle) -> Result<ScratchPadData, String> {
    load_scratch_pad(&app)
}

#[tauri::command]
fn create_note_cmd(app: tauri::AppHandle) -> Result<Note, String> {
    create_note(&app)
}

#[tauri::command]
fn update_note_cmd(
    app: tauri::AppHandle,
    note_id: String,
    content: String,
) -> Result<Note, String> {
    update_note(&app, note_id, content)
}

#[tauri::command]
fn delete_note_cmd(app: tauri::AppHandle, note_id: String) -> Result<ScratchPadData, String> {
    delete_note(&app, note_id)
}

#[tauri::command]
fn set_active_note_cmd(app: tauri::AppHandle, note_id: String) -> Result<ScratchPadData, String> {
    set_active_note(&app, note_id)
}

#[tauri::command]
fn export_to_file_cmd(content: String, path: String) -> Result<(), String> {
    export_to_file(content, path)
}

#[tauri::command]
fn format_json_cmd(input: String, indent_size: usize) -> JsonFormatResult {
    format_json(&input, indent_size)
}

#[tauri::command]
fn validate_json_cmd(input: String) -> JsonValidateResult {
    validate_json(&input)
}

#[tauri::command]
fn minify_json_cmd(input: String) -> JsonMinifyResult {
    minify_json(&input)
}

#[tauri::command]
fn parse_json_to_tree_cmd(input: String) -> JsonParseResult {
    parse_to_tree(&input)
}

#[tauri::command]
fn search_json_cmd(
    input: String,
    query: String,
    search_keys: bool,
    search_values: bool,
) -> JsonSearchResult {
    search_json(&input, &query, search_keys, search_values)
}

#[tauri::command]
fn encode_base64_cmd(input: String, url_safe: bool) -> Base64EncodeResult {
    encode_base64(&input, url_safe)
}

#[tauri::command]
fn decode_base64_cmd(input: String, url_safe: bool) -> Base64DecodeResult {
    decode_base64(&input, url_safe)
}

#[tauri::command]
fn encode_image_to_base64_cmd(path: String) -> Base64ImageResult {
    encode_image_to_base64(&path)
}

#[tauri::command]
fn decode_base64_image_cmd(input: String) -> Base64DecodeImageResult {
    decode_base64_image(&input)
}

#[tauri::command]
fn unix_to_datetime_cmd(
    timestamp: i64,
    unit: TimestampUnit,
    timezone: TimezoneOption,
) -> UnixToDateTimeResult {
    unix_to_datetime(timestamp, unit, timezone)
}

#[tauri::command]
fn datetime_to_unix_cmd(datetime_str: String, timezone: TimezoneOption) -> DateTimeToUnixResult {
    datetime_to_unix(&datetime_str, timezone)
}

#[tauri::command]
fn get_current_unix_time_cmd() -> CurrentUnixTimeResult {
    get_current_unix_time()
}

#[tauri::command]
fn count_chars_cmd(text: String) -> CharCountResult {
    count_chars(&text)
}

use tauri::{Emitter, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|_app| Ok(()))
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
            generate_passphrases_cmd,
            convert_length_cmd,
            convert_weight_cmd,
            convert_data_size_cmd,
            convert_temperature_cmd,
            convert_time_cmd,
            convert_area_cmd,
            convert_volume_cmd,
            compute_diff_cmd,
            get_text_file_info_cmd,
            test_regex_cmd,
            replace_regex_cmd,
            load_scratch_pad_cmd,
            create_note_cmd,
            update_note_cmd,
            delete_note_cmd,
            set_active_note_cmd,
            export_to_file_cmd,
            format_json_cmd,
            validate_json_cmd,
            minify_json_cmd,
            parse_json_to_tree_cmd,
            search_json_cmd,
            encode_base64_cmd,
            decode_base64_cmd,
            encode_image_to_base64_cmd,
            decode_base64_image_cmd,
            unix_to_datetime_cmd,
            datetime_to_unix_cmd,
            get_current_unix_time_cmd,
            count_chars_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
