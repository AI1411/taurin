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
pub struct MarkdownInfo {
    pub file_name: String,
    pub file_size: u64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownToHtmlResult {
    pub success: bool,
    pub html: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownToPdfResult {
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
struct ReadMarkdownArgs {
    path: String,
}

#[derive(Serialize)]
struct MarkdownToHtmlArgs {
    markdown: String,
}

#[derive(Serialize)]
struct ConvertToPdfArgs {
    markdown: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    #[serde(rename = "sourcePath")]
    source_path: Option<String>,
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

#[derive(Properties, PartialEq)]
pub struct MarkdownToPdfProps {
    #[prop_or_default]
    pub dropped_file: Option<String>,
    #[prop_or_default]
    pub on_file_processed: Callback<()>,
}

#[function_component(MarkdownToPdf)]
pub fn markdown_to_pdf(props: &MarkdownToPdfProps) -> Html {
    let is_processing = use_state(|| false);
    let input_path = use_state(|| String::new());
    let markdown_info = use_state(|| Option::<MarkdownInfo>::None);
    let html_preview = use_state(|| String::new());
    let convert_result = use_state(|| Option::<MarkdownToPdfResult>::None);

    // Handle dropped file
    {
        let dropped_file = props.dropped_file.clone();
        let on_file_processed = props.on_file_processed.clone();
        let input_path = input_path.clone();
        let markdown_info = markdown_info.clone();
        let html_preview = html_preview.clone();
        let convert_result = convert_result.clone();

        use_effect_with(dropped_file.clone(), move |dropped_file| {
            if let Some(path) = dropped_file.clone() {
                let input_path = input_path.clone();
                let markdown_info = markdown_info.clone();
                let html_preview = html_preview.clone();
                let convert_result = convert_result.clone();
                let on_file_processed = on_file_processed.clone();

                spawn_local(async move {
                    let args =
                        serde_wasm_bindgen::to_value(&ReadMarkdownArgs { path: path.clone() })
                            .unwrap();
                    let info_result = invoke("read_markdown_cmd", args).await;

                    if let Ok(info) = serde_wasm_bindgen::from_value::<MarkdownInfo>(info_result) {
                        input_path.set(path);
                        convert_result.set(None);

                        // Generate HTML preview
                        let html_args = serde_wasm_bindgen::to_value(&MarkdownToHtmlArgs {
                            markdown: info.content.clone(),
                        })
                        .unwrap();
                        let html_result = invoke("markdown_to_html_cmd", html_args).await;

                        if let Ok(html_res) =
                            serde_wasm_bindgen::from_value::<MarkdownToHtmlResult>(html_result)
                        {
                            if html_res.success {
                                html_preview.set(html_res.html);
                            }
                        }

                        markdown_info.set(Some(info));
                    }

                    on_file_processed.emit(());
                });
            }
            || {}
        });
    }

    let on_select_file = {
        let input_path = input_path.clone();
        let markdown_info = markdown_info.clone();
        let html_preview = html_preview.clone();
        let convert_result = convert_result.clone();
        Callback::from(move |_| {
            let input_path = input_path.clone();
            let markdown_info = markdown_info.clone();
            let html_preview = html_preview.clone();
            let convert_result = convert_result.clone();
            spawn_local(async move {
                let options = OpenDialogOptions {
                    multiple: false,
                    directory: false,
                    filters: vec![FileFilter {
                        name: "Markdown".to_string(),
                        extensions: vec!["md".to_string(), "markdown".to_string()],
                    }],
                };
                let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
                let result = open(options_js).await;

                if let Some(path) = result.as_string() {
                    input_path.set(path.clone());
                    convert_result.set(None);

                    let args = serde_wasm_bindgen::to_value(&ReadMarkdownArgs { path }).unwrap();
                    let info_result = invoke("read_markdown_cmd", args).await;

                    if let Ok(info) = serde_wasm_bindgen::from_value::<MarkdownInfo>(info_result) {
                        // Generate HTML preview
                        let html_args = serde_wasm_bindgen::to_value(&MarkdownToHtmlArgs {
                            markdown: info.content.clone(),
                        })
                        .unwrap();
                        let html_result = invoke("markdown_to_html_cmd", html_args).await;

                        if let Ok(html_res) =
                            serde_wasm_bindgen::from_value::<MarkdownToHtmlResult>(html_result)
                        {
                            if html_res.success {
                                html_preview.set(html_res.html);
                            }
                        }

                        markdown_info.set(Some(info));
                    }
                }
            });
        })
    };

    let on_convert = {
        let input_path = input_path.clone();
        let markdown_info = markdown_info.clone();
        let convert_result = convert_result.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let markdown_content = match &*markdown_info {
                Some(info) => info.content.clone(),
                None => return,
            };

            let input_path_val = (*input_path).clone();
            if input_path_val.is_empty() {
                return;
            }

            let convert_result = convert_result.clone();
            let is_processing = is_processing.clone();

            is_processing.set(true);

            let source_path = input_path_val.clone();
            spawn_local(async move {
                let default_name = input_path_val
                    .rsplit('/')
                    .next()
                    .unwrap_or("output")
                    .replace(".md", ".pdf")
                    .replace(".markdown", ".pdf");

                let save_options = SaveDialogOptions {
                    filters: vec![FileFilter {
                        name: "PDF".to_string(),
                        extensions: vec!["pdf".to_string()],
                    }],
                    default_path: Some(default_name),
                };
                let save_options_js = serde_wasm_bindgen::to_value(&save_options).unwrap();
                let save_result = save(save_options_js).await;

                if let Some(output_path) = save_result.as_string() {
                    let args = ConvertToPdfArgs {
                        markdown: markdown_content,
                        output_path,
                        source_path: Some(source_path),
                    };
                    let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                    let result = invoke("convert_markdown_to_pdf_cmd", args_js).await;

                    if let Ok(res) = serde_wasm_bindgen::from_value::<MarkdownToPdfResult>(result) {
                        convert_result.set(Some(res));
                    }
                }

                is_processing.set(false);
            });
        })
    };

    let on_reset = {
        let input_path = input_path.clone();
        let markdown_info = markdown_info.clone();
        let html_preview = html_preview.clone();
        let convert_result = convert_result.clone();
        Callback::from(move |_| {
            input_path.set(String::new());
            markdown_info.set(None);
            html_preview.set(String::new());
            convert_result.set(None);
        })
    };

    html! {
        <div class="markdown-to-pdf">
            // Processing Overlay
            {if *is_processing {
                html! {
                    <div class="processing-overlay">
                        <div class="processing-content">
                            <div class="processing-spinner"></div>
                            <p class="processing-title">{"Converting..."}</p>
                            <p class="processing-hint">{"Converting Markdown to PDF"}</p>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

            // File Selection
            <div class="section" onclick={on_select_file.clone()}>
                <div class="drop-zone">
                    <div class="drop-zone-icon">{"📝"}</div>
                    <p class="drop-zone-text">{"Click or drag & drop a Markdown file"}</p>
                    <p class="drop-zone-hint">{"Supports .md and .markdown files"}</p>
                </div>
                {if !input_path.is_empty() {
                    html! { <p class="file-path">{&*input_path}</p> }
                } else {
                    html! {}
                }}
            </div>

            // Markdown Info
            {if let Some(info) = &*markdown_info {
                html! {
                    <div class="section info-box">
                        <h3>{"File Info"}</h3>
                        <div class="info-grid">
                            <div class="info-item">
                                <div class="info-item-label">{"File"}</div>
                                <div class="info-item-value file-name-value">{&info.file_name}</div>
                            </div>
                            <div class="info-item">
                                <div class="info-item-label">{"Size"}</div>
                                <div class="info-item-value">{format_size(info.file_size)}</div>
                            </div>
                            <div class="info-item">
                                <div class="info-item-label">{"Characters"}</div>
                                <div class="info-item-value">{info.content.len()}</div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

            // Preview
            {if !html_preview.is_empty() {
                html! {
                    <div class="section">
                        <h3>{"Preview"}</h3>
                        <div class="markdown-preview">
                            <div class="markdown-preview-content">
                                {Html::from_html_unchecked(AttrValue::from((*html_preview).clone()))}
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

            // Action Buttons
            <div class="pdf-action-buttons">
                <button
                    onclick={on_convert}
                    disabled={input_path.is_empty() || *is_processing}
                    class="primary-btn compress-btn"
                >
                    {"Convert to PDF"}
                </button>
                {if !input_path.is_empty() {
                    html! {
                        <button
                            onclick={on_reset.clone()}
                            class="secondary-btn reset-btn"
                        >
                            {"Reset"}
                        </button>
                    }
                } else {
                    html! {}
                }}
            </div>

            // Convert Result
            {if let Some(result) = &*convert_result {
                html! {
                    <div class={if result.success { "section result-box success" } else { "section result-box error" }}>
                        {if result.success {
                            html! {
                                <>
                                    <h3>{"Conversion Complete!"}</h3>
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
                                    <p class="output-path">{format!("{} {}", "📁", result.output_path)}</p>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <h3>{"Conversion Failed"}</h3>
                                    <p>{result.error.clone().unwrap_or_default()}</p>
                                </>
                            }
                        }}
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
