use std::env;
use blockchain::Blockchain;
use once_cell::sync::Lazy;

mod blockchain;

pub static BLOCKCHAIN: Lazy<Blockchain> = Lazy::new(|| {
    let initial_difficulty = env::var("INITIAL_DIFFICULTY")
        .unwrap_or_else(|_| "1".to_string()) // Assuming a default value
        .parse::<u32>()
        .expect("INITIAL_DIFFICULTY must be an unsigned integer");

    let mining_reward = env::var("MINING_REWARD")
        .unwrap_or_else(|_| "0.0".to_string()) // Assuming a default value
        .parse::<f32>()
        .expect("MINING_REWARD must be a floating point number");

    Blockchain::new(initial_difficulty, mining_reward)
});

fn init() {
    dotenv::dotenv().ok();

    let _ = &*BLOCKCHAIN;
    println!("Blockchain module initialized with the following parameters:");
    println!("Initial Difficulty: {:?}", BLOCKCHAIN.get_difficulty());
    println!("Mining Reward: {:?}", BLOCKCHAIN.get_mining_reward());
}

fn main() {
    init();
}