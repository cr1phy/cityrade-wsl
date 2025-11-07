use actix_web::{get, App, HttpServer, Responder};
use std::io;

#[get("/")]
async fn index() -> impl Responder {
    "Hello! It's 8:12, and your city grows up every day! Good job!"
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = HttpServer::new(move || App::new().service(index));
    server.bind("0.0.0.0:8080")?.run().await?;
    Ok(())
}
