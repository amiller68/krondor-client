use ethers::{
    utils,
    prelude::*,
    types::{Address, Bytes, Filter, Log, TransactionRequest, U256},
};
use std::{
    sync::Arc,
};
use anyhow::{Error, Result, anyhow};
use ethers::{
    abi::{Abi, Token, Tokenizable},
    contract::Contract,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};
use std::{
    convert::{From, TryFrom},
    path::PathBuf,
    collections::HashMap,
    env
};
use ethers_contract_derive::EthEvent;
use lazy_static::lazy_static;
use rand::Rng;
use crate::{
    types::{
        crud_file::CrudFile,
        cid::Cid,
    },
    backend::eth::{
        EthClient, EthSigner
    }
};

abigen! (
    CrudFs,
    // TODO: Relative path that works
    "/Users/alexmiller/krondor-client/contracts/artifacts/contracts/crudFs.sol/CrudFs.json",
    event_derives(serde::Deserialize, serde::Serialize),
);

// TODO: Refactor this into a trait so we can support multiple backends

pub struct BackendClient {
    signer: EthSigner,
    contract: CrudFs<EthSigner>
}

impl Default for BackendClient {
    fn default() -> Self {
        let eth_client = EthClient::default();
        let contract_address = std::env::var("CONTRACT_ADDRESS")
            .expect("CONTRACT_ADDRESS must be set")
            .parse::<Address>()
            .expect("CONTRACT_ADDRESS must be a valid Ethereum Address");
        Self::new(eth_client, contract_address)
    }
}

impl BackendClient {
    pub fn new(eth_client: EthClient, contract_address: Address) -> Self {
        // Get the underlying client type from the EthClient struct
        let signer = eth_client.signer;
        let contract = CrudFs::new(contract_address.clone(), Arc::new(signer.clone()));
        Self {
            signer,
            contract,
        }
    }

    /// Create a new file in the backend
    /// # Arguments
    /// - `path` - The path of the file to create
    /// - `cid` - The CID of the file to create
    /// - `metadata` - The metadata of the file to create
    /// # Returns
    /// - `Result<CrudFile, Error>` - The created file or an error
    pub async fn create_file(
        &self,
        path: PathBuf,
        cid: Cid,
        metadata: HashMap<String, String>,
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
        let tx = self.contract.create_file(
            path_string, cid_string, metadata_string
        ).send().await?.await?.unwrap();

        // Get the block number from the transaction receipt
        let block_number = tx.block_number.unwrap();

        // Query the events from the contract
        let events = self.contract.event::<CreateFileFilter>().from_block(block_number).query().await?;

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
    pub async fn read_file(&mut self, key: [u8; 32]) -> Result< CrudFile, Error> {
        // Get a Bytes token from the key
        // let key = Bytes::from(key.to_vec());
        println!("Reading file with key: {:?}", key);
        // Get the provider
        // let provider = self.provider.as_ref();
        // Get the file
        let res  = self.contract.read_file(key).await;
        let crud_file =  CrudFile::from_token(res?.into_token())?;
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
    pub async fn update_file(
        &self,
        key: [u8; 32],
        cid: Cid,
        metadata: HashMap<String, String>,
    ) -> Result<([u8; 32], u64), Error> {
        // Convert the CID to a string
        let cid_string = cid.to_string();
        // Convert the metadata to a string
        let metadata_string = serde_json::to_string(&metadata)?;

        // Send the transaction to the contract
        let tx = self.contract.update_file(
            key, cid_string, metadata_string
        ).send().await?.await?.unwrap();

        // Get the block number from the transaction receipt
        let block_number = tx.block_number.unwrap();

        // Query the events from the contract
        let events = self.contract.event::<UpdateFileFilter>().from_block(block_number).query().await?;

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
    pub async fn delete_file(
        &self, key: [u8; 32]
    ) -> Result<(), Error> {
        // Send the transaction to the contract
        let tx = self.contract.delete_file(
            key
        ).send().await?.await?.unwrap();

        // Get the block number from the transaction receipt
        let block_number = tx.block_number.unwrap();

        // Query the events from the contract
        let events = self.contract.event::<DeleteFileFilter>().from_block(block_number).query().await?;

        // Take the first event
        let event = events.first().unwrap();

        // Get the key from the event
        let key = event.key;

        // Assert that the key is the same as the one we sent
        assert_eq!(key, key);

        Ok(())
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
        dotenv::from_path("./../../env/.env").ok();
        let mut client = BackendClient::default();

        // Use a random string as the path
        let path = PathBuf::from(format!("/tmp/{}", rand::thread_rng().gen::<u64>()));
        // Hash the path
        let key = hash_path(&path).unwrap();
        // Create a new CID
        let cid = Cid::from_str("bafybeic3gsbthvobb2jenjpeam32yj2hrznnmk4ei4rwbdosexc4ewyz7e".to_string()).unwrap();
        // Create a new metadata with 'test: value'
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), "value".to_string());

        // Create the file
        let result = client.create_file(
            path.clone(),
            cid.clone(),
            metadata.clone(),
        ).await;

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
        let result = client.read_file(key).await;
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
        let cid = Cid::from_str("bafkreiai4g4rxn3kkeqyi3vi4ovjs4ewugqxpek4x2kr7tlhbmhd2gw6nq".to_string()).unwrap();
        // Create a new metadata with 'test: value'
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), "value2".to_string());

        // Update the file
        let result = client.update_file(
            key,
            cid.clone(),
            metadata.clone(),
        ).await;

        // Assert that the result is Ok
        assert!(result.is_ok());

        // Get the CrudFile from the result
        let (key, timestamp) = result.unwrap();
        // Verify the contents of the file

        // Read the file to verify the contents
        let result = client.read_file(key).await;
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
        let result = client.delete_file(key).await;
        // Assert that the result is Ok
        assert!(result.is_ok());
    }
}
