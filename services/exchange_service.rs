use dotenv::dotenv;
use std::borrow::Cow;
use std::env;
use std::num::ParseFloatError;

struct RustTokenExchange {
    exchange_rate: f64,
}

enum ExchangeError {
    EnvVarMissing(Cow<'static, str>),
    ParseError(ParseFloatError),
}

impl From<ParseFloatError> for ExchangeError {
    fn from(err: ParseFloatError) -> Self {
        ExchangeError::ParseError(err)
    }
}

impl RustTokenExchange {
    fn new() -> Result<Self, ExchangeError> {
        let exchange_rate_str = env::var("EXCHANGE_RATE")
            .map_err(|_| ExchangeError::EnvVarMissing("EXCHANGE_RATE not set in .env file.".into()))?;
        let exchange_rate: f64 = exchange_rate_str.parse()?;
        Ok(RustTokenExchange { exchange_rate })
    }

    fn calculate_exchange_amount(&self, tokens: f64) -> f64 {
        tokens * self.exchange_rate
    }

    fn process_transaction(&self, sender: &str, receiver: &str, amount: f64) {
        println!(
            "Processing transaction: {} -> {} [{} tokens], Exchanged amount: {:.2} at rate: {:.2}",
            sender, receiver, amount, self.calculate_exchange_amount(amount), self.exchange_rate
        );
    }
}

fn main() {
    dotenv().ok();

    match RustTokenExchange::new() {
        Ok(exchange_service) => {
            let sender = "Alice";
            let receiver = "Bob";
            let tokens = 100.0;
            exchange_service.process_transaction(sender, receiver, tokens);
        }
        Err(error) => match error {
            ExchangeError::EnvVarMissing(message) => eprintln!("Error: {}", message),
            ExchangeError::ParseError(_e) => eprintln!("Error parsing EXCHANGE_RATE from environment. Ensure it's a valid floating point number."),
        },
    }
}