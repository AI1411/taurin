use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    fn convertFileSrc(path: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(options: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn save(options: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageEditorInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditResult {
    pub success: bool,
    pub output_path: String,
    pub original_size: u64,
    pub new_size: u64,
    pub new_width: u32,
    pub new_height: u32,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RotationAngle {
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ImageFilter {
    Grayscale,
    Sepia,
    Invert,
    Blur,
    Sharpen,
}

#[derive(Clone, PartialEq)]
enum EditMode {
    Resize,
    Rotate,
    Crop,
    Brightness,
    Contrast,
    Filter,
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
struct ResizeArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    width: u32,
    height: u32,
    #[serde(rename = "maintainAspect")]
    maintain_aspect: bool,
}

#[derive(Serialize)]
struct RotateArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    angle: RotationAngle,
}

#[derive(Serialize)]
struct CropArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Serialize)]
struct BrightnessArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    value: i32,
}

#[derive(Serialize)]
struct ContrastArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    value: f32,
}

#[derive(Serialize)]
struct FilterArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
    #[serde(rename = "outputPath")]
    output_path: String,
    filter: ImageFilter,
}

#[derive(Serialize)]
struct FlipArgs {
    #[serde(rename = "inputPath")]
    input_path: String,
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

#[derive(Properties, PartialEq)]
pub struct ImageEditorProps {
    #[prop_or_default]
    pub dropped_file: Option<String>,
    #[prop_or_default]
    pub on_file_processed: Callback<()>,
}

#[function_component(ImageEditor)]
pub fn image_editor(props: &ImageEditorProps) -> Html {
    let input_path = use_state(|| String::new());
    let image_info = use_state(|| Option::<ImageEditorInfo>::None);
    let image_preview_url = use_state(|| String::new());
    let edit_mode = use_state(|| EditMode::Resize);
    let edit_result = use_state(|| Option::<EditResult>::None);
    let is_processing = use_state(|| false);

    // Resize options
    let resize_width = use_state(|| 800u32);
    let resize_height = use_state(|| 600u32);
    let maintain_aspect = use_state(|| true);

    // Rotate options
    let rotation_angle = use_state(|| RotationAngle::Rotate90);

    // Crop options
    let crop_x = use_state(|| 0u32);
    let crop_y = use_state(|| 0u32);
    let crop_width = use_state(|| 400u32);
    let crop_height = use_state(|| 300u32);

    // Brightness/Contrast options
    let brightness = use_state(|| 0i32);
    let contrast = use_state(|| 1.0f32);

    // Filter option
    let selected_filter = use_state(|| ImageFilter::Grayscale);

    // Handle dropped file
    {
        let dropped_file = props.dropped_file.clone();
        let on_file_processed = props.on_file_processed.clone();
        let input_path = input_path.clone();
        let image_info = image_info.clone();
        let image_preview_url = image_preview_url.clone();
        let edit_result = edit_result.clone();
        let resize_width = resize_width.clone();
        let resize_height = resize_height.clone();
        let crop_width = crop_width.clone();
        let crop_height = crop_height.clone();

        use_effect_with(dropped_file.clone(), move |dropped_file| {
            if let Some(path) = dropped_file.clone() {
                let input_path = input_path.clone();
                let image_info = image_info.clone();
                let image_preview_url = image_preview_url.clone();
                let edit_result = edit_result.clone();
                let on_file_processed = on_file_processed.clone();
                let resize_width = resize_width.clone();
                let resize_height = resize_height.clone();
                let crop_width = crop_width.clone();
                let crop_height = crop_height.clone();

                spawn_local(async move {
                    input_path.set(path.clone());
                    edit_result.set(None);

                    // Generate preview URL
                    let preview_url = convertFileSrc(&path);
                    if let Some(url) = preview_url.as_string() {
                        image_preview_url.set(url);
                    }

                    let args = serde_wasm_bindgen::to_value(&GetImageInfoArgs { path }).unwrap();
                    let info_result = invoke("get_editor_image_info_cmd", args).await;

                    if let Ok(info) = serde_wasm_bindgen::from_value::<ImageEditorInfo>(info_result)
                    {
                        resize_width.set(info.width);
                        resize_height.set(info.height);
                        crop_width.set(info.width.min(400));
                        crop_height.set(info.height.min(300));
                        image_info.set(Some(info));
                    }

                    on_file_processed.emit(());
                });
            }
            || {}
        });
    }

    let on_select_file = {
        let input_path = input_path.clone();
        let image_info = image_info.clone();
        let image_preview_url = image_preview_url.clone();
        let edit_result = edit_result.clone();
        let resize_width = resize_width.clone();
        let resize_height = resize_height.clone();
        let crop_width = crop_width.clone();
        let crop_height = crop_height.clone();
        Callback::from(move |_| {
            let input_path = input_path.clone();
            let image_info = image_info.clone();
            let image_preview_url = image_preview_url.clone();
            let edit_result = edit_result.clone();
            let resize_width = resize_width.clone();
            let resize_height = resize_height.clone();
            let crop_width = crop_width.clone();
            let crop_height = crop_height.clone();
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
                            "gif".to_string(),
                            "bmp".to_string(),
                        ],
                    }],
                };
                let options_js = serde_wasm_bindgen::to_value(&options).unwrap();
                let result = open(options_js).await;

