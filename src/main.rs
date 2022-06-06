use actix_web::{get, web, App, HttpServer, Responder};
use std::process::Command;

#[get("/register/{phone}")]
async fn greet(phone: web::Path<String>) -> impl Responder {
    let command_output = Command::new("docker")
            .args(["exec", "-i", "signal-cli" , "signal-cli", "-a", &phone, "register"])
            .output()
            .unwrap();

    println!("{:?}", command_output);

    format!("Registered {}!", phone)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}