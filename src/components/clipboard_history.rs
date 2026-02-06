use i18nrs::yew::use_translation;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen)]
    async fn tauri_listen(event: &str, handler: &Closure<dyn Fn(JsValue)>) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Text,
    Code,
    Url,
    Email,
    Password,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClipboardEntry {
    pub id: String,
    pub content: String,
    pub content_type: ContentType,
    pub pinned: bool,
    pub created_at: String,
    pub copied_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardSettings {
    pub max_entries: u32,
    pub exclude_passwords: bool,
    pub exclude_patterns: Vec<String>,
}

impl Default for ClipboardSettings {
    fn default() -> Self {
        Self {
            max_entries: 100,
            exclude_passwords: true,
            exclude_patterns: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardHistoryData {
    pub entries: Vec<ClipboardEntry>,
    pub settings: ClipboardSettings,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EntryIdArgs {
    entry_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchArgs {
    query: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddEntryArgs {
    content: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSettingsArgs {
    settings: ClipboardSettings,
}

#[function_component(ClipboardHistory)]
pub fn clipboard_history() -> Html {
    let (i18n, _) = use_translation();
    let entries = use_state(Vec::<ClipboardEntry>::new);
    let settings = use_state(ClipboardSettings::default);
    let search_query = use_state(String::new);
    let is_loading = use_state(|| true);
    let show_settings = use_state(|| false);
    let copied_id = use_state(|| Option::<String>::None);
    let new_entry_content = use_state(String::new);

    // Load data on mount
    {
        let entries = entries.clone();
        let settings = settings.clone();
        let is_loading = is_loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(result) = invoke("load_clipboard_history_cmd", JsValue::null()).await {
                    if let Ok(data) =
                        serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                    {
                        entries.set(data.entries);
                        settings.set(data.settings);
                    }
                }
                is_loading.set(false);
            });
            || {}
        });
    }

    // Listen for clipboard changes from backend monitor
    {
        let entries = entries.clone();
        let settings = settings.clone();

        use_effect_with((), move |_| {
            let entries = entries.clone();
            let settings = settings.clone();

            spawn_local(async move {
                let handler = {
                    let entries = entries.clone();
                    let settings = settings.clone();
                    Closure::new(move |_event: JsValue| {
                        let entries = entries.clone();
                        let settings = settings.clone();
                        spawn_local(async move {
                            if let Ok(result) =
                                invoke("load_clipboard_history_cmd", JsValue::null()).await
                            {
                                if let Ok(data) =
                                    serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                                {
                                    entries.set(data.entries);
                                    settings.set(data.settings);
                                }
                            }
                        });
                    })
                };
                let _ = tauri_listen("clipboard-changed", &handler).await;
                handler.forget();
            });

            || {}
        });
    }

    let on_search = {
        let search_query = search_query.clone();
        let entries = entries.clone();

        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let query = input.value();
            search_query.set(query.clone());

            if query.is_empty() {
                let entries = entries.clone();
                spawn_local(async move {
                    if let Ok(result) =
                        invoke("load_clipboard_history_cmd", JsValue::null()).await
                    {
                        if let Ok(data) =
                            serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                        {
                            entries.set(data.entries);
                        }
                    }
                });
            } else {
                let entries = entries.clone();
                spawn_local(async move {
                    let args = SearchArgs { query };
                    if let Ok(args_js) = serde_wasm_bindgen::to_value(&args) {
                        if let Ok(result) =
                            invoke("search_clipboard_history_cmd", args_js).await
                        {
                            if let Ok(results) =
                                serde_wasm_bindgen::from_value::<Vec<ClipboardEntry>>(result)
                            {
                                entries.set(results);
                            }
                        }
                    }
                });
            }
        })
    };

    let on_copy = {
        let entries = entries.clone();
        let copied_id = copied_id.clone();

        Callback::from(move |entry_id: String| {
            let entries = entries.clone();
            let copied_id = copied_id.clone();
            let entry_id_clone = entry_id.clone();

            // Find and copy content to clipboard
            if let Some(entry) = entries.iter().find(|e| e.id == entry_id) {
                let content = entry.content.clone();
                spawn_local(async move {
                    // Copy to system clipboard using JS
                    if let Some(window) = web_sys::window() {
                        let navigator = window.navigator();
                        let clipboard = navigator.clipboard();
                        let _ = clipboard.write_text(&content);
                    }

                    // Update backend
                    let args = EntryIdArgs {
                        entry_id: entry_id_clone.clone(),
                    };
                    if let Ok(args_js) = serde_wasm_bindgen::to_value(&args) {
                        if let Ok(_result) = invoke("copy_clipboard_entry_cmd", args_js).await {
                            // Reload entries
                            if let Ok(result) =
                                invoke("load_clipboard_history_cmd", JsValue::null()).await
                            {
                                if let Ok(data) =
                                    serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                                {
                                    entries.set(data.entries);
                                }
                            }
                        }
                    }

                    copied_id.set(Some(entry_id_clone));

                    // Reset copied state after 2 seconds
                    gloo_timers::callback::Timeout::new(2000, move || {
                        copied_id.set(None);
                    })
                    .forget();
                });
            }
        })
    };

    let on_toggle_pin = {
        let entries = entries.clone();

        Callback::from(move |entry_id: String| {
            let entries = entries.clone();

            spawn_local(async move {
                let args = EntryIdArgs { entry_id };
                if let Ok(args_js) = serde_wasm_bindgen::to_value(&args) {
                    if let Ok(_result) = invoke("toggle_clipboard_pinned_cmd", args_js).await {
                        // Reload entries
                        if let Ok(result) =
                            invoke("load_clipboard_history_cmd", JsValue::null()).await
                        {
                            if let Ok(data) =
                                serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                            {
                                entries.set(data.entries);
                            }
                        }
                    }
                }
            });
        })
    };

    let on_delete = {
        let entries = entries.clone();
        let settings = settings.clone();

        Callback::from(move |entry_id: String| {
            let entries = entries.clone();
            let settings = settings.clone();

            spawn_local(async move {
                let args = EntryIdArgs { entry_id };
                if let Ok(args_js) = serde_wasm_bindgen::to_value(&args) {
                    if let Ok(result) = invoke("delete_clipboard_entry_cmd", args_js).await {
                        if let Ok(data) =
                            serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                        {
                            entries.set(data.entries);
                            settings.set(data.settings);
                        }
                    }
                }
            });
        })
    };

    let on_clear_all = {
        let entries = entries.clone();
        let settings = settings.clone();

        Callback::from(move |_| {
            let entries = entries.clone();
            let settings = settings.clone();

            spawn_local(async move {
                if let Ok(result) = invoke("clear_clipboard_history_cmd", JsValue::null()).await {
                    if let Ok(data) =
                        serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                    {
                        entries.set(data.entries);
                        settings.set(data.settings);
                    }
                }
            });
        })
    };

    let on_add_entry = {
        let entries = entries.clone();
        let new_entry_content = new_entry_content.clone();

        Callback::from(move |_| {
            let content = (*new_entry_content).clone();
            if content.is_empty() {
                return;
            }

            let entries = entries.clone();
            let new_entry_content = new_entry_content.clone();

            spawn_local(async move {
                let args = AddEntryArgs { content };
                if let Ok(args_js) = serde_wasm_bindgen::to_value(&args) {
                    if let Ok(_result) = invoke("add_clipboard_entry_cmd", args_js).await {
                        new_entry_content.set(String::new());
                        // Reload entries
                        if let Ok(result) =
                            invoke("load_clipboard_history_cmd", JsValue::null()).await
                        {
                            if let Ok(data) =
                                serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                            {
                                entries.set(data.entries);
                            }
                        }
                    }
                }
            });
        })
    };

    let on_toggle_settings = {
        let show_settings = show_settings.clone();
        Callback::from(move |_| {
            show_settings.set(!*show_settings);
        })
    };

    let on_settings_change = {
        let settings = settings.clone();
        let entries = entries.clone();

        Callback::from(move |new_settings: ClipboardSettings| {
            let settings = settings.clone();
            let entries = entries.clone();

            spawn_local(async move {
                let args = UpdateSettingsArgs {
                    settings: new_settings,
                };
                if let Ok(args_js) = serde_wasm_bindgen::to_value(&args) {
                    if let Ok(result) = invoke("update_clipboard_settings_cmd", args_js).await {
                        if let Ok(data) =
                            serde_wasm_bindgen::from_value::<ClipboardHistoryData>(result)
                        {
                            entries.set(data.entries);
                            settings.set(data.settings);
                        }
                    }
                }
            });
        })
    };

    let format_time = |created_at: &str| -> String {
        // Parse RFC3339 date and show simplified format
        if created_at.len() >= 10 {
            // Extract date part (YYYY-MM-DD)
            created_at[..10].to_string()
        } else {
            created_at.to_string()
        }
    };

    let get_content_type_icon = |content_type: &ContentType| -> Html {
        match content_type {
            ContentType::Text => html! {
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"/>
                    <path d="M14 2v6h6"/>
                    <line x1="8" y1="13" x2="16" y2="13"/>
                    <line x1="8" y1="17" x2="16" y2="17"/>
                </svg>
            },
            ContentType::Code => html! {
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="16 18 22 12 16 6"/>
                    <polyline points="8 6 2 12 8 18"/>
                </svg>
            },
            ContentType::Url => html! {
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M10 13a5 5 0 007.54.54l3-3a5 5 0 00-7.07-7.07l-1.72 1.71"/>
                    <path d="M14 11a5 5 0 00-7.54-.54l-3 3a5 5 0 007.07 7.07l1.71-1.71"/>
                </svg>
            },
            ContentType::Email => html! {
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"/>
                    <polyline points="22,6 12,13 2,6"/>
                </svg>
            },
            ContentType::Password => html! {
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="3" y="11" width="18" height="11" rx="2"/>
                    <path d="M7 11V7a5 5 0 0110 0v4"/>
                </svg>
            },
        }
    };

    let truncate_content = |content: &str, max_len: usize| -> String {
        if content.len() > max_len {
            format!("{}...", &content[..max_len])
        } else {
            content.to_string()
        }
    };

    // Separate pinned and regular entries
    let pinned_entries: Vec<_> = entries.iter().filter(|e| e.pinned).cloned().collect();
    let regular_entries: Vec<_> = entries.iter().filter(|e| !e.pinned).cloned().collect();

    html! {
        <div class="clipboard-history-container">
            <div class="clipboard-history-header">
                <h2 class="clipboard-history-title">{i18n.t("clipboard_history.title")}</h2>
                <div class="clipboard-history-actions">
                    <button
                        class="clipboard-settings-btn"
                        onclick={on_toggle_settings}
                        title={i18n.t("clipboard_history.settings")}
                    >
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="12" cy="12" r="3"/>
                            <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 010 2.83 2 2 0 01-2.83 0l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-2 2 2 2 0 01-2-2v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 01-2.83 0 2 2 0 010-2.83l.06-.06a1.65 1.65 0 00.33-1.82 1.65 1.65 0 00-1.51-1H3a2 2 0 01-2-2 2 2 0 012-2h.09A1.65 1.65 0 004.6 9a1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 010-2.83 2 2 0 012.83 0l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 012-2 2 2 0 012 2v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 012.83 0 2 2 0 010 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 012 2 2 2 0 01-2 2h-.09a1.65 1.65 0 00-1.51 1z"/>
                        </svg>
                    </button>
                    <button
                        class="clipboard-clear-btn"
                        onclick={on_clear_all}
                        title={i18n.t("common.clear_all")}
                    >
                        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polyline points="3 6 5 6 21 6"/>
                            <path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
                        </svg>
                    </button>
                </div>
            </div>

            if *show_settings {
                <div class="clipboard-settings-panel">
                    <h3>{i18n.t("clipboard_history.settings")}</h3>
                    <div class="settings-group">
                        <label>{i18n.t("clipboard_history.max_entries")}</label>
                        <input
                            type="number"
                            value={settings.max_entries.to_string()}
                            min="10"
                            max="1000"
                            onchange={{
                                let settings = settings.clone();
                                let on_settings_change = on_settings_change.clone();
                                Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    if let Ok(value) = input.value().parse::<u32>() {
                                        let mut new_settings = (*settings).clone();
                                        new_settings.max_entries = value.clamp(10, 1000);
                                        on_settings_change.emit(new_settings);
                                    }
                                })
                            }}
                        />
                    </div>
                    <div class="settings-group">
                        <label class="checkbox-label">
                            <input
                                type="checkbox"
                                checked={settings.exclude_passwords}
                                onchange={{
                                    let settings = settings.clone();
                                    let on_settings_change = on_settings_change.clone();
                                    Callback::from(move |e: Event| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        let mut new_settings = (*settings).clone();
                                        new_settings.exclude_passwords = input.checked();
                                        on_settings_change.emit(new_settings);
                                    })
                                }}
                            />
                            {i18n.t("clipboard_history.exclude_passwords")}
                        </label>
                    </div>
                </div>
            }

            <div class="clipboard-search-section">
                <div class="search-input-wrapper">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="11" cy="11" r="8"/>
                        <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                    </svg>
                    <input
                        type="text"
                        class="clipboard-search-input"
                        placeholder={i18n.t("clipboard_history.search_placeholder")}
                        value={(*search_query).clone()}
                        oninput={on_search}
                    />
                </div>
            </div>

            <div class="clipboard-add-section">
                <textarea
                    class="clipboard-add-input"
                    placeholder={i18n.t("clipboard_history.add_placeholder")}
                    value={(*new_entry_content).clone()}
                    oninput={{
                        let new_entry_content = new_entry_content.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                            new_entry_content.set(input.value());
                        })
                    }}
                />
                <button
                    class="clipboard-add-btn"
                    onclick={on_add_entry}
                    disabled={new_entry_content.is_empty()}
                >
                    {i18n.t("clipboard_history.add")}
                </button>
            </div>

            <div class="clipboard-entries-list">
                if *is_loading {
                    <div class="clipboard-loading">{i18n.t("common.loading")}</div>
                } else if entries.is_empty() {
                    <div class="clipboard-empty">{i18n.t("clipboard_history.empty")}</div>
                } else {
                    if !pinned_entries.is_empty() {
                        <div class="clipboard-section">
                            <div class="clipboard-section-header">
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                                </svg>
                                {i18n.t("clipboard_history.pinned")}
                            </div>
                            { for pinned_entries.iter().map(|entry| {
                                let entry_id_copy = entry.id.clone();
                                let entry_id_pin = entry.id.clone();
                                let entry_id_delete = entry.id.clone();
                                let is_copied = *copied_id == Some(entry.id.clone());
                                let on_copy = on_copy.clone();
                                let on_toggle_pin = on_toggle_pin.clone();
                                let on_delete = on_delete.clone();

                                let is_code = entry.content_type == ContentType::Code;
                                html! {
                                    <div class={classes!("clipboard-entry", "pinned", is_code.then_some("code-entry"))}>
                                        <div class="entry-content-wrapper">
                                            <span class="entry-type-icon">
                                                {get_content_type_icon(&entry.content_type)}
                                            </span>
                                            <div class="entry-content">
                                                <pre class="entry-text">{truncate_content(&entry.content, 200)}</pre>
                                                <div class="entry-meta">
                                                    <span class="entry-time">{format_time(&entry.created_at)}</span>
                                                    <span class="entry-count">{format!("{} {}", entry.copied_count, i18n.t("clipboard_history.copies"))}</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="entry-actions">
                                            <button
                                                class={classes!("entry-btn", "copy-btn", is_copied.then_some("copied"))}
                                                onclick={Callback::from(move |_| on_copy.emit(entry_id_copy.clone()))}
                                                title={i18n.t("common.copy")}
                                            >
                                                if is_copied {
                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                        <polyline points="20 6 9 17 4 12"/>
                                                    </svg>
                                                } else {
                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                        <rect x="9" y="9" width="13" height="13" rx="2"/>
                                                        <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                                    </svg>
                                                }
                                            </button>
                                            <button
                                                class={classes!("entry-btn", "pin-btn", "active")}
                                                onclick={Callback::from(move |_| on_toggle_pin.emit(entry_id_pin.clone()))}
                                                title={i18n.t("clipboard_history.unpin")}
                                            >
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                                                    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
                                                </svg>
                                            </button>
                                            <button
                                                class="entry-btn delete-btn"
                                                onclick={Callback::from(move |_| on_delete.emit(entry_id_delete.clone()))}
                                                title={i18n.t("common.delete")}
                                            >
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                    <line x1="18" y1="6" x2="6" y2="18"/>
                                                    <line x1="6" y1="6" x2="18" y2="18"/>
                                                </svg>
                                            </button>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    }

                    if !regular_entries.is_empty() {
                        <div class="clipboard-section">
                            <div class="clipboard-section-header">
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <circle cx="12" cy="12" r="10"/>
                                    <polyline points="12 6 12 12 16 14"/>
                                </svg>
                                {i18n.t("clipboard_history.recent")}
                            </div>
                            { for regular_entries.iter().map(|entry| {
                                let entry_id_copy = entry.id.clone();
                                let entry_id_pin = entry.id.clone();
                                let entry_id_delete = entry.id.clone();
                                let is_copied = *copied_id == Some(entry.id.clone());
                                let is_code = entry.content_type == ContentType::Code;
                                let on_copy = on_copy.clone();
                                let on_toggle_pin = on_toggle_pin.clone();
                                let on_delete = on_delete.clone();

                                html! {
                                    <div class={classes!("clipboard-entry", is_code.then_some("code-entry"))}>
                                        <div class="entry-content-wrapper">
                                            <span class="entry-type-icon">
                                                {get_content_type_icon(&entry.content_type)}
                                            </span>
                                            <div class="entry-content">
                                                <pre class="entry-text">{truncate_content(&entry.content, 200)}</pre>
                                                <div class="entry-meta">
                                                    <span class="entry-time">{format_time(&entry.created_at)}</span>
                                                    <span class="entry-count">{format!("{} {}", entry.copied_count, i18n.t("clipboard_history.copies"))}</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="entry-actions">
                                            <button
                                                class={classes!("entry-btn", "copy-btn", is_copied.then_some("copied"))}
                                                onclick={Callback::from(move |_| on_copy.emit(entry_id_copy.clone()))}
                                                title={i18n.t("common.copy")}
                                            >
                                                if is_copied {
                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                        <polyline points="20 6 9 17 4 12"/>
                                                    </svg>
                                                } else {
                                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                        <rect x="9" y="9" width="13" height="13" rx="2"/>
                                                        <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                                    </svg>
                                                }
                                            </button>
                                            <button
                                                class="entry-btn pin-btn"
                                                onclick={Callback::from(move |_| on_toggle_pin.emit(entry_id_pin.clone()))}
                                                title={i18n.t("clipboard_history.pin")}
                                            >
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
                                                </svg>
                                            </button>
                                            <button
                                                class="entry-btn delete-btn"
                                                onclick={Callback::from(move |_| on_delete.emit(entry_id_delete.clone()))}
                                                title={i18n.t("common.delete")}
                                            >
                                                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                    <line x1="18" y1="6" x2="6" y2="18"/>
                                                    <line x1="6" y1="6" x2="18" y2="18"/>
                                                </svg>
                                            </button>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    }
                }
            </div>

            <div class="clipboard-footer">
                <span class="entries-count">
                    {format!("{} {}", entries.len(), i18n.t("clipboard_history.entries"))}
                </span>
            </div>
        </div>
    }
}
