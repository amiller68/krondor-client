use std::path::PathBuf;
use anyhow::{Error, Result, anyhow};
use ethers::{utils, prelude::*};
use crate::types::cid::Cid;
use crate::types::crud_file::CrudFile;

use super::backend::BackendClient;
use super::store::StoreClient;
// use super::local::LocalClient;

/// A CRUD filesystem representation
/// # Fields
/// * `backend_client` - The backend client - this maintains FS state on a remote backend
/// * `store` - The store client - this maintains FS state on a remote store
/// # `local_client` - The local client - this maintains FS state on the local machine or filesystem
pub struct CrudFs {
    backend_client: BackendClient,
    store_client: StoreClient,
    // local_client: LocalClient,
}

impl Default for CrudFs {
    fn default() -> Self {
        let backend_client = BackendClient::default();
        let store_client = StoreClient::default();
        // let local_client = LocalClient::default();
        Self::new(backend_client, store_client) // , local_client)
    }
}

impl CrudFs {
    /// New CrudFs
    /// # Arguments
    /// * `backend_client` - The backend client
    /// * `store_client` - The store client
//     * `local_client` - The local client
    pub fn new(backend_client: BackendClient, store_client: StoreClient) -> Self { // , local_client: LocalClient) -> Self {
        Self {
            backend_client,
            store_client,
            // local_client,
        }
    }
    // C is for Create
    /// Create a file in the backend, store, and local
    /// # Arguments
    /// * `crud_file` - The CrudFile to create
    /// # Returns
    /// * `Result<CrudFile, Error>` - The result of the operation
    pub async fn create(&self, crud_file: CrudFile) -> Result<(), Error> {
        let crud_file = self.backend_client.create_file(
            crud_file.path.clone(),
            crud_file.cid.clone(),
            crud_file.metadata.clone(),
        ).await?;
        self.store_client.put(crud_file.clone()).await?;
        // self.local_client.create_file(crud_file.path.clone());
        Ok(())
    }

    // R is for Read
    /// Read a file from the backend, store, and local
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



