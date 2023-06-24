use wasm_bindgen::prelude::*;

mod app;
mod markdown_page;
mod posts_router;
mod config;

use app::App;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
