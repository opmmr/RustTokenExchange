use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

#[derive(Deserialize)]
struct ExchangeRequest {
    from: String,
    to: String,
    amount: f32,
}

#[derive(Serialize)]
struct BalanceResponse {
    balance: f32,
}

#[derive(Serialize)]
struct Transaction {
    from: String,
    to: String,
    amount: f32,
    timestamp: String,
}

async fn authenticate_user(info: web::Json<AuthRequest>) -> impl Responder {
    HttpResponse::Ok().json(AuthResponse {
        token: "sample_token".to_string(),
    })
}

async fn exchange_tokens(info: web::Json<ExchangeRequest>) -> impl Responder {
    HttpResponse::Ok().body("Token exchange successful")
}

async fn get_token_balance() -> impl Responder {
    HttpResponse::Ok().json(BalanceResponse { balance: 100.0 })
}

async fn get_transaction_history() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Transaction {
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 50.0,
            timestamp: "2023-01-01T12:00:00Z".to_string(),
        },
        Transaction {
            from: "Charlie".to_string(),
            to: "Dave".to_string(),
            amount: 75.0,
            timestamp: "2023-01-02T15:30:00Z".to_string(),
        },
    ])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| {
        println!("Warning: SERVER_ADDRESS not set in .env file, using default.");
        "127.0.0.1:8080".into()
    });

    HttpServer::new(|| {
        App::new()
            .route("/authenticate", web::post().to(authenticate_user))
            .route("/exchange", web::post().to(exchange_tokens))
            .route("/balance", web::get().to(get_token_balance))
            .route("/transactions", web::get().to(get_transaction_history))
    })
    .bind(&server_address)?
    .run()
    .await
}