use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root))
        .bind("127.0.0.1:8085")?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("This server is running")
}
