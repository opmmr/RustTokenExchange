use std::env;
use serde::{Serialize, Deserialize};

fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    password: String,
    email: String,
}

fn register_user(user: User) -> Result<User, String> {
    Ok(user)
}

fn authenticate_user(username: &str, password: &str) -> Result<User, String> {
    if password == "secret" {
        Ok(User { id: 1, username: username.to_string(), password: password.to_string(), email: "user@example.com".to_string() })
    } else {
        Err("Authentication failed".to_string())
    }
}

fn update_user_profile(user_id: u64, email: String) -> Result<User, String> {
    Ok(User { 
        id: user_id, 
        username: "existing_user".to_string(), 
        password: "hashed_password".to_string(), 
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
        Err(e) => println!("Registration failed: {}", e),
    }

    match authenticate_user("new_user", "password123") {
        Ok(user) => println!("User authenticated: {}", user.username),
        Err(e) => println!("Authentication failed: {}", e),
    }

    match update_user_profile(1, "new_email@example.com".to_string()) {
        Ok(user) => println!("User email updated to: {}", user.email),
        Err(e) => println!("Failed to update profile: {}", e),
    }
}