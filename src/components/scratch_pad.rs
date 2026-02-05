use gloo_timers::callback::Timeout;
use i18nrs::yew::use_translation;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"], catch)]
    async fn save(options: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Note {
    fn title(&self) -> String {
        let first_line = self.content.lines().next().unwrap_or("");
        let title = first_line.trim_start_matches('#').trim();
        if title.is_empty() {
            "New Note".to_string()
        } else if title.len() > 30 {
            format!("{}...", &title[..27])
        } else {
            title.to_string()
        }
    }

    fn preview(&self) -> String {
        let lines: Vec<&str> = self.content.lines().skip(1).take(2).collect();
        let preview = lines.join(" ").trim().to_string();
        if preview.len() > 50 {
            format!("{}...", &preview[..47])
        } else if preview.is_empty() {
            "No additional text".to_string()
        } else {
            preview
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchPadData {
    pub notes: Vec<Note>,
    pub active_note_id: Option<String>,
}

impl Default for ScratchPadData {
    fn default() -> Self {
        // Use js_sys for generating UUID-like ID and timestamp in WASM
        let now = js_sys::Date::new_0()
            .to_iso_string()
            .as_string()
            .unwrap_or_default();
        let id = format!("{:x}", js_sys::Math::random().to_bits());
        let default_note = Note {
            id: id.clone(),
            content: String::new(),
            created_at: now.clone(),
            updated_at: now,
        };
        Self {
            notes: vec![default_note],
            active_note_id: Some(id),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownToHtmlResult {
    pub success: bool,
    pub html: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize)]
struct EmptyArgs {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateNoteArgs {
    note_id: String,
    content: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteNoteArgs {
    note_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SetActiveNoteArgs {
    note_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MarkdownToHtmlArgs {
    markdown: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportToFileArgs {
    content: String,
    path: String,
}

#[derive(Serialize)]
struct SaveDialogOptions {
    title: String,
    filters: Vec<SaveFilter>,
}

#[derive(Serialize)]
struct SaveFilter {
    name: String,
    extensions: Vec<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Edit,
    Preview,
    Split,
}

impl ViewMode {
    fn translation_key(&self) -> &'static str {
        match self {
            ViewMode::Edit => "common.edit",
            ViewMode::Preview => "common.preview",
            ViewMode::Split => "common.split",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ScratchPadProps {}

#[function_component(ScratchPad)]
pub fn scratch_pad(_props: &ScratchPadProps) -> Html {
    let (i18n, _) = use_translation();
    let data = use_state(|| Option::<ScratchPadData>::None);
    let preview_html = use_state(String::new);
    let is_loading = use_state(|| true);
    let view_mode = use_state(|| ViewMode::Split);
    let auto_save_pending = use_state(|| false);
    let save_status = use_state(|| "");

    // Load data on mount
    {
        let data = data.clone();
        let preview_html = preview_html.clone();
        let is_loading = is_loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&EmptyArgs {}).unwrap();

                let pad_data = match invoke("load_scratch_pad_cmd", args).await {
                    Ok(result) => serde_wasm_bindgen::from_value::<ScratchPadData>(result)
                        .unwrap_or_else(|_| ScratchPadData::default()),
                    Err(_) => ScratchPadData::default(),
                };

                // Generate initial preview for active note
                if let Some(active_id) = &pad_data.active_note_id {
                    if let Some(note) = pad_data.notes.iter().find(|n| &n.id == active_id) {
                        let md_args = serde_wasm_bindgen::to_value(&MarkdownToHtmlArgs {
                            markdown: note.content.clone(),
                        })
                        .unwrap();
                        if let Ok(html_result) = invoke("markdown_to_html_cmd", md_args).await {
                            if let Ok(res) =
                                serde_wasm_bindgen::from_value::<MarkdownToHtmlResult>(html_result)
                            {
                                if let Some(html) = res.html {
                                    preview_html.set(html);
                                }
                            }
                        }
                    }
                }

                data.set(Some(pad_data));
                is_loading.set(false);
            });
            || {}
        });
    }

    let active_note = {
        let data = (*data).clone();
        data.and_then(|d| {
            let active_id = d.active_note_id.clone();
            active_id.and_then(|id| d.notes.into_iter().find(|n| n.id == id))
        })
    };

    let on_create_note = {
        let data = data.clone();
        let preview_html = preview_html.clone();
        Callback::from(move |_| {
            let data = data.clone();
            let preview_html = preview_html.clone();
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&EmptyArgs {}).unwrap();
                if let Ok(result) = invoke("create_note_cmd", args).await {
                    if let Ok(new_note) = serde_wasm_bindgen::from_value::<Note>(result) {
                        if let Some(d) = (*data).clone() {
                            let mut new_data = d;
                            new_data.notes.insert(0, new_note.clone());
                            new_data.active_note_id = Some(new_note.id);
                            data.set(Some(new_data));
                            preview_html.set(String::new());
                        }
                    }
                }
            });
        })
    };

    let on_select_note = {
        let data = data.clone();
        let preview_html = preview_html.clone();
        Callback::from(move |note_id: String| {
            let data = data.clone();
            let preview_html = preview_html.clone();
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&SetActiveNoteArgs {
                    note_id: note_id.clone(),
                })
                .unwrap();
                if let Ok(result) = invoke("set_active_note_cmd", args).await {
                    if let Ok(pad_data) = serde_wasm_bindgen::from_value::<ScratchPadData>(result) {
                        // Update preview
                        if let Some(note) = pad_data.notes.iter().find(|n| n.id == note_id) {
                            let md_args = serde_wasm_bindgen::to_value(&MarkdownToHtmlArgs {
                                markdown: note.content.clone(),
                            })
                            .unwrap();
                            if let Ok(html_result) = invoke("markdown_to_html_cmd", md_args).await {
                                if let Ok(res) = serde_wasm_bindgen::from_value::<
                                    MarkdownToHtmlResult,
                                >(html_result)
                                {
                                    if let Some(html) = res.html {
                                        preview_html.set(html);
                                    }
                                }
                            }
                        }
                        data.set(Some(pad_data));
                    }
                }
            });
        })
    };

    let on_delete_note = {
        let data = data.clone();
        let preview_html = preview_html.clone();
        Callback::from(move |note_id: String| {
            let data = data.clone();
            let preview_html = preview_html.clone();
            spawn_local(async move {
                let args = serde_wasm_bindgen::to_value(&DeleteNoteArgs { note_id }).unwrap();
                if let Ok(result) = invoke("delete_note_cmd", args).await {
                    if let Ok(pad_data) = serde_wasm_bindgen::from_value::<ScratchPadData>(result) {
                        // Update preview for new active note
                        if let Some(active_id) = &pad_data.active_note_id {
                            if let Some(note) = pad_data.notes.iter().find(|n| &n.id == active_id) {
                                let md_args = serde_wasm_bindgen::to_value(&MarkdownToHtmlArgs {
                                    markdown: note.content.clone(),
                                })
                                .unwrap();
                                if let Ok(html_result) =
                                    invoke("markdown_to_html_cmd", md_args).await
                                {
                                    if let Ok(res) =
                                        serde_wasm_bindgen::from_value::<MarkdownToHtmlResult>(
                                            html_result,
                                        )
                                    {
                                        if let Some(html) = res.html {
                                            preview_html.set(html);
                                        }
                                    }
                                }
                            }
                        }
                        data.set(Some(pad_data));
                    }
                }
            });
        })
    };

    let on_content_change = {
        let data = data.clone();
        let preview_html = preview_html.clone();
        let auto_save_pending = auto_save_pending.clone();
        let save_status = save_status.clone();
        let active_note = active_note.clone();

        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            let new_content = textarea.value();

            // Update local state immediately
            if let Some(note) = active_note.clone() {
                let data_clone = data.clone();
                if let Some(d) = (*data_clone).clone() {
                    let mut new_data = d;
                    if let Some(n) = new_data.notes.iter_mut().find(|n| n.id == note.id) {
                        n.content = new_content.clone();
                    }
                    data_clone.set(Some(new_data));
                }

                // Update preview
                let preview_html_clone = preview_html.clone();
                let content_for_preview = new_content.clone();
                spawn_local(async move {
                    let args = serde_wasm_bindgen::to_value(&MarkdownToHtmlArgs {
                        markdown: content_for_preview,
                    })
                    .unwrap();
                    if let Ok(result) = invoke("markdown_to_html_cmd", args).await {
                        if let Ok(res) =
                            serde_wasm_bindgen::from_value::<MarkdownToHtmlResult>(result)
                        {
                            if let Some(html) = res.html {
                                preview_html_clone.set(html);
                            }
                        }
                    }
                });

                // Debounced auto-save
                if !*auto_save_pending {
                    auto_save_pending.set(true);
                    save_status.set("Saving...");

                    let save_status_clone = save_status.clone();
                    let auto_save_pending_clone = auto_save_pending.clone();
                    let content_for_save = new_content;
                    let note_id = note.id.clone();

                    Timeout::new(1000, move || {
                        let save_status_after = save_status_clone.clone();
                        let pending_after = auto_save_pending_clone.clone();

                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&UpdateNoteArgs {
                                note_id,
                                content: content_for_save,
                            })
                            .unwrap();
                            match invoke("update_note_cmd", args).await {
                                Ok(result) => {
                                    if serde_wasm_bindgen::from_value::<Note>(result).is_ok() {
                                        save_status_after.set("Saved");
                                    } else {
                                        save_status_after.set("Save failed");
                                    }
                                }
                                Err(_) => {
                                    save_status_after.set("Save failed");
                                }
                            }

                            let status_reset = save_status_after.clone();
                            Timeout::new(2000, move || {
                                status_reset.set("");
                            })
                            .forget();

                            pending_after.set(false);
                        });
                    })
                    .forget();
                }
            }
        })
    };

    let on_view_mode_change = {
        let view_mode = view_mode.clone();
        Callback::from(move |mode: ViewMode| {
            view_mode.set(mode);
        })
    };

    let on_save_file = {
        let active_note = active_note.clone();
        let save_status = save_status.clone();
        Callback::from(move |_| {
            if let Some(note) = active_note.clone() {
                let content = note.content.clone();
                let save_status = save_status.clone();
                spawn_local(async move {
                    let options = serde_wasm_bindgen::to_value(&SaveDialogOptions {
                        title: "Save Markdown File".to_string(),
                        filters: vec![SaveFilter {
                            name: "Markdown".to_string(),
                            extensions: vec!["md".to_string()],
                        }],
                    })
                    .unwrap();

                    if let Ok(path_result) = save(options).await {
                        if let Some(path) = path_result.as_string() {
                            let args =
                                serde_wasm_bindgen::to_value(&ExportToFileArgs { content, path })
                                    .unwrap();
                            let _ = invoke("export_to_file_cmd", args).await;
                            save_status.set("Exported!");
                            let status_reset = save_status.clone();
                            Timeout::new(2000, move || {
                                status_reset.set("");
                            })
                            .forget();
                        }
                    }
                });
            }
        })
    };

    if *is_loading {
        return html! {
            <div class="container container-wide">
                <div class="section">
                    <div class="scratch-pad-loading">
                        <div class="processing-spinner"></div>
                        <p>{i18n.t("common.loading")}</p>
                    </div>
                </div>
            </div>
        };
    }

    let notes = (*data).clone().map(|d| d.notes).unwrap_or_default();
    let view_modes = [ViewMode::Edit, ViewMode::Split, ViewMode::Preview];

    html! {
        <div class="container container-wide">
            <div class="notes-app">
                // Sidebar
                <div class="notes-sidebar">
                    <div class="notes-sidebar-header">
                        <span class="notes-count">{format!("{} {}", notes.len(), i18n.t("app.tabs.notes"))}</span>
                        <button class="new-note-btn" onclick={on_create_note} title={i18n.t("scratch_pad.new_note")}>
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="12" y1="5" x2="12" y2="19"/>
                                <line x1="5" y1="12" x2="19" y2="12"/>
                            </svg>
                        </button>
                    </div>
                    <div class="notes-list">
                        { for notes.iter().map(|note| {
                            let is_active = active_note.as_ref().map(|n| n.id == note.id).unwrap_or(false);
                            let on_select = on_select_note.clone();
                            let on_delete = on_delete_note.clone();
                            let id_for_select = note.id.clone();
                            let id_for_delete = note.id.clone();
                            html! {
                                <div
                                    class={classes!("note-item", is_active.then_some("active"))}
                                    onclick={Callback::from(move |_| on_select.emit(id_for_select.clone()))}
                                >
                                    <div class="note-item-content">
                                        <div class="note-item-title">{note.title()}</div>
                                        <div class="note-item-preview">{note.preview()}</div>
                                    </div>
                                    if notes.len() > 1 {
                                        <button
                                            class="note-delete-btn"
                                            onclick={Callback::from(move |e: MouseEvent| {
                                                e.stop_propagation();
                                                on_delete.emit(id_for_delete.clone());
                                            })}
                                            title={i18n.t("common.delete")}
                                        >
                                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <line x1="18" y1="6" x2="6" y2="18"/>
                                                <line x1="6" y1="6" x2="18" y2="18"/>
                                            </svg>
                                        </button>
                                    }
                                </div>
                            }
                        })}
                    </div>
                </div>

                // Editor
                <div class="notes-editor">
                    if let Some(note) = active_note.clone() {
                        <div class="notes-editor-header">
                            <div class="notes-editor-status">
                                if !(*save_status).is_empty() {
                                    <span class="save-status">{*save_status}</span>
                                }
                            </div>
                            <div class="notes-editor-actions">
                                <button class="export-btn" onclick={on_save_file} title={i18n.t("common.export")}>
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4"/>
                                        <polyline points="7 10 12 15 17 10"/>
                                        <line x1="12" y1="15" x2="12" y2="3"/>
                                    </svg>
                                </button>
                                <div class="view-mode-tabs">
                                    { for view_modes.iter().map(|mode| {
                                        let is_active = *view_mode == *mode;
                                        let on_click = on_view_mode_change.clone();
                                        let m = *mode;
                                        let label = i18n.t(mode.translation_key());
                                        html! {
                                            <button
                                                class={classes!("view-mode-tab", is_active.then_some("active"))}
                                                onclick={Callback::from(move |_| on_click.emit(m))}
                                            >
                                                {label}
                                            </button>
                                        }
                                    })}
                                </div>
                            </div>
                        </div>
                        <div class={classes!(
                            "notes-editor-content",
                            match *view_mode {
                                ViewMode::Edit => "edit-only",
                                ViewMode::Preview => "preview-only",
                                ViewMode::Split => "split-view",
                            }
                        )}>
                            if *view_mode != ViewMode::Preview {
                                <div class="editor-pane">
                                    <textarea
                                        class="markdown-editor"
                                        value={note.content.clone()}
                                        oninput={on_content_change}
                                        placeholder="# Start writing...

Write your notes in Markdown format.

**Bold**, *italic*, `code`

- List items
- More items

> Quotes"
                                        spellcheck="false"
                                    />
                                </div>
                            }
                            if *view_mode != ViewMode::Edit {
                                <div class="preview-pane">
                                    <div class="markdown-preview">
                                        { Html::from_html_unchecked(AttrValue::from((*preview_html).clone())) }
                                    </div>
                                </div>
                            }
                        </div>
                        <div class="notes-editor-footer">
                            <span class="char-count">
                                {format!("{} {}", note.content.len(), i18n.t("common.characters"))}
                            </span>
                            <span class="line-count">
                                {format!("{} {}", note.content.lines().count().max(1), i18n.t("common.lines"))}
                            </span>
                        </div>
                    } else {
                        <div class="no-note-selected">
                            <p>{i18n.t("scratch_pad.select_or_create")}</p>
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
