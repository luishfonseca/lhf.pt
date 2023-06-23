use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app_router::Route;
use crate::markdown_page::MarkdownPage;

pub struct PostsRouter {
    slug_to_path: HashMap<String, String>,
    tag_to_slugs: HashMap<String, Vec<String>>,
}

impl PostsRouter {
    pub fn parse_index(index: &String) -> Result<Self, String> {
        let mut slug_to_path = HashMap::new();
        let mut tag_to_slugs = HashMap::new();

        for post in index.lines() {
            let parts = post.split(":").collect::<Vec<&str>>();

            slug_to_path.insert(
                parts
                    .get(1)
                    .ok_or("Invalid post index: missing slug")?
                    .to_string(),
                parts
                    .get(0)
                    .ok_or("Invalid post index: missing path")?
                    .to_string(),
            );

            let tags = parts.get(2..parts.len()).unwrap_or_default();
            for tag in tags {
                let tag = tag.trim();
                if tag.is_empty() {
                    continue;
                }

                let slugs = tag_to_slugs.entry(tag.to_string()).or_insert_with(Vec::new);
                slugs.push(
                    parts
                        .get(1)
                        .ok_or("Invalid post index: missing slug")?
                        .to_string(),
                );
            }
        }

        Ok(Self {
            slug_to_path,
            tag_to_slugs,
        })
    }

    fn get_path(&self, slug: &str) -> Option<String> {
        self.slug_to_path
            .get(slug)
            .map(|path| "post/".to_string() + path)
    }

    pub fn view_post(&self, slug: &str) -> Html {
        match self.get_path(slug) {
            Some(path) => html! { <MarkdownPage md={ path } /> },
            None => html! { <h1>{ "404" }</h1> },
        }
    }

    pub fn view_index(&self, tag: Option<&str>) -> Html {
        let slugs = match tag {
            Some(tag) => self.tag_to_slugs.get(tag).cloned().unwrap_or_default(),
            None => self.slug_to_path.keys().cloned().collect(),
        };

        let title = match tag {
            Some(tag) => format!("Posts tagged {}", tag),
            None => "Posts".to_string(),
        };

        html! { <>
            <h1>{ title }</h1>
            { for slugs.iter().map(|slug| html! { <div>
                <Link<Route> to={ Route::Post { slug: slug.to_string() }}>{ slug }</Link<Route>>
            </div> })}
        </> }
    }
}
