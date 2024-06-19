use actix_web::{web, App, HttpServer, Responder, Result};
use dotenv::dotenv;
use std::env;

async fn exchange_tokens() -> impl Responder {
    format!("Token exchange endpoint")
}

async fn user_interaction() -> impl Responder {
    format!("User interaction endpoint")
}

fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/exchange_tokens", web::get().to(exchange_tokens))
            .route("/user_interaction", web::get().to(user_interaction)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new().configure(config_services) // Modularizing route configuration
    })
    .bind(&server_url)?
    .run()
    .await
}