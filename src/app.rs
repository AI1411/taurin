use crate::components::csv_viewer::CsvViewer;
use crate::components::image_compressor::ImageCompressor;
use crate::components::image_editor::ImageEditor;
use crate::components::kanban_board::KanbanBoardComponent;
use crate::components::markdown_to_pdf::MarkdownToPdf;
use crate::components::pdf_tools::PdfTools;
use crate::components::uuid_generator::UuidGenerator;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"], js_name = listen)]
    async fn tauri_listen(event: &str, handler: &Closure<dyn Fn(JsValue)>) -> JsValue;
}

#[derive(Clone, PartialEq)]
enum Tab {
    ImageCompressor,
    ImageEditor,
    CsvViewer,
    PdfTools,
    MarkdownToPdf,
    KanbanBoard,
    UuidGenerator,
}

fn get_file_extension(path: &str) -> Option<String> {
    path.rsplit('.').next().map(|s| s.to_lowercase())
}

fn is_image_file(path: &str) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("png")
            | Some("jpg")
            | Some("jpeg")
            | Some("webp")
            | Some("avif")
            | Some("gif")
            | Some("bmp")
    )
}

fn is_csv_file(path: &str) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("csv") | Some("tsv") | Some("txt")
    )
}

fn is_pdf_file(path: &str) -> bool {
    matches!(get_file_extension(path).as_deref(), Some("pdf"))
}

fn is_markdown_file(path: &str) -> bool {
    matches!(
        get_file_extension(path).as_deref(),
        Some("md") | Some("markdown")
    )
}

