use std::env;
use blockchain::Blockchain;
use once_cell::sync::Lazy;

mod blockchain;

pub static BLOCKCHAIN: Lazy<Blockchain> = Lazy::new(|| {
    let initial_difficulty = env::var("INITIAL_DIFFICULTY")
        .expect("Expected INITIAL_DIFFICULTY in .env")
        .parse::<u32>()
        .expect("INITIAL_DIFFICULTY must be an unsigned integer");

    let mining_reward = env::var("MINING_REWARD")
        .expect("Expected MINING_REWARD in .env")
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