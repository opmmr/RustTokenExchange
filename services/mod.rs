use dotenv::dotenv;
use std::env;

mod services;

fn main() {
    dotenv().ok();

    let database_connection_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");
    let external_api fast_key = env::var("API_KEY")
        .expect("API_KEY must be set in the .env file");

    services::initialize_services(&database_connection_url, &external_api_key);
}