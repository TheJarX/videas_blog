pub mod db;
pub mod blog_config;

// Common templates
pub mod templates {
    use askama_actix::Template;

    #[derive(Template)]
    #[template(path = "404.html")]
    pub struct NotFoundTemplate {}

    pub mod filters {
        pub fn join_options(values: &Vec<Option<String>>, sep: &str) -> askama::Result<String>{
            let joined = values.into_iter()
                .filter_map(|opt| opt.as_ref())
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(sep);
            Ok(joined)
        }
    }
}