use ethcontract::prelude::*;
use std::{env, fs, path::Path};

#[derive(Clone)]
pub struct RustTokenExchange {
    web3: Web3<Http>,
    contract: Contract<Http>,
}

impl RustTokenExchange {
    pub async fn new() -> Self {
        dotenv::dotenv().ok();
        let http = Http::new(&env::var("NODE_URL").expect("NODE_URL must be set"))
            .expect("Error creating HTTP transport");
        let web3 = Web3::new(http);
        let contract_abi_path = Path::new(&env::var("CONTRACT_ABI_PATH").expect("CONTRACT_ABI_PATH must be set"));
        let contract_abi = fs::read_to_string(contract_abi_path)
            .expect("Could not load contract ABI");
        let contract_address = env::var("CONTRACT_ADDRESS")
            .expect("CONTRACT_ADDRESS must be set")
            .parse::<Address>()
            .expect("Failed to parse CONTRACT_ADDRESS");
        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            contract_abi.as_bytes(),
        )
        .expect("Error creating contract instance");

        RustTokenExchange { web3, contract }
    }

    pub async fn deploy(&self) -> Result<TxHash, Box<dyn std::error::Error>> {
        unimplemented!()
    }

    pub async fn transfer_tokens(&self, to: Address, value: U256) -> Result<TxHash, Box<dyn std::error::Error>> {
        self.contract
            .method::<_, H256>("transfer", (to, value))
            .expect("transfer function not found")
            .send()
            .await
            .map_err(Into::into)
    }

}

#[tokio::main]
async fn main() {
    let exchange = RustTokenExchange::new().await;
}