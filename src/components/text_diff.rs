use gloo_timers::callback::Timeout;
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DiffMode {
    Line,
    Word,
    Character,
}

impl DiffMode {
    fn label(&self) -> &'static str {
        match self {
            DiffMode::Line => "Line",
            DiffMode::Word => "Word",
            DiffMode::Character => "Character",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    SideBySide,
    Unified,
    DiffOnly,
}

impl ViewMode {
    fn label(&self) -> &'static str {
        match self {
            ViewMode::SideBySide => "Side by Side",
            ViewMode::Unified => "Unified",
            ViewMode::DiffOnly => "Diff Only",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            ViewMode::SideBySide => "||",
            ViewMode::Unified => "=",
            ViewMode::DiffOnly => "+-",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InlineChange {
    tag: String,
    value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LineDiff {
    line_number_old: Option<usize>,
    line_number_new: Option<usize>,
    tag: String,
    content: String,
    #[allow(dead_code)]
    inline_changes: Vec<InlineChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiffStats {
    additions: usize,
    deletions: usize,
    #[allow(dead_code)]
    modifications: usize,
    unchanged: usize,
    total_lines_old: usize,
    total_lines_new: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiffResult {
    success: bool,
    lines: Vec<LineDiff>,
    stats: DiffStats,
    unified_diff: String,
    error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileInfo {
    path: String,
    name: String,
    size: u64,
    content: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ComputeDiffArgs {
    old_text: String,
    new_text: String,
    mode: DiffMode,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetFileInfoArgs {
    path: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub dropped_file: Option<String>,
    #[prop_or_default]
    pub on_file_processed: Callback<()>,
}

#[function_component(TextDiffComponent)]
pub fn text_diff(props: &Props) -> Html {
    let old_text = use_state(String::new);
    let new_text = use_state(String::new);
    let old_file_name = use_state(|| Option::<String>::None);
    let new_file_name = use_state(|| Option::<String>::None);
    let diff_result = use_state(|| Option::<DiffResult>::None);
    let is_comparing = use_state(|| false);
    let diff_mode = use_state(|| DiffMode::Line);
    let view_mode = use_state(|| ViewMode::SideBySide);
    let copied = use_state(|| false);
    let error_message = use_state(|| Option::<String>::None);

    // Handle dropped file
    {
        let dropped_file = props.dropped_file.clone();
        let old_text = old_text.clone();
        let old_file_name = old_file_name.clone();
        let new_text = new_text.clone();
        let new_file_name = new_file_name.clone();
        let on_file_processed = props.on_file_processed.clone();
        let error_message = error_message.clone();

        use_effect_with(dropped_file, move |dropped_file| {
            if let Some(path) = dropped_file.clone() {
                let old_text = old_text.clone();
                let old_file_name = old_file_name.clone();
                let new_text = new_text.clone();
                let new_file_name = new_file_name.clone();
                let on_file_processed = on_file_processed.clone();
                let error_message = error_message.clone();

                spawn_local(async move {
                    let args = serde_wasm_bindgen::to_value(&GetFileInfoArgs { path }).unwrap();
                    let result = invoke("get_text_file_info_cmd", args).await;

                    if let Ok(file_info) = serde_wasm_bindgen::from_value::<FileInfo>(result) {
                        if (*old_text).is_empty() {
                            old_text.set(file_info.content);
                            old_file_name.set(Some(file_info.name));
                        } else if (*new_text).is_empty() {
                            new_text.set(file_info.content);
                            new_file_name.set(Some(file_info.name));
                        } else {
                            old_text.set((*new_text).clone());
                            old_file_name.set((*new_file_name).clone());
                            new_text.set(file_info.content);
                            new_file_name.set(Some(file_info.name));
                        }
                        error_message.set(None);
                    } else {
                        error_message.set(Some("Failed to load file".to_string()));
                    }
                    on_file_processed.emit(());
                });
            }
            || {}
        });
    }

    let on_compare = {
        let old_text = old_text.clone();
        let new_text = new_text.clone();
        let diff_result = diff_result.clone();
        let is_comparing = is_comparing.clone();
        let diff_mode = diff_mode.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            if (*old_text).is_empty() && (*new_text).is_empty() {
                return;
            }

            let old_text_val = (*old_text).clone();
            let new_text_val = (*new_text).clone();
            let diff_result = diff_result.clone();
            let is_comparing = is_comparing.clone();
            let mode = (*diff_mode).clone();
            let error_message = error_message.clone();

            is_comparing.set(true);

            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&ComputeDiffArgs {
                    old_text: old_text_val,
                    new_text: new_text_val,
                    mode,
                })
                .unwrap();

                let result = invoke("compute_diff_cmd", args).await;

                if let Ok(res) = serde_wasm_bindgen::from_value::<DiffResult>(result) {
                    if res.success {
                        diff_result.set(Some(res));
                        error_message.set(None);
                    } else {
                        error_message.set(res.error);
                    }
                } else {
                    error_message.set(Some("Failed to compute diff".to_string()));
                }
                is_comparing.set(false);
            });
        })
    };

    let on_old_text_change = {
        let old_text = old_text.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            old_text.set(textarea.value());
        })
    };

    let on_new_text_change = {
        let new_text = new_text.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            new_text.set(textarea.value());
        })
    };

