extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use actix_web::{error, get, http::StatusCode, post, web, App, HttpServer, Responder, Result};
use derive_more::{Display, Error};
use dotenv::dotenv;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io;

// Get Icecream

#[derive(Serialize)]
struct GetIceCreamResponse {
  icecream: String,
}

#[get("/api/icecream/{qty}")]
async fn get_ice_cream(path: web::Path<usize>) -> Result<impl Responder> {
  let qty = path.into_inner();
  let body = GetIceCreamResponse {
    icecream: "🍨".repeat(qty),
  };
  Ok(web::Json(body))
}

// Post Ice Cream

#[derive(Deserialize)]
struct PostIceCreamRequest {
  icecream: String,
}

#[derive(Serialize)]
struct PostIceCreamResponse {
  message: String,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "{}", message)]
struct MyError {
  message: &'static str,
}

impl error::ResponseError for MyError {
  fn status_code(&self) -> StatusCode {
    StatusCode::BAD_REQUEST
  }
}

#[post("/api/icecream")]
async fn post_ice_cream(body: web::Json<PostIceCreamRequest>) -> Result<impl Responder, MyError> {
  let icecream = body.into_inner().icecream;
  let re = Regex::new(r"[🍨]").unwrap();
  let is_match = re.is_match(icecream.as_str());
  if !is_match {
    return Err(MyError {
      message: "Sorry, we only accept 🍨",
    });
  }
  let count = icecream.as_str().chars().count();
  let response = PostIceCreamResponse {
    message: format!("You have {count} 🍨!"),
  };
  Ok(web::Json(response))
}

// Server

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
