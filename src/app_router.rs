use gloo_net::{http::Request, Error};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::config::CONFIG;
use crate::markdown_page::MarkdownPage;
use crate::posts_router::PostsRouter;

pub enum LoadState {
    Loading,
    Loaded(String),
}

pub struct AppRouter {
    posts_index: LoadState,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
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

fn switch(routes: Route, posts: &PostsRouter) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::About => html! { <MarkdownPage md={ "about" } /> },
        Route::Posts => posts.view_index(),
        Route::Post { slug } => posts.view_post(&slug),
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

async fn fetch_posts_index() -> Result<String, Error> {
    let url = CONFIG.content_source_url.to_string() + CONFIG.posts_index_path;
    Request::get(&url).send().await?.text().await
}

impl Component for AppRouter {
    type Message = Result<String, Error>;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            posts_index: LoadState::Loading,
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
                self.posts_index = LoadState::Loaded(value);
                true
            }
            Err(error) => {
                log::error!("Failed to load posts: {}", error.to_string());
                false
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        match &self.posts_index {
            LoadState::Loading => html! { <h1>{ "Loading..." }</h1> },
            LoadState::Loaded(posts_index) => match PostsRouter::parse_index(posts_index) {
                Ok(posts) => {
                    let switch = move |routes: Route| switch(routes, &posts);
                    html! {
                        <BrowserRouter>
                            <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                            <span>{ " | " }</span>
                            <Link<Route> to={Route::Posts}>{ "Posts Index" }</Link<Route>>
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
