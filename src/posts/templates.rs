use crate::templates::filters;
use askama_actix::Template;
use crate::db::models::*;
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub posts: &'a [Post],
}

#[derive(Template)]
#[template(path = "posts/show.html")]
pub struct ShowTemplate<'a> {
    pub post: &'a Post,
}