                if let Some(path) = result.as_string() {
                    input_path.set(path.clone());
                    edit_result.set(None);

                    // Generate preview URL
                    let preview_url = convertFileSrc(&path);
                    if let Some(url) = preview_url.as_string() {
                        image_preview_url.set(url);
                    }

                    let args = serde_wasm_bindgen::to_value(&GetImageInfoArgs { path }).unwrap();
                    let info_result = invoke("get_editor_image_info_cmd", args).await;

                    if let Ok(info) = serde_wasm_bindgen::from_value::<ImageEditorInfo>(info_result)
                    {
                        resize_width.set(info.width);
                        resize_height.set(info.height);
                        crop_width.set(info.width.min(400));
                        crop_height.set(info.height.min(300));
                        image_info.set(Some(info));
                    }
                }
            });
        })
    };

    let on_apply_edit = {
        let input_path = input_path.clone();
        let edit_mode = edit_mode.clone();
        let edit_result = edit_result.clone();
        let is_processing = is_processing.clone();
        let resize_width = resize_width.clone();
        let resize_height = resize_height.clone();
        let maintain_aspect = maintain_aspect.clone();
        let rotation_angle = rotation_angle.clone();
        let crop_x = crop_x.clone();
        let crop_y = crop_y.clone();
        let crop_width = crop_width.clone();
        let crop_height = crop_height.clone();
        let brightness = brightness.clone();
        let contrast = contrast.clone();
        let selected_filter = selected_filter.clone();

        Callback::from(move |_| {
            let input_path_val = (*input_path).clone();
            if input_path_val.is_empty() {
                return;
            }

            let edit_mode_val = (*edit_mode).clone();
            let edit_result = edit_result.clone();
            let is_processing = is_processing.clone();
            let resize_width_val = *resize_width;
            let resize_height_val = *resize_height;
            let maintain_aspect_val = *maintain_aspect;
            let rotation_angle_val = *rotation_angle;
            let crop_x_val = *crop_x;
            let crop_y_val = *crop_y;
            let crop_width_val = *crop_width;
            let crop_height_val = *crop_height;
            let brightness_val = *brightness;
            let contrast_val = *contrast;
            let selected_filter_val = *selected_filter;

            is_processing.set(true);

            spawn_local(async move {
                let default_name = "edited.png".to_string();
                let save_options = SaveDialogOptions {
                    filters: vec![FileFilter {
                        name: "PNG Image".to_string(),
                        extensions: vec!["png".to_string()],
                    }],
                    default_path: Some(default_name),
                };
                let save_options_js = serde_wasm_bindgen::to_value(&save_options).unwrap();
                let save_result = save(save_options_js).await;

                if let Some(output_path) = save_result.as_string() {
                    let result: JsValue = match edit_mode_val {
                        EditMode::Resize => {
                            let args = ResizeArgs {
                                input_path: input_path_val,
                                output_path,
                                width: resize_width_val,
                                height: resize_height_val,
                                maintain_aspect: maintain_aspect_val,
                            };
                            let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                            invoke("resize_image_cmd", args_js).await
                        }
                        EditMode::Rotate => {
                            let args = RotateArgs {
                                input_path: input_path_val,
                                output_path,
                                angle: rotation_angle_val,
                            };
                            let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                            invoke("rotate_image_cmd", args_js).await
                        }
                        EditMode::Crop => {
                            let args = CropArgs {
                                input_path: input_path_val,
                                output_path,
                                x: crop_x_val,
                                y: crop_y_val,
                                width: crop_width_val,
                                height: crop_height_val,
                            };
                            let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                            invoke("crop_image_cmd", args_js).await
                        }
                        EditMode::Brightness => {
                            let args = BrightnessArgs {
                                input_path: input_path_val,
                                output_path,
                                value: brightness_val,
                            };
                            let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                            invoke("adjust_brightness_cmd", args_js).await
                        }
                        EditMode::Contrast => {
                            let args = ContrastArgs {
                                input_path: input_path_val,
                                output_path,
                                value: contrast_val,
                            };
                            let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                            invoke("adjust_contrast_cmd", args_js).await
                        }
                        EditMode::Filter => {
                            let args = FilterArgs {
                                input_path: input_path_val,
                                output_path,
                                filter: selected_filter_val,
                            };
                            let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                            invoke("apply_filter_cmd", args_js).await
                        }
                    };

                    if let Ok(res) = serde_wasm_bindgen::from_value::<EditResult>(result) {
                        edit_result.set(Some(res));
                    }
                }

                is_processing.set(false);
            });
        })
    };

    let on_flip_horizontal = {
        let input_path = input_path.clone();
        let edit_result = edit_result.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let input_path_val = (*input_path).clone();
            if input_path_val.is_empty() {
                return;
            }

            let edit_result = edit_result.clone();
            let is_processing = is_processing.clone();

            is_processing.set(true);

            spawn_local(async move {
                let save_options = SaveDialogOptions {
                    filters: vec![FileFilter {
                        name: "PNG Image".to_string(),
                        extensions: vec!["png".to_string()],
                    }],
                    default_path: Some("flipped.png".to_string()),
                };
                let save_options_js = serde_wasm_bindgen::to_value(&save_options).unwrap();
                let save_result = save(save_options_js).await;

                if let Some(output_path) = save_result.as_string() {
                    let args = FlipArgs {
                        input_path: input_path_val,
                        output_path,
                    };
                    let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                    let result = invoke("flip_horizontal_cmd", args_js).await;

                    if let Ok(res) = serde_wasm_bindgen::from_value::<EditResult>(result) {
                        edit_result.set(Some(res));
                    }
                }

                is_processing.set(false);
            });
        })
    };

    let on_flip_vertical = {
        let input_path = input_path.clone();
        let edit_result = edit_result.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let input_path_val = (*input_path).clone();
            if input_path_val.is_empty() {
                return;
            }

            let edit_result = edit_result.clone();
            let is_processing = is_processing.clone();

            is_processing.set(true);

            spawn_local(async move {
                let save_options = SaveDialogOptions {
                    filters: vec![FileFilter {
                        name: "PNG Image".to_string(),
                        extensions: vec!["png".to_string()],
                    }],
                    default_path: Some("flipped.png".to_string()),
                };
                let save_options_js = serde_wasm_bindgen::to_value(&save_options).unwrap();
                let save_result = save(save_options_js).await;

                if let Some(output_path) = save_result.as_string() {
                    let args = FlipArgs {
                        input_path: input_path_val,
                        output_path,
                    };
                    let args_js = serde_wasm_bindgen::to_value(&args).unwrap();
                    let result = invoke("flip_vertical_cmd", args_js).await;

                    if let Ok(res) = serde_wasm_bindgen::from_value::<EditResult>(result) {
                        edit_result.set(Some(res));
                    }
                }

                is_processing.set(false);
            });
        })
    };

    let on_mode_change = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |mode: EditMode| {
            edit_mode.set(mode);
        })
    };

    html! {
        <div class="image-editor">
            // Loading Overlay
            {if *is_processing {
                html! {
                    <div class="processing-overlay">
                        <div class="processing-content">
                            <div class="processing-spinner"></div>
                            <p class="processing-title">{"Processing..."}</p>
                            <p class="processing-hint">{"Please wait while your image is being edited"}</p>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

            // File Selection
            <div class="section" onclick={on_select_file.clone()}>
                <div class="drop-zone">
                    <div class="drop-zone-icon">{"🎨"}</div>
                    <p class="drop-zone-text">{"Click or drag & drop an image"}</p>
                    <p class="drop-zone-hint">{"PNG, JPEG, WebP, GIF, BMP"}</p>
                </div>
                {if !input_path.is_empty() {
                    html! { <p class="file-path">{&*input_path}</p> }
                } else {
                    html! {}
                }}
            </div>

            // Image Preview
            {if !image_preview_url.is_empty() {
                html! {
                    <div class="section image-preview-section">
                        <h3>{"Preview"}</h3>
                        <div class="image-preview-container">
                            <img
                                src={(*image_preview_url).clone()}
                                alt="Preview"
                                class="image-preview"
                            />
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

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

            // Edit Mode Selection
            <div class="section">
                <h3>{"Edit Mode"}</h3>
                <div class="mode-toggle edit-mode-toggle">
                    {render_mode_button(&edit_mode, EditMode::Resize, "Resize", on_mode_change.clone())}
                    {render_mode_button(&edit_mode, EditMode::Rotate, "Rotate", on_mode_change.clone())}
                    {render_mode_button(&edit_mode, EditMode::Crop, "Crop", on_mode_change.clone())}
                    {render_mode_button(&edit_mode, EditMode::Brightness, "Brightness", on_mode_change.clone())}
                    {render_mode_button(&edit_mode, EditMode::Contrast, "Contrast", on_mode_change.clone())}
                    {render_mode_button(&edit_mode, EditMode::Filter, "Filter", on_mode_change.clone())}
                </div>
            </div>

            // Edit Options based on mode
            {render_edit_options(
                &edit_mode,
                &resize_width,
                &resize_height,
                &maintain_aspect,
                &rotation_angle,
                &crop_x,
                &crop_y,
                &crop_width,
                &crop_height,
                &brightness,
                &contrast,
                &selected_filter,
            )}

            // Quick Actions
            <div class="section">
                <h3>{"Quick Actions"}</h3>
                <div class="quick-actions">
                    <button
                        class="quick-action-btn"
                        onclick={on_flip_horizontal}
                        disabled={input_path.is_empty() || *is_processing}
                    >
                        {"↔ Flip Horizontal"}
                    </button>
                    <button
                        class="quick-action-btn"
                        onclick={on_flip_vertical}
                        disabled={input_path.is_empty() || *is_processing}
                    >
                        {"↕ Flip Vertical"}
                    </button>
                </div>
            </div>

            // Apply Button
            <button
                onclick={on_apply_edit}
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
                    html! { <>{"Apply & Save"}</> }
                }}
            </button>

            // Result
            {if let Some(result) = &*edit_result {
                html! {
                    <div class={if result.success { "section result-box success" } else { "section result-box error" }}>
                        {if result.success {
                            html! {
                                <>
                                    <h3>{"Edit Complete!"}</h3>
                                    <div class="result-stats">
                                        <div class="result-stat">
                                            <div class="result-stat-label">{"Original"}</div>
                                            <div class="result-stat-value original">{format_size(result.original_size)}</div>
                                        </div>
                                        <div class="result-stat">
                                            <div class="result-stat-label">{"New Size"}</div>
                                            <div class="result-stat-value compressed">{format_size(result.new_size)}</div>
                                        </div>
                                        <div class="result-stat">
                                            <div class="result-stat-label">{"Dimensions"}</div>
                                            <div class="result-stat-value">{format!("{}×{}", result.new_width, result.new_height)}</div>
                                        </div>
                                    </div>
                                    <p class="output-path">{format!("📁 {}", result.output_path)}</p>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <h3>{"Edit Failed"}</h3>
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

fn render_mode_button(
    current_mode: &UseStateHandle<EditMode>,
    mode: EditMode,
    label: &str,
    on_click: Callback<EditMode>,
) -> Html {
    let is_active = **current_mode == mode;
    let mode_clone = mode.clone();
    html! {
        <button
            class={if is_active { "mode-btn active" } else { "mode-btn" }}
            onclick={Callback::from(move |_| on_click.emit(mode_clone.clone()))}
        >
            {label}
        </button>
    }
}

#[allow(clippy::too_many_arguments)]
fn render_edit_options(
    edit_mode: &UseStateHandle<EditMode>,
    resize_width: &UseStateHandle<u32>,
    resize_height: &UseStateHandle<u32>,
    maintain_aspect: &UseStateHandle<bool>,
    rotation_angle: &UseStateHandle<RotationAngle>,
    crop_x: &UseStateHandle<u32>,
    crop_y: &UseStateHandle<u32>,
    crop_width: &UseStateHandle<u32>,
    crop_height: &UseStateHandle<u32>,
    brightness: &UseStateHandle<i32>,
    contrast: &UseStateHandle<f32>,
    selected_filter: &UseStateHandle<ImageFilter>,
) -> Html {
    match **edit_mode {
        EditMode::Resize => render_resize_options(resize_width, resize_height, maintain_aspect),
        EditMode::Rotate => render_rotate_options(rotation_angle),
        EditMode::Crop => render_crop_options(crop_x, crop_y, crop_width, crop_height),
        EditMode::Brightness => render_brightness_options(brightness),
        EditMode::Contrast => render_contrast_options(contrast),
        EditMode::Filter => render_filter_options(selected_filter),
    }
}

fn render_resize_options(
    width: &UseStateHandle<u32>,
    height: &UseStateHandle<u32>,
    maintain_aspect: &UseStateHandle<bool>,
) -> Html {
    let on_width_change = {
        let width = width.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(w) = input.value().parse::<u32>() {
                width.set(w);
            }
        })
    };

    let on_height_change = {
        let height = height.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(h) = input.value().parse::<u32>() {
                height.set(h);
            }
        })
    };

    let on_aspect_toggle = {
        let maintain_aspect = maintain_aspect.clone();
        Callback::from(move |_| {
            maintain_aspect.set(!*maintain_aspect);
        })
    };

    html! {
        <div class="section">
            <h3>{"Resize Options"}</h3>
            <div class="resize-inputs">
                <input
                    type="number"
                    value={width.to_string()}
                    oninput={on_width_change}
                    placeholder="Width"
                />
                <span>{"×"}</span>
                <input
                    type="number"
                    value={height.to_string()}
                    oninput={on_height_change}
                    placeholder="Height"
                />
            </div>
            <div class="checkbox-option" onclick={on_aspect_toggle}>
                <input type="checkbox" checked={**maintain_aspect} />
                <label>{"Maintain aspect ratio"}</label>
            </div>
        </div>
    }
}

fn render_rotate_options(rotation_angle: &UseStateHandle<RotationAngle>) -> Html {
    let angles = vec![
        (RotationAngle::Rotate90, "90°"),
        (RotationAngle::Rotate180, "180°"),
        (RotationAngle::Rotate270, "270°"),
    ];

    let on_angle_change = {
        let rotation_angle = rotation_angle.clone();
        Callback::from(move |angle: RotationAngle| {
            rotation_angle.set(angle);
        })
    };

    html! {
        <div class="section">
            <h3>{"Rotation"}</h3>
            <div class="format-options">
                {for angles.iter().map(|(angle, label)| {
                    let is_selected = **rotation_angle == *angle;
                    let angle_value = *angle;
                    let on_click = {
                        let on_angle_change = on_angle_change.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_angle_change.emit(angle_value);
                        })
                    };
                    html! {
                        <div class="format-option" onclick={on_click}>
                            <input
                                type="radio"
                                name="rotation"
                                checked={is_selected}
                            />
                            <label>
                                <span class="format-name">{label}</span>
                            </label>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

fn render_crop_options(
    crop_x: &UseStateHandle<u32>,
    crop_y: &UseStateHandle<u32>,
    crop_width: &UseStateHandle<u32>,
    crop_height: &UseStateHandle<u32>,
) -> Html {
    let on_x_change = {
        let crop_x = crop_x.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<u32>() {
                crop_x.set(v);
            }
        })
    };

    let on_y_change = {
        let crop_y = crop_y.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<u32>() {
                crop_y.set(v);
            }
        })
    };

    let on_width_change = {
        let crop_width = crop_width.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<u32>() {
                crop_width.set(v);
            }
        })
    };

    let on_height_change = {
        let crop_height = crop_height.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<u32>() {
                crop_height.set(v);
            }
        })
    };

    html! {
        <div class="section">
            <h3>{"Crop Area"}</h3>
            <div class="crop-inputs">
                <div class="crop-row">
                    <div class="crop-input-group">
                        <label>{"X"}</label>
                        <input
                            type="number"
                            value={crop_x.to_string()}
                            oninput={on_x_change}
                        />
                    </div>
                    <div class="crop-input-group">
                        <label>{"Y"}</label>
                        <input
                            type="number"
                            value={crop_y.to_string()}
                            oninput={on_y_change}
                        />
                    </div>
                </div>
                <div class="crop-row">
                    <div class="crop-input-group">
                        <label>{"Width"}</label>
                        <input
                            type="number"
                            value={crop_width.to_string()}
                            oninput={on_width_change}
                        />
                    </div>
                    <div class="crop-input-group">
                        <label>{"Height"}</label>
                        <input
                            type="number"
                            value={crop_height.to_string()}
                            oninput={on_height_change}
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_brightness_options(brightness: &UseStateHandle<i32>) -> Html {
    let on_brightness_change = {
        let brightness = brightness.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<i32>() {
                brightness.set(v);
            }
        })
    };

    html! {
        <div class="section">
            <h3>{"Brightness"}</h3>
            <div class="quality-slider">
                <input
                    type="range"
                    min="-100"
                    max="100"
                    value={brightness.to_string()}
                    oninput={on_brightness_change}
                />
                <span class="quality-value">{format!("{}", **brightness)}</span>
            </div>
        </div>
    }
}

fn render_contrast_options(contrast: &UseStateHandle<f32>) -> Html {
    let on_contrast_change = {
        let contrast = contrast.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(v) = input.value().parse::<f32>() {
                let clamped = v / 100.0;
                contrast.set(clamped);
            }
        })
    };

    let display_value = (**contrast * 100.0) as i32;

    html! {
        <div class="section">
            <h3>{"Contrast"}</h3>
            <div class="quality-slider">
                <input
                    type="range"
                    min="-100"
                    max="100"
                    value={display_value.to_string()}
                    oninput={on_contrast_change}
                />
                <span class="quality-value">{format!("{}", display_value)}</span>
            </div>
        </div>
    }
}

fn render_filter_options(selected_filter: &UseStateHandle<ImageFilter>) -> Html {
    let filters = vec![
        (ImageFilter::Grayscale, "Grayscale"),
        (ImageFilter::Sepia, "Sepia"),
        (ImageFilter::Invert, "Invert"),
        (ImageFilter::Blur, "Blur"),
        (ImageFilter::Sharpen, "Sharpen"),
    ];

    let on_filter_change = {
        let selected_filter = selected_filter.clone();
        Callback::from(move |filter: ImageFilter| {
            selected_filter.set(filter);
        })
    };

    html! {
        <div class="section">
            <h3>{"Filter"}</h3>
            <div class="filter-options">
                {for filters.iter().map(|(filter, label)| {
                    let is_selected = **selected_filter == *filter;
                    let filter_value = *filter;
                    let on_click = {
                        let on_filter_change = on_filter_change.clone();
                        Callback::from(move |_: MouseEvent| {
                            on_filter_change.emit(filter_value);
                        })
                    };
                    html! {
                        <div class={if is_selected { "filter-option selected" } else { "filter-option" }} onclick={on_click}>
                            <span class="filter-name">{label}</span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
