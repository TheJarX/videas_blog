pub mod db;
pub mod blog_config;
pub mod posts;

// Common templates
pub mod templates {
    use askama_actix::Template;

    #[derive(Template)]
    #[template(path = "404.html")]
    pub struct NotFoundTemplate {}

    pub mod filters {
        use chrono::{DateTime, NaiveDateTime, Utc};

        pub fn join_options(values: &Vec<Option<String>>, sep: &str) -> askama::Result<String>{
            let joined = values.into_iter()
                .filter_map(|opt| opt.as_ref())
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .join(sep);
            Ok(joined)
        }

        pub fn parse_and_format_date(naive_date: &NaiveDateTime) ->askama::Result<String> {
            let datetime_utc = DateTime::<Utc>::from_naive_utc_and_offset(*naive_date, Utc);
            Ok(datetime_utc.format("%d.%m.%Y").to_string())
        }
    }
}