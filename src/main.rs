mod commands;
mod controllers;
mod routes;

use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::signal;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(signal::config))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
