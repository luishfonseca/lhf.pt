use serde_json::Value;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app_router::Route;
use crate::markdown_page::MarkdownPage;

pub struct PostsRouter {
    posts: HashMap<String, Vec<String>>,
}

impl PostsRouter {
    pub fn parse_index(value: &Value) -> Result<Self, String> {
        let root = value
            .as_array()
            .ok_or("Expected array")?
            .get(0)
            .ok_or("Expected index to have root as first element")?
            .as_object()
            .ok_or("Expected root to be an object")?
            .get("contents")
            .ok_or("Expected root to have contents")?
            .as_array()
            .ok_or("Expected contents to be an array")?;

        let posts = parse_index(root.to_vec(), None)?;

        log::debug!("Built posts index: {:#?}", posts);

        Ok(Self { posts })
    }

    fn get_path(&self, slug: &str) -> Option<String> {
        self.posts.get(slug).map(|atoms| {
            let mut path = "/content/posts".to_string();

            for atom in atoms {
                path.push('/');
                path.push_str(atom);
            }

            path
        })
    }

    pub fn view_post(&self, slug: &str) -> Html {
        match self.get_path(slug) {
            Some(path) => html! { <MarkdownPage path ={path} /> },
            None => html! { <h1>{ "404" }</h1> },
        }
    }

    pub fn view_index(&self) -> Html {
        html! { <>
            <h1>{ "Posts Index" }</h1>
            { for self.posts.keys().map(|slug| {
                html! { <div>
                    <Link<Route> to={Route::Post { slug: slug.to_string() }}>{ slug }</Link<Route>>
                </div> }
            })}
        </> }
    }
}

fn parse_index(
    values: Vec<Value>,
    atoms: Option<Vec<String>>,
) -> Result<HashMap<String, Vec<String>>, String> {
    let atoms = atoms.unwrap_or_default();
    let mut map = HashMap::new();

    for value in values {
        let value_type = value
            .get("type")
            .ok_or("Missing type")?
            .as_str()
            .ok_or("Expected type to be a str")?;

        let value_name = value
            .get("name")
            .ok_or("Missing name")?
            .as_str()
            .ok_or("Expected name to be a str")?;

        match value_type {
            "directory" => {
                let mut atoms = atoms.clone();
                atoms.push(value_name.to_string());

                let contents = value
                    .get("contents")
                    .ok_or("Missing contents")?
                    .as_array()
                    .ok_or("Expected contents to be an array")?;

                let map2 = parse_index(contents.to_vec(), Some(atoms))?;

                map.extend(map2);
            }
            "file" => {
                let slug = match atoms.len() {
                    0 => value_name.trim_end_matches(".md").to_string(),
                    _ => {
                        let mut atoms = atoms.clone();
                        atoms.push(value_name.trim_end_matches(".md").to_string());
                        atoms.join("-")
                    }
                };

                let mut atoms = atoms.clone();
                atoms.push(value_name.to_string());

                map.insert(slug, atoms);
            }
            _ => return Err("Unexpected type".to_string()),
        }
    }

    Ok(map)
}
