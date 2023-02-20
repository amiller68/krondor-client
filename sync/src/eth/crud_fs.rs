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
    crud_fs::AsyncCrudFs,
};
use rand::Rng;
use crate::eth::client::Client;
use async_trait::async_trait;

#[async_trait]
impl AsyncCrudFs for Client {
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

    async fn read_file(&mut self, key: [u8; 32]) -> Result<FileObject, Error> {
        // Get a Bytes token from the key
        let key = Bytes::from(key.to_vec());
        println!("Reading file with key: {:?}", key);
        // Encode the read_file function call
        let data = self.contract.encode("readFile", key.into_token())?;
        // Create a new TransactionRequest
        let tx = TransactionRequest::new()
            .to(self.contract.address())
            .data(data);
        // Call the contract
        let result = self.provider.call(tx, None).await?;
        // Decode the result
        let file_object = self.contract.decode("readFile", &result)?;
        // Convert the result to a FileObject
        let file_object = FileObject::from_token(file_object)?;
        // Return the FileObject
        Ok(file_object)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    /// Test creating a file
    async fn test_crud() {
        use super::*;
        use crate::utils::hash::hash_path;

        // Initialize the Client from the .env file
        dotenv::from_path("../../env/.env").ok();
        let mut client = Client::default();

        // Create a new FileObject
        // Use a random string as the path
        let mut buffer = [0u8; 10];
        rand::thread_rng().fill(&mut buffer[..]);
        let random_string = String::from_utf8_lossy(&buffer[..]);
        let path = PathBuf::from(format!(
            "/tmp/{}",
            random_string
        ));
        // Get the hash of the path
        let key = hash_path(&path).unwrap();
        // Create a new Cid
        let cid = Cid::from_str("QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8i3PqQ6CMRVtPb".to_string()).unwrap();
        // Create the FileObject
        let file_object = FileObject {
            // Create a random file path
            path: path.clone(),
            key,
            cid: cid.clone(),
            timestamp: 0,
            metadata: HashMap::new(),
        };

        // C is for create

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

        // R is for read

        // Read the file
        println!("Reading file with key: {:?}", key);
        let result = client.read_file(key).await;
        println!("Result: {:?}", result);
        // Assert that the result is Ok
        assert!(result.is_ok());
        // Borrow the result
        let file_object = result.unwrap();
        // Assert that the path is the same as the one we created
        assert_eq!(file_object.path, path);
        // Assert that the key is the same as the one we created
        assert_eq!(file_object.key, key);
        // Assert that the cid is the same as the one we created
        assert_eq!(file_object.cid.clone(), cid);
        // Assert that the timestamp is greater than 0
        assert!(file_object.timestamp > 0);


    }
}