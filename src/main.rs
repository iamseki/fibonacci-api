use actix_web::*;

mod routes;
use routes::{ping, root};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(root))
        .route("/ping", web::get().to(ping))
    });

    let port = 8080;
    let server = api.bind(format!("127.0.0.1:{}",port))
        .expect("something went wrong on start up bind api...");

    println!("Hello, world connected! HTTP server exposed at localhost:{}", port);

    server.run().await
}
