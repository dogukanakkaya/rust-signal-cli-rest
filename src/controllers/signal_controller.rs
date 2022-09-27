use actix_web::{web, Responder};
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use uuid::Uuid;
pub struct SignalController {}

impl SignalController {
    pub async fn register(phone: web::Path<String>) -> impl Responder {
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

    pub async fn register_captcha(
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

    pub async fn link_device(name: web::Path<String>) -> impl Responder {
        let mut command = command_factory();

        let command_output = command
            .arg("signal-cli")
            .arg("link")
            .arg("-n")
            .arg(&name.as_ref())
            .output()
            .unwrap();

        let qr_id = Uuid::new_v4();
        let path = format!("qrcodes/qrcode-{}.png", qr_id);

        let code = QrCode::new(&command_output.stdout).unwrap();

        let image = code.render::<image::Luma<u8>>().build();

        image.save(format!("qrcodes/qrcode-{}.png", qr_id)).unwrap();

        web::Json(LinkDeviceResponse { path })
    }

    // @todo: later change to post method and get info data from post body instead of query string
    pub async fn send(phone: web::Path<String>, info: web::Query<SendInfo>) -> impl Responder {
        let mut command = command_factory();

        command
            .arg("signal-cli")
            .arg("-a")
            .arg(&phone.as_ref())
            .arg("send")
            .arg("-m")
            .arg(&info.message)
            .arg(&info.recipient);

        let command_output = command.output().unwrap();

        println!("{:?}", command_output);

        format!(
            "Send called -> phone: {}, message: {}, recipient: {}",
            phone, info.message, info.recipient
        )
    }
}

fn command_factory() -> Command {
    let mut command = Command::new("docker");
    command.arg("exec").arg("-i").arg("signal-cli");
    command
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

#[derive(Serialize)]
struct LinkDeviceResponse {
    path: String,
}
