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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UuidVersion {
    V4,
    V7,
}

impl UuidVersion {
    #[allow(dead_code)]
    fn translation_key(&self) -> &'static str {
        match self {
            UuidVersion::V4 => "uuid_generator.version_v4",
            UuidVersion::V7 => "uuid_generator.version_v7",
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
    fn translation_key(&self) -> &'static str {
        match self {
            UuidFormat::Standard => "uuid_generator.format_standard",
            UuidFormat::NoHyphens => "uuid_generator.format_no_hyphens",
            UuidFormat::Uppercase => "uuid_generator.format_uppercase",
            UuidFormat::UppercaseNoHyphens => "uuid_generator.format_uppercase_no_hyphens",
            UuidFormat::Braces => "uuid_generator.format_braces",
            UuidFormat::Urn => "uuid_generator.format_urn",
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
    let (i18n, _) = use_translation();
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
                <h3>{i18n.t("uuid_generator.generate_section")}</h3>

                <div class="uuid-options">
                    <div class="form-group">
                        <label>{i18n.t("uuid_generator.version_label")}</label>
                        <select class="form-select" onchange={on_version_change}>
                            <option value="V4" selected={*selected_version == UuidVersion::V4}>
                                {i18n.t("uuid_generator.version_v4")}
                            </option>
                            <option value="V7" selected={*selected_version == UuidVersion::V7}>
                                {i18n.t("uuid_generator.version_v7")}
                            </option>
                        </select>
                    </div>

                    <div class="form-group">
                        <label>{i18n.t("uuid_generator.format_label")}</label>
                        <select class="form-select" onchange={on_format_change}>
                            <option value="Standard" selected={*selected_format == UuidFormat::Standard}>
                                {i18n.t(UuidFormat::Standard.translation_key())}
                            </option>
                            <option value="NoHyphens" selected={*selected_format == UuidFormat::NoHyphens}>
                                {i18n.t(UuidFormat::NoHyphens.translation_key())}
                            </option>
                            <option value="Uppercase" selected={*selected_format == UuidFormat::Uppercase}>
                                {i18n.t(UuidFormat::Uppercase.translation_key())}
                            </option>
                            <option value="UppercaseNoHyphens" selected={*selected_format == UuidFormat::UppercaseNoHyphens}>
                                {i18n.t(UuidFormat::UppercaseNoHyphens.translation_key())}
                            </option>
                            <option value="Braces" selected={*selected_format == UuidFormat::Braces}>
                                {i18n.t(UuidFormat::Braces.translation_key())}
                            </option>
                            <option value="Urn" selected={*selected_format == UuidFormat::Urn}>
                                {i18n.t(UuidFormat::Urn.translation_key())}
                            </option>
                        </select>
                        <div class="format-example">
                            {i18n.t("common.example")}{": "}{selected_format.example()}
                        </div>
                    </div>

                    <div class="form-group">
                        <label>{i18n.t("uuid_generator.count_label")}</label>
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
                            {i18n.t("common.generating")}
                        </span>
                    } else {
                        {i18n.t("common.generate")}
                    }
                </button>
            </div>

            // Generated UUIDs Section
            if !generated_uuids.is_empty() {
                <div class="section uuid-results-section">
                    <div class="uuid-results-header">
                        <h3>{format!("{} ({})", i18n.t("uuid_generator.results_title"), generated_uuids.len())}</h3>
                        <button
                            class={classes!("secondary-btn", "copy-all-btn", (*copy_all_feedback).then_some("copied"))}
                            onclick={on_copy_all}
                        >
                            if *copy_all_feedback {
                                {format!("✓ {}", i18n.t("common.copied"))}
                            } else {
                                {i18n.t("common.copy_all")}
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
                <h3>{i18n.t("uuid_generator.validate_section")}</h3>
                <div class="validate-input-row">
                    <input
                        type="text"
                        class="form-input"
                        placeholder={i18n.t("uuid_generator.validate_input_placeholder")}
                        value={(*validate_input).clone()}
                        oninput={on_validate_input_change}
                    />
                    <button class="secondary-btn" onclick={on_validate}>
                        {i18n.t("common.validate")}
                    </button>
                </div>

                if let Some(result) = &*validate_result {
                    <div class={classes!("validate-result", result.valid.then_some("valid").or(Some("invalid")))}>
                        if result.valid {
                            <div class="validate-status">{format!("✓ {}", i18n.t("uuid_generator.valid_uuid"))}</div>
                            if let Some(version) = &result.version {
                                <div class="validate-info">
                                    <span class="info-label">{i18n.t("uuid_generator.version_info")}</span>
                                    <span class="info-value">{version}</span>
                                </div>
                            }
                            if let Some(variant) = &result.variant {
                                <div class="validate-info">
                                    <span class="info-label">{i18n.t("uuid_generator.variant_info")}</span>
                                    <span class="info-value">{variant}</span>
                                </div>
                            }
                        } else {
                            <div class="validate-status">{format!("✕ {}", i18n.t("uuid_generator.invalid_uuid"))}</div>
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
