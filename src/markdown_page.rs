use gloo_net::{http::Request, Error};
use markdown::to_html;
use yew::prelude::*;

pub enum LoadState {
    Loading,
    Loaded(String),
}

pub struct MarkdownPage {
    new_path: bool,
    content: LoadState,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: AttrValue,
}

async fn fetch_markdown(path: &str) -> Result<String, Error> {
    Request::get(path).send().await?.text().await
}

impl Component for MarkdownPage {
    type Message = Result<String, Error>;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            new_path: true,
            content: LoadState::Loading,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        self.new_path = ctx.props().path != old_props.path;
        self.new_path
    }

    fn rendered(&mut self, ctx: &Context<Self>, _: bool) {
        if self.new_path {
            let path = ctx.props().path.to_string();
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                link.send_message(fetch_markdown(&path).await.map(|md| to_html(&md)));
            });
            self.new_path = false;
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(content) => {
                self.content = LoadState::Loaded(content);
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
            LoadState::Loading => html! { <h1>{ "Loading..." }</h1> },
            LoadState::Loaded(content) => {
                let parsed = Html::from_html_unchecked(AttrValue::from(content.clone()));
                html! { {parsed} }
            }
        }
    }
}
