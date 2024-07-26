use std::env;

struct RustTokenExchange {
    exchange_rate: f64,
}

impl RustTokenExchange {
    fn new() -> Self {
        let exchange_rate: f64 = env::var("EXCHANGE_RATE")
            .expect("EXCHANGE_RATE not set in .env file.")
            .parse()
            .expect("EXCHANGE_RATE must be a floating point number");
        RustTokenExchange { exchange_rate }
    }

    fn calculate_exchange_amount(&self, tokens: f64) -> f64 {
        tokens * self.exchange_rate
    }

    fn process_transaction(&self, sender: &str, receiver: &str, amount: f64) {
        println!("Processing transaction: {} -> {} [{} tokens]", sender, receiver, amount);
        let exchanged_amount = self.calculate_exchange_amount(amount);
        println!("Exchanged amount: {:.2} at rate: {:.2}", exchanged_amount, self.exchange_rate);
    }
}

fn main() {
    dotenv::dotenv().ok();
    let exchange_service = RustTokenExchange::new();

    let sender = "Alice";
    let receiver = "Bob";
    let tokens = 100.0;

    exchange_service.process_transaction(sender, receiver, tokens);
}