extern crate ini;
mod persistence;

use actix_web::{
    dev::HttpResponseBuilder, get, http::StatusCode, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder, Result,
};
use persistence::{get_bill_in_base64, get_settings, Settings};

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
    let period: u32 = req.match_info().query("year").parse().unwrap();
    let settings: Settings = get_settings();
    let read_process: Result<String, Error> =
        get_bill_in_base64(settings.path, bill_id, period, settings.extension);
    match read_process {
        Ok(encoded_file) => HttpResponse::Ok().body(encoded_file), //CHANGE
        Err(error) => HttpResponseBuilder::new(StatusCode::NOT_FOUND).body(error.to_string()),
    }
}
