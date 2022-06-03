extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use std::io;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv().ok();

  let host = dotenv!("BINDING_HOST");
  let port = dotenv!("BINDING_PORT");

  HttpServer::new(|| App::new().service(hello))
    .bind(format!("{host}:{port}"))?
    .run()
    .await
}
