extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize)]
struct GetIceCreamResponse {
  icecream: String,
}

#[get("/api/icecream/{qty}")]
async fn get_ice_cream(path: web::Path<usize>) -> Result<impl Responder> {
  let qty = path.into_inner();
  let icecream = GetIceCreamResponse {
    icecream: "🍨".repeat(qty),
  };
  Ok(web::Json(icecream))
}

#[post("/")]
async fn post_ice_cream() -> impl Responder {
  HttpResponse::Ok().body("Hello, world!")
}

// async fn index(icecream: web::Json<IceCream>) -> Result<String> {
//   Ok(format!("Welcome {}!", icecream.icecream))
// }

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv().ok();

  let host = dotenv!("BINDING_HOST");
  let port = dotenv!("BINDING_PORT");

  HttpServer::new(|| App::new().service(get_ice_cream).service(post_ice_cream))
    .bind(format!("{host}:{port}"))?
    .run()
    .await
}