#[function_component(App)]
pub fn app() -> Html {
    let active_tab = use_state(|| Tab::ImageCompressor);
    let dropped_image_path = use_state(|| Option::<String>::None);
    let dropped_editor_path = use_state(|| Option::<String>::None);
    let dropped_csv_path = use_state(|| Option::<String>::None);
    let dropped_pdf_path = use_state(|| Option::<String>::None);
    let dropped_markdown_path = use_state(|| Option::<String>::None);
    // Set up drag-drop event listeners (only once on mount)
    {
        let active_tab = active_tab.clone();
        let dropped_image_path = dropped_image_path.clone();
        let dropped_editor_path = dropped_editor_path.clone();
        let dropped_csv_path = dropped_csv_path.clone();
        let dropped_pdf_path = dropped_pdf_path.clone();
        let dropped_markdown_path = dropped_markdown_path.clone();

        use_effect_with((), move |_| {
            let active_tab = active_tab.clone();
            let dropped_image_path = dropped_image_path.clone();
            let dropped_editor_path = dropped_editor_path.clone();
            let dropped_csv_path = dropped_csv_path.clone();
            let dropped_pdf_path = dropped_pdf_path.clone();
            let dropped_markdown_path = dropped_markdown_path.clone();

            spawn_local(async move {
                // Listen for file drop only
                let drop_handler = {
                    let active_tab = active_tab.clone();
                    let dropped_image_path = dropped_image_path.clone();
                    let dropped_editor_path = dropped_editor_path.clone();
                    let dropped_csv_path = dropped_csv_path.clone();
                    let dropped_pdf_path = dropped_pdf_path.clone();
                    let dropped_markdown_path = dropped_markdown_path.clone();
                    Closure::new(move |event: JsValue| {
                        if let Ok(paths) = serde_wasm_bindgen::from_value::<DropEvent>(event) {
                            if let Some(first_path) = paths.payload.first() {
                                if is_image_file(first_path) {
                                    // Check if currently on ImageEditor tab, keep it there
                                    if *active_tab == Tab::ImageEditor {
                                        dropped_editor_path.set(Some(first_path.clone()));
                                    } else {
                                        dropped_image_path.set(Some(first_path.clone()));
                                        active_tab.set(Tab::ImageCompressor);
                                    }
                                } else if is_csv_file(first_path) {
                                    dropped_csv_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::CsvViewer);
                                } else if is_pdf_file(first_path) {
                                    dropped_pdf_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::PdfTools);
                                } else if is_markdown_file(first_path) {
                                    dropped_markdown_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::MarkdownToPdf);
                                }
                            }
                        }
                    })
                };
                let _ = tauri_listen("file-drop", &drop_handler).await;
                drop_handler.forget();
            });

            || {}
        });
    }

    let on_tab_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: Tab| {
            active_tab.set(tab);
        })
    };

    let on_image_file_processed = {
        let dropped_image_path = dropped_image_path.clone();
        Callback::from(move |_| {
            dropped_image_path.set(None);
        })
    };

    let on_csv_file_processed = {
        let dropped_csv_path = dropped_csv_path.clone();
        Callback::from(move |_| {
            dropped_csv_path.set(None);
        })
    };

    let on_pdf_file_processed = {
        let dropped_pdf_path = dropped_pdf_path.clone();
        Callback::from(move |_| {
            dropped_pdf_path.set(None);
        })
    };

    let on_editor_file_processed = {
        let dropped_editor_path = dropped_editor_path.clone();
        Callback::from(move |_| {
            dropped_editor_path.set(None);
        })
    };

    let on_markdown_file_processed = {
        let dropped_markdown_path = dropped_markdown_path.clone();
        Callback::from(move |_| {
            dropped_markdown_path.set(None);
        })
    };

    html! {
        <main class="container container-wide">
            <div class="tab-navigation">
                <button
                    class={if *active_tab == Tab::ImageCompressor { "tab-btn active" } else { "tab-btn" }}
                    onclick={
                        let on_click = on_tab_click.clone();
                        Callback::from(move |_| on_click.emit(Tab::ImageCompressor))
                    }
                >
                    <span class="tab-icon">{"🖼️"}</span>
                    <span class="tab-label">{"Image Compressor"}</span>
                </button>
                <button
                    class={if *active_tab == Tab::ImageEditor { "tab-btn active" } else { "tab-btn" }}
                    onclick={
                        let on_click = on_tab_click.clone();
                        Callback::from(move |_| on_click.emit(Tab::ImageEditor))
                    }
                >
                    <span class="tab-icon">{"🎨"}</span>
                    <span class="tab-label">{"Image Editor"}</span>
                </button>
                <button
                    class={if *active_tab == Tab::CsvViewer { "tab-btn active" } else { "tab-btn" }}
                    onclick={
                        let on_click = on_tab_click.clone();
                        Callback::from(move |_| on_click.emit(Tab::CsvViewer))
                    }
                >
                    <span class="tab-icon">{"📊"}</span>
                    <span class="tab-label">{"CSV Viewer"}</span>
                </button>
                <button
                    class={if *active_tab == Tab::PdfTools { "tab-btn active" } else { "tab-btn" }}
                    onclick={
                        let on_click = on_tab_click.clone();
                        Callback::from(move |_| on_click.emit(Tab::PdfTools))
                    }
                >
                    <span class="tab-icon">{"📄"}</span>
                    <span class="tab-label">{"PDF Tools"}</span>
                </button>
                <button
                    class={if *active_tab == Tab::MarkdownToPdf { "tab-btn active" } else { "tab-btn" }}
                    onclick={
                        let on_click = on_tab_click.clone();
                        Callback::from(move |_| on_click.emit(Tab::MarkdownToPdf))
                    }
                >
                    <span class="tab-icon">{"📝"}</span>
                    <span class="tab-label">{"MD to PDF"}</span>
                </button>
                <button
                    class={if *active_tab == Tab::KanbanBoard { "tab-btn active" } else { "tab-btn" }}
                    onclick={
                        let on_click = on_tab_click.clone();
                        Callback::from(move |_| on_click.emit(Tab::KanbanBoard))
                    }
                >
                    <span class="tab-icon">{"📋"}</span>
                    <span class="tab-label">{"Kanban"}</span>
                </button>
                <button
                    class={if *active_tab == Tab::UuidGenerator { "tab-btn active" } else { "tab-btn" }}
                    onclick={
                        let on_click = on_tab_click.clone();
                        Callback::from(move |_| on_click.emit(Tab::UuidGenerator))
                    }
                >
                    <span class="tab-icon">{"🔑"}</span>
                    <span class="tab-label">{"UUID"}</span>
                </button>
            </div>

            <div class={if *active_tab == Tab::ImageCompressor { "tab-panel active" } else { "tab-panel" }}>
                <ImageCompressor
                    dropped_file={(*dropped_image_path).clone()}
                    on_file_processed={on_image_file_processed}
                />
            </div>
            <div class={if *active_tab == Tab::ImageEditor { "tab-panel active" } else { "tab-panel" }}>
                <ImageEditor
                    dropped_file={(*dropped_editor_path).clone()}
                    on_file_processed={on_editor_file_processed}
                />
            </div>
            <div class={if *active_tab == Tab::CsvViewer { "tab-panel active" } else { "tab-panel" }}>
                <CsvViewer
                    dropped_file={(*dropped_csv_path).clone()}
                    on_file_processed={on_csv_file_processed}
                />
            </div>
            <div class={if *active_tab == Tab::PdfTools { "tab-panel active" } else { "tab-panel" }}>
                <PdfTools
                    dropped_file={(*dropped_pdf_path).clone()}
                    on_file_processed={on_pdf_file_processed}
                />
            </div>
            <div class={if *active_tab == Tab::MarkdownToPdf { "tab-panel active" } else { "tab-panel" }}>
                <MarkdownToPdf
                    dropped_file={(*dropped_markdown_path).clone()}
                    on_file_processed={on_markdown_file_processed}
                />
            </div>
            <div class={if *active_tab == Tab::KanbanBoard { "tab-panel active" } else { "tab-panel" }}>
                <KanbanBoardComponent />
            </div>
            <div class={if *active_tab == Tab::UuidGenerator { "tab-panel active" } else { "tab-panel" }}>
                <UuidGenerator />
            </div>
        </main>
    }
}

#[derive(serde::Deserialize)]
struct DropEvent {
    payload: Vec<String>,
}
