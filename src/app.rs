use crate::components::csv_viewer::CsvViewer;
use crate::components::image_compressor::ImageCompressor;
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
    CsvViewer,
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

#[function_component(App)]
pub fn app() -> Html {
    let active_tab = use_state(|| Tab::ImageCompressor);
    let dropped_image_path = use_state(|| Option::<String>::None);
    let dropped_csv_path = use_state(|| Option::<String>::None);
    let is_drag_over = use_state(|| false);

    // Set up drag-drop event listeners
    {
        let active_tab = active_tab.clone();
        let dropped_image_path = dropped_image_path.clone();
        let dropped_csv_path = dropped_csv_path.clone();
        let is_drag_over = is_drag_over.clone();

        use_effect_with((), move |_| {
            let active_tab = active_tab.clone();
            let dropped_image_path = dropped_image_path.clone();
            let dropped_csv_path = dropped_csv_path.clone();
            let is_drag_over_enter = is_drag_over.clone();
            let is_drag_over_leave = is_drag_over.clone();

            spawn_local(async move {
                // Listen for file drop
                let drop_handler = {
                    let active_tab = active_tab.clone();
                    let dropped_image_path = dropped_image_path.clone();
                    let dropped_csv_path = dropped_csv_path.clone();
                    Closure::new(move |event: JsValue| {
                        if let Ok(paths) = serde_wasm_bindgen::from_value::<DropEvent>(event) {
                            if let Some(first_path) = paths.payload.first() {
                                if is_image_file(first_path) {
                                    dropped_image_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::ImageCompressor);
                                } else if is_csv_file(first_path) {
                                    dropped_csv_path.set(Some(first_path.clone()));
                                    active_tab.set(Tab::CsvViewer);
                                }
                            }
                        }
                    })
                };
                let _ = tauri_listen("file-drop", &drop_handler).await;
                drop_handler.forget();

                // Listen for drag enter
                let enter_handler = {
                    let is_drag_over = is_drag_over_enter.clone();
                    Closure::new(move |_: JsValue| {
                        is_drag_over.set(true);
                    })
                };
                let _ = tauri_listen("file-drag-enter", &enter_handler).await;
                enter_handler.forget();

                // Listen for drag leave
                let leave_handler = {
                    let is_drag_over = is_drag_over_leave.clone();
                    Closure::new(move |_: JsValue| {
                        is_drag_over.set(false);
                    })
                };
                let _ = tauri_listen("file-drag-leave", &leave_handler).await;
                leave_handler.forget();
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

    let container_class = if *is_drag_over {
        "container container-wide drag-over"
    } else {
        "container container-wide"
    };

    html! {
        <main class={container_class}>
            {if *is_drag_over {
                html! {
                    <div class="drop-overlay">
                        <div class="drop-overlay-content">
                            <div class="drop-overlay-icon">{"📁"}</div>
                            <p>{"Drop file here"}</p>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}

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
                    class={if *active_tab == Tab::CsvViewer { "tab-btn active" } else { "tab-btn" }}
                    onclick={
                        let on_click = on_tab_click.clone();
                        Callback::from(move |_| on_click.emit(Tab::CsvViewer))
                    }
                >
                    <span class="tab-icon">{"📊"}</span>
                    <span class="tab-label">{"CSV Viewer"}</span>
                </button>
            </div>

            <div class={if *active_tab == Tab::ImageCompressor { "tab-panel active" } else { "tab-panel" }}>
                <ImageCompressor
                    dropped_file={(*dropped_image_path).clone()}
                    on_file_processed={on_image_file_processed}
                />
            </div>
            <div class={if *active_tab == Tab::CsvViewer { "tab-panel active" } else { "tab-panel" }}>
                <CsvViewer
                    dropped_file={(*dropped_csv_path).clone()}
                    on_file_processed={on_csv_file_processed}
                />
            </div>
        </main>
    }
}

#[derive(serde::Deserialize)]
struct DropEvent {
    payload: Vec<String>,
}
