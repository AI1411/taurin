use gloo_timers::callback::Timeout;
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegexFlags {
    pub global: bool,
    pub case_insensitive: bool,
    pub multiline: bool,
    pub dot_all: bool,
}

impl Default for RegexFlags {
    fn default() -> Self {
        Self {
            global: true,
            case_insensitive: false,
            multiline: false,
            dot_all: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub full_match: String,
    pub start: usize,
    pub end: usize,
    pub groups: Vec<GroupInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfo {
    pub index: usize,
    pub name: Option<String>,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegexResult {
    pub success: bool,
    pub matches: Vec<MatchInfo>,
    pub match_count: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceResult {
    pub success: bool,
    pub result: String,
    pub replacements: usize,
    pub error: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TestRegexArgs {
    pattern: String,
    test_text: String,
    flags: RegexFlags,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReplaceRegexArgs {
    pattern: String,
    test_text: String,
    replacement: String,
    flags: RegexFlags,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PresetCategory {
    Common,
    Validation,
    Web,
    DateTime,
}

impl PresetCategory {
    fn translation_key(&self) -> &'static str {
        match self {
            PresetCategory::Common => "regex_tester.category_common",
            PresetCategory::Validation => "regex_tester.category_validation",
            PresetCategory::Web => "regex_tester.category_web",
            PresetCategory::DateTime => "regex_tester.category_datetime",
        }
    }
}

#[derive(Debug, Clone)]
struct RegexPreset {
    name: &'static str,
    pattern: &'static str,
    description: &'static str,
    category: PresetCategory,
}

fn get_presets() -> Vec<RegexPreset> {
    vec![
        // Common
        RegexPreset {
            name: "Email",
            pattern: r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
            description: "Match email addresses",
            category: PresetCategory::Validation,
        },
        RegexPreset {
            name: "Phone (JP)",
            pattern: r"0\d{1,4}-?\d{1,4}-?\d{4}",
            description: "Match Japanese phone numbers",
            category: PresetCategory::Validation,
        },
        RegexPreset {
            name: "URL",
            pattern: r"https?://[\w\-._~:/?#\[\]@!$&'()*+,;=%]+",
            description: "Match HTTP/HTTPS URLs",
            category: PresetCategory::Web,
        },
        RegexPreset {
            name: "IPv4",
            pattern: r"\b(?:(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\.){3}(?:25[0-5]|2[0-4]\d|[01]?\d\d?)\b",
            description: "Match IPv4 addresses",
            category: PresetCategory::Web,
        },
        RegexPreset {
            name: "Date (YYYY-MM-DD)",
            pattern: r"\d{4}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12]\d|3[01])",
            description: "Match ISO date format",
            category: PresetCategory::DateTime,
        },
        RegexPreset {
            name: "Time (HH:MM:SS)",
            pattern: r"(?:[01]\d|2[0-3]):[0-5]\d(?::[0-5]\d)?",
            description: "Match 24-hour time format",
            category: PresetCategory::DateTime,
        },
        RegexPreset {
            name: "Postal Code (JP)",
            pattern: r"\d{3}-?\d{4}",
            description: "Match Japanese postal codes",
            category: PresetCategory::Validation,
        },
        RegexPreset {
            name: "Credit Card",
            pattern: r"\b(?:\d{4}[- ]?){3}\d{4}\b",
            description: "Match credit card numbers",
            category: PresetCategory::Validation,
        },
        RegexPreset {
            name: "Hex Color",
            pattern: r"#(?:[0-9a-fA-F]{3}){1,2}\b",
            description: "Match hex color codes",
            category: PresetCategory::Common,
        },
        RegexPreset {
            name: "HTML Tag",
            pattern: r"<([a-zA-Z][a-zA-Z0-9]*)\b[^>]*>.*?</\1>|<[a-zA-Z][a-zA-Z0-9]*\b[^/>]*/>",
            description: "Match HTML tags",
            category: PresetCategory::Web,
        },
        RegexPreset {
            name: "Whitespace",
            pattern: r"\s+",
            description: "Match whitespace characters",
            category: PresetCategory::Common,
        },
        RegexPreset {
            name: "Numbers",
            pattern: r"-?\d+\.?\d*",
            description: "Match integers and decimals",
            category: PresetCategory::Common,
        },
        RegexPreset {
            name: "Word Boundary",
            pattern: r"\b\w+\b",
            description: "Match whole words",
            category: PresetCategory::Common,
        },
    ]
}

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(RegexTester)]
pub fn regex_tester(_props: &Props) -> Html {
    let (i18n, _) = use_translation();
    let pattern = use_state(String::new);
    let test_text = use_state(String::new);
    let replacement = use_state(String::new);
    let flags = use_state(RegexFlags::default);
    let result = use_state(|| Option::<RegexResult>::None);
    let replace_result = use_state(|| Option::<ReplaceResult>::None);
    let is_testing = use_state(|| false);
    let copied = use_state(|| false);
    let error_message = use_state(|| Option::<String>::None);
    let selected_category = use_state(|| Option::<PresetCategory>::None);
    let show_presets = use_state(|| false);
    let show_replace = use_state(|| false);

    let presets = get_presets();

    // Auto-test on pattern or test_text change with debounce
    {
        let pattern = pattern.clone();
        let test_text = test_text.clone();
        let flags = flags.clone();
        let result = result.clone();
        let error_message = error_message.clone();
        let is_testing = is_testing.clone();

        use_effect_with(
            ((*pattern).clone(), (*test_text).clone(), *flags),
            move |(pattern_val, test_text_val, flags_val)| {
                let pattern_val = pattern_val.clone();
                let test_text_val = test_text_val.clone();
                let flags_val = *flags_val;
                let result = result.clone();
                let error_message = error_message.clone();
                let is_testing = is_testing.clone();

                if pattern_val.is_empty() {
                    result.set(None);
                    error_message.set(None);
                } else {
                    let result_clone = result.clone();
                    let error_message_clone = error_message.clone();
                    let is_testing_clone = is_testing.clone();

                    let timeout = Timeout::new(300, move || {
                        if pattern_val.is_empty() {
                            return;
                        }

                        is_testing_clone.set(true);

                        let result_inner = result_clone.clone();
                        let error_message_inner = error_message_clone.clone();
                        let is_testing_inner = is_testing_clone.clone();

                        spawn_local(async move {
                            let args = match serde_wasm_bindgen::to_value(&TestRegexArgs {
                                pattern: pattern_val,
                                test_text: test_text_val,
                                flags: flags_val,
                            }) {
                                Ok(a) => a,
                                Err(e) => {
                                    error_message_inner
                                        .set(Some(format!("Failed to serialize: {}", e)));
                                    result_inner.set(None);
                                    is_testing_inner.set(false);
                                    return;
                                }
                            };

                            let res = invoke("test_regex_cmd", args).await;

                            match serde_wasm_bindgen::from_value::<RegexResult>(res) {
                                Ok(regex_result) => {
                                    if regex_result.success {
                                        result_inner.set(Some(regex_result));
                                        error_message_inner.set(None);
                                    } else {
                                        error_message_inner.set(regex_result.error);
                                        result_inner.set(None);
                                    }
                                }
                                Err(e) => {
                                    error_message_inner
                                        .set(Some(format!("Failed to parse result: {}", e)));
                                    result_inner.set(None);
                                }
                            }
                            is_testing_inner.set(false);
                        });
                    });

                    timeout.forget();
                }

                || {}
            },
        );
    }

    let on_pattern_change = {
        let pattern = pattern.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            pattern.set(input.value());
        })
    };

    let on_test_text_change = {
        let test_text = test_text.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            test_text.set(textarea.value());
        })
    };

    let on_replacement_change = {
        let replacement = replacement.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            replacement.set(input.value());
        })
    };

