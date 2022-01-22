extern crate ini;
use actix_web::{
    error::ContentTypeError, get, post, web, App, HttpResponse, HttpServer, Responder,
};
use ini::Ini;
use serde::Serialize;
use std::{fs, future::Future, process::Output};

const LOCAL_ADRRESS: &'static str = "127.0.0.1:";
const PORT: &'static str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut bind_address: String = LOCAL_ADRRESS.to_owned();
    bind_address.push_str(PORT);
    HttpServer::new(|| App::new().service(root))
        .bind(bind_address)?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("This server is running")
}

fn read_config() {
    let content = Ini::load_from_file("./conf.ini").unwrap();
    let server_section = content.section(Some("Server")).unwrap();
    let address = server_section.get("address").unwrap();
    let port = server_section.get("port").unwrap();
    let directory_section = content.section(Some("Directory")).unwrap();
    let path = directory_section.get("path").unwrap();
    print!("{:?}", address);
}

fn print_dir() {
    for file in fs::read_dir("$HOME/Downloads").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}
