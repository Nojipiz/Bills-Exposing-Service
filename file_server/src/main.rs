extern crate ini;
mod persistence;

use actix_web::{
    error::ContentTypeError, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use persistence::{get_bill_by_number, get_settings, Settings};
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings: Settings = get_settings();
    HttpServer::new(|| App::new().service(root).service(get_bill_file))
        .bind(settings.address + ":" + &settings.port)?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("This server is running")
}

#[get("/bills/{year}/{id}")]
async fn get_bill_file(req: HttpRequest) -> impl Responder {
    let bill_id: u32 = req.match_info().query("id").parse().unwrap();
    let year: u32 = req.match_info().query("year").parse().unwrap();
    let settings: Settings = get_settings();
    let file: Vec<u8> =
        get_bill_by_number(settings.path, bill_id, year, settings.extension).unwrap();
    HttpResponse::Ok().body(file)
}