    let toggle_flag = {
        let flags = flags.clone();
        move |flag_name: &'static str| {
            let flags = flags.clone();
            Callback::from(move |_| {
                let mut new_flags = *flags;
                match flag_name {
                    "g" => new_flags.global = !new_flags.global,
                    "i" => new_flags.case_insensitive = !new_flags.case_insensitive,
                    "m" => new_flags.multiline = !new_flags.multiline,
                    "s" => new_flags.dot_all = !new_flags.dot_all,
                    _ => {}
                }
                flags.set(new_flags);
            })
        }
    };

    let on_preset_select = {
        let pattern = pattern.clone();
        let show_presets = show_presets.clone();
        Callback::from(move |preset_pattern: String| {
            pattern.set(preset_pattern);
            show_presets.set(false);
        })
    };

    let on_category_select = {
        let selected_category = selected_category.clone();
        Callback::from(move |cat: Option<PresetCategory>| {
            selected_category.set(cat);
        })
    };

    let toggle_presets = {
        let show_presets = show_presets.clone();
        Callback::from(move |_| {
            show_presets.set(!*show_presets);
        })
    };

    let toggle_replace = {
        let show_replace = show_replace.clone();
        Callback::from(move |_| {
            show_replace.set(!*show_replace);
        })
    };

    let on_replace = {
        let pattern = pattern.clone();
        let test_text = test_text.clone();
        let replacement = replacement.clone();
        let flags = flags.clone();
        let replace_result = replace_result.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            let pattern_val = (*pattern).clone();
            let test_text_val = (*test_text).clone();
            let replacement_val = (*replacement).clone();
            let flags_val = *flags;
            let replace_result = replace_result.clone();
            let error_message = error_message.clone();

            if pattern_val.is_empty() {
                return;
            }

            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&ReplaceRegexArgs {
                    pattern: pattern_val,
                    test_text: test_text_val,
                    replacement: replacement_val,
                    flags: flags_val,
                })
                .unwrap();

                let res = invoke("replace_regex_cmd", args).await;

                if let Ok(result) = serde_wasm_bindgen::from_value::<ReplaceResult>(res) {
                    if result.success {
                        replace_result.set(Some(result));
                        error_message.set(None);
                    } else {
                        error_message.set(result.error);
                    }
                } else {
                    error_message.set(Some("Failed to replace".to_string()));
                }
            });
        })
    };

    let on_copy_result = {
        let replace_result = replace_result.clone();
        let copied = copied.clone();

        Callback::from(move |_| {
            if let Some(ref res) = *replace_result {
                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    let text = res.result.clone();
                    let copied = copied.clone();

                    spawn_local(async move {
                        let _ =
                            wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&text)).await;
                        copied.set(true);

                        Timeout::new(2000, move || {
                            copied.set(false);
                        })
                        .forget();
                    });
                }
            }
        })
    };

    let on_clear = {
        let pattern = pattern.clone();
        let test_text = test_text.clone();
        let replacement = replacement.clone();
        let result = result.clone();
        let replace_result = replace_result.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            pattern.set(String::new());
            test_text.set(String::new());
            replacement.set(String::new());
            result.set(None);
            replace_result.set(None);
            error_message.set(None);
        })
    };

    let on_export = {
        let result = result.clone();
        let pattern = pattern.clone();
        let flags = flags.clone();

        Callback::from(move |_| {
            if let Some(ref res) = *result {
                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    let flags_str = format!(
                        "{}{}{}{}",
                        if flags.global { "g" } else { "" },
                        if flags.case_insensitive { "i" } else { "" },
                        if flags.multiline { "m" } else { "" },
                        if flags.dot_all { "s" } else { "" },
                    );
                    let export_text = format!(
                        "Pattern: /{}/{}\nMatches: {}\n\n{}",
                        *pattern,
                        flags_str,
                        res.match_count,
                        res.matches
                            .iter()
                            .enumerate()
                            .map(|(i, m)| {
                                let groups = if m.groups.is_empty() {
                                    String::new()
                                } else {
                                    format!(
                                        "\n  Groups: {}",
                                        m.groups
                                            .iter()
                                            .map(|g| {
                                                if let Some(ref name) = g.name {
                                                    format!("{}={:?}", name, g.value)
                                                } else {
                                                    format!("${}={:?}", g.index, g.value)
                                                }
                                            })
                                            .collect::<Vec<_>>()
                                            .join(", ")
                                    )
                                };
                                format!(
                                    "Match #{}: {:?} ({}..{}){}",
                                    i + 1,
                                    m.full_match,
                                    m.start,
                                    m.end,
                                    groups
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n")
                    );

                    spawn_local(async move {
                        let _ = wasm_bindgen_futures::JsFuture::from(
                            clipboard.write_text(&export_text),
                        )
                        .await;
                    });
                }
            }
        })
    };

    // Render highlighted text
    let render_highlighted_text = |text: &str, matches: &[MatchInfo]| -> Html {
        if matches.is_empty() {
            return html! { <span class="no-match">{text}</span> };
        }

        let mut parts: Vec<Html> = Vec::new();
        let mut last_end = 0;

        for (i, m) in matches.iter().enumerate() {
            // Add non-matched text before this match
            if m.start > last_end {
                parts.push(html! {
                    <span class="no-match">{&text[last_end..m.start]}</span>
                });
            }

            // Add matched text with highlight
            parts.push(html! {
                <span class={format!("match-highlight match-{}", i % 4)} title={format!("Match #{}", i + 1)}>
                    {&text[m.start..m.end]}
                </span>
            });

            last_end = m.end;
        }

        // Add remaining text
        if last_end < text.len() {
            parts.push(html! {
                <span class="no-match">{&text[last_end..]}</span>
            });
        }

        html! { <>{parts}</> }
    };

    let filtered_presets: Vec<&RegexPreset> = if let Some(ref cat) = *selected_category {
        presets.iter().filter(|p| &p.category == cat).collect()
    } else {
        presets.iter().collect()
    };

    html! {
        <div class="regex-tester-container">
            <div class="section regex-header">
                <h3>{i18n.t("regex_tester.options_title")}</h3>
                <div class="regex-controls">
                    <div class="pattern-input-group">
                        <span class="pattern-prefix">{"/"}</span>
                        <input
                            type="text"
                            class="pattern-input"
                            placeholder={i18n.t("regex_tester.pattern_placeholder")}
                            value={(*pattern).clone()}
                            oninput={on_pattern_change}
                        />
                        <span class="pattern-suffix">{"/"}</span>
                    </div>
                    <div class="flags-group">
                        <button
                            class={classes!("flag-btn", flags.global.then_some("active"))}
                            onclick={toggle_flag("g")}
                            title={i18n.t("regex_tester.flags_global")}
                        >
                            {"g"}
                        </button>
                        <button
                            class={classes!("flag-btn", flags.case_insensitive.then_some("active"))}
                            onclick={toggle_flag("i")}
                            title={i18n.t("regex_tester.flags_case_insensitive")}
                        >
                            {"i"}
                        </button>
                        <button
                            class={classes!("flag-btn", flags.multiline.then_some("active"))}
                            onclick={toggle_flag("m")}
                            title={i18n.t("regex_tester.flags_multiline")}
                        >
                            {"m"}
                        </button>
                        <button
                            class={classes!("flag-btn", flags.dot_all.then_some("active"))}
                            onclick={toggle_flag("s")}
                            title={i18n.t("regex_tester.flags_dot_all")}
                        >
                            {"s"}
                        </button>
                    </div>
                </div>
                <div class="preset-toggle-row">
                    <button class="preset-toggle-btn" onclick={toggle_presets}>
                        if *show_presets {
                            {format!("{} ▲", i18n.t("regex_tester.hide_presets"))}
                        } else {
                            {format!("{} ▼", i18n.t("regex_tester.show_presets"))}
                        }
                    </button>
                    <button class="replace-toggle-btn" onclick={toggle_replace}>
                        if *show_replace {
                            {format!("{} ▲", i18n.t("regex_tester.hide_replace"))}
                        } else {
                            {format!("{} ▼", i18n.t("regex_tester.show_replace"))}
                        }
                    </button>
                </div>
            </div>

            if *show_presets {
                <div class="section presets-section">
                    <h3>{i18n.t("regex_tester.presets_title")}</h3>
                    <div class="preset-categories">
                        <button
                            class={classes!("category-btn", selected_category.is_none().then_some("active"))}
                            onclick={
                                let on_category_select = on_category_select.clone();
                                Callback::from(move |_| on_category_select.emit(None))
                            }
                        >
                            {i18n.t("regex_tester.category_all")}
                        </button>
                        {
                            [PresetCategory::Common, PresetCategory::Validation, PresetCategory::Web, PresetCategory::DateTime]
                                .iter()
                                .map(|cat| {
                                    let is_active = selected_category.as_ref() == Some(cat);
                                    let cat_clone = cat.clone();
                                    let on_click = on_category_select.clone();
                                    let label = i18n.t(cat.translation_key());
                                    html! {
                                        <button
                                            class={classes!("category-btn", is_active.then_some("active"))}
                                            onclick={Callback::from(move |_| on_click.emit(Some(cat_clone.clone())))}
                                        >
                                            {label}
                                        </button>
                                    }
                                })
                                .collect::<Html>()
                        }
                    </div>
                    <div class="presets-grid">
                        {
                            filtered_presets.iter().map(|preset| {
                                let pattern = preset.pattern.to_string();
                                let on_select = on_preset_select.clone();
                                html! {
                                    <button
                                        class="preset-btn"
                                        onclick={Callback::from(move |_| on_select.emit(pattern.clone()))}
                                        title={preset.description}
                                    >
                                        <span class="preset-name">{preset.name}</span>
                                        <span class="preset-pattern">{preset.pattern}</span>
                                    </button>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            }

            <div class="section input-section">
                <h3>{i18n.t("regex_tester.test_string_title")}</h3>
                <textarea
                    class="test-textarea"
                    placeholder={i18n.t("regex_tester.test_placeholder")}
                    value={(*test_text).clone()}
                    oninput={on_test_text_change}
                />
                <div class="action-buttons">
                    <button class="secondary-btn" onclick={on_clear}>
                        {i18n.t("common.clear_all")}
                    </button>
                    if result.is_some() {
                        <button class="secondary-btn" onclick={on_export}>
                            {i18n.t("regex_tester.export_results")}
                        </button>
                    }
                </div>
            </div>

            if let Some(ref error) = *error_message {
                <div class="section error-section">
                    <p class="error-message">{error}</p>
                </div>
            }

            if *show_replace {
                <div class="section replace-section">
                    <h3>{i18n.t("regex_tester.replace_title")}</h3>
                    <div class="replace-input-group">
                        <input
                            type="text"
                            class="replace-input"
                            placeholder={i18n.t("regex_tester.replace_placeholder")}
                            value={(*replacement).clone()}
                            oninput={on_replacement_change}
                        />
                        <button class="primary-btn" onclick={on_replace}>
                            {i18n.t("common.replace")}
                        </button>
                    </div>
                    if let Some(ref res) = *replace_result {
                        <div class="replace-result">
                            <div class="replace-header">
                                <span class="replace-count">{format!("{} {}", res.replacements, i18n.t("common.replace"))}</span>
                                <button
                                    class={classes!("copy-btn", (*copied).then_some("copied"))}
                                    onclick={on_copy_result}
                                >
                                    if *copied {
                                        {i18n.t("common.copied")}
                                    } else {
                                        {i18n.t("regex_tester.copy_result")}
                                    }
                                </button>
                            </div>
                            <pre class="replace-output">{&res.result}</pre>
                        </div>
                    }
                </div>
            }

            if *is_testing {
                <div class="section loading-section">
                    <span class="spinner"></span>
                    <span>{i18n.t("regex_tester.testing")}</span>
                </div>
            }

            if let Some(ref res) = *result {
                <div class="section stats-section">
                    <h3>{i18n.t("regex_tester.stats_title")}</h3>
                    <div class="stats-grid">
                        <div class="stat-item">
                            <span class="stat-value">{res.match_count}</span>
                            <span class="stat-label">{i18n.t("regex_tester.matches")}</span>
                        </div>
                        <div class="stat-item">
                            <span class="stat-value">{res.matches.iter().map(|m| m.groups.len()).sum::<usize>()}</span>
                            <span class="stat-label">{i18n.t("regex_tester.groups")}</span>
                        </div>
                    </div>
                </div>

                <div class="section result-section">
                    <h3>{i18n.t("regex_tester.highlighted_title")}</h3>
                    <div class="highlighted-text">
                        {render_highlighted_text(&test_text, &res.matches)}
                    </div>
                </div>

                if !res.matches.is_empty() {
                    <div class="section matches-section">
                        <h3>{i18n.t("regex_tester.details_title")}</h3>
                        <div class="matches-list">
                            {
                                res.matches.iter().enumerate().map(|(i, m)| {
                                    html! {
                                        <div class="match-item">
                                            <div class="match-header">
                                                <span class={format!("match-badge match-{}", i % 4)}>
                                                    {format!("#{}", i + 1)}
                                                </span>
                                                <span class="match-position">
                                                    {format!("{}: {}..{}", i18n.t("regex_tester.position"), m.start, m.end)}
                                                </span>
                                            </div>
                                            <div class="match-content">
                                                <code class="match-value">{&m.full_match}</code>
                                            </div>
                                            if !m.groups.is_empty() {
                                                <div class="match-groups">
                                                    <span class="groups-label">{i18n.t("regex_tester.capture_groups")}</span>
                                                    <div class="groups-list">
                                                        {
                                                            m.groups.iter().map(|g| {
                                                                let label = if let Some(ref name) = g.name {
                                                                    name.clone()
                                                                } else {
                                                                    format!("${}", g.index)
                                                                };
                                                                html! {
                                                                    <div class="group-item">
                                                                        <span class="group-label">{label}</span>
                                                                        <code class="group-value">{&g.value}</code>
                                                                    </div>
                                                                }
                                                            }).collect::<Html>()
                                                        }
                                                    </div>
                                                </div>
                                            }
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                }
            }
        </div>
    }
}
