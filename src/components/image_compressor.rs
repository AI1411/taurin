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
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    pub success: bool,
    pub output_path: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub error: Option<String>,
}

#[derive(Serialize)]
struct OpenDialogOptions {
    multiple: bool,
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
struct GetImageInfoArgs {
    path: String,
}

#[derive(Serialize)]
struct CompressImageArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    quality: u8,
    width: Option<u32>,
    height: Option<u32>,
    #[serde(rename = "outputFormat")]
    output_format: String,
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else {
        format!("{} KB", bytes / 1024)
    }
}

#[function_component(ImageCompressor)]
pub fn image_compressor() -> Html {
    let input_path = use_state(|| String::new());
    let image_info = use_state(|| Option::<ImageInfo>::None);
    let quality = use_state(|| 80u8);
    let output_format = use_state(|| "avif".to_string());
    let custom_width = use_state(|| Option::<u32>::None);
    let custom_height = use_state(|| Option::<u32>::None);
    let compression_result = use_state(|| Option::<CompressionResult>::None);
    let is_processing = use_state(|| false);

    let on_select_file = {
        let input_path = input_path.clone();
        let image_info = image_info.clone();
        let compression_result = compression_result.clone();
        Callback::from(move |_| {
            let input_path = input_path.clone();
            let image_info = image_info.clone();
            let compression_result = compression_result.clone();
            spawn_local(async move {
                let options = OpenDialogOptions {
                    multiple: false,
                    filters: vec![FileFilter {
                        name: "Images".to_string(),
                        extensions: vec![
                            "png".to_string(),
                            "jpg".to_string(),
                            "jpeg".to_string(),
                            "webp".to_string(),
                            "avif".to_string(),
                            "gif".to_string(),
                            "bmp".to_string(),
                        ],
                    }],
                };
                let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
                let result = open(options_js).await;

                if let Some(path) = result.as_string() {
                    input_path.set(path.clone());
                    compression_result.set(None);

                    let args = serde_wasm_bindgen::to_value(&GetImageInfoArgs { path }).unwrap();
                    let info_result = invoke("get_image_info_cmd", args).await;

                    if let Ok(info) = serde_wasm_bindgen::from_value::<ImageInfo>(info_result) {
                        image_info.set(Some(info));
                    }
                }
            });
        })
    };

    let on_compress = {
        let input_path = input_path.clone();
        let quality = quality.clone();
        let output_format = output_format.clone();
        let custom_width = custom_width.clone();
        let custom_height = custom_height.clone();
        let compression_result = compression_result.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let input_path_val = (*input_path).clone();
            if input_path_val.is_empty() {
                return;
            }

            let quality_val = *quality;
            let format_val = (*output_format).clone();
            let width_val = *custom_width;
            let height_val = *custom_height;
            let compression_result = compression_result.clone();
            let is_processing = is_processing.clone();

            is_processing.set(true);

            spawn_local(async move {
                let ext = format_val.clone();
                let default_name = format!("compressed.{}", ext);

                let save_options = SaveDialogOptions {
                    filters: vec![FileFilter {
                        name: format!("{} Image", ext.to_uppercase()),
                        extensions: vec![ext.clone()],
                    }],
                    default_path: Some(default_name),
                };
                let save_options_js = serde_wasm_bindgen::to_value(&save_options).unwrap();
                let save_result = save(save_options_js).await;

                if let Some(output_path) = save_result.as_string() {
                    let args = CompressImageArgs {
                        input_path: input_path_val,
                        output_path,
                        quality: quality_val,
                        width: width_val,
                        height: height_val,
                        output_format: format_val,
                    };
                    let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                    let result = invoke("compress_image_cmd", args_js).await;

                    if let Ok(res) = serde_wasm_bindgen::from_value::<CompressionResult>(result) {
                        compression_result.set(Some(res));
                    }
                }

                is_processing.set(false);
            });
        })
    };

    let on_quality_change = {
        let quality = quality.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(val) = input.value().parse::<u8>() {
                quality.set(val);
            }
        })
    };

    let on_format_change = {
        let output_format = output_format.clone();
        Callback::from(move |format: String| {
            output_format.set(format);
        })
    };

    let on_width_change = {
        let custom_width = custom_width.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            if val.is_empty() {
                custom_width.set(None);
            } else if let Ok(w) = val.parse::<u32>() {
                custom_width.set(Some(w));
            }
        })
    };

    let on_height_change = {
        let custom_height = custom_height.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let val = input.value();
            if val.is_empty() {
                custom_height.set(None);
            } else if let Ok(h) = val.parse::<u32>() {
                custom_height.set(Some(h));
            }
        })
    };

    let formats = vec![
        ("avif", "AVIF", Some("Best")),
        ("webp", "WebP", Some("Good")),
        ("jpeg", "JPEG", None),
        ("png", "PNG", None),
    ];

    html! {
        <div class="image-compressor">
            // File Selection
            <div class="section" onclick={on_select_file.clone()}>
                <div class="drop-zone">
                    <div class="drop-zone-icon">{"🖼️"}</div>
                    <p class="drop-zone-text">{"Click to select an image"}</p>
                    <p class="drop-zone-hint">{"PNG, JPEG, WebP, AVIF, GIF, BMP"}</p>
                </div>
                {if !input_path.is_empty() {
                    html! { <p class="file-path">{&*input_path}</p> }
                } else {
                    html! {}
                }}
            </div>

            // Image Info
            {if let Some(info) = &*image_info {
                html! {
                    <div class="section info-box">
                        <h3>{"Image Info"}</h3>
                        <div class="info-grid">
                            <div class="info-item">
                                <div class="info-item-label">{"Dimensions"}</div>
                                <div class="info-item-value">{format!("{}×{}", info.width, info.height)}</div>
                            </div>
                            <div class="info-item">
                                <div class="info-item-label">{"Format"}</div>
                                <div class="info-item-value">{&info.format}</div>
                            </div>
                            <div class="info-item">
                                <div class="info-item-label">{"Size"}</div>
                                <div class="info-item-value">{format_size(info.file_size)}</div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

            // Compression Options
            <div class="section">
                <h3>{"Output Format"}</h3>
                <div class="format-options">
                    {for formats.iter().map(|(value, name, badge)| {
                        let is_selected = *output_format == *value;
                        let format_value = value.to_string();
                        let on_click = {
                            let on_format_change = on_format_change.clone();
                            let format_value = format_value.clone();
                            Callback::from(move |_: MouseEvent| {
                                on_format_change.emit(format_value.clone());
                            })
                        };
                        html! {
                            <div class="format-option" onclick={on_click}>
                                <input
                                    type="radio"
                                    name="format"
                                    value={*value}
                                    checked={is_selected}
                                />
                                <label>
                                    <span class="format-name">{name}</span>
                                    {if let Some(b) = badge {
                                        html! { <span class="format-badge">{b}</span> }
                                    } else {
                                        html! {}
                                    }}
                                </label>
                            </div>
                        }
                    })}
                </div>
            </div>

            <div class="section">
                <h3>{"Quality"}</h3>
                <div class="quality-slider">
                    <input
                        type="range"
                        min="1"
                        max="100"
                        value={quality.to_string()}
                        oninput={on_quality_change}
                    />
                    <span class="quality-value">{format!("{}%", *quality)}</span>
                </div>
            </div>

            <div class="section">
                <h3>{"Resize (Optional)"}</h3>
                <div class="resize-inputs">
                    <input
                        type="number"
                        placeholder="Width"
                        oninput={on_width_change}
                    />
                    <span>{"×"}</span>
                    <input
                        type="number"
                        placeholder="Height"
                        oninput={on_height_change}
                    />
                </div>
            </div>

            // Compress Button
            <button
                onclick={on_compress}
                disabled={input_path.is_empty() || *is_processing}
                class="primary-btn compress-btn"
            >
                {if *is_processing {
                    html! {
                        <span class="processing">
                            <span class="spinner"></span>
                            {"Processing..."}
                        </span>
                    }
                } else {
                    html! { <>{"Compress & Save"}</> }
                }}
            </button>

            // Result
            {if let Some(result) = &*compression_result {
                html! {
                    <div class={if result.success { "section result-box success" } else { "section result-box error" }}>
                        {if result.success {
                            html! {
                                <>
                                    <h3>{"Compression Complete!"}</h3>
                                    <div class="result-stats">
                                        <div class="result-stat">
                                            <div class="result-stat-label">{"Original"}</div>
                                            <div class="result-stat-value original">{format_size(result.original_size)}</div>
                                        </div>
                                        <div class="result-stat">
                                            <div class="result-stat-label">{"Compressed"}</div>
                                            <div class="result-stat-value compressed">{format_size(result.compressed_size)}</div>
                                        </div>
                                        <div class="result-stat">
                                            <div class="result-stat-label">{"Saved"}</div>
                                            <div class="result-stat-value saved">{format!("{:.1}%", result.compression_ratio)}</div>
                                        </div>
                                    </div>
                                    <p class="output-path">{format!("📁 {}", result.output_path)}</p>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <h3>{"Compression Failed"}</h3>
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
