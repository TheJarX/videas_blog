use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse, Error};
use diesel::prelude::*;
use videas_blog::db::establish_connection;
use videas_blog::db::models::Post;
use videas_blog::db::schema::posts::dsl::*;

async fn list_posts() -> Result<HttpResponse, Error> {
    let connection = &mut establish_connection();
    let results = posts.select(Post::as_select()).load(connection).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(results))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./wwwroot"))
            .service(web::resource("/").route(web::get().to(list_posts)))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}