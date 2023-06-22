use gloo_net::{http::Request, Error};
use serde_json::Value;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::markdown_page::MarkdownPage;
use crate::posts_index::PostsIndex;

pub enum LoadState {
    Loading,
    Loaded(Value),
}

pub struct AppRouter {
    posts_index_value: LoadState,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/env")]
    Environment,
    #[at("/about")]
    About,
    #[at("/post")]
    Posts,
    #[at("/post/:slug")]
    Post { slug: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route, idx: &PostsIndex) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Environment => html! { <Environment /> },
        Route::About => html! { <MarkdownPage path ="/content/about.md" /> },
        Route::Posts => html! { {idx.html()} },
        Route::Post { slug } => match idx.get_path(&slug) {
            Some(path) => html! { <MarkdownPage path ={path} /> },
            None => {
                html! { <h1>{ "404" }</h1> }
            }
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

async fn fetch_posts_index() -> Result<Value, Error> {
    Request::get("/content/posts.json")
        .send()
        .await?
        .json()
        .await
}

impl Component for AppRouter {
    type Message = Result<Value, Error>;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            posts_index_value: LoadState::Loading,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                link.send_message(fetch_posts_index().await);
            })
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(value) => {
                self.posts_index_value = LoadState::Loaded(value);
                true
            }
            Err(error) => {
                log::error!("Failed to load posts: {}", error.to_string());
                false
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        match &self.posts_index_value {
            LoadState::Loading => html! { <h1>{ "Loading..." }</h1> },
            LoadState::Loaded(posts_index_value) => match PostsIndex::parse(posts_index_value) {
                Ok(idx) => {
                    let switch = move |routes: Route| switch(routes, &idx);
                    html! {
                        <BrowserRouter>
                            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                            <span>{ " | " }</span>
                            <Link<Route> to={Route::Posts}>{ "Post Index" }</Link<Route>>
                            <span>{ " | " }</span>
                            <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                            <Switch<Route> render={switch} />
                        </BrowserRouter>
                    }
                }
                Err(error) => {
                    log::error!("Failed to parse posts index: {}", error.to_string());
                    html! { <h1>{ "Failed to parse posts index" }</h1> }
                }
            },
        }
    }
}

#[function_component(Environment)]
fn environment() -> Html {
    let navigator = use_navigator().unwrap();

    let env = option_env!("CARGO_PROFILE").unwrap_or("UNKNOWN");

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ env }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
