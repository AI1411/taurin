use i18nrs::yew::use_translation;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(options: JsValue) -> JsValue;
}

#[derive(Serialize)]
struct OpenDialogOptions {
    multiple: bool,
    filters: Vec<FileFilter>,
}

#[derive(Serialize)]
struct FileFilter {
    name: String,
    extensions: Vec<String>,
}

#[derive(Clone, PartialEq, Copy)]
enum Mode {
    Encode,
    Decode,
    Image,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EncodeArgs {
    input: String,
    url_safe: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DecodeArgs {
    input: String,
    url_safe: bool,
}

#[derive(Serialize)]
struct ImageEncodeArgs {
    path: String,
}

#[derive(Serialize)]
struct ImageDecodeArgs {
    input: String,
}

#[derive(Debug, Clone, Deserialize)]
struct EncodeResult {
    success: bool,
    output: String,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DecodeResult {
    success: bool,
    output: String,
    is_valid_utf8: bool,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImageEncodeResult {
    success: bool,
    output: String,
    mime_type: String,
    data_url: String,
    size_bytes: usize,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImageDecodeResult {
    success: bool,
    mime_type: Option<String>,
    size_bytes: usize,
    preview_data_url: Option<String>,
    error: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub dropped_file: Option<String>,
    #[prop_or_default]
    pub on_file_processed: Callback<()>,
}

#[function_component(Base64Encoder)]
pub fn base64_encoder(props: &Props) -> Html {
    let (i18n, _) = use_translation();
    let mode = use_state(|| Mode::Encode);
    let input = use_state(String::new);
    let output = use_state(String::new);
    let url_safe = use_state(|| false);
    let is_processing = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let copy_feedback = use_state(|| false);
    let is_binary = use_state(|| false);

    // Image mode states
    let image_preview = use_state(|| Option::<String>::None); // data URL after encoding
    let image_info = use_state(|| Option::<(String, usize)>::None); // (mime_type, size)
    let decoded_image_preview = use_state(|| Option::<String>::None);

    // Handle dropped file
    {
        let dropped_file = props.dropped_file.clone();
        let on_file_processed = props.on_file_processed.clone();
        let mode = mode.clone();
        let output = output.clone();
        let image_preview = image_preview.clone();
        let image_info = image_info.clone();
        let error = error.clone();
        let is_processing = is_processing.clone();

        use_effect_with(dropped_file.clone(), move |dropped_file| {
            if let Some(path) = dropped_file {
                let path = path.clone();
                let mode = mode.clone();
                let output = output.clone();
                let image_preview = image_preview.clone();
                let image_info = image_info.clone();
                let error = error.clone();
                let is_processing = is_processing.clone();
                let on_file_processed = on_file_processed.clone();

                spawn_local(async move {
                    mode.set(Mode::Image);
                    is_processing.set(true);

                    let args = serde_wasm_bindgen::to_value(&ImageEncodeArgs { path }).unwrap();
                    let result = invoke("encode_image_to_base64_cmd", args).await;

                    match serde_wasm_bindgen::from_value::<ImageEncodeResult>(result) {
                        Ok(res) => {
                            if res.success {
                                output.set(res.output.clone());
                                image_preview.set(Some(res.data_url));
                                image_info.set(Some((res.mime_type, res.size_bytes)));
                                error.set(None);
                            } else {
                                error.set(res.error.or(Some("Encoding failed".to_string())));
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to parse result: {:?}", e)));
                        }
                    }

                    is_processing.set(false);
                    on_file_processed.emit(());
                });
            }

            || {}
        });
    }

    let on_mode_change = {
        let mode = mode.clone();
        let input = input.clone();
        let output = output.clone();
        let error = error.clone();
        let image_preview = image_preview.clone();
        let image_info = image_info.clone();
        let decoded_image_preview = decoded_image_preview.clone();
        let is_binary = is_binary.clone();
        Callback::from(move |new_mode: Mode| {
            mode.set(new_mode);
            input.set(String::new());
            output.set(String::new());
            error.set(None);
            image_preview.set(None);
            image_info.set(None);
            decoded_image_preview.set(None);
            is_binary.set(false);
        })
    };

    let on_input_change = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let target: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            input.set(target.value());
        })
    };

    let on_url_safe_change = {
        let url_safe = url_safe.clone();
        Callback::from(move |e: Event| {
            let checkbox: web_sys::HtmlInputElement = e.target_unchecked_into();
            url_safe.set(checkbox.checked());
        })
    };

    let on_convert = {
        let mode = mode.clone();
        let input = input.clone();
        let output = output.clone();
        let url_safe = url_safe.clone();
        let is_processing = is_processing.clone();
        let error = error.clone();
        let is_binary = is_binary.clone();
        let decoded_image_preview = decoded_image_preview.clone();

        Callback::from(move |_| {
            let current_mode = *mode;
            let input_val = (*input).clone();
            let url_safe_val = *url_safe;
            let output = output.clone();
            let is_processing = is_processing.clone();
            let error = error.clone();
            let is_binary = is_binary.clone();
            let decoded_image_preview = decoded_image_preview.clone();

            if input_val.trim().is_empty() {
                return;
            }

            is_processing.set(true);

            spawn_local(async move {
                match current_mode {
                    Mode::Encode => {
                        let args = serde_wasm_bindgen::to_value(&EncodeArgs {
                            input: input_val,
                            url_safe: url_safe_val,
                        })
                        .unwrap();
                        let result = invoke("encode_base64_cmd", args).await;

                        if let Ok(res) = serde_wasm_bindgen::from_value::<EncodeResult>(result) {
                            if res.success {
                                output.set(res.output);
                                error.set(None);
                            } else {
                                error.set(res.error);
                            }
                        }
                    }
                    Mode::Decode => {
                        // First try to decode as image
                        let img_args = serde_wasm_bindgen::to_value(&ImageDecodeArgs {
                            input: input_val.clone(),
                        })
                        .unwrap();
                        let img_result = invoke("decode_base64_image_cmd", img_args).await;

                        if let Ok(img_res) =
                            serde_wasm_bindgen::from_value::<ImageDecodeResult>(img_result)
                        {
                            if img_res.success && img_res.preview_data_url.is_some() {
                                decoded_image_preview.set(img_res.preview_data_url);
                            } else {
                                decoded_image_preview.set(None);
                            }
                        }

                        // Also decode as text
                        let args = serde_wasm_bindgen::to_value(&DecodeArgs {
                            input: input_val,
                            url_safe: url_safe_val,
                        })
                        .unwrap();
                        let result = invoke("decode_base64_cmd", args).await;

                        if let Ok(res) = serde_wasm_bindgen::from_value::<DecodeResult>(result) {
                            if res.success {
                                output.set(res.output);
                                is_binary.set(!res.is_valid_utf8);
                                error.set(None);
                            } else {
                                error.set(res.error);
                            }
                        }
                    }
                    Mode::Image => {
                        // Image mode uses file drop
                    }
                }

                is_processing.set(false);
            });
        })
    };

    let on_copy = {
        let output = output.clone();
        let copy_feedback = copy_feedback.clone();
        Callback::from(move |_| {
            let output_val = (*output).clone();
            let copy_feedback = copy_feedback.clone();

            if !output_val.is_empty() {
                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    spawn_local(async move {
                        let _ =
                            wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&output_val))
                                .await;
                        copy_feedback.set(true);

                        let copy_feedback_reset = copy_feedback.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            copy_feedback_reset.set(false);
                        })
                        .forget();
                    });
                }
            }
        })
    };

    let on_copy_data_url = {
        let image_preview = image_preview.clone();
        let copy_feedback = copy_feedback.clone();
        Callback::from(move |_| {
            if let Some(data_url) = (*image_preview).clone() {
                let copy_feedback = copy_feedback.clone();
                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    spawn_local(async move {
                        let _ =
                            wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&data_url))
                                .await;
                        copy_feedback.set(true);

                        let copy_feedback_reset = copy_feedback.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            copy_feedback_reset.set(false);
                        })
                        .forget();
                    });
                }
            }
        })
    };

    let on_clear = {
        let input = input.clone();
        let output = output.clone();
        let error = error.clone();
        let image_preview = image_preview.clone();
        let image_info = image_info.clone();
        let decoded_image_preview = decoded_image_preview.clone();
        let is_binary = is_binary.clone();
        Callback::from(move |_| {
            input.set(String::new());
            output.set(String::new());
            error.set(None);
            image_preview.set(None);
            image_info.set(None);
            decoded_image_preview.set(None);
            is_binary.set(false);
        })
    };

    let on_swap = {
        let input = input.clone();
        let output = output.clone();
        let mode = mode.clone();
        Callback::from(move |_| {
            let current_output = (*output).clone();
            let current_mode = *mode;

            if !current_output.is_empty() && current_mode != Mode::Image {
                input.set(current_output);
                output.set(String::new());
                // Toggle mode
                match current_mode {
                    Mode::Encode => mode.set(Mode::Decode),
                    Mode::Decode => mode.set(Mode::Encode),
                    Mode::Image => {}
                }
            }
        })
    };

    let on_select_file = {
        let output = output.clone();
        let image_preview = image_preview.clone();
        let image_info = image_info.clone();
        let error = error.clone();
        let is_processing = is_processing.clone();
        Callback::from(move |_| {
            let output = output.clone();
            let image_preview = image_preview.clone();
            let image_info = image_info.clone();
            let error = error.clone();
            let is_processing = is_processing.clone();

            spawn_local(async move {
                let options = OpenDialogOptions {
                    multiple: false,
                    filters: vec![FileFilter {
                        name: "Images".to_string(),
                        extensions: vec![
                            "png".to_string(),
                            "jpg".to_string(),
                            "jpeg".to_string(),
                            "gif".to_string(),
                            "webp".to_string(),
                            "bmp".to_string(),
                            "svg".to_string(),
                            "avif".to_string(),
                            "ico".to_string(),
                        ],
                    }],
                };

                let opts = serde_wasm_bindgen::to_value(&options).unwrap();
                let selected = open(opts).await;

                if let Some(path) = selected.as_string() {
                    is_processing.set(true);

                    let args = serde_wasm_bindgen::to_value(&ImageEncodeArgs { path }).unwrap();
                    let result = invoke("encode_image_to_base64_cmd", args).await;

                    match serde_wasm_bindgen::from_value::<ImageEncodeResult>(result) {
                        Ok(res) => {
                            if res.success {
                                output.set(res.output.clone());
                                image_preview.set(Some(res.data_url));
                                image_info.set(Some((res.mime_type, res.size_bytes)));
                                error.set(None);
                            } else {
                                error.set(res.error.or(Some("Encoding failed".to_string())));
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to parse result: {:?}", e)));
                        }
                    }

                    is_processing.set(false);
                }
            });
        })
    };

    html! {
        <div class="base64-encoder">
            // Mode selector
            <div class="section mode-section">
                <div class="mode-tabs">
                    <button
                        class={classes!("mode-tab", (*mode == Mode::Encode).then_some("active"))}
                        onclick={
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(Mode::Encode))
                        }
                    >
                        {i18n.t("base64_encoder.mode_encode")}
                    </button>
                    <button
                        class={classes!("mode-tab", (*mode == Mode::Decode).then_some("active"))}
                        onclick={
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(Mode::Decode))
                        }
                    >
                        {i18n.t("base64_encoder.mode_decode")}
                    </button>
                    <button
                        class={classes!("mode-tab", (*mode == Mode::Image).then_some("active"))}
                        onclick={
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(Mode::Image))
                        }
                    >
                        {i18n.t("base64_encoder.mode_image")}
                    </button>
                </div>
            </div>

            // Options section
            if *mode != Mode::Image {
                <div class="section options-section">
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            checked={*url_safe}
                            onchange={on_url_safe_change}
                        />
                        <span>{i18n.t("base64_encoder.url_safe")}</span>
                    </label>
                </div>
            }

            // Input section
            <div class="section input-section">
                <div class="section-header">
                    <h3>
                        {match *mode {
                            Mode::Encode => i18n.t("base64_encoder.text_input"),
                            Mode::Decode => i18n.t("base64_encoder.base64_input"),
                            Mode::Image => i18n.t("base64_encoder.image_input"),
                        }}
                    </h3>
                    <button class="secondary-btn" onclick={on_clear}>
                        {i18n.t("common.clear")}
                    </button>
                </div>

                if *mode == Mode::Image {
                    <div class="drop-zone" onclick={on_select_file.clone()}>
                        if *is_processing {
                            <div class="drop-zone-content">
                                <span class="spinner"></span>
                                <p>{i18n.t("common.processing")}</p>
                            </div>
                        } else if let Some(preview_url) = (*image_preview).clone() {
                            <div class="image-preview-container">
                                <img src={preview_url} alt="Preview" class="image-preview" />
                                if let Some((mime_type, size)) = (*image_info).clone() {
                                    <div class="image-info">
                                        <span class="info-item">{mime_type}</span>
                                        <span class="info-item">{format_file_size(size)}</span>
                                    </div>
                                }
                                <p class="change-image-hint">{i18n.t("base64_encoder.click_to_change")}</p>
                            </div>
                        } else {
                            <div class="drop-zone-content">
                                <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                                    <rect x="3" y="3" width="18" height="18" rx="2"/>
                                    <circle cx="8.5" cy="8.5" r="1.5"/>
                                    <path d="M21 15l-5-5L5 21"/>
                                </svg>
                                <p>{i18n.t("base64_encoder.drop_image")}</p>
                                <span class="formats">{i18n.t("base64_encoder.supported_formats")}</span>
                            </div>
                        }
                    </div>
                } else {
                    <textarea
                        class="input-textarea"
                        placeholder={match *mode {
                            Mode::Encode => i18n.t("base64_encoder.encode_placeholder"),
                            Mode::Decode => i18n.t("base64_encoder.decode_placeholder"),
                            Mode::Image => String::new(),
                        }}
                        value={(*input).clone()}
                        oninput={on_input_change}
                    />
                }
            </div>

            // Action buttons (only for text encode/decode modes)
            if *mode != Mode::Image {
                <div class="action-buttons">
                    <button
                        class="primary-btn"
                        onclick={on_convert}
                        disabled={*is_processing || (*input).is_empty()}
                    >
                        if *is_processing {
                            <span class="processing">
                                <span class="spinner"></span>
                                {i18n.t("common.processing")}
                            </span>
                        } else {
                            {match *mode {
                                Mode::Encode => i18n.t("base64_encoder.encode_btn"),
                                Mode::Decode => i18n.t("base64_encoder.decode_btn"),
                                Mode::Image => String::new(),
                            }}
                        }
                    </button>

                    if !(*output).is_empty() {
                        <button class="secondary-btn swap-btn" onclick={on_swap}>
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M7 16V4M7 4L3 8M7 4L11 8M17 8V20M17 20L21 16M17 20L13 16"/>
                            </svg>
                            {i18n.t("common.swap")}
                        </button>
                    }
                </div>
            }

            // Error display
            if let Some(err) = (*error).clone() {
                <div class="section error-section">
                    <div class="error-message">
                        {"⚠ "}{err}
                    </div>
                </div>
            }

            // Output section (show only after encoding)
            if !(*output).is_empty() {
                <div class="section output-section">
                    <div class="section-header">
                        <h3>
                            {match *mode {
                                Mode::Encode => i18n.t("base64_encoder.base64_output"),
                                Mode::Decode => {
                                    if *is_binary {
                                        i18n.t("base64_encoder.binary_output")
                                    } else {
                                        i18n.t("base64_encoder.text_output")
                                    }
                                },
                                Mode::Image => i18n.t("base64_encoder.base64_output"),
                            }}
                        </h3>
                        <div class="output-actions">
                            <button
                                class={classes!("secondary-btn", (*copy_feedback).then_some("copied"))}
                                onclick={on_copy.clone()}
                            >
                                if *copy_feedback {
                                    {format!("✓ {}", i18n.t("common.copied"))}
                                } else {
                                    {i18n.t("common.copy")}
                                }
                            </button>
                            if *mode == Mode::Image && (*image_preview).is_some() {
                                <button
                                    class="secondary-btn"
                                    onclick={on_copy_data_url}
                                >
                                    {i18n.t("base64_encoder.copy_data_url")}
                                </button>
                            }
                        </div>
                    </div>

                    // Decoded image preview
                    if *mode == Mode::Decode {
                        if let Some(preview_url) = (*decoded_image_preview).clone() {
                            <div class="decoded-image-preview">
                                <img src={preview_url} alt="Decoded image" />
                            </div>
                        }
                    }

                    <div class={classes!("output-textarea-wrapper", (*is_binary).then_some("binary"))}>
                        <textarea
                            class="output-textarea"
                            readonly=true
                            value={(*output).clone()}
                        />
                    </div>

                    // Output stats
                    <div class="output-stats">
                        <span class="stat-item">
                            {format!("{} {}", (*output).len(), i18n.t("common.characters"))}
                        </span>
                        if *mode == Mode::Encode || *mode == Mode::Image {
                            <span class="stat-item">
                                {format!("≈ {}", format_file_size((*output).len() * 3 / 4))}
                            </span>
                        }
                        if let Some((mime_type, size)) = (*image_info).clone() {
                            <span class="stat-item">{mime_type}</span>
                            <span class="stat-item">{format!("{} {}", i18n.t("base64_encoder.original_size"), format_file_size(size))}</span>
                        }
                    </div>
                </div>
            }
        </div>
    }
}

fn format_file_size(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}
