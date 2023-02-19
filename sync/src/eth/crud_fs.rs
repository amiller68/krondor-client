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
    env
};
use ethers_contract_derive::EthEvent;
use lazy_static::lazy_static;
use crate::types::{
    cid::Cid,
    file_object::FileObject,
    crud_fs::CrudFs,
};
use rand::Rng;
use crate::eth::client::Client;
use async_trait::async_trait;

#[async_trait]
impl CrudFs for Client {
    /// C is for Create
    /// Create a new file on the contract
    /// Returns the key and timestamp of the file
    /// # Arguments
    /// * `FileObject` - The FileObject to create in CrudFs
    /// # Returns
    /// * `Result<(Bytes, U256), Error>` - The key and timestamp of the file
    async fn create_file(&mut self, file_object: FileObject) -> Result<([u8; 32], u64), Error> {
        if !self.has_signer() {
            return Err(anyhow!("No signer available"));
        }
        // Borrow our signer and contract
        let signer = self.signer.as_ref().unwrap();
        // Create a new deal proposal Transaction
        let data = self.contract.encode("createFile", file_object.into_token())?;
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data)
            // TODO: Make gas and gas price configurable
            .gas(1_000_000u64) // 1 million gas
            .gas_price(80_000_000_000u64) // 80 Gwei
            .chain_id(self.chain_id);
        // Sign the transaction and send it
        let pending_tx = match signer.send_transaction(tx, None).await {
            Ok(tx) => tx,
            // TODO - Handle errors that can occur here from the contract
            Err(e) => {
                return Err(anyhow!("Error signing transaction: {}", &e.to_string()));
            }
        };
        // Wait for the transaction to be mined
        let receipt = pending_tx.await?;

        // TODO: This is a hack to get around the fact that I can't read the emitted event
        // Read the log emitted by the contract from the receipt
        let log = receipt.as_ref().unwrap().logs[0].clone();

        // declare a buffer to hold the key
        let mut key_bytes = [0u8; 32];
        // Copy the key bytes into the buffer
        key_bytes.copy_from_slice(&log.topics[1].as_bytes()[..]);
        // Turn the H256 bytes into a U256
        let timestamp = U256::from(log.topics[2].as_bytes());
        // Convert the U256 to a u64
        let timestamp = u64::try_from(timestamp).unwrap();

        // Return the key and timestamp
        Ok((key_bytes, timestamp))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    /// Test is creates a file
    async fn test_create_file() {
        use super::*;
        use crate::utils::hash::hash_path;
        // Read the .env file at ../../env/.env
        dotenv::from_path("../../env/.env").ok();
        // Initialize the Client
        let mut client = Client::default();
        // Create a new FileObject to test with
        // Generate a random 10 character string
        // make a buffer to hold the string
        let mut buffer = [0u8; 10];
        // Fill the buffer with random characters
        rand::thread_rng().fill(&mut buffer[..]);
        // Convert the buffer to a string
        let random_string = String::from_utf8_lossy(&buffer[..]);

        let path = PathBuf::from(format!(
            "/tmp/{}",
            random_string
        ));
        // Get the hash of the path
        let key = hash_path(&path).unwrap();
        // Create the FileObject
        let file_object = FileObject {
            // Create a random file path
            path: path.clone(),
            key,
            cid: Cid::from_str("QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8i3PqQ6CMRVtPb".to_string()).unwrap(),
            timestamp: 0,
            metadata: HashMap::new(),
        };
        // Create the file
        let result = client.create_file(file_object).await;
        // Assert that the result is Ok
        assert!(result.is_ok());
        // Borrow the result
        let (res_key, timestamp) = result.unwrap();
        // Assert that the key is the same as the one we created
        assert_eq!(res_key, key);
        // Assert that the timestamp is greater than 0
        assert!(timestamp > 0);
    }
}