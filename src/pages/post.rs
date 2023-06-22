use gloo_net::{http::Request, Error};
use yew::prelude::*;

pub enum LoadState {
    Loading,
    Loaded(String),
}

pub struct Post {
    pub content: LoadState,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub path: AttrValue,
}

async fn fetch_post(path: &str) -> Result<String, Error> {
    Request::get(path).send().await?.text().await
}

impl Component for Post {
    type Message = Result<String, Error>;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            content: LoadState::Loading,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let path = ctx.props().path.to_string();
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                link.send_message(fetch_post(&path).await);
            })
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(content) => {
                self.content = LoadState::Loaded(content);
                true
            }
            Err(error) => {
                log::error!("Error fetching post: {:?}", error);
                true
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        match &self.content {
            LoadState::Loading => html! { <h1>{ "Loading..." }</h1> },
            LoadState::Loaded(content) => html! { <p>{ content }</p> }
        }
    }
}
