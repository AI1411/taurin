use crate::components::csv_viewer::CsvViewer;
use crate::components::image_compressor::ImageCompressor;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum Tab {
    ImageCompressor,
    CsvViewer,
}

#[function_component(App)]
pub fn app() -> Html {
    let active_tab = use_state(|| Tab::ImageCompressor);

    let on_tab_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: Tab| {
            active_tab.set(tab);
        })
    };

    html! {
        <main class="container">
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

            <div class="tab-content">
                {match *active_tab {
                    Tab::ImageCompressor => html! { <ImageCompressor /> },
                    Tab::CsvViewer => html! { <CsvViewer /> },
                }}
            </div>
        </main>
    }
}
