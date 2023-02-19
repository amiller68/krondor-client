use anyhow::{Error, Result, anyhow};
use ethers::{
    abi::{Abi, Token, Tokenizable},
    contract::Contract,
    middleware::SignerMiddleware,
    prelude::H256,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, Filter, Log, TransactionRequest, U256},
};
use std::{
    convert::{From, TryFrom},
    path::PathBuf,
    collections::HashMap,
};
use std::thread::sleep;
use ethers_contract_derive::EthEvent;
use lazy_static::lazy_static;
use crate::types::{
    cid::Cid,
    file_object::FileObject,
};
use rand::Rng;

// TODO: Use abigen! macro to generate this
// Load the contract ABI from Hardhat's artifacts
lazy_static! {
    static ref CONTRACT_ABI_STR: &'static str = {
        // TODO (amiller68): Make this configurable by env var
        let contract_abi_ref: &'static str = include_str!("/home/al/krondor-client/contracts/artifacts/contracts/crudFs.sol/CrudFs.json");
        let contract_abi: serde_json::Value = serde_json::from_str(contract_abi_ref).unwrap();
        let contract_abi = contract_abi["abi"].to_string();
        Box::leak(contract_abi.into_boxed_str())
    };
}

/// EthClient - Everything needed to interact with Banyan's Ethereum Stack
pub struct Client {
    /// An Eth Provider. This is required to interact with the Ethereum Blockchain.
    pub provider: Provider<Http>,
    /// The chain ID of the network we're connected to. This is Required for signing transactions.
    pub chain_id: u64,
    /// An (optional) Eth Signer for singing transactions. This is required for interacting with payable functions.
    pub signer: Option<SignerMiddleware<Provider<Http>, LocalWallet>>,
    /// A Deployed Solidity Contract Address. This is required to interact with the Banyan Contract.
    pub contract: Contract<Provider<Http>>,
}

impl Default for Client {
    fn default() -> Self {
        // Get the API URL from the .env file
        let api_url = std::env::var("API_URL").expect("API_URL must be set");
        // Get the API Key from the .env file
        let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
        // Get the Chain ID from the .env file
        let chain_id = std::env::var("CHAIN_ID")
            .expect("CHAIN_ID must be set")
            .parse::<u64>()
            .expect("CHAIN_ID must be a number");
        // Get the Private Key from the .env file
        let private_key = std::env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
        // Get the Contract Address from the .env file
        let contract_address = std::env::var("CONTRACT_ADDRESS")
            .expect("CONTRACT_ADDRESS must be set")
            .parse::<Address>()
            .expect("CONTRACT_ADDRESS must be a valid Ethereum Address");
        // Create a new EthClient
        Client::new(
            api_url,
            api_key,
            Some(chain_id),
            Some(private_key),
            contract_address,
        ).unwrap()
    }
}


// TODO: Very insecure. Use a proper signer
/// The EthProvider is a wrapper around the ethers-rs Provider that handles all Ethereum
/// interactions.
impl Client {
    /// Create a new EthClient - Uses EthClientBuilder::new()
    /// # Arguments
    /// * `api_url` - The URL of the Ethereum API to connect to. This is required to interact with
    ///                 the Ethereum Blockchain.
    /// * `api_key` - The API Key for the Ethereum API. This is required.
    /// * `chain_id` - The (Optional) Chain ID of the network we're connected to.
    ///                 Defaults to 1 (mainnet)
    /// * `private_key` - The (Optional) Private Key for the Ethereum Account we're using to sign.
    ///                 This is required for interacting with payable functions.
    /// * `contract_address` - The (Optional) Deployed Solidity Contract Address to interact with.
    pub fn new(
        api_url: String,
        api_key: String,
        chain_id: Option<u64>,
        private_key: Option<String>,
        contract_address: Address,
    ) -> Result<Client, Error> {
        // Determine an API URL and Initialize the Provider
        let url = format!("{}/{}", api_url, api_key);
        let provider = Provider::<Http>::try_from(url).expect("Failed to create provider");

        // Get the Chain ID. If None, set to 1 (Eth Mainnet)
        let chain_id = chain_id.unwrap_or(1);

        // Check if we have a private key to set up a Signer
        let signer = if let Some(private_key) = &private_key {
            let wallet = private_key
                .parse::<LocalWallet>()
                .expect("Failed to parse private key");
            Some(SignerMiddleware::new(
                provider.clone(),
                wallet.with_chain_id(chain_id),
            ))
        } else {
            None
        };

        // Check if we have a contract address to set up a Contract
        let abi: Abi = serde_json::from_str(&CONTRACT_ABI_STR).expect("Failed to parse ABI");
        let contract = Contract::new(contract_address, abi, provider.clone());

        // Determine the timeout as a Duration in seconds, assign default if not provided
        // let timeout = Duration::from_secs(timeout.unwrap_or(15));
        Ok(Client {
            provider,
            chain_id,
            signer,
            contract,
        })
    }

    pub fn has_signer(&self) -> bool {
        self.signer.is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // Test Initialization from .env file
    fn init() {
        // Load the .env file from the root of the project
        dotenv::from_filename("../env/.env").expect("Failed to load .env file");
        // Initialize the Client from the environment variables
        let _ = Client::default();
    }
}
