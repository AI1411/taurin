use i18nrs::yew::use_translation;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use crate::components::input_history::{save_history, InputHistoryPanel};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Deserialize, Default)]
struct CharCountResult {
    char_count: usize,
    char_count_no_spaces: usize,
    word_count: usize,
    line_count: usize,
    paragraph_count: usize,
    byte_count_utf8: usize,
    byte_count_sjis: usize,
    alphanumeric_count: usize,
    hiragana_count: usize,
    katakana_count: usize,
    kanji_count: usize,
    fullwidth_count: usize,
    halfwidth_count: usize,
}

#[derive(Clone, PartialEq)]
enum CountMode {
    WithSpaces,
    WithoutSpaces,
}

#[derive(serde::Serialize)]
struct CountCharsArgs {
    text: String,
}

#[function_component(CharCounter)]
pub fn char_counter() -> Html {
    let (i18n, _) = use_translation();
    let input = use_state(String::new);
    let count_result = use_state(CharCountResult::default);
    let count_mode = use_state(|| CountMode::WithSpaces);
    let copied = use_state(|| false);
    let history_refresh = use_state(|| 0u32);

    let on_input_change = {
        let input = input.clone();
        let count_result = count_result.clone();
        let history_refresh = history_refresh.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            let value = textarea.value();
            input.set(value.clone());

            let count_result = count_result.clone();
            let history_refresh = history_refresh.clone();
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&CountCharsArgs {
                    text: value.clone(),
                })
                .unwrap_or(JsValue::NULL);
                let result = invoke("count_chars_cmd", args).await;
                if let Ok(res) = serde_wasm_bindgen::from_value::<CharCountResult>(result) {
                    count_result.set(res);
                    if !value.is_empty() {
                        save_history("char_counter", serde_json::json!({"input": value}), None);
                        history_refresh.set(*history_refresh + 1);
                    }
                }
            });
        })
    };

    let on_mode_change = {
        let count_mode = count_mode.clone();
        Callback::from(move |mode: CountMode| {
            count_mode.set(mode);
        })
    };

    let on_clear = {
        let input = input.clone();
        let count_result = count_result.clone();
        Callback::from(move |_| {
            input.set(String::new());
            count_result.set(CharCountResult::default());
        })
    };

    let on_copy_stats = {
        let count_result = count_result.clone();
        let copied = copied.clone();
        let i18n = i18n.clone();
        Callback::from(move |_| {
            let res = (*count_result).clone();
            let copied = copied.clone();
            let i18n = i18n.clone();
            if let Some(win) = window() {
                let clipboard = win.navigator().clipboard();
                let stats = format!(
                    "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\nUTF-8: {} bytes\nShift_JIS: {} bytes\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
                    i18n.t("char_counter.char_count"),
                    res.char_count,
                    i18n.t("char_counter.char_count_no_spaces"),
                    res.char_count_no_spaces,
                    i18n.t("char_counter.word_count"),
                    res.word_count,
                    i18n.t("char_counter.line_count"),
                    res.line_count,
                    i18n.t("char_counter.paragraph_count"),
                    res.paragraph_count,
                    res.byte_count_utf8,
                    res.byte_count_sjis,
                    i18n.t("char_counter.alphanumeric"),
                    res.alphanumeric_count,
                    i18n.t("char_counter.hiragana"),
                    res.hiragana_count,
                    i18n.t("char_counter.katakana"),
                    res.katakana_count,
                    i18n.t("char_counter.kanji"),
                    res.kanji_count,
                );
                spawn_local(async move {
                    let _ =
                        wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&stats)).await;
                    copied.set(true);
                    let copied_reset = copied.clone();
                    gloo_timers::callback::Timeout::new(2000, move || {
                        copied_reset.set(false);
                    })
                    .forget();
                });
            }
        })
    };

    let on_restore = {
        let input = input.clone();
        let count_result = count_result.clone();
        Callback::from(move |inputs: serde_json::Value| {
            if let Some(val) = inputs.get("input").and_then(|v| v.as_str()) {
                let value = val.to_string();
                input.set(value.clone());
                let count_result = count_result.clone();
                spawn_local(async move {
                    let args = serde_wasm_bindgen::to_value(&CountCharsArgs { text: value })
                        .unwrap_or(JsValue::NULL);
                    let result = invoke("count_chars_cmd", args).await;
                    if let Ok(res) = serde_wasm_bindgen::from_value::<CharCountResult>(result) {
                        count_result.set(res);
                    }
                });
            }
        })
    };

    let res = &*count_result;
    let display_char_count = match *count_mode {
        CountMode::WithSpaces => res.char_count,
        CountMode::WithoutSpaces => res.char_count_no_spaces,
    };

    let on_mode_with_spaces = {
        let on_mode_change = on_mode_change.clone();
        Callback::from(move |_| on_mode_change.emit(CountMode::WithSpaces))
    };

    let on_mode_without_spaces = {
        let on_mode_change = on_mode_change.clone();
        Callback::from(move |_| on_mode_change.emit(CountMode::WithoutSpaces))
    };

    html! {
        <div class="char-counter">
            <div class="section char-counter-main">
                <div class="char-counter-header">
                    <h3>{i18n.t("char_counter.title")}</h3>
                    <div class="char-counter-actions">
                        <InputHistoryPanel
                            tool_id="char_counter"
                            on_restore={on_restore}
                            refresh_trigger={*history_refresh}
                        />
                        <button
                            class={classes!("secondary-btn", (*copied).then_some("copied"))}
                            onclick={on_copy_stats}
                        >
                            if *copied {
                                {format!("✓ {}", i18n.t("common.copied"))}
                            } else {
                                {i18n.t("char_counter.copy_stats")}
                            }
                        </button>
                        <button class="secondary-btn" onclick={on_clear}>
                            {i18n.t("common.clear")}
                        </button>
                    </div>
                </div>

                <div class="char-counter-display">
                    <div class="char-count-main">
                        <span class="char-count-number">{display_char_count}</span>
                        <span class="char-count-label">{i18n.t("common.characters")}</span>
                    </div>
                    <div class="char-count-mode-toggle">
                        <button
                            class={classes!("mode-btn", (*count_mode == CountMode::WithSpaces).then_some("active"))}
                            onclick={on_mode_with_spaces}
                        >
                            {i18n.t("char_counter.with_spaces")}
                        </button>
                        <button
                            class={classes!("mode-btn", (*count_mode == CountMode::WithoutSpaces).then_some("active"))}
                            onclick={on_mode_without_spaces}
                        >
                            {i18n.t("char_counter.without_spaces")}
                        </button>
                    </div>
                </div>

                <textarea
                    class="char-counter-input"
                    placeholder={i18n.t("char_counter.placeholder")}
                    value={(*input).clone()}
                    oninput={on_input_change}
                    rows="12"
                />
            </div>

            <div class="section char-counter-stats">
                <h3>{i18n.t("char_counter.basic_stats")}</h3>
                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="stat-value">{res.char_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.char_count")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.char_count_no_spaces}</div>
                        <div class="stat-label">{i18n.t("char_counter.char_count_no_spaces")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.word_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.word_count")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.line_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.line_count")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.paragraph_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.paragraph_count")}</div>
                    </div>
                </div>
            </div>

            <div class="section char-counter-bytes">
                <h3>{i18n.t("char_counter.byte_count")}</h3>
                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="stat-value">{res.byte_count_utf8}</div>
                        <div class="stat-label">{"UTF-8"}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.byte_count_sjis}</div>
                        <div class="stat-label">{"Shift_JIS"}</div>
                    </div>
                </div>
            </div>

            <div class="section char-counter-char-types">
                <h3>{i18n.t("char_counter.char_types")}</h3>
                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="stat-value">{res.alphanumeric_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.alphanumeric")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.hiragana_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.hiragana")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.katakana_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.katakana")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.kanji_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.kanji")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.fullwidth_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.fullwidth")}</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">{res.halfwidth_count}</div>
                        <div class="stat-label">{i18n.t("char_counter.halfwidth")}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}
