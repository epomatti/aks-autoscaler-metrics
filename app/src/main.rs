extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

use actix_web::{error, get, http::StatusCode, post, web, App, HttpServer, Responder, Result};
use derive_more::{Display, Error};
use dotenv::dotenv;
use factorial::Factorial;
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

// Factorial

#[derive(Serialize)]
struct GetIceCreamFactorialResponse {
  icecreams: String,
}

#[get("/api/icecream/factorial/{qty}")]
async fn get_ice_cream_factorial(path: web::Path<String>) -> Result<impl Responder> {
  let qty: String = path.into_inner();
  let qty_num: u128 = qty.parse().unwrap();
  let product = qty_num.factorial();
  let results = product.to_string();
  let body = GetIceCreamFactorialResponse { icecreams: results };
  Ok(web::Json(body))
}

#[derive(Serialize)]
struct GetIceCreamFibonacciResponse {
  icecreams: String,
}

#[get("/api/fibonacci/{nstr}")]
async fn get_fibonacci(path: web::Path<String>) -> Result<impl Responder> {
  let nstr: String = path.into_inner();
  let n: u128 = nstr.parse().unwrap();
  let product = fibonacci(n);
  let results = product.to_string();
  let body = GetIceCreamFactorialResponse { icecreams: results };
  Ok(web::Json(body))
}

pub fn fibonacci(n: u128) -> u128 {
  match n {
    0 => 1,
    1 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

// Server

#[actix_web::main]
async fn main() -> io::Result<()> {
  dotenv().ok();

  let host = dotenv!("BINDING_HOST");
  let port = dotenv!("BINDING_PORT");

  HttpServer::new(|| {
    App::new()
      .service(get_ice_cream)
      .service(post_ice_cream)
      .service(get_ice_cream_factorial)
      .service(get_fibonacci)
  })
  .bind(format!("{host}:{port}"))?
  .run()
  .await
}

#[cfg(test)]
mod main_tests {

  use super::*;

  #[test]
  fn fibonacci_test() {
    let f = fibonacci(9);
    assert_eq!(f, 55)
  }
}
