use crate::app::Route;
use gloo_net::{http::Request, Error};
use gray_matter::engine::YAML;
use gray_matter::Matter;
use gray_matter::Pod;
use pulldown_cmark::html::push_html;
use pulldown_cmark::HeadingLevel;
use pulldown_cmark::{Event, Parser, Tag};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::config::CONFIG;

pub struct PostData {
    title: String,
    description: Option<String>,
    tags: Vec<String>,
    html: String,
}

pub enum LoadState {
    Loading,
    Loaded(PostData),
}

pub struct MarkdownPage {
    path_changed: bool,
    content: LoadState,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub md: AttrValue,
}

async fn fetch_markdown(path: &str) -> Result<String, Error> {
    Request::get(path).send().await?.text().await
}

impl Component for MarkdownPage {
    type Message = Result<String, Error>;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            path_changed: true,
            content: LoadState::Loading,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        self.path_changed = ctx.props().md != old_props.md;
        self.path_changed
    }

    fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
        if self.path_changed || CONFIG.dev {
            let path = CONFIG.content_source_url.to_string() + &ctx.props().md + ".md";
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                link.send_message(fetch_markdown(&path).await);
            });
            self.path_changed = false;
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(md_content) => {
                let mut post = PostData {
                    title: String::new(),
                    description: None,
                    tags: Vec::new(),
                    html: String::new(),
                };

                let fm_parser = Matter::<YAML>::new();
                let fm = fm_parser.parse(&md_content);

                if let Some(Pod::Hash(data)) = fm.data {
                    if let Some(Pod::String(title)) = data.get("title") {
                        post.title = title.to_string();
                    }

                    if let Some(Pod::String(description)) = data.get("description") {
                        post.description = Some(description.to_string());
                    }

                    if let Some(Pod::String(tags)) = data.get("tags") {
                        for tag in tags.split(" ") {
                            post.tags.push(tag.trim().to_string());
                        }
                    }
                }

                let md_parser = Parser::new(&fm.content);

                let md_parser = md_parser.map(|event| match event {
                    Event::Start(Tag::Heading(heading, id, classes)) => {
                        let heading = heading as usize + 1;
                        let heading = HeadingLevel::try_from(heading).unwrap_or(HeadingLevel::H6);
                        Event::Start(Tag::Heading(heading, id, classes))
                    }
                    _ => event,
                });

                push_html(&mut post.html, md_parser);

                self.content = LoadState::Loaded(post);
                true
            }
            Err(error) => {
                log::error!("Error loading content: {:?}", error);
                false
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        match &self.content {
            LoadState::Loading => html! { },
            LoadState::Loaded(post) => {
                html! {
                    <article>
                        <header>
                            <hgroup>
                                <h1>{ &post.title }</h1>
                                if let Some(description) = &post.description {
                                    <p>{ description }</p>
                                }
                            </hgroup>

                            if post.tags.len() > 0 {
                                <span>{"Tags:"}</span>
                                { for post.tags.iter().map(|tag| html! { <>
                                    <span>{ " " }</span>
                                    <Link<Route> to={ Route::Tag { tag: tag.to_string() }}>{ tag }</Link<Route>>
                                </> })}
                            }
                        </header>

                        { Html::from_html_unchecked(AttrValue::from(post.html.clone())) }
                    </article>
               }
            }
        }
    }
}
