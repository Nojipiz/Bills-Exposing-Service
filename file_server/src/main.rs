extern crate ini;
mod persistence;

use actix_web::{
    dev::HttpResponseBuilder, get, http::StatusCode, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder,
};
use persistence::{get_bill_by_number, get_settings, Settings};

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
    let read_process: Result<Vec<u8>, Error> =
        get_bill_by_number(settings.path, bill_id, period, settings.extension);
    match read_process {
        Ok(file) => HttpResponse::Ok().body(file),
        Err(error) => HttpResponseBuilder::new(StatusCode::NOT_FOUND).body(error.to_string()),
    }
}
