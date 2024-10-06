use crate::posts::posts_service;
use super::templates::*;
use crate::templates::NotFoundTemplate;
use actix_web::{get, web, HttpResponse, Responder};
use askama_actix::TemplateToResponse;
use log::{info};

const NO_SLUG: &str = "NO_SLUG";

#[get("/")]
#[doc(hidden)]
pub async fn index() -> impl Responder {
    let posts_ = posts_service::all();
    info!("Rendering index with {} posts", posts_.len());
    IndexTemplate { posts: &posts_ }.to_response()
}

#[get("/post/{slug}")]
#[doc(hidden)]
pub async fn show_post(path: web::Path<String>) -> impl Responder {
    let slug = path.into_inner();

    if slug.eq(NO_SLUG) {
        return log_and_render_not_found("Default slug was found!");
    }
    if let Ok(post_) = posts_service::one_by_slug(&slug) {
        ShowTemplate { post: &post_ }.to_response()
    } else {
        log_and_render_not_found("Not found")
    }
}

fn log_and_render_not_found(message: &str) -> HttpResponse {
    info!("{}; rendering 404", message);
    NotFoundTemplate {}.to_response()
}
