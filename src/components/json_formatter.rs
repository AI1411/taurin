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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonFormatResult {
    pub success: bool,
    pub formatted: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonValidateResult {
    pub valid: bool,
    pub error: Option<String>,
    pub error_position: Option<ErrorPosition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonTreeNode {
    pub key: String,
    pub value_type: JsonValueType,
    pub value: Option<String>,
    pub path: String,
    pub children: Vec<JsonTreeNode>,
    pub expanded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum JsonValueType {
    Object,
    Array,
    String,
    Number,
    Boolean,
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonParseResult {
    pub success: bool,
    pub tree: Option<JsonTreeNode>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonMinifyResult {
    pub success: bool,
    pub minified: String,
    pub original_size: usize,
    pub minified_size: usize,
    pub savings_percent: f64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSearchResult {
    pub success: bool,
    pub matches: Vec<JsonSearchMatch>,
    pub total_count: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSearchMatch {
    pub path: String,
    pub key: String,
    pub value: String,
    pub value_type: JsonValueType,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct FormatJsonArgs {
    input: String,
    indent_size: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ValidateJsonArgs {
    input: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MinifyJsonArgs {
    input: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ParseJsonArgs {
    input: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchJsonArgs {
    input: String,
    query: String,
    search_keys: bool,
    search_values: bool,
}

#[derive(Clone, PartialEq)]
enum ViewMode {
    Text,
    Tree,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub dropped_file: Option<String>,
    #[prop_or_default]
    pub on_file_processed: Callback<()>,
}

#[function_component(JsonFormatter)]
pub fn json_formatter(props: &Props) -> Html {
    let input = use_state(String::new);
    let output = use_state(String::new);
    let indent_size = use_state(|| 2usize);
    let validation_result = use_state(|| Option::<JsonValidateResult>::None);
    let tree_data = use_state(|| Option::<JsonTreeNode>::None);
    let search_query = use_state(String::new);
    let search_results = use_state(|| Option::<JsonSearchResult>::None);
    let search_keys = use_state(|| true);
    let search_values = use_state(|| true);
    let view_mode = use_state(|| ViewMode::Text);
    let is_processing = use_state(|| false);
    let copied = use_state(|| false);
    let collapsed_paths = use_state(|| std::collections::HashSet::<String>::new());

    // Handle dropped file
    {
        let dropped_file = props.dropped_file.clone();
        let input = input.clone();
        let on_file_processed = props.on_file_processed.clone();

        use_effect_with(dropped_file, move |dropped_file| {
            if let Some(path) = dropped_file {
                let path = path.clone();
                let input = input.clone();
                let on_file_processed = on_file_processed.clone();

                spawn_local(async move {
                    if let Some(win) = window() {
                        if let Ok(fs) = js_sys::Reflect::get(&win, &JsValue::from_str("__TAURI__"))
                        {
                            if let Ok(fs) = js_sys::Reflect::get(&fs, &JsValue::from_str("fs")) {
                                if let Ok(read_text_file) =
                                    js_sys::Reflect::get(&fs, &JsValue::from_str("readTextFile"))
                                {
                                    if let Ok(func) = read_text_file.dyn_into::<js_sys::Function>()
                                    {
                                        if let Ok(promise) =
                                            func.call1(&JsValue::NULL, &JsValue::from_str(&path))
                                        {
                                            if let Ok(promise) =
                                                promise.dyn_into::<js_sys::Promise>()
                                            {
                                                if let Ok(result) =
                                                    wasm_bindgen_futures::JsFuture::from(promise)
                                                        .await
                                                {
                                                    if let Some(content) = result.as_string() {
                                                        input.set(content);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    on_file_processed.emit(());
                });
            }
            || {}
        });
    }

    // Auto-validate on input change with debounce
    {
        let input_val = (*input).clone();
        let validation_result = validation_result.clone();
        let tree_data = tree_data.clone();
        let output = output.clone();
        let indent_size = *indent_size;

        use_effect_with(input_val.clone(), move |input_val| {
            let input_val = input_val.clone();
            let validation_result = validation_result.clone();
            let tree_data = tree_data.clone();
            let output = output.clone();

            if input_val.is_empty() {
                validation_result.set(None);
                tree_data.set(None);
                output.set(String::new());
            } else {
                let timeout = Timeout::new(300, move || {
                    let input_val = input_val.clone();
                    let validation_result = validation_result.clone();
                    let tree_data = tree_data.clone();
                    let output = output.clone();

                    spawn_local(async move {
                        // Validate
                        let args = serde_wasm_bindgen::to_value(&ValidateJsonArgs {
                            input: input_val.clone(),
                        })
                        .unwrap();
                        let res = invoke("validate_json_cmd", args).await;
                        if let Ok(result) = serde_wasm_bindgen::from_value::<JsonValidateResult>(res) {
                            validation_result.set(Some(result.clone()));

                            if result.valid {
                                // Format
                                let args = serde_wasm_bindgen::to_value(&FormatJsonArgs {
                                    input: input_val.clone(),
                                    indent_size,
                                })
                                .unwrap();
                                let res = invoke("format_json_cmd", args).await;
                                if let Ok(format_result) =
                                    serde_wasm_bindgen::from_value::<JsonFormatResult>(res)
                                {
                                    if format_result.success {
                                        output.set(format_result.formatted);
                                    }
                                }

                                // Parse to tree
                                let args =
                                    serde_wasm_bindgen::to_value(&ParseJsonArgs { input: input_val })
                                        .unwrap();
                                let res = invoke("parse_json_to_tree_cmd", args).await;
                                if let Ok(tree_result) =
                                    serde_wasm_bindgen::from_value::<JsonParseResult>(res)
                                {
                                    if tree_result.success {
                                        tree_data.set(tree_result.tree);
                                    }
                                }
                            }
                        }
                    });
                });

                timeout.forget();
            }

            || {}
        });
    }

    let on_input_change = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            input.set(textarea.value());
        })
    };

    let on_format = {
        let input = input.clone();
        let output = output.clone();
        let indent_size = *indent_size;
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let input_val = (*input).clone();
            let output = output.clone();
            let is_processing = is_processing.clone();

            if input_val.is_empty() {
                return;
            }

            is_processing.set(true);

            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&FormatJsonArgs {
                    input: input_val,
                    indent_size,
                })
                .unwrap();
                let res = invoke("format_json_cmd", args).await;
                if let Ok(result) = serde_wasm_bindgen::from_value::<JsonFormatResult>(res) {
                    if result.success {
                        output.set(result.formatted);
                    }
                }
                is_processing.set(false);
            });
        })
    };

    let on_minify = {
        let input = input.clone();
        let output = output.clone();
        let is_processing = is_processing.clone();

        Callback::from(move |_| {
            let input_val = (*input).clone();
            let output = output.clone();
            let is_processing = is_processing.clone();

            if input_val.is_empty() {
                return;
            }

            is_processing.set(true);

            spawn_local(async move {
                let args =
                    serde_wasm_bindgen::to_value(&MinifyJsonArgs { input: input_val }).unwrap();
                let res = invoke("minify_json_cmd", args).await;
                if let Ok(result) = serde_wasm_bindgen::from_value::<JsonMinifyResult>(res) {
                    if result.success {
                        output.set(result.minified);
                    }
                }
                is_processing.set(false);
            });
        })
    };

    let on_copy = {
        let output = output.clone();
        let copied = copied.clone();

        Callback::from(move |_| {
            let text = (*output).clone();
            let copied = copied.clone();

            if text.is_empty() {
                return;
            }

            if let Some(win) = window() {
                let clipboard = win.navigator().clipboard();
                spawn_local(async move {
                    let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&text)).await;
                    copied.set(true);

                    Timeout::new(2000, move || {
                        copied.set(false);
                    })
                    .forget();
                });
            }
        })
    };

    let on_copy_path = {
        let copied = copied.clone();

        Callback::from(move |path: String| {
            let copied = copied.clone();

            if let Some(win) = window() {
                let clipboard = win.navigator().clipboard();
                spawn_local(async move {
                    let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&path)).await;
                    copied.set(true);

                    Timeout::new(2000, move || {
                        copied.set(false);
                    })
                    .forget();
                });
            }
        })
    };

    let on_clear = {
        let input = input.clone();
        let output = output.clone();
        let validation_result = validation_result.clone();
        let tree_data = tree_data.clone();
        let search_query = search_query.clone();
        let search_results = search_results.clone();

        Callback::from(move |_| {
            input.set(String::new());
            output.set(String::new());
            validation_result.set(None);
            tree_data.set(None);
            search_query.set(String::new());
            search_results.set(None);
        })
    };

    let on_indent_change = {
        let indent_size = indent_size.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            if let Ok(val) = select.value().parse::<usize>() {
                indent_size.set(val);
            }
        })
    };

    let on_view_mode_change = {
        let view_mode = view_mode.clone();
        Callback::from(move |mode: ViewMode| {
            view_mode.set(mode);
        })
    };

    let on_search_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input_el: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input_el.value());
        })
    };

    let do_search = {
        let input = input.clone();
        let search_query = search_query.clone();
        let search_results = search_results.clone();
        let search_keys = *search_keys;
        let search_values = *search_values;

        move || {
            let input_val = (*input).clone();
            let query = (*search_query).clone();
            let search_results = search_results.clone();

            if input_val.is_empty() || query.is_empty() {
                return;
            }

            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&SearchJsonArgs {
                    input: input_val,
                    query,
                    search_keys,
                    search_values,
                })
                .unwrap();
                let res = invoke("search_json_cmd", args).await;
                if let Ok(result) = serde_wasm_bindgen::from_value::<JsonSearchResult>(res) {
                    search_results.set(Some(result));
                }
            });
        }
    };

    let on_search_click = {
        let do_search = do_search.clone();
        Callback::from(move |_: MouseEvent| {
            do_search();
        })
    };

    let on_search_keydown = {
        let do_search = do_search.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                do_search();
            }
        })
    };

    let toggle_search_keys = {
        let search_keys = search_keys.clone();
        Callback::from(move |_| {
            search_keys.set(!*search_keys);
        })
    };

    let toggle_search_values = {
        let search_values = search_values.clone();
        Callback::from(move |_| {
            search_values.set(!*search_values);
        })
    };

    let toggle_node = {
        let collapsed_paths = collapsed_paths.clone();
        Callback::from(move |path: String| {
            let mut paths = (*collapsed_paths).clone();
            if paths.contains(&path) {
                paths.remove(&path);
            } else {
                paths.insert(path);
            }
            collapsed_paths.set(paths);
        })
    };

    let render_tree_node = {
        let collapsed_paths = collapsed_paths.clone();
        let on_copy_path = on_copy_path.clone();
        let toggle_node = toggle_node.clone();

        fn render_node(
            node: &JsonTreeNode,
            depth: usize,
            collapsed_paths: &std::collections::HashSet<String>,
            on_copy_path: &Callback<String>,
            toggle_node: &Callback<String>,
        ) -> Html {
            let is_collapsed = collapsed_paths.contains(&node.path);
            let has_children = !node.children.is_empty();
            let indent = depth * 20;

            let value_class = match node.value_type {
                JsonValueType::String => "json-string",
                JsonValueType::Number => "json-number",
                JsonValueType::Boolean => "json-boolean",
                JsonValueType::Null => "json-null",
                JsonValueType::Object => "json-object",
                JsonValueType::Array => "json-array",
            };

            let toggle_cb = {
                let path = node.path.clone();
                let toggle_node = toggle_node.clone();
                Callback::from(move |_| toggle_node.emit(path.clone()))
            };

            let copy_cb = {
                let path = node.path.clone();
                let on_copy_path = on_copy_path.clone();
                Callback::from(move |_| on_copy_path.emit(path.clone()))
            };

            html! {
                <div class="tree-node" style={format!("padding-left: {}px", indent)}>
                    <div class="tree-node-content">
                        if has_children {
                            <button class="tree-toggle" onclick={toggle_cb}>
                                if is_collapsed {
                                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M9 18l6-6-6-6"/>
                                    </svg>
                                } else {
                                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M19 9l-7 7-7-7"/>
                                    </svg>
                                }
                            </button>
                        } else {
                            <span class="tree-toggle-spacer"></span>
                        }
                        <span class="tree-key">{&node.key}</span>
                        <span class="tree-colon">{":"}</span>
                        <span class={classes!("tree-value", value_class)}>
                            {node.value.as_deref().unwrap_or("")}
                        </span>
                        <button class="copy-path-btn" onclick={copy_cb} title="Copy path">
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <rect x="9" y="9" width="13" height="13" rx="2"/>
                                <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                            </svg>
                        </button>
                    </div>
                    if has_children && !is_collapsed {
                        <div class="tree-children">
                            { for node.children.iter().map(|child| {
                                render_node(child, depth + 1, collapsed_paths, on_copy_path, toggle_node)
                            })}
                        </div>
                    }
                </div>
            }
        }

        move |node: &JsonTreeNode| {
            render_node(node, 0, &collapsed_paths, &on_copy_path, &toggle_node)
        }
    };

    html! {
        <div class="json-formatter-container">
            <div class="section json-header">
                <h3>{"// JSON FORMATTER"}</h3>
                <div class="json-controls">
                    <div class="view-toggle">
                        <button
                            class={classes!("view-btn", (*view_mode == ViewMode::Text).then_some("active"))}
                            onclick={
                                let on_view_mode_change = on_view_mode_change.clone();
                                Callback::from(move |_| on_view_mode_change.emit(ViewMode::Text))
                            }
                        >
                            {"Text"}
                        </button>
                        <button
                            class={classes!("view-btn", (*view_mode == ViewMode::Tree).then_some("active"))}
                            onclick={
                                let on_view_mode_change = on_view_mode_change.clone();
                                Callback::from(move |_| on_view_mode_change.emit(ViewMode::Tree))
                            }
                        >
                            {"Tree"}
                        </button>
                    </div>
                    <div class="indent-selector">
                        <label>{"Indent:"}</label>
                        <select onchange={on_indent_change} value={indent_size.to_string()}>
                            <option value="2" selected={*indent_size == 2}>{"2 spaces"}</option>
                            <option value="4" selected={*indent_size == 4}>{"4 spaces"}</option>
                            <option value="8" selected={*indent_size == 8}>{"Tab (8)"}</option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="section validation-status">
                if let Some(ref result) = *validation_result {
                    if result.valid {
                        <div class="status-badge valid">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M20 6L9 17l-5-5"/>
                            </svg>
                            {"Valid JSON"}
                        </div>
                    } else {
                        <div class="status-badge invalid">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <circle cx="12" cy="12" r="10"/>
                                <line x1="15" y1="9" x2="9" y2="15"/>
                                <line x1="9" y1="9" x2="15" y2="15"/>
                            </svg>
                            {"Invalid JSON"}
                            if let Some(ref pos) = result.error_position {
                                <span class="error-position">
                                    {format!(" (Line {}, Column {})", pos.line, pos.column)}
                                </span>
                            }
                        </div>
                    }
                }
            </div>

            <div class="section search-section">
                <div class="search-input-group">
                    <input
                        type="text"
                        class="search-input"
                        placeholder="Search in JSON..."
                        value={(*search_query).clone()}
                        oninput={on_search_change}
                        onkeydown={on_search_keydown}
                    />
                    <button class="search-btn" onclick={on_search_click}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="11" cy="11" r="8"/>
                            <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                        </svg>
                    </button>
                </div>
                <div class="search-options">
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            checked={*search_keys}
                            onchange={toggle_search_keys}
                        />
                        {"Keys"}
                    </label>
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            checked={*search_values}
                            onchange={toggle_search_values}
                        />
                        {"Values"}
                    </label>
                </div>
                if let Some(ref results) = *search_results {
                    <div class="search-results">
                        <span class="results-count">{format!("{} matches found", results.total_count)}</span>
                        if !results.matches.is_empty() {
                            <div class="matches-list">
                                { for results.matches.iter().take(20).map(|m| {
                                    let path = m.path.clone();
                                    let on_copy_path = on_copy_path.clone();
                                    html! {
                                        <div class="match-item" onclick={Callback::from(move |_| on_copy_path.emit(path.clone()))}>
                                            <span class="match-path">{&m.path}</span>
                                            <span class="match-value">{&m.value}</span>
                                        </div>
                                    }
                                })}
                                if results.total_count > 20 {
                                    <div class="more-results">
                                        {format!("... and {} more", results.total_count - 20)}
                                    </div>
                                }
                            </div>
                        }
                    </div>
                }
            </div>

            <div class="section input-output-section">
                <div class="panel input-panel">
                    <div class="panel-header">
                        <h4>{"Input"}</h4>
                        <div class="panel-actions">
                            <button class="secondary-btn" onclick={on_clear}>{"Clear"}</button>
                        </div>
                    </div>
                    <textarea
                        class="json-textarea"
                        placeholder="Paste your JSON here..."
                        value={(*input).clone()}
                        oninput={on_input_change}
                    />
                </div>

                <div class="panel output-panel">
                    <div class="panel-header">
                        <h4>{"Output"}</h4>
                        <div class="panel-actions">
                            <button class="primary-btn" onclick={on_format}>{"Format"}</button>
                            <button class="secondary-btn" onclick={on_minify}>{"Minify"}</button>
                            <button
                                class={classes!("secondary-btn", (*copied).then_some("copied"))}
                                onclick={on_copy}
                            >
                                if *copied {
                                    {"Copied!"}
                                } else {
                                    {"Copy"}
                                }
                            </button>
                        </div>
                    </div>
                    if *view_mode == ViewMode::Text {
                        <pre class="json-output">{&*output}</pre>
                    } else {
                        <div class="tree-view">
                            if let Some(ref tree) = *tree_data {
                                {render_tree_node(tree)}
                            } else {
                                <div class="tree-placeholder">
                                    {"Enter valid JSON to see the tree view"}
                                </div>
                            }
                        </div>
                    }
                </div>
            </div>

            if *is_processing {
                <div class="processing-overlay">
                    <span class="spinner"></span>
                    <span>{"Processing..."}</span>
                </div>
            }

            if let Some(ref result) = *validation_result {
                if !result.valid {
                    if let Some(ref error) = result.error {
                        <div class="section error-section">
                            <h3>{"// ERROR DETAILS"}</h3>
                            <p class="error-message">{error}</p>
                        </div>
                    }
                }
            }
        </div>
    }
}
