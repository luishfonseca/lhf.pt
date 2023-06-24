use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::config::CONFIG;
use crate::markdown_page::MarkdownPage;

pub struct PostMeta {
    title: String,
    path: String,
    date: String,
}

pub struct PostsRouter {
    slug_to_meta: HashMap<String, PostMeta>,
    tag_to_slugs: HashMap<String, Vec<String>>,
}

impl PostsRouter {
    pub fn parse_index(index: &String) -> Result<Self, String> {
        let mut slug_to_meta = HashMap::new();
        let mut tag_to_slugs = HashMap::new();

        for post in index.lines() {
            let (post_meta, title) = post
                .split_once(" ")
                .ok_or("Invalid post index: missing title")?;

            let parts = post_meta.split(":").collect::<Vec<&str>>();
            let date = parts
                .get(0)
                .ok_or("Invalid post index: missing date")?;
            let path = parts
                .get(1)
                .ok_or("Invalid post index: missing path")?;
            let slug = parts
                .get(2)
                .ok_or("Invalid post index: missing slug")?;

            slug_to_meta.insert(
                slug.to_string(),
                PostMeta {
                    title: title.to_string(),
                    path: path.to_string(),
                    date: date.to_string(),
                },
            );

            let tags = parts.get(3..parts.len()).unwrap_or_default();
            for tag in tags {
                let tag = tag.trim();
                if tag.is_empty() {
                    continue;
                }

                let slugs = tag_to_slugs.entry(tag.to_string()).or_insert_with(Vec::new);
                slugs.push(
                    slug.to_string(),
                );
            }
        }

        Ok(Self {
            slug_to_meta,
            tag_to_slugs,
        })
    }

    fn get_path(&self, slug: &str) -> Option<String> {
        self.slug_to_meta
            .get(slug)
            .map(|meta| CONFIG.posts_prefix.to_string() + &meta.path)
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
            None => self.slug_to_meta.keys().cloned().collect(),
        };

        let index_title = match tag {
            Some(tag) => format!("Posts tagged {}", tag),
            None => "Posts".to_string(),
        };

        html! { <>
            <h1>{ index_title }</h1>
            { for slugs.iter().map(|slug| {
                let meta = self.slug_to_meta.get(slug).unwrap();
                html! {
                    <div>
                        { &meta.date }
                        { " - " }
                        <Link<Route> to={ Route::Post { slug: slug.to_string() }}>{ &meta.title }</Link<Route>>
                    </div>
            }})}
        </> }
    }
}
