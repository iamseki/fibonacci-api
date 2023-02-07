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
