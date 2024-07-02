use serde::{Deserialize, Serialize};
use std::env;
use std::collections::HashMap;

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TransactionType {
    Buy,
    Sell,
}

fn filter_transactions_by_type(transactions: &[ExchangeTransaction], transaction_type: TransactionType) -> Vec<&ExchangeTransaction> {
    transactions.iter()
        .filter(|&transaction| transaction.transaction_type == transaction_type)
        .collect()
}

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);

    let transactions = vec![
        ExchangeTransaction {
            id: 1,
            user_id: 1,
            token_id: 1,
            transaction_type: TransactionType::Buy,
            amount: 100,
            transaction_date: "2023-08-01".to_string(),
        },
        ExchangeTransaction {
            id: 2,
            user_id: 1,
            token_id: 2,
            transaction_type: TransactionType::Sell,
            amount: 150,
            transaction_date: "2023-08-02".to_string(),
        },
    ];

    let buy_transactions = filter_transactions_by_type(&transactions, TransactionType::Buy);
    println!("Buy Transactions: {:?}", buy_transactions);

    let sell_transactions = filter_transactions_by_type(&transactions, TransactionType::Sell);
    println!("Sell Transactions: {:?}", sell_transactions);
}