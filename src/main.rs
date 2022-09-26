mod controllers;

use actix_web::{web, App, HttpServer};
use controllers::signal_controller::SignalController;

fn signal_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register/{phone}").route(web::get().to(SignalController::register)),
    )
    .service(
        web::resource("/register/captcha/{phone}")
            .route(web::get().to(SignalController::register_captcha)),
    )
    .service(web::resource("/verify/{phone}").route(web::get().to(SignalController::verify_code)));
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").configure(signal_config)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
