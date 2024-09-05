use ethcontract::prelude::*;
use std::{env, fs, path::Path};
use anyhow::{Result, Context};
use log::{info, warn};

#[derive(Clone)]
pub struct RustTokenExchange {
    web3: Web3<Http>,
    contract: Contract<Http>,
}

impl RustTokenExchange {
    pub async fn new() -> Result<Self> {
        // Initialize logging
        env_logger::init();

        dotenv::dotenv().ok();
        let http = Http::new(&env::var("NODE_URL").context("NODE_URL must be set")?)
            .context("Error creating HTTP transport")?;
        let web3 = Web3::new(http);

        let contract_abi_path = Path::new(&env::var("CONTRACT_ABI_PATH").context("CONTRACT_ABI_PATH must be set")?);
        let contract_abi = fs::read_to_string(contract_abi_path)
            .context("Could not load contract ABI")?;
        let contract_address = env::var("CONTRACT_ADDRESS")
            .context("CONTRACT_ADDRESS must be set")?
            .parse::<Address>()
            .context("Failed to parse CONTRACT_ADDRESS")?;
        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            contract_abi.as_bytes(),
        )
        .context("Error creating contract instance")?;

        info!("RustTokenExchange instance created successfully");
        
        Ok(RustTokenExchange { web3, contract })
    }

    pub async fn deploy(&self) -> Result<TxHash> {
        unimplemented!()
    }

    pub async fn transfer_tokens(&self, to: Address, value: U256) -> Result<TxHash> {
        info!("Attempting to transfer tokens...");
        self.contract
            .method::<_, H256>("transfer", (to, value))
            .context("transfer function not found")?
            .send()
            .await
            .map_err(|err| {
                warn!("Error occurred during token transfer: {}", err);
                err.into()
            })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init(); // Initialize the logger here as well
    info!("Starting RustTokenExchange...");

    let exchange = RustTokenExchange::new().await.context("Failed to create RustTokenExchange")?;
    
    info!("RustTokenExchange started successfully.");

    Ok(())
}