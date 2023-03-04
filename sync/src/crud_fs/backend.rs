/* Backend Implementation */

use anyhow::{anyhow, Error, Result};
use ethers::{
    abi::{Abi, Token, Tokenizable},
    contract::Contract,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};
use ethers::{
    prelude::*,
    types::{Address, Bytes, Filter, Log, TransactionRequest, U256},
    utils,
};
use ethers_contract_derive::EthEvent;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::{
    convert::{From, TryFrom},
    env,
    path::PathBuf,
};
use std::str::FromStr;
// use rand::Rng;
use crate::types::{cid::Cid, crud_file::CrudFile, metadata::Metadata};

abigen!(
    CrudFsContract,
    // This path is relative to `sync`
    "./../contracts/artifacts/contracts/crudFs.sol/CrudFs.json",
    event_derives(serde::Deserialize, serde::Serialize),
);

pub struct BackendClient {
    signer: EthSigner,
    contract: CrudFsContract<EthSigner>,
}

impl Default for BackendClient {
    fn default() -> Self {
        let contract_address = env::var("CONTRACT_ADDRESS")
            .expect("CONTRACT_ADDRESS must be set");
        Self::new( contract_address)
    }
}

impl BackendClient {
    pub fn new(contract_address: String) -> Self {
        // Parse the contract address
        let contract_address = Address::from_str(&contract_address).unwrap();
        // Get an Eth client from the environment
        let eth_client = EthClient::default();
        // Get the underlying client type from the EthClient struct
        let signer = eth_client.signer;
        let contract = CrudFsContract::new(contract_address.clone(), Arc::new(signer.clone()));
        Self { signer, contract }
    }

    /// Create a new file in the backend
    /// # Arguments
    /// - `path` - The path of the file to create
    /// - `cid` - The CID of the file to create
    /// - `metadata` - The metadata of the file to create
    /// # Returns
    /// - `Result<CrudFile, Error>` - The created file or an error
    pub async fn create(
        &self,
        path: PathBuf,
        cid: Cid,
        metadata: Metadata,
    ) -> Result<CrudFile, Error> {
        // Convert the file path to a string
        let path_string = path.to_str().unwrap().to_string();
        // Get the file name from the path
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        // Convert the CID to a string
        let cid_string = cid.to_string();
        // Convert the metadata to a string
        let metadata_string = serde_json::to_string(&metadata)?;

        // Send the transaction to the contract
        let tx = self
            .contract
            .create_file(path_string, cid_string, metadata_string)
            .send()
            .await?
            .await?
            .unwrap();

        // Get the block number from the transaction receipt
        let block_number = tx.block_number.unwrap();

        // Query the events from the contract
        let events = self
            .contract
            .event::<CreateFileFilter>()
            .from_block(block_number)
            .query()
            .await?;

        // Take the first event
        let event = events.first().unwrap();

        // Get the key from the event
        let key = event.key;
        // Get the timestamp from the event
        let timestamp = event.timestamp.as_u64();

        Ok(CrudFile {
            path,
            filename,
            key,
            cid,
            timestamp,
            metadata,
        })
    }

    /// Read a file from the backend
    /// # Arguments
    /// - `key` - The key of the file to read
    /// # Returns
    /// - `Result<CrudFile, Error>` - The read file or an error
    pub async fn read(&mut self, key: [u8; 32]) -> Result<CrudFile, Error> {
        // Get a Bytes token from the key
        // let key = Bytes::from(key.to_vec());
        println!("Reading file with key: {:?}", key);
        // Get the provider
        // let provider = self.provider.as_ref();
        // Get the file
        let res = self.contract.read_file(key).await;
        let crud_file = CrudFile::from_token(res?.into_token())?;
        // Return the file
        Ok(crud_file)
    }

    /// Update a file in the backend
    /// # Arguments
    /// - `key` - The key of the file to update
    /// - `cid` - The CID of the file to update
    /// - `metadata` - The metadata of the file to update
    /// # Returns
    /// - `Result<([u8; 32], u64), Error>` - The key and updated timestamp of the file or an error
    pub async fn update(
        &self,
        key: [u8; 32],
        cid: Cid,
        metadata: Metadata,
    ) -> Result<([u8; 32], u64), Error> {
        // Convert the CID to a string
        let cid_string = cid.to_string();
        // Convert the metadata to a string
        let metadata_string = serde_json::to_string(&metadata)?;

        // Send the transaction to the contract
        let tx = self
            .contract
            .update_file(key, cid_string, metadata_string)
            .send()
            .await?
            .await?
            .unwrap();

        // Get the block number from the transaction receipt
        let block_number = tx.block_number.unwrap();

        // Query the events from the contract
        let events = self
            .contract
            .event::<UpdateFileFilter>()
            .from_block(block_number)
            .query()
            .await?;

        // Take the first event
        let event = events.first().unwrap();

        // Get the key from the event
        let key = event.key;
        // Get the timestamp from the event
        let timestamp = event.timestamp.as_u64();

        Ok((key, timestamp))
    }

