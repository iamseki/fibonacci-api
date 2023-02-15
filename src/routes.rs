#[path = "./fibonacci.rs"]
mod fibonacci;

use actix_web::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

pub async fn root() -> HttpResponse {
  HttpResponse::Ok()
  .content_type("application/json; charset=utf-8")
  .body(format!("{{\"version\": \"{}\", \"authors\": \"{}\"}}", VERSION, AUTHORS))
}

pub async fn ping() -> HttpResponse {
  HttpResponse::Ok().body("pong")
}


#[get("/fibonacci/{nth}")]
async fn fibonacci_route(path: web::Path<u32>) -> HttpResponse {
  let nth = path.into_inner();
  let fibonacci_sequence_value = fibonacci::fibonacci_nth(nth);

  HttpResponse::Ok()
  .content_type("application/json")
  .body(format!("{{\"fibonacci_sequence\": \"{}\", \"value\": \"{}\"}}", nth, fibonacci_sequence_value))
}