use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UuidVersion {
    V4,
    V7,
}

impl UuidVersion {
    fn label(&self) -> &'static str {
        match self {
            UuidVersion::V4 => "v4 (Random)",
            UuidVersion::V7 => "v7 (Time-based)",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UuidFormat {
    Standard,
    NoHyphens,
    Uppercase,
    UppercaseNoHyphens,
    Braces,
    Urn,
}

impl UuidFormat {
    fn label(&self) -> &'static str {
        match self {
            UuidFormat::Standard => "標準",
            UuidFormat::NoHyphens => "ハイフンなし",
            UuidFormat::Uppercase => "大文字",
            UuidFormat::UppercaseNoHyphens => "大文字+ハイフンなし",
            UuidFormat::Braces => "波括弧付き",
            UuidFormat::Urn => "URN形式",
        }
    }

    fn example(&self) -> &'static str {
        match self {
            UuidFormat::Standard => "550e8400-e29b-41d4-a716-446655440000",
            UuidFormat::NoHyphens => "550e8400e29b41d4a716446655440000",
            UuidFormat::Uppercase => "550E8400-E29B-41D4-A716-446655440000",
            UuidFormat::UppercaseNoHyphens => "550E8400E29B41D4A716446655440000",
            UuidFormat::Braces => "{550e8400-e29b-41d4-a716-446655440000}",
            UuidFormat::Urn => "urn:uuid:550e8400-e29b-41d4-a716-446655440000",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerateUuidsArgs {
    version: UuidVersion,
    format: UuidFormat,
    count: u32,
}

#[derive(Serialize)]
struct ValidateUuidArgs {
    input: String,
}

#[derive(Debug, Clone, Deserialize)]
struct UuidGenerateResult {
    success: bool,
    uuids: Vec<String>,
    #[allow(dead_code)]
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UuidValidateResult {
    valid: bool,
    version: Option<String>,
    variant: Option<String>,
    error: Option<String>,
}

#[derive(Clone, PartialEq)]
struct GeneratedUuid {
    value: String,
    copied: bool,
}

#[function_component(UuidGenerator)]
pub fn uuid_generator() -> Html {
    let selected_version = use_state(|| UuidVersion::V4);
    let selected_format = use_state(|| UuidFormat::Standard);
    let count = use_state(|| 1u32);
    let generated_uuids = use_state(Vec::<GeneratedUuid>::new);
    let is_generating = use_state(|| false);
    let validate_input = use_state(String::new);
    let validate_result = use_state(|| Option::<UuidValidateResult>::None);
    let copy_all_feedback = use_state(|| false);

    let on_version_change = {
        let selected_version = selected_version.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let version = match select.value().as_str() {
                "V7" => UuidVersion::V7,
                _ => UuidVersion::V4,
            };
            selected_version.set(version);
        })
    };

    let on_format_change = {
        let selected_format = selected_format.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let format = match select.value().as_str() {
                "NoHyphens" => UuidFormat::NoHyphens,
                "Uppercase" => UuidFormat::Uppercase,
                "UppercaseNoHyphens" => UuidFormat::UppercaseNoHyphens,
                "Braces" => UuidFormat::Braces,
                "Urn" => UuidFormat::Urn,
                _ => UuidFormat::Standard,
            };
            selected_format.set(format);
        })
    };