    let on_diff_mode_change = {
        let diff_mode = diff_mode.clone();
        Callback::from(move |mode: DiffMode| {
            diff_mode.set(mode);
        })
    };

    let on_view_mode_change = {
        let view_mode = view_mode.clone();
        Callback::from(move |mode: ViewMode| {
            view_mode.set(mode);
        })
    };

    let on_copy_unified = {
        let diff_result = diff_result.clone();
        let copied = copied.clone();

        Callback::from(move |_| {
            if let Some(ref result) = *diff_result {
                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    let text = result.unified_diff.clone();
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
        let old_text = old_text.clone();
        let new_text = new_text.clone();
        let old_file_name = old_file_name.clone();
        let new_file_name = new_file_name.clone();
        let diff_result = diff_result.clone();
        let error_message = error_message.clone();

        Callback::from(move |_| {
            old_text.set(String::new());
            new_text.set(String::new());
            old_file_name.set(None);
            new_file_name.set(None);
            diff_result.set(None);
            error_message.set(None);
        })
    };

    let on_swap = {
        let old_text = old_text.clone();
        let new_text = new_text.clone();
        let old_file_name = old_file_name.clone();
        let new_file_name = new_file_name.clone();

        Callback::from(move |_| {
            let temp_text = (*old_text).clone();
            let temp_name = (*old_file_name).clone();
            old_text.set((*new_text).clone());
            old_file_name.set((*new_file_name).clone());
            new_text.set(temp_text);
            new_file_name.set(temp_name);
        })
    };

    html! {
        <div class="text-diff-container">
            <div class="section diff-header">
                <h3>{"// DIFF OPTIONS"}</h3>
                <div class="diff-controls">
                    <div class="control-group">
                        <label class="control-label">{"Diff Mode"}</label>
                        <div class="mode-buttons">
                            {
                                [DiffMode::Line, DiffMode::Word, DiffMode::Character].iter().map(|mode| {
                                    let is_active = *diff_mode == *mode;
                                    let on_click = on_diff_mode_change.clone();
                                    let m = mode.clone();
                                    html! {
                                        <button
                                            class={classes!("mode-btn", is_active.then_some("active"))}
                                            onclick={Callback::from(move |_| on_click.emit(m.clone()))}
                                        >
                                            {mode.label()}
                                        </button>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                    <div class="control-group">
                        <label class="control-label">{"View Mode"}</label>
                        <div class="mode-buttons">
                            {
                                [ViewMode::SideBySide, ViewMode::Unified, ViewMode::DiffOnly].iter().map(|mode| {
                                    let is_active = *view_mode == *mode;
                                    let on_click = on_view_mode_change.clone();
                                    let m = mode.clone();
                                    html! {
                                        <button
                                            class={classes!("mode-btn", is_active.then_some("active"))}
                                            onclick={Callback::from(move |_| on_click.emit(m.clone()))}
                                        >
                                            <span class="mode-icon">{mode.icon()}</span>
                                            <span>{mode.label()}</span>
                                        </button>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    </div>
                </div>
            </div>

            <div class="section input-section">
                <div class="input-panels">
                    <div class="input-panel">
                        <div class="panel-header">
                            <span class="panel-title">
                                {"Original"}
                                if let Some(ref name) = *old_file_name {
                                    <span class="file-name">{format!(" ({})", name)}</span>
                                }
                            </span>
                            <span class="line-count">{format!("{} lines", (*old_text).lines().count())}</span>
                        </div>
                        <textarea
                            class="diff-textarea"
                            placeholder="Paste original text here or drop a file..."
                            value={(*old_text).clone()}
                            oninput={on_old_text_change}
                        />
                    </div>
                    <div class="swap-button-container">
                        <button class="swap-btn" onclick={on_swap} title="Swap texts">
                            {"<->"}
                        </button>
                    </div>
                    <div class="input-panel">
                        <div class="panel-header">
                            <span class="panel-title">
                                {"Modified"}
                                if let Some(ref name) = *new_file_name {
                                    <span class="file-name">{format!(" ({})", name)}</span>
                                }
                            </span>
                            <span class="line-count">{format!("{} lines", (*new_text).lines().count())}</span>
                        </div>
                        <textarea
                            class="diff-textarea"
                            placeholder="Paste modified text here or drop a file..."
                            value={(*new_text).clone()}
                            oninput={on_new_text_change}
                        />
                    </div>
                </div>
                <div class="action-buttons">
                    <button
                        class="primary-btn"
                        onclick={on_compare.clone()}
                        disabled={*is_comparing || ((*old_text).is_empty() && (*new_text).is_empty())}
                    >
                        if *is_comparing {
                            <span class="spinner"></span>
                            <span>{"Comparing..."}</span>
                        } else {
                            <span>{"Compare"}</span>
                        }
                    </button>
                    <button class="secondary-btn" onclick={on_clear}>
                        {"Clear All"}
                    </button>
                </div>
            </div>

            if let Some(ref error) = *error_message {
                <div class="section error-section">
                    <p class="error-message">{error}</p>
                </div>
            }

            if let Some(ref result) = *diff_result {
                <div class="section stats-section">
                    <h3>{"// DIFF STATISTICS"}</h3>
                    <div class="stats-grid">
                        <div class="stat-item additions">
                            <span class="stat-value">{format!("+{}", result.stats.additions)}</span>
                            <span class="stat-label">{"Additions"}</span>
                        </div>
                        <div class="stat-item deletions">
                            <span class="stat-value">{format!("-{}", result.stats.deletions)}</span>
                            <span class="stat-label">{"Deletions"}</span>
                        </div>
                        <div class="stat-item unchanged">
                            <span class="stat-value">{result.stats.unchanged}</span>
                            <span class="stat-label">{"Unchanged"}</span>
                        </div>
                        <div class="stat-item total">
                            <span class="stat-value">{format!("{} -> {}", result.stats.total_lines_old, result.stats.total_lines_new)}</span>
                            <span class="stat-label">{"Lines"}</span>
                        </div>
                    </div>
                </div>

                <div class="section result-section">
                    <div class="result-header">
                        <h3>{"// DIFF RESULT"}</h3>
                        <button
                            class={classes!("copy-btn", (*copied).then_some("copied"))}
                            onclick={on_copy_unified}
                        >
                            if *copied {
                                {"Copied!"}
                            } else {
                                {"Copy Unified Diff"}
                            }
                        </button>
                    </div>

                    {
                        match *view_mode {
                            ViewMode::SideBySide => render_side_by_side(&result.lines),
                            ViewMode::Unified => render_unified(&result.lines),
                            ViewMode::DiffOnly => render_diff_only(&result.lines),
                        }
                    }
                </div>
            }
        </div>
    }
}

fn render_side_by_side(lines: &[LineDiff]) -> Html {
    let mut old_lines: Vec<&LineDiff> = Vec::new();
    let mut new_lines: Vec<&LineDiff> = Vec::new();

    for line in lines {
        match line.tag.as_str() {
            "delete" => old_lines.push(line),
            "insert" => new_lines.push(line),
            "equal" => {
                old_lines.push(line);
                new_lines.push(line);
            }
            _ => {}
        }
    }

    let max_len = old_lines.len().max(new_lines.len());

    html! {
        <div class="diff-view side-by-side">
            <div class="diff-panel old-panel">
                <div class="diff-panel-header">{"Original"}</div>
                <div class="diff-lines">
                    { for (0..max_len).map(|i| {
                        if let Some(line) = old_lines.get(i) {
                            let class = match line.tag.as_str() {
                                "delete" => "diff-line delete",
                                _ => "diff-line equal",
                            };
                            html! {
                                <div class={class}>
                                    <span class="line-number">
                                        {line.line_number_old.map(|n| n.to_string()).unwrap_or_default()}
                                    </span>
                                    <span class="line-content">{&line.content}</span>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="diff-line empty">
                                    <span class="line-number"></span>
                                    <span class="line-content"></span>
                                </div>
                            }
                        }
                    })}
                </div>
            </div>
            <div class="diff-panel new-panel">
                <div class="diff-panel-header">{"Modified"}</div>
                <div class="diff-lines">
                    { for (0..max_len).map(|i| {
                        if let Some(line) = new_lines.get(i) {
                            let class = match line.tag.as_str() {
                                "insert" => "diff-line insert",
                                _ => "diff-line equal",
                            };
                            html! {
                                <div class={class}>
                                    <span class="line-number">
                                        {line.line_number_new.map(|n| n.to_string()).unwrap_or_default()}
                                    </span>
                                    <span class="line-content">{&line.content}</span>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="diff-line empty">
                                    <span class="line-number"></span>
                                    <span class="line-content"></span>
                                </div>
                            }
                        }
                    })}
                </div>
            </div>
        </div>
    }
}

fn render_unified(lines: &[LineDiff]) -> Html {
    html! {
        <div class="diff-view unified">
            <div class="diff-lines">
                { for lines.iter().map(|line| {
                    let (class, prefix) = match line.tag.as_str() {
                        "delete" => ("diff-line delete", "-"),
                        "insert" => ("diff-line insert", "+"),
                        _ => ("diff-line equal", " "),
                    };
                    html! {
                        <div class={class}>
                            <span class="line-number old">
                                {line.line_number_old.map(|n| n.to_string()).unwrap_or_default()}
                            </span>
                            <span class="line-number new">
                                {line.line_number_new.map(|n| n.to_string()).unwrap_or_default()}
                            </span>
                            <span class="line-prefix">{prefix}</span>
                            <span class="line-content">{&line.content}</span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}

fn render_diff_only(lines: &[LineDiff]) -> Html {
    let diff_lines: Vec<&LineDiff> = lines.iter().filter(|l| l.tag != "equal").collect();

    if diff_lines.is_empty() {
        return html! {
            <div class="diff-view diff-only">
                <div class="no-diff-message">
                    {"No differences found"}
                </div>
            </div>
        };
    }

    html! {
        <div class="diff-view diff-only">
            <div class="diff-lines">
                { for diff_lines.iter().map(|line| {
                    let (class, prefix) = match line.tag.as_str() {
                        "delete" => ("diff-line delete", "-"),
                        "insert" => ("diff-line insert", "+"),
                        _ => ("diff-line equal", " "),
                    };
                    html! {
                        <div class={class}>
                            <span class="line-number old">
                                {line.line_number_old.map(|n| n.to_string()).unwrap_or_default()}
                            </span>
                            <span class="line-number new">
                                {line.line_number_new.map(|n| n.to_string()).unwrap_or_default()}
                            </span>
                            <span class="line-prefix">{prefix}</span>
                            <span class="line-content">{&line.content}</span>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
