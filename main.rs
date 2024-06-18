use std::env;
use act consulate_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;

async fn exchange_tokens() -> impl Responder {
    format!("Token exchange endpoint")
}

async fn user_interaction() -> impl Responder {
    format!("User interaction endpoint")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").unwrap_or("127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .route("/exchange_tokens", web::get().to(exchange_tokens))
            .route("/user_interaction", web::get().to(user_interaction))
    })
    .bind(&server_url)?
    .run()
    .await
}