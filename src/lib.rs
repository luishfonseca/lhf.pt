use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod app_router;
mod markdown_page;
mod posts_router;
mod config;

use app_router::AppRouter;

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <AppRouter />
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
