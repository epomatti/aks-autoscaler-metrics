extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::io;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv().ok();
  let hostKey = "HOST";
  match env::var(hostKey) {
    Ok(val) => let host = val,
    Err(e) => println!("couldn't interpret {hostKey}: {e}"),
  }

  let port = env::var("PORT");

  println!(host);

  let bind = format!("{}:{}", host, port);

  HttpServer::new(|| App::new().service(hello))
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
