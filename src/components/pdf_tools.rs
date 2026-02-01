use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(options: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn save(options: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PdfInfo {
    pub page_count: u32,
    pub file_size: u64,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfSplitResult {
    pub success: bool,
    pub output_paths: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMergeResult {
    pub success: bool,
    pub output_path: String,
    pub page_count: u32,
    pub file_size: u64,
    pub error: Option<String>,
}

#[derive(Serialize)]
struct OpenDialogOptions {
    multiple: bool,
    directory: bool,
    filters: Vec<FileFilter>,
}

#[derive(Serialize)]
struct SaveDialogOptions {
    filters: Vec<FileFilter>,
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
}

#[derive(Serialize)]
struct FileFilter {
    name: String,
    extensions: Vec<String>,
}

#[derive(Serialize)]
struct GetPdfInfoArgs {
    path: String,
}

#[derive(Serialize)]
struct SplitByPagesArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputDir")]
    output_dir: String,
}

#[derive(Serialize)]
struct SplitByRangeArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    #[serde(rename = "startPage")]
    start_page: u32,
    #[serde(rename = "endPage")]
    end_page: u32,
}

#[derive(Serialize)]
struct MergePdfsArgs {
    #[serde(rename = "inputPaths")]
    input_paths: Vec<String>,
    #[serde(rename = "outputPath")]
    output_path: String,
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else {
        format!("{} KB", bytes / 1024)
    }
}

#[derive(Clone, PartialEq)]
enum PdfMode {
    Split,
    Merge,
}

#[derive(Clone, PartialEq)]
enum SplitType {
    AllPages,
    Range,
}

#[derive(Clone)]
struct PdfFile {
    path: String,
    info: PdfInfo,
}

#[derive(Properties, PartialEq)]
pub struct PdfToolsProps {
    #[prop_or_default]
    pub dropped_file: Option<String>,
    #[prop_or_default]
    pub on_file_processed: Callback<()>,
}

#[function_component(PdfTools)]
pub fn pdf_tools(props: &PdfToolsProps) -> Html {
    let mode = use_state(|| PdfMode::Split);
    let split_type = use_state(|| SplitType::AllPages);
    let is_processing = use_state(|| false);

    // Split mode state
    let split_input_path = use_state(|| String::new());
    let split_pdf_info = use_state(|| Option::<PdfInfo>::None);
    let start_page = use_state(|| 1u32);
    let end_page = use_state(|| 1u32);
    let split_result = use_state(|| Option::<PdfSplitResult>::None);

    // Merge mode state
    let merge_files = use_state(|| Vec::<PdfFile>::new());
    let merge_result = use_state(|| Option::<PdfMergeResult>::None);

    // Handle dropped file
    {
        let dropped_file = props.dropped_file.clone();
        let on_file_processed = props.on_file_processed.clone();
        let mode = mode.clone();
        let split_input_path = split_input_path.clone();
        let split_pdf_info = split_pdf_info.clone();
        let end_page = end_page.clone();
        let split_result = split_result.clone();
        let merge_files = merge_files.clone();
        let merge_result = merge_result.clone();

        use_effect_with(dropped_file.clone(), move |dropped_file| {
            if let Some(path) = dropped_file.clone() {
                let mode = mode.clone();
                let split_input_path = split_input_path.clone();
                let split_pdf_info = split_pdf_info.clone();
                let end_page = end_page.clone();
                let split_result = split_result.clone();
                let merge_files = merge_files.clone();
                let merge_result = merge_result.clone();
                let on_file_processed = on_file_processed.clone();

                spawn_local(async move {
                    let args = serde_wasm_bindgen::to_value(&GetPdfInfoArgs { path: path.clone() })
                        .unwrap();
                    let info_result = invoke("get_pdf_info_cmd", args).await;

                    if let Ok(info) = serde_wasm_bindgen::from_value::<PdfInfo>(info_result) {
                        if *mode == PdfMode::Split {
                            split_input_path.set(path);
                            end_page.set(info.page_count);
                            split_pdf_info.set(Some(info));
                            split_result.set(None);
                        } else {
                            let mut files = (*merge_files).clone();
                            files.push(PdfFile { path, info });
                            merge_files.set(files);
                            merge_result.set(None);
                        }
                    }

                    on_file_processed.emit(());
                });
            }
            || {}
        });
    }

    let on_mode_change = {
        let mode = mode.clone();
        Callback::from(move |new_mode: PdfMode| {
            mode.set(new_mode);
        })
    };

    // Split mode handlers
    let on_select_split_file = {
        let split_input_path = split_input_path.clone();
        let split_pdf_info = split_pdf_info.clone();
        let end_page = end_page.clone();
        let split_result = split_result.clone();
        Callback::from(move |_| {
            let split_input_path = split_input_path.clone();
            let split_pdf_info = split_pdf_info.clone();
            let end_page = end_page.clone();
            let split_result = split_result.clone();
            spawn_local(async move {
                let options = OpenDialogOptions {
                    multiple: false,
                    directory: false,
                    filters: vec![FileFilter {
                        name: "PDF".to_string(),
                        extensions: vec!["pdf".to_string()],
                    }],
                };
                let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
                let result = open(options_js).await;

                if let Some(path) = result.as_string() {
                    split_input_path.set(path.clone());
                    split_result.set(None);

                    let args = serde_wasm_bindgen::to_value(&GetPdfInfoArgs { path }).unwrap();
                    let info_result = invoke("get_pdf_info_cmd", args).await;

                    if let Ok(info) = serde_wasm_bindgen::from_value::<PdfInfo>(info_result) {
                        end_page.set(info.page_count);
                        split_pdf_info.set(Some(info));
                    }
                }
            });
        })
    };

    let on_split = {
        let split_input_path = split_input_path.clone();
        let split_type = split_type.clone();
        let start_page = start_page.clone();
        let end_page = end_page.clone();
        let split_result = split_result.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let input_path = (*split_input_path).clone();
            if input_path.is_empty() {
                return;
            }

            let split_type_val = (*split_type).clone();
            let start_val = *start_page;
            let end_val = *end_page;
            let split_result = split_result.clone();
            let is_processing = is_processing.clone();

            is_processing.set(true);

            spawn_local(async move {
                match split_type_val {
                    SplitType::AllPages => {
                        let dir_options = OpenDialogOptions {
                            multiple: false,
                            directory: true,
                            filters: vec![],
                        };
                        let dir_options_js = serde_wasm_bindgen::to_value(&dir_options).unwrap();
                        let dir_result = open(dir_options_js).await;

                        if let Some(output_dir) = dir_result.as_string() {
                            let args = SplitByPagesArgs {
                                input_path,
                                output_dir,
                            };
                            let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                            let result = invoke("split_pdf_by_pages_cmd", args_js).await;

                            if let Ok(res) =
                                serde_wasm_bindgen::from_value::<PdfSplitResult>(result)
                            {
                                split_result.set(Some(res));
                            }
                        }
                    }
                    SplitType::Range => {
                        let save_options = SaveDialogOptions {
                            filters: vec![FileFilter {
                                name: "PDF".to_string(),
                                extensions: vec!["pdf".to_string()],
                            }],
                            default_path: Some("extracted.pdf".to_string()),
                        };
                        let save_options_js = serde_wasm_bindgen::to_value(&save_options).unwrap();
                        let save_result = save(save_options_js).await;

                        if let Some(output_path) = save_result.as_string() {
                            let args = SplitByRangeArgs {
                                input_path,
                                output_path,
                                start_page: start_val,
                                end_page: end_val,
                            };
                            let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                            let result = invoke("split_pdf_by_range_cmd", args_js).await;

                            if let Ok(res) =
                                serde_wasm_bindgen::from_value::<PdfSplitResult>(result)
                            {
                                split_result.set(Some(res));
                            }
                        }
                    }
                }

                is_processing.set(false);
            });
        })
    };

    // Merge mode handlers
    let on_add_merge_files = {
        let merge_files = merge_files.clone();
        let merge_result = merge_result.clone();
        Callback::from(move |_| {
            let merge_files = merge_files.clone();
            let merge_result = merge_result.clone();
            spawn_local(async move {
                let options = OpenDialogOptions {
                    multiple: true,
                    directory: false,
                    filters: vec![FileFilter {
                        name: "PDF".to_string(),
                        extensions: vec!["pdf".to_string()],
                    }],
                };
                let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
                let result = open(options_js).await;

                if let Ok(paths) = serde_wasm_bindgen::from_value::<Vec<String>>(result.clone()) {
                    let mut files = (*merge_files).clone();

                    for path in paths {
                        let args =
                            serde_wasm_bindgen::to_value(&GetPdfInfoArgs { path: path.clone() })
                                .unwrap();
                        let info_result = invoke("get_pdf_info_cmd", args).await;

                        if let Ok(info) = serde_wasm_bindgen::from_value::<PdfInfo>(info_result) {
                            files.push(PdfFile { path, info });
                        }
                    }

                    merge_files.set(files);
                    merge_result.set(None);
                } else if let Some(path) = result.as_string() {
                    let args = serde_wasm_bindgen::to_value(&GetPdfInfoArgs { path: path.clone() })
                        .unwrap();
                    let info_result = invoke("get_pdf_info_cmd", args).await;

                    if let Ok(info) = serde_wasm_bindgen::from_value::<PdfInfo>(info_result) {
                        let mut files = (*merge_files).clone();
                        files.push(PdfFile { path, info });
                        merge_files.set(files);
                        merge_result.set(None);
                    }
                }
            });
        })
    };

    let on_remove_merge_file = {
        let merge_files = merge_files.clone();
        Callback::from(move |index: usize| {
            let mut files = (*merge_files).clone();
            files.remove(index);
            merge_files.set(files);
        })
    };

    let on_move_up = {
        let merge_files = merge_files.clone();
        Callback::from(move |index: usize| {
            if index > 0 {
                let mut files = (*merge_files).clone();
                files.swap(index, index - 1);
                merge_files.set(files);
            }
        })
    };

    let on_move_down = {
        let merge_files = merge_files.clone();
        Callback::from(move |index: usize| {
            let files_len = merge_files.len();
            if index < files_len - 1 {
                let mut files = (*merge_files).clone();
                files.swap(index, index + 1);
                merge_files.set(files);
            }
        })
    };

    let on_merge = {
        let merge_files = merge_files.clone();
        let merge_result = merge_result.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let files = (*merge_files).clone();
            if files.is_empty() {
                return;
            }

            let merge_result = merge_result.clone();
            let is_processing = is_processing.clone();

            is_processing.set(true);

            spawn_local(async move {
                let save_options = SaveDialogOptions {
                    filters: vec![FileFilter {
                        name: "PDF".to_string(),
                        extensions: vec!["pdf".to_string()],
                    }],
                    default_path: Some("merged.pdf".to_string()),
                };
                let save_options_js = serde_wasm_bindgen::to_value(&save_options).unwrap();
                let save_result = save(save_options_js).await;

                if let Some(output_path) = save_result.as_string() {
                    let input_paths: Vec<String> = files.iter().map(|f| f.path.clone()).collect();
                    let args = MergePdfsArgs {
                        input_paths,
                        output_path,
                    };
                    let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                    let result = invoke("merge_pdfs_cmd", args_js).await;

                    if let Ok(res) = serde_wasm_bindgen::from_value::<PdfMergeResult>(result) {
                        merge_result.set(Some(res));
                    }
                }

                is_processing.set(false);
            });
        })
    };

    let on_start_page_change = {
        let start_page = start_page.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<u32>() {
                start_page.set(val);
            }
        })
    };

    let on_end_page_change = {
        let end_page = end_page.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<u32>() {
                end_page.set(val);
            }
        })
    };

    let on_split_type_change = {
        let split_type = split_type.clone();
        Callback::from(move |new_type: SplitType| {
            split_type.set(new_type);
        })
    };

    // Reset handlers
    let on_reset_split = {
        let split_input_path = split_input_path.clone();
        let split_pdf_info = split_pdf_info.clone();
        let split_result = split_result.clone();
        let start_page = start_page.clone();
        let end_page = end_page.clone();
        Callback::from(move |_| {
            split_input_path.set(String::new());
            split_pdf_info.set(None);
            split_result.set(None);
            start_page.set(1);
            end_page.set(1);
        })
    };

    let on_reset_merge = {
        let merge_files = merge_files.clone();
        let merge_result = merge_result.clone();
        Callback::from(move |_| {
            merge_files.set(Vec::new());
            merge_result.set(None);
        })
    };

    html! {
        <div class="pdf-tools">
            // Processing Overlay
            {if *is_processing {
                html! {
                    <div class="processing-overlay">
                        <div class="processing-content">
                            <div class="processing-spinner"></div>
                            <p class="processing-title">{"Processing..."}</p>
                            <p class="processing-hint">{"Please wait while your PDF is being processed"}</p>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

            // Mode Toggle
            <div class="section">
                <div class="mode-toggle">
                    <button
                        class={if *mode == PdfMode::Split { "mode-btn active" } else { "mode-btn" }}
                        onclick={
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(PdfMode::Split))
                        }
                    >
                        {"Split PDF"}
                    </button>
                    <button
                        class={if *mode == PdfMode::Merge { "mode-btn active" } else { "mode-btn" }}
                        onclick={
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(PdfMode::Merge))
                        }
                    >
                        {"Merge PDFs"}
                    </button>
                </div>
            </div>

            // Split Mode
            {if *mode == PdfMode::Split {
                html! {
                    <>
                        // File Selection
                        <div class="section" onclick={on_select_split_file.clone()}>
                            <div class="drop-zone">
                                <div class="drop-zone-icon">{"📄"}</div>
                                <p class="drop-zone-text">{"Click or drag & drop a PDF"}</p>
                                <p class="drop-zone-hint">{"Select a PDF file to split"}</p>
                            </div>
                            {if !split_input_path.is_empty() {
                                html! { <p class="file-path">{&*split_input_path}</p> }
                            } else {
                                html! {}
                            }}
                        </div>

                        // PDF Info
                        {if let Some(info) = &*split_pdf_info {
                            html! {
                                <div class="section info-box">
                                    <h3>{"PDF Info"}</h3>
                                    <div class="info-grid">
                                        <div class="info-item">
                                            <div class="info-item-label">{"Pages"}</div>
                                            <div class="info-item-value">{info.page_count}</div>
                                        </div>
                                        <div class="info-item">
                                            <div class="info-item-label">{"Size"}</div>
                                            <div class="info-item-value">{format_size(info.file_size)}</div>
                                        </div>
                                        <div class="info-item">
                                            <div class="info-item-label">{"File"}</div>
                                            <div class="info-item-value file-name-value">{&info.file_name}</div>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        // Split Options
                        {if split_pdf_info.is_some() {
                            html! {
                                <div class="section">
                                    <h3>{"Split Type"}</h3>
                                    <div class="format-options split-options">
                                        <div class="format-option">
                                            <input
                                                type="radio"
                                                name="split_type"
                                                id="split_all"
                                                checked={*split_type == SplitType::AllPages}
                                                onclick={
                                                    let on_split_type_change = on_split_type_change.clone();
                                                    Callback::from(move |_| on_split_type_change.emit(SplitType::AllPages))
                                                }
                                            />
                                            <label for="split_all">
                                                <span class="format-name">{"All Pages"}</span>
                                            </label>
                                        </div>
                                        <div class="format-option">
                                            <input
                                                type="radio"
                                                name="split_type"
                                                id="split_range"
                                                checked={*split_type == SplitType::Range}
                                                onclick={
                                                    let on_split_type_change = on_split_type_change.clone();
                                                    Callback::from(move |_| on_split_type_change.emit(SplitType::Range))
                                                }
                                            />
                                            <label for="split_range">
                                                <span class="format-name">{"Page Range"}</span>
                                            </label>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        // Page Range
                        {if split_pdf_info.is_some() && *split_type == SplitType::Range {
                            html! {
                                <div class="section">
                                    <h3>{"Page Range"}</h3>
                                    <div class="page-range-inputs">
                                        <input
                                            type="number"
                                            min="1"
                                            max={split_pdf_info.as_ref().map(|i| i.page_count.to_string()).unwrap_or_default()}
                                            value={start_page.to_string()}
                                            oninput={on_start_page_change}
                                            placeholder="Start"
                                        />
                                        <span>{"to"}</span>
                                        <input
                                            type="number"
                                            min="1"
                                            max={split_pdf_info.as_ref().map(|i| i.page_count.to_string()).unwrap_or_default()}
                                            value={end_page.to_string()}
                                            oninput={on_end_page_change}
                                            placeholder="End"
                                        />
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        // Action Buttons
                        <div class="pdf-action-buttons">
                            <button
                                onclick={on_split}
                                disabled={split_input_path.is_empty() || *is_processing}
                                class="primary-btn compress-btn"
                            >
                                {if *split_type == SplitType::AllPages {
                                    "Split All Pages"
                                } else {
                                    "Extract Pages"
                                }}
                            </button>
                            {if !split_input_path.is_empty() {
                                html! {
                                    <button
                                        onclick={on_reset_split.clone()}
                                        class="secondary-btn reset-btn"
                                    >
                                        {"Reset"}
                                    </button>
                                }
                            } else {
                                html! {}
                            }}
                        </div>

                        // Split Result
                        {if let Some(result) = &*split_result {
                            html! {
                                <div class={if result.success { "section result-box success" } else { "section result-box error" }}>
                                    {if result.success {
                                        html! {
                                            <>
                                                <h3>{"Split Complete!"}</h3>
                                                <p class="output-path">{format!("Created {} file(s)", result.output_paths.len())}</p>
                                            </>
                                        }
                                    } else {
                                        html! {
                                            <>
                                                <h3>{"Split Failed"}</h3>
                                                <p>{result.error.clone().unwrap_or_default()}</p>
                                            </>
                                        }
                                    }}
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </>
                }
            } else {
                // Merge Mode
                html! {
                    <>
                        // File Selection
                        <div class="section" onclick={on_add_merge_files.clone()}>
                            <div class="drop-zone">
                                <div class="drop-zone-icon">{"📑"}</div>
                                <p class="drop-zone-text">{"Click or drag & drop PDFs"}</p>
                                <p class="drop-zone-hint">{"Select multiple PDF files to merge"}</p>
                            </div>
                        </div>

                        // File List
                        {if !merge_files.is_empty() {
                            let files_len = merge_files.len();
                            html! {
                                <div class="section">
                                    <h3>{format!("Files to Merge ({})", files_len)}</h3>
                                    <div class="merge-file-list">
                                        {for merge_files.iter().enumerate().map(|(index, file)| {
                                            let on_remove = {
                                                let on_remove_merge_file = on_remove_merge_file.clone();
                                                Callback::from(move |_: MouseEvent| on_remove_merge_file.emit(index))
                                            };
                                            let on_up = {
                                                let on_move_up = on_move_up.clone();
                                                Callback::from(move |_: MouseEvent| on_move_up.emit(index))
                                            };
                                            let on_down = {
                                                let on_move_down = on_move_down.clone();
                                                Callback::from(move |_: MouseEvent| on_move_down.emit(index))
                                            };
                                            html! {
                                                <div class="merge-file-item">
                                                    <div class="merge-file-order">
                                                        <button
                                                            class="order-btn"
                                                            onclick={on_up}
                                                            disabled={index == 0}
                                                        >{"↑"}</button>
                                                        <button
                                                            class="order-btn"
                                                            onclick={on_down}
                                                            disabled={index == files_len - 1}
                                                        >{"↓"}</button>
                                                    </div>
                                                    <div class="merge-file-info">
                                                        <span class="merge-file-name">{&file.info.file_name}</span>
                                                        <span class="merge-file-pages">{format!("{} pages", file.info.page_count)}</span>
                                                    </div>
                                                    <button class="remove-file-btn" onclick={on_remove}>{"×"}</button>
                                                </div>
                                            }
                                        })}
                                    </div>
                                    <div class="merge-total">
                                        {format!("Total: {} pages", merge_files.iter().map(|f| f.info.page_count).sum::<u32>())}
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }}

                        // Action Buttons
                        <div class="pdf-action-buttons">
                            <button
                                onclick={on_merge}
                                disabled={merge_files.len() < 2 || *is_processing}
                                class="primary-btn compress-btn"
                            >
                                {"Merge PDFs"}
                            </button>
                            {if !merge_files.is_empty() {
                                html! {
                                    <button
                                        onclick={on_reset_merge.clone()}
                                        class="secondary-btn reset-btn"
                                    >
                                        {"Reset"}
                                    </button>
                                }
                            } else {
                                html! {}
                            }}
                        </div>

                        // Merge Result
                        {if let Some(result) = &*merge_result {
                            html! {
                                <div class={if result.success { "section result-box success" } else { "section result-box error" }}>
                                    {if result.success {
                                        html! {
                                            <>
                                                <h3>{"Merge Complete!"}</h3>
                                                <div class="result-stats">
                                                    <div class="result-stat">
                                                        <div class="result-stat-label">{"Pages"}</div>
                                                        <div class="result-stat-value compressed">{result.page_count}</div>
                                                    </div>
                                                    <div class="result-stat">
                                                        <div class="result-stat-label">{"Size"}</div>
                                                        <div class="result-stat-value compressed">{format_size(result.file_size)}</div>
                                                    </div>
                                                </div>
                                                <p class="output-path">{format!("📁 {}", result.output_path)}</p>
                                            </>
                                        }
                                    } else {
                                        html! {
                                            <>
                                                <h3>{"Merge Failed"}</h3>
                                                <p>{result.error.clone().unwrap_or_default()}</p>
                                            </>
                                        }
                                    }}
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                    </>
                }
            }}
        </div>
    }
}
