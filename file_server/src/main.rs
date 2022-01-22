extern crate ini;

use actix_web::{
    error::ContentTypeError, get, post, web, App, HttpResponse, HttpServer, Responder,
};
use futures::executor::block_on;
use persistence::Settings;
use serde::Serialize;
use std::{fs, future::Future, process::Output};

mod persistence;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root))
        .bind("")?
        .run()
        .await
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("This server is running")
}

fn print_dir() {
    for file in fs::read_dir("$HOME/Downloads").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}
