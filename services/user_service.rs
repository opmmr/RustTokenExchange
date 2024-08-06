[dependencies]
serde = { version = "1.0", features = ["derive"] }
dotenv = "0.15.0"
bcrypt = "0.10.0"
```
```rust
use std::env;
use serde::{Serialize, Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};

fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn get_hashing_cost() -> u32 {
    env::var("HASHING_COST")
        .unwrap_or_else(|_| DEFAULT_COST.to_string())
        .parse()
        .unwrap_or(DEFAULT_COST)
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    password: String, // This will now store hashed password
    email: String,
}

#[derive(Debug)]
enum UserError {
    HashingError,
    AuthenticationFailed,
    OperationFailed(String),
}

fn hash_password(password: &str) -> Result<String, UserError> {
    hash(password, get_hashing_cost()).map_err(|_| UserError::HashingError)
}

fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap_or(false)
}

fn register_user(mut user: User) -> Result<User, UserError> {
    user.password = hash_password(&user.password)?;
    // Placeholder for actual database insertion
    Ok(user)
}

fn authenticate_user(username: &str, password: &str) -> Result<User, UserError> {
    // Placeholder for fetching user from a database
    if username == "existing_user" && verify_password(password, "$2b$12$somehashedpasswordhere") {
        Ok(User { 
            id: 1, 
            username: username.to_string(), 
            password: "$2b$12$somehashedpasswordhere".to_string(), // This would be fetched from the database
            email: "user@example.com".to_string() 
        })
    } else {
        Err(UserError::AuthenticationFailed)
    }
}

fn update_user_profile(user_id: u64, email: String) -> Result<User, UserError> {
    // Placeholder for actual database update operation
    Ok(User { 
        id: user_id, 
        username: "existing_user".to_string(), 
        password: "hashed_password_here".to_string(), 
        email 
    })
}

fn main() {
    dotenv::dotenv().ok();

    let new_user = User {
        id: 0,
        username: "new_user".to_string(),
        password: "password123".to_string(),
        email: "new_user@example.com".to_string(),
    };

    match register_user(new_user) {
        Ok(user) => println!("User registered: {}", user.username),
        Err(e) => println!("Registration failed: {:?}", e),
    }

    match authenticate_user("existing_user", "password123") {
        Ok(user) => println!("User authenticated: {}", user.username),
        Err(e) => println!("Authentication failed: {:?}", e),
    }

    match update_user_profile(1, "new_email@example.com".to_string()) {
        Ok(user) => println!("User email updated to: {}", user.email),
        Err(e) => println!("Failed to update profile: {:?}", e),
    }
}