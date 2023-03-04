use crate::types::{cid::Cid, crud_file::CrudFile, metadata::Metadata};
use anyhow::{anyhow, Error, Result};
// use ethers::{prelude::*, utils};
use std::path::PathBuf;

use super::backend::BackendClient;
use super::store::StoreClient;

/// A CRUD filesystem representation
/// # Fields
/// * `backend_client` - The backend client - this maintains FS state on a remote backend
/// * `store_client` - The store client - this maintains FS state on a remote store
pub struct CrudFs {
    backend_client: BackendClient,
    store_client: StoreClient,
}

impl Default for CrudFs {
    fn default() -> Self {
        let contract_address = std::env::var("CRUDFS_CONTRACT_ADDRESS")
            .map_err(|_| anyhow!("CRUDFS_CONTRACT_ADDRESS not set")).unwrap();
        Self::new(contract_address)
    }
}

impl CrudFs {
    /// New CrudFs
    /// # Arguments
    /// * `contract_address` - The address of the contract that serves as our CrudFs backend
    pub fn new(
        contract_address: String,
    ) -> Self {
        let backend_client = BackendClient::new(contract_address);
        let store_client = StoreClient::default();
        Self {
            backend_client,
            store_client,
        }
    }
    // C is for Create
    /// Create a CrudFile in the backend and store
    /// # Arguments
    /// * `path: PathBuf` - The path to the file
    /// * `cid: Cid` - The Cid of the file
    /// * `metadata: Metadata` - The metadata of the file
    /// # Returns
    /// * `Result<CrudFile, Error>` - The result of the operation
    pub async fn create(&self, path: PathBuf, cid: Cid, metadata: Metadata) -> Result<CrudFile, Error> {
        let crud_file = self.backend_client.create(path, cid, metadata).await?;
        let _ = self.store_client.put(crud_file.clone()).await?;
        Ok(crud_file.clone())
    }

    // R is for Read
    /// Read a file from the backend and store
    /// # Arguments
    /// * `path` - The path to the file
    /// # Returns
    /// * `Result<CrudFile, Error>` - The result of the operation
    pub async fn read(&self, path: PathBuf) -> Result<CrudFile, Error> {
        todo!()
    }

    // U is for Update
    /// Update a file in the backend, store, and local
    /// # Arguments
    /// * `crud_file` - The CrudFile to update
    pub async fn update(&self, crud_file: CrudFile) -> Result<(), Error> {
        todo!()
    }

    // D is for Delete
    /// Delete a file from the backend, store, and local
    /// # Arguments
    /// * `path` - The path to the file
    pub async fn delete(&self, path: PathBuf) -> Result<(), Error> {
        todo!()
    }
}
