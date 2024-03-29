use crate::controllers::signal_controller::SignalController;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/register/{phone}").route(web::get().to(SignalController::register)),
    )
    .service(
        web::resource("/register/captcha/{phone}")
            .route(web::get().to(SignalController::register_captcha)),
    )
    .service(web::resource("/verify/{phone}").route(web::get().to(SignalController::verify_code)))
    .service(web::resource("/link/{name}").route(web::get().to(SignalController::link_device)))
    .service(
        web::resource("/trust-unsafe/{phone}").route(web::get().to(SignalController::trust_unsafe)),
    )
    .service(web::resource("/send/{phone}").route(web::post().to(SignalController::send)));
}
