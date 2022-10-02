use crate::commands::signal_cli;
use actix_web::{
    http::{header::ContentType, StatusCode},
    web, HttpResponse, Responder,
};
use qrcode::QrCode;
use serde::Deserialize;
use std::{
    io::{BufRead, BufReader},
    process::{Output, Stdio},
};
use uuid::Uuid;

pub struct SignalController {}

impl SignalController {
    pub async fn register(phone: web::Path<String>) -> impl Responder {
        let output = signal_cli::command()
            .arg("signal-cli")
            .arg("-a")
            .arg(&phone.as_ref())
            .arg("register")
            .output()
            .unwrap();

        Self::respond_output(output)
    }

    pub async fn register_captcha(
        phone: web::Path<String>,
        info: web::Query<RegisterCaptchaInfo>,
    ) -> impl Responder {
        let output = signal_cli::command()
            .arg("signal-cli")
            .arg("-a")
            .arg(&phone.as_ref())
            .arg("register")
            .arg("--captcha")
            .arg(&info.token)
            .output()
            .unwrap();

        Self::respond_output(output)
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

        let output = command.output().unwrap();

        Self::respond_output(output)
    }

    pub async fn link_device(name: web::Path<String>) -> impl Responder {
        let mut output = signal_cli::command()
            .arg("signal-cli")
            .arg("link")
            .arg("-n")
            .arg(&name.as_ref())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = output.stdout.take().unwrap();

        let mut bufread = BufReader::new(stdout);
        let mut buf = String::new();

        while let Ok(n) = bufread.read_line(&mut buf) {
            if n > 0 {
                let qr_data = buf.trim();
                let qr_id = Uuid::new_v4();

                let code = QrCode::new(qr_data).unwrap();
                let image = code.render::<image::Luma<u8>>().build();

                // FIXME: image.to_vec() is not resulting with the expected output so i save the image and then read it again
                // uuid will not be needed after that since i don't have to save the image
                let path = format!("qrcodes/qrcode-{}.png", qr_id); // this variable will not be needed after above FIXME
                image.save(&path).unwrap();
                let body = std::fs::read(&path).unwrap();
                std::fs::remove_file(&path).unwrap();

                buf.clear();

                return HttpResponse::build(StatusCode::OK)
                    .content_type("image/jpeg")
                    .body(body);
            } else {
                break;
            }
        }

        let _ = output.wait();

        HttpResponse::InternalServerError().finish()
    }

    pub async fn trust_unsafe(phone: web::Path<String>) -> impl Responder {
        let output = signal_cli::command()
            .arg("signal-cli")
            .arg("trust")
            .arg("-a")
            .arg(&phone.as_ref())
            .output()
            .unwrap();

        Self::respond_output(output)
    }

    pub async fn send(phone: web::Path<String>, info: web::Json<SendInfo>) -> impl Responder {
        let output = signal_cli::command()
            .arg("signal-cli")
            .arg("-a")
            .arg(&phone.as_ref())
            .arg("send")
            .arg("-m")
            .arg(&info.message)
            .arg(&info.recipient)
            .output()
            .unwrap();

        Self::respond_output(output)
    }

    fn respond_output(output: Output) -> HttpResponse {
        if output.stderr.is_empty() == false {
            let stderr = std::str::from_utf8(&output.stderr).unwrap();
            if stderr.starts_with("INFO") == false {
                return Self::stderr_response(&stderr);
            }
        }

        HttpResponse::NoContent().finish()
    }

    fn stderr_response(stderr: &str) -> HttpResponse {
        HttpResponse::build(StatusCode::BAD_REQUEST)
            .content_type(ContentType::json())
            .body(format!(
                r#"
                    {{
                        "error": {{
                            "reason": "{}"
                        }}
                    }}
                "#,
                stderr
            ))
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

#[derive(Deserialize)]
pub struct SendInfo {
    recipient: String,
    message: String,
}
