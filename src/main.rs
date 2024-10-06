use actix_files as fs;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use videas_blog::posts::handlers::{index, show_post};

#[get("ping")]
#[doc(hidden)]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/static", "./wwwroot"))
            .service(ping)
            .service(index)
            .service(show_post)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}