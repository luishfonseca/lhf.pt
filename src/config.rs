use lazy_static::lazy_static;

pub struct Config {
    pub dev: bool,
    pub posts_index_path: &'static str,
    pub posts_prefix: &'static str,
    pub content_source_url: &'static str,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

impl Config {
    pub fn new() -> Self {
        let dev = option_env!("CARGO_PROFILE").unwrap_or("unknown") == "dev";
        Self {
            dev,

            posts_index_path: "posts.index.txt",
            posts_prefix: "posts/",

            content_source_url: if dev {
                "http://localhost:5003/"
            } else {
                "https://cdn.jsdelivr.net/gh/luishfonseca/lhf.pt-content/content/"
            },
        }
    }
}
