use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExchangeTransaction {
    pub id: u32,
    pub user_id: u32,
    pub token_id: u32,
    pub transaction_type: TransactionType,
    pub amount: u64,
    pub transaction_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    Buy,
    Sell,
}

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);
}