use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;
use std::process::Command;

fn command_factory() -> Command {
    let mut command = Command::new("docker");
    command.arg("exec").arg("-i").arg("signal-cli");
    command
}

#[get("/register/{phone}")]
async fn register(phone: web::Path<String>) -> impl Responder {
    let command_output = command_factory()
        .arg("signal-cli")
        .arg("-a")
        .arg(&phone.as_ref())
        .arg("register")
        .output()
        .unwrap();

    println!("{:?}", command_output);

    format!("Register Captcha called -> phone: {}", phone)
}

#[derive(Deserialize)]
struct RegisterCaptchaInfo {
    token: String,
}

#[get("/register/captcha/{phone}")]
async fn register_captcha(
    phone: web::Path<String>,
    info: web::Query<RegisterCaptchaInfo>,
) -> impl Responder {
    let command_output = command_factory()
        .arg("signal-cli")
        .arg("-a")
        .arg(&phone.as_ref())
        .arg("register")
        .arg("--captcha")
        .arg(&info.token)
        .output()
        .unwrap();

    println!("{:?}", command_output);

    format!(
        "Register Captcha called -> phone: {}, token: {}",
        phone, info.token
    )
}

#[derive(Deserialize)]
struct VerifyInfo {
    code: String,
    pin: Option<String>,
}

#[get("/verify/{phone}")]
async fn verify_code(phone: web::Path<String>, info: web::Query<VerifyInfo>) -> impl Responder {
    let mut command = command_factory();

    command
        .arg("signal-cli")
        .arg("-a")
        .arg(&phone.as_ref())
        .arg("verify")
        .arg(&info.code);

    if let Some(pin) = &info.pin {
        command.arg("--pin").arg(pin);
    }

    let command_output = command.output().unwrap();

    println!("{:?}", command_output);

    format!(
        "Verify called -> phone: {}, code: {}, pin: {:?}",
        phone, info.code, info.pin
    )
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register)
            .service(register_captcha)
            .service(verify_code)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
