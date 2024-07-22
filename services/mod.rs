[dependencies]
dotenv = "0.15.0"
log = "0.4"
env_logger = "0.9"
```

```rust
use dotenv::dotenv;
use log::{info, warn};
use std::env;

mod services;

fn main() {
    dotenv().ok();
    env_logger::init();

    let database_connection_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");
    info!("Successfully retrieved DATABASE_URL from environment.");

    let external_api_key = env::var("API_KEY")
        .expect("API_KEY must be set in the .env file");
    info!("Successfully retrieved API_KEY from environment.");

    info!("Initializing services...");
    if let Err(e) = services::initialize_services(&database_connection_cutoff, &external_api_key) {
        warn!("An error occurred during service initialization: {}", e);
    }
}