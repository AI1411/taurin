use crate::components::image_compressor::ImageCompressor;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="container">
            <h1>{"Image Compressor"}</h1>
            <ImageCompressor />
        </main>
    }
}
