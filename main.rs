use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use serde_json::json;
use actix_web::{middleware, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ExchangeStats {
    num_requests: u64,
}

static mut EXCHANGE_STATS: ExchangeStats = ExchangeStats { num_requests: 0 };

async fn exchange_tokens() -> Result<impl Responder, Error> {
    unsafe {
        EXCHANGE_STATS.num_requests += 1;
    }

    Ok(HttpResponse::Ok().json(json!({
        "message": "Token exchange endpoint",
        "stats": EXCHANGE_STATS,
    })))
}

async fn user_interaction() -> Result<impl Responder, Error> {
    Ok(format!("User interaction endpoint"))
}

async fn stats() -> Result<impl Responder, Error> {
    let stats;
    unsafe {
        stats = json!({
            "num_requests": EXCHANGE_TS.num_requests,
        });
    }
    
    Ok(HttpResponse::Ok().json(stats))
}

fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/exchange_tokens", web::get().to(exchange_tokens))
            .route("/user_interaction", web::get().to(user_interaction))
            .route("/stats", web::get().to(stats)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(config_services)
    })
    .bind(&server_url)?
    .run()
    .await
}