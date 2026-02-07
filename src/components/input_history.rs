use i18nrs::yew::use_translation;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub id: String,
    pub tool_id: String,
    pub inputs: serde_json::Value,
    pub label: Option<String>,
    pub created_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolHistorySettings {
    pub enabled: bool,
    pub max_entries: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddHistoryEntryArgs {
    tool_id: String,
    inputs: serde_json::Value,
    label: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetToolHistoryArgs {
    tool_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchToolHistoryArgs {
    tool_id: String,
    query: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteHistoryEntryArgs {
    entry_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClearToolHistoryArgs {
    tool_id: String,
}

pub fn save_history(tool_id: &str, inputs: serde_json::Value, label: Option<String>) {
    let tool_id = tool_id.to_string();
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&AddHistoryEntryArgs {
            tool_id,
            inputs,
            label,
        })
        .unwrap_or(JsValue::NULL);
        let _ = invoke("add_history_entry_cmd", args).await;
    });
}

fn format_preview(inputs: &serde_json::Value) -> String {
    if let Some(obj) = inputs.as_object() {
        let parts: Vec<String> = obj
            .iter()
            .take(3)
            .map(|(k, v)| {
                let val_str = match v {
                    serde_json::Value::String(s) => {
                        if s.len() > 40 {
                            format!("{}...", &s[..40])
                        } else {
                            s.clone()
                        }
                    }
                    other => {
                        let s = other.to_string();
                        if s.len() > 40 {
                            format!("{}...", &s[..40])
                        } else {
                            s
                        }
                    }
                };
                format!("{}: {}", k, val_str)
            })
            .collect();
        parts.join(" | ")
    } else {
        let s = inputs.to_string();
        if s.len() > 80 {
            format!("{}...", &s[..80])
        } else {
            s
        }
    }
}

fn format_time_ago(created_at: &str) -> String {
    // Simple relative time display - just show the date/time portion
    if created_at.len() >= 16 {
        created_at[..16].replace('T', " ").to_string()
    } else {
        created_at.to_string()
    }
}

#[derive(Properties, PartialEq)]
pub struct InputHistoryPanelProps {
    pub tool_id: String,
    pub on_restore: Callback<serde_json::Value>,
    #[prop_or(0)]
    pub refresh_trigger: u32,
}

#[function_component(InputHistoryPanel)]
pub fn input_history_panel(props: &InputHistoryPanelProps) -> Html {
    let (i18n, _) = use_translation();
    let is_open = use_state(|| false);
    let entries = use_state(Vec::<HistoryEntry>::new);
    let search_query = use_state(String::new);

    // Load history when panel opens or refresh_trigger changes
    {
        let tool_id = props.tool_id.clone();
        let entries = entries.clone();
        let is_open = *is_open;
        let refresh_trigger = props.refresh_trigger;

        use_effect_with(
            (is_open, refresh_trigger),
            move |(is_open, _refresh_trigger)| {
                if *is_open {
                    let entries = entries.clone();
                    let tool_id = tool_id.clone();
                    spawn_local(async move {
                        let args =
                            serde_wasm_bindgen::to_value(&GetToolHistoryArgs { tool_id }).unwrap();
                        let result = invoke("get_tool_history_cmd", args).await;
                        if let Ok(res) = serde_wasm_bindgen::from_value::<Vec<HistoryEntry>>(result)
                        {
                            entries.set(res);
                        }
                    });
                }
                || {}
            },
        );
    }

    let on_toggle = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(!*is_open);
        })
    };

    let on_search_change = {
        let search_query = search_query.clone();
        let entries = entries.clone();
        let tool_id = props.tool_id.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let query = input.value();
            search_query.set(query.clone());

            if query.is_empty() {
                let entries = entries.clone();
                let tool_id = tool_id.clone();
                spawn_local(async move {
                    let args =
                        serde_wasm_bindgen::to_value(&GetToolHistoryArgs { tool_id }).unwrap();
                    let result = invoke("get_tool_history_cmd", args).await;
                    if let Ok(res) = serde_wasm_bindgen::from_value::<Vec<HistoryEntry>>(result) {
                        entries.set(res);
                    }
                });
            } else {
                let entries = entries.clone();
                let tool_id = tool_id.clone();
                spawn_local(async move {
                    let args =
                        serde_wasm_bindgen::to_value(&SearchToolHistoryArgs { tool_id, query })
                            .unwrap();
                    let result = invoke("search_tool_history_cmd", args).await;
                    if let Ok(res) = serde_wasm_bindgen::from_value::<Vec<HistoryEntry>>(result) {
                        entries.set(res);
                    }
                });
            }
        })
    };

    let on_clear_all = {
        let tool_id = props.tool_id.clone();
        let entries = entries.clone();
        Callback::from(move |_| {
            let tool_id = tool_id.clone();
            let entries = entries.clone();
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&ClearToolHistoryArgs { tool_id }).unwrap();
                let _ = invoke("clear_tool_history_cmd", args).await;
                entries.set(Vec::new());
            });
        })
    };

    let entry_count = entries.len();

    html! {
        <div class="input-history-panel">
            <button
                class={classes!("history-toggle-btn", (*is_open).then_some("active"))}
                onclick={on_toggle}
                title={i18n.t("input_history.toggle")}
            >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="12" cy="12" r="10"/>
                    <polyline points="12 6 12 12 16 14"/>
                </svg>
                {i18n.t("input_history.title")}
                if entry_count > 0 {
                    <span class="history-badge">{entry_count}</span>
                }
            </button>

            if *is_open {
                <div class="history-dropdown">
                    <div class="history-dropdown-header">
                        <input
                            type="text"
                            class="history-search"
                            placeholder={i18n.t("input_history.search_placeholder")}
                            value={(*search_query).clone()}
                            oninput={on_search_change}
                        />
                        if !entries.is_empty() {
                            <button
                                class="history-clear-btn"
                                onclick={on_clear_all}
                                title={i18n.t("input_history.clear_all")}
                            >
                                {i18n.t("input_history.clear_all")}
                            </button>
                        }
                    </div>
                    <div class="history-entries-list">
                        if entries.is_empty() {
                            <div class="history-empty">
                                {i18n.t("input_history.no_entries")}
                            </div>
                        } else {
                            { for entries.iter().map(|entry| {
                                let entry_id = entry.id.clone();
                                let inputs = entry.inputs.clone();
                                let on_restore = props.on_restore.clone();
                                let entries_state = entries.clone();

                                let on_restore_click = {
                                    let inputs = inputs.clone();
                                    let on_restore = on_restore.clone();
                                    Callback::from(move |_| {
                                        on_restore.emit(inputs.clone());
                                    })
                                };

                                let on_delete = {
                                    let entry_id = entry_id.clone();
                                    let entries_state = entries_state.clone();
                                    Callback::from(move |e: MouseEvent| {
                                        e.stop_propagation();
                                        let entry_id = entry_id.clone();
                                        let entries_state = entries_state.clone();
                                        spawn_local(async move {
                                            let args = serde_wasm_bindgen::to_value(
                                                &DeleteHistoryEntryArgs {
                                                    entry_id: entry_id.clone(),
                                                },
                                            )
                                            .unwrap();
                                            let _ = invoke("delete_history_entry_cmd", args).await;
                                            let mut current = (*entries_state).clone();
                                            current.retain(|e| e.id != entry_id);
                                            entries_state.set(current);
                                        });
                                    })
                                };

                                html! {
                                    <div class="history-entry-item" onclick={on_restore_click}>
                                        <div class="history-entry-content">
                                            <div class="history-entry-preview">
                                                {format_preview(&entry.inputs)}
                                            </div>
                                            <div class="history-entry-time">
                                                {format_time_ago(&entry.created_at)}
                                            </div>
                                        </div>
                                        <button
                                            class="history-entry-delete"
                                            onclick={on_delete}
                                            title={i18n.t("input_history.delete_entry")}
                                        >
                                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <line x1="18" y1="6" x2="6" y2="18"/>
                                                <line x1="6" y1="6" x2="18" y2="18"/>
                                            </svg>
                                        </button>
                                    </div>
                                }
                            })}
                        }
                    </div>
                </div>
            }
        </div>
    }
}
