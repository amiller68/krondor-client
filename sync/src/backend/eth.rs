use anyhow::{Error, Result, anyhow};
use ethers::{
    abi::{Abi, Token, Tokenizable},
    contract::Contract,
    middleware::SignerMiddleware,
    prelude::*,
    utils,
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
use rand::Rng;
use std::sync::Arc;

pub type EthSigner = SignerMiddleware<Provider<Http>,Wallet<k256::ecdsa::SigningKey>>;

/// A multi-purpose Ethereum Client - just a wrapper around ethers::SignerMiddleware
#[derive(Debug, Clone)]
pub struct EthClient {
    pub signer: EthSigner,
}

impl Default for EthClient {
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
        // Create a new EthClient
        EthClient::new(
            api_url,
            api_key,
            chain_id,
            private_key,
        ).unwrap()
    }
}

impl EthClient {
    /// Create a new EthClient - Uses EthClientBuilder::new()
    /// # Arguments
    /// * `api_url` - The URL of the Ethereum API to connect to.
    /// * `api_key` - The API Key for the Ethereum API. This is required.
    /// * `chain_id` - The Chain ID of the network we're connected to.
    /// * `private_key` - The Private Key for the Ethereum Account we're using to sign.
    pub fn new(
        api_url: String,
        api_key: String,
        chain_id: u64,
        private_key: String,
    ) -> Result<Self, Error> {
        // Determine an API URL and Initialize the Provider
        let url = format!("{}/{}", api_url, api_key);
        let provider = Provider::<Http>::try_from(url).expect("Failed to create provider");
        // Initialize a Wallet to use
        let wallet = private_key
            .parse::<LocalWallet>()
            .expect("Failed to parse private key");
        // Check if we have a private key to set up a Signer
        let signer = SignerMiddleware::new(provider.clone(), wallet.with_chain_id(chain_id));
        // Return the Client
        Ok(Self{ signer })
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
        let _ = EthClient::default();
    }
}
