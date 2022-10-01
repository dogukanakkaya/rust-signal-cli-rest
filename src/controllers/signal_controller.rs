use crate::commands::signal_cli;
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader},
    process::Stdio,
};
use uuid::Uuid;

pub struct SignalController {}

impl SignalController {
    pub async fn register(phone: web::Path<String>) -> impl Responder {
        let command_output = signal_cli::command()
            .arg("signal-cli")
            .arg("-a")
            .arg(&phone.as_ref())
            .arg("register")
            .output()
            .unwrap();

        println!("{:?}", command_output);

        format!("Register Captcha called -> phone: {}", phone)
    }

    pub async fn register_captcha(
        phone: web::Path<String>,
        info: web::Query<RegisterCaptchaInfo>,
    ) -> impl Responder {
        let command_output = signal_cli::command()
            .arg("signal-cli")
            .arg("-a")
            .arg(&phone.as_ref())
            .arg("register")
            .arg("--captcha")
            .arg(&info.token)
            .output()
            .unwrap();

        println!("{:?}", command_output);

        if command_output.stderr.is_empty() == false {
            format!(
                "An error occured: {}",
                std::str::from_utf8(&command_output.stderr).unwrap()
            )
        } else {
            format!(
                "Register Captcha called with phone: {}, token: {}",
                phone, info.token
            )
        }
    }

    pub async fn verify_code(
        phone: web::Path<String>,
        info: web::Query<VerifyInfo>,
    ) -> impl Responder {
        let mut command = signal_cli::command();

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

    pub async fn link_device(name: web::Path<String>) -> impl Responder {
        let mut command = signal_cli::command();

        let mut command_output = command
            .arg("signal-cli")
            .arg("link")
            .arg("-n")
            .arg(&name.as_ref())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = command_output.stdout.take().unwrap();

        let mut bufread = BufReader::new(stdout);
        let mut buf = String::new();

        while let Ok(n) = bufread.read_line(&mut buf) {
            if n > 0 {
                let qr_data = buf.trim();
                let qr_id = Uuid::new_v4();

                let code = QrCode::new(qr_data).unwrap();
                let image = code.render::<image::Luma<u8>>().build();

                // FIXME: image.to_vec() is not resulting with the expected output so i save the image and then read it again
                let path = format!("qrcodes/qrcode-{}.png", qr_id); // this variable will not be needed after above FIXME
                image.save(&path).unwrap();
                let body = std::fs::read(&path).unwrap();
                std::fs::remove_file(&path).unwrap();

                buf.clear();

                return HttpResponse::build(StatusCode::OK)
                    .content_type("image/jpeg")
                    .body(body);
            } else {
                return HttpResponse::Ok().content_type("plain/text").body("data");
            }
        }

        let _ = command_output.wait();

        HttpResponse::InternalServerError().finish()
    }

    pub async fn trust_unsafe(phone: web::Path<String>) -> impl Responder {
        let mut command = signal_cli::command();

        let command_output = command
            .arg("signal-cli")
            .arg("trust")
            .arg("-a")
            .arg(&phone.as_ref())
            .output()
            .unwrap();

        println!("{:?}", command_output);

        format!("Trust called -> phone: {}", phone)
    }

    pub async fn send(phone: web::Path<String>, info: web::Json<SendInfo>) -> impl Responder {
        let info = info.into_inner();

        let mut command = signal_cli::command();

        command
            .arg("signal-cli")
            .arg("-a")
            .arg(&phone.as_ref())
            .arg("send")
            .arg("-m")
            .arg(&info.message)
            .arg(&info.recipient);

        // @todo: later add logger for this
        let _ = command.output().unwrap();

        web::Json(SuccessResponse::<SendInfo> {
            data: SendInfo {
                recipient: info.recipient,
                message: info.message,
            },
        })
    }
}

#[derive(Deserialize)]
pub struct RegisterCaptchaInfo {
    token: String,
}

#[derive(Deserialize)]
pub struct VerifyInfo {
    code: String,
    pin: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SendInfo {
    recipient: String,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct SuccessResponse<T> {
    data: T,
}