    let on_count_change = {
        let count = count.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u32>() {
                count.set(value.min(100).max(1));
            }
        })
    };

    let on_generate = {
        let selected_version = selected_version.clone();
        let selected_format = selected_format.clone();
        let count = count.clone();
        let generated_uuids = generated_uuids.clone();
        let is_generating = is_generating.clone();

        Callback::from(move |_| {
            let version = (*selected_version).clone();
            let format = (*selected_format).clone();
            let count_value = *count;
            let generated_uuids = generated_uuids.clone();
            let is_generating = is_generating.clone();

            is_generating.set(true);

            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&GenerateUuidsArgs {
                    version,
                    format,
                    count: count_value,
                })
                .unwrap();

                let result = invoke("generate_uuids_cmd", args).await;

                if let Ok(res) = serde_wasm_bindgen::from_value::<UuidGenerateResult>(result) {
                    if res.success {
                        let new_uuids: Vec<GeneratedUuid> = res
                            .uuids
                            .into_iter()
                            .map(|value| GeneratedUuid {
                                value,
                                copied: false,
                            })
                            .collect();
                        generated_uuids.set(new_uuids);
                    }
                }

                is_generating.set(false);
            });
        })
    };

    let on_copy_single = {
        let generated_uuids = generated_uuids.clone();
        Callback::from(move |index: usize| {
            let generated_uuids = generated_uuids.clone();
            if let Some(uuid) = (*generated_uuids).get(index) {
                let value = uuid.value.clone();
                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    let generated_uuids_inner = generated_uuids.clone();
                    spawn_local(async move {
                        let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&value))
                            .await;

                        let mut uuids = (*generated_uuids_inner).clone();
                        if let Some(uuid) = uuids.get_mut(index) {
                            uuid.copied = true;
                        }
                        generated_uuids_inner.set(uuids);

                        // Reset copied state after 2 seconds
                        let generated_uuids_reset = generated_uuids_inner.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            let mut uuids = (*generated_uuids_reset).clone();
                            if let Some(uuid) = uuids.get_mut(index) {
                                uuid.copied = false;
                            }
                            generated_uuids_reset.set(uuids);
                        })
                        .forget();
                    });
                }
            }
        })
    };

    let on_copy_all = {
        let generated_uuids = generated_uuids.clone();
        let copy_all_feedback = copy_all_feedback.clone();
        Callback::from(move |_| {
            let uuids = (*generated_uuids).clone();
            let copy_all_feedback = copy_all_feedback.clone();
            if !uuids.is_empty() {
                let all_values: String = uuids
                    .iter()
                    .map(|u| u.value.clone())
                    .collect::<Vec<_>>()
                    .join("\n");

                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    spawn_local(async move {
                        let _ =
                            wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&all_values))
                                .await;
                        copy_all_feedback.set(true);

                        let copy_all_feedback_reset = copy_all_feedback.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            copy_all_feedback_reset.set(false);
                        })
                        .forget();
                    });
                }
            }
        })
    };

    let on_validate_input_change = {
        let validate_input = validate_input.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            validate_input.set(input.value());
        })
    };

    let on_validate = {
        let validate_input = validate_input.clone();
        let validate_result = validate_result.clone();
        Callback::from(move |_| {
            let input = (*validate_input).clone();
            let validate_result = validate_result.clone();

            if input.trim().is_empty() {
                validate_result.set(None);
                return;
            }

            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&ValidateUuidArgs { input }).unwrap();
                let result = invoke("validate_uuid_cmd", args).await;

                if let Ok(res) = serde_wasm_bindgen::from_value::<UuidValidateResult>(result) {
                    validate_result.set(Some(res));
                }
            });
        })
    };

    html! {
        <div class="uuid-generator">
            // Generate Section
            <div class="section uuid-generate-section">
                <h3>{"UUID 生成"}</h3>

                <div class="uuid-options">
                    <div class="form-group">
                        <label>{"バージョン"}</label>
                        <select class="form-select" onchange={on_version_change}>
                            <option value="V4" selected={*selected_version == UuidVersion::V4}>
                                {UuidVersion::V4.label()}
                            </option>
                            <option value="V7" selected={*selected_version == UuidVersion::V7}>
                                {UuidVersion::V7.label()}
                            </option>
                        </select>
                    </div>

                    <div class="form-group">
                        <label>{"形式"}</label>
                        <select class="form-select" onchange={on_format_change}>
                            <option value="Standard" selected={*selected_format == UuidFormat::Standard}>
                                {UuidFormat::Standard.label()}
                            </option>
                            <option value="NoHyphens" selected={*selected_format == UuidFormat::NoHyphens}>
                                {UuidFormat::NoHyphens.label()}
                            </option>
                            <option value="Uppercase" selected={*selected_format == UuidFormat::Uppercase}>
                                {UuidFormat::Uppercase.label()}
                            </option>
                            <option value="UppercaseNoHyphens" selected={*selected_format == UuidFormat::UppercaseNoHyphens}>
                                {UuidFormat::UppercaseNoHyphens.label()}
                            </option>
                            <option value="Braces" selected={*selected_format == UuidFormat::Braces}>
                                {UuidFormat::Braces.label()}
                            </option>
                            <option value="Urn" selected={*selected_format == UuidFormat::Urn}>
                                {UuidFormat::Urn.label()}
                            </option>
                        </select>
                        <div class="format-example">
                            {"例: "}{selected_format.example()}
                        </div>
                    </div>

                    <div class="form-group">
                        <label>{"生成個数"}</label>
                        <input
                            type="number"
                            class="form-input"
                            min="1"
                            max="100"
                            value={count.to_string()}
                            oninput={on_count_change}
                        />
                    </div>
                </div>

                <button
                    class="primary-btn generate-btn"
                    onclick={on_generate}
                    disabled={*is_generating}
                >
                    if *is_generating {
                        <span class="processing">
                            <span class="spinner"></span>
                            {"生成中..."}
                        </span>
                    } else {
                        {"UUID を生成"}
                    }
                </button>
            </div>

            // Generated UUIDs Section
            if !generated_uuids.is_empty() {
                <div class="section uuid-results-section">
                    <div class="uuid-results-header">
                        <h3>{format!("生成結果 ({} 件)", generated_uuids.len())}</h3>
                        <button
                            class={classes!("secondary-btn", "copy-all-btn", (*copy_all_feedback).then_some("copied"))}
                            onclick={on_copy_all}
                        >
                            if *copy_all_feedback {
                                {"✓ コピー完了"}
                            } else {
                                {"すべてコピー"}
                            }
                        </button>
                    </div>
                    <div class="uuid-list">
                        { for (*generated_uuids).iter().enumerate().map(|(index, uuid)| {
                            let on_copy = {
                                let on_copy_single = on_copy_single.clone();
                                Callback::from(move |_| on_copy_single.emit(index))
                            };
                            html! {
                                <div class="uuid-item">
                                    <code class="uuid-value">{&uuid.value}</code>
                                    <button
                                        class={classes!("copy-btn", uuid.copied.then_some("copied"))}
                                        onclick={on_copy}
                                    >
                                        if uuid.copied {
                                            {"✓"}
                                        } else {
                                            {"📋"}
                                        }
                                    </button>
                                </div>
                            }
                        })}
                    </div>
                </div>
            }

            // Validate Section
            <div class="section uuid-validate-section">
                <h3>{"UUID 検証"}</h3>
                <div class="validate-input-row">
                    <input
                        type="text"
                        class="form-input"
                        placeholder="検証するUUIDを入力..."
                        value={(*validate_input).clone()}
                        oninput={on_validate_input_change}
                    />
                    <button class="secondary-btn" onclick={on_validate}>
                        {"検証"}
                    </button>
                </div>

                if let Some(result) = &*validate_result {
                    <div class={classes!("validate-result", result.valid.then_some("valid").or(Some("invalid")))}>
                        if result.valid {
                            <div class="validate-status">{"✓ 有効なUUID"}</div>
                            if let Some(version) = &result.version {
                                <div class="validate-info">
                                    <span class="info-label">{"バージョン:"}</span>
                                    <span class="info-value">{version}</span>
                                </div>
                            }
                            if let Some(variant) = &result.variant {
                                <div class="validate-info">
                                    <span class="info-label">{"バリアント:"}</span>
                                    <span class="info-value">{variant}</span>
                                </div>
                            }
                        } else {
                            <div class="validate-status">{"✕ 無効なUUID"}</div>
                            if let Some(error) = &result.error {
                                <div class="validate-error">{error}</div>
                            }
                        }
                    </div>
                }
            </div>
        </div>
    }
}
