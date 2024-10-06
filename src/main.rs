use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use askama_actix::Template;
use diesel::prelude::*;
use videas_blog::db::establish_connection;
use videas_blog::db::models::Post;
use videas_blog::db::schema::posts::dsl::*;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    posts: Vec<Post>,
}

async fn index() -> impl Responder {
    let connection = &mut establish_connection();
    let _posts = posts.load::<Post>(connection).expect("Error loading posts");

    IndexTemplate { posts: _posts }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./wwwroot"))
            .service(web::resource("/").route(web::get().to(index)))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}