    /// Delete a file from the backend
    /// # Arguments
    /// - `key` - The key of the file to delete
    /// # Returns
    /// - `Result<(), Error>` - An error if the file could not be deleted
    pub async fn delete(&self, key: [u8; 32]) -> Result<(), Error> {
        // Send the transaction to the contract
        let tx = self.contract.delete_file(key).send().await?.await?.unwrap();

        // Get the block number from the transaction receipt
        let block_number = tx.block_number.unwrap();

        // Query the events from the contract
        let events = self
            .contract
            .event::<DeleteFileFilter>()
            .from_block(block_number)
            .query()
            .await?;

        // Take the first event
        let event = events.first().unwrap();

        // Get the key from the event
        let _key = event.key;

        // Assert that the key is the same as the one we sent
        assert_eq!(key, _key);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test Initialization from .env file
    fn client_init() {
        // Load the .env file from the root of the project
        dotenv::from_filename("../env/.env").expect("Failed to load .env file");
        // Initialize the Client from the environment variables
        let _ = BackendClient::default();
    }

    #[tokio::test]
    /// Test creating a file
    async fn test_crud() {
        use crate::utils::hash::hash_path;
        use rand::Rng;

        // Initialize the Client from the .env file
        dotenv::from_path("./../../env/.env").ok();
        let mut client = BackendClient::default();

        // Use a random string as the path
        let path = PathBuf::from(format!("/tmp/{}", rand::thread_rng().gen::<u64>()));
        // Hash the path
        let key = hash_path(&path).unwrap();
        // Create a new CID
        let cid = Cid::from_str(
            "bafybeic3gsbthvobb2jenjpeam32yj2hrznnmk4ei4rwbdosexc4ewyz7e".to_string(),
        )
        .unwrap();
        // Create a new metadata with 'test: value'
        let mut metadata = Metadata::new();
        metadata.insert("test".to_string(), "value".to_string());

        // Create the file
        let result = client
            .create(path.clone(), cid.clone(), metadata.clone())
            .await;

        // Assert that the result is Ok
        assert!(result.is_ok());

        // Get the CrudFile from the result
        let crud_file = result.unwrap();
        // Verify the contents of the file
        assert_eq!(crud_file.path, path);
        assert_eq!(crud_file.key, key);
        assert_eq!(crud_file.cid, cid);
        assert_eq!(crud_file.metadata, metadata);
        assert!(crud_file.timestamp > 0);
        let timestamp = crud_file.timestamp;

        // R is for read

        // Read the file
        println!("Reading file with key: {:?}", Bytes::from(key.to_vec()));
        let result = client.read(key).await;
        // Assert that the result is Ok
        assert!(result.is_ok());
        // Get the CrudFile from the result
        let crud_file = result.unwrap();
        // Verify the contents of the file
        assert_eq!(crud_file.path, path);
        assert_eq!(crud_file.key, key);
        assert_eq!(crud_file.cid, cid);
        assert_eq!(crud_file.metadata, metadata);
        assert_eq!(crud_file.timestamp, timestamp);

        // U is for update

        // Create a new CID
        let cid = Cid::from_str(
            "bafkreiai4g4rxn3kkeqyi3vi4ovjs4ewugqxpek4x2kr7tlhbmhd2gw6nq".to_string(),
        )
        .unwrap();
        // Create a new metadata with 'test: value'
        let mut metadata = Metadata::new();
        metadata.insert("test".to_string(), "value2".to_string());

        // Update the file
        let result = client.update(key, cid.clone(), metadata.clone()).await;

        // Assert that the result is Ok
        assert!(result.is_ok());

        // Get the CrudFile from the result
        let (key, timestamp) = result.unwrap();
        // Verify the contents of the file

        // Read the file to verify the contents
        let result = client.read(key).await;
        // Assert that the result is Ok
        assert!(result.is_ok());
        // Get the CrudFile from the result
        let crud_file = result.unwrap();
        // Verify the contents of the file
        assert_eq!(crud_file.path, path);
        assert_eq!(crud_file.key, key);
        assert_eq!(crud_file.cid, cid);
        assert_eq!(crud_file.metadata, metadata);
        assert_eq!(crud_file.timestamp, timestamp);

        // D is for delete

        // Delete the file
        let result = client.delete(key).await;
        // Assert that the result is Ok
        assert!(result.is_ok());
    }
}

/* Eth Backend */

type EthSigner = SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>;

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
        EthClient::new(api_url, api_key, chain_id, private_key).unwrap()
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
        Ok(Self { signer })
    }
}
