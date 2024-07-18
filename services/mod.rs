use dotenv::dotenv;
use std::env;

mod services;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_ID must be set in .env");
    let api_key = env::var("API_KEY").expect("API_KEY must be set in .env");

    services::init_services(&database[Filtered]);
}