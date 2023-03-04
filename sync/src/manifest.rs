use anyhow::{Error, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::{Write, Read}, path::PathBuf};
use ethers::utils::hex;
use sync::{
    utils::hash::hash_path,
    types::{
        cid::Cid,
        crud_file::CrudFile
    }
};


// TODO (amiller68): Obake this
/// Our manifest data structure. This tracks all the files in the local filesystem
/// This is used to determine which files need to be synced and how to sync them
/// # Fields
/// * `files` - The list of files in the manifest
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// The address of the contract that serves as our CrudFs backend
    pub contract_address: String,
    // /// The api of the Estuary node that serves as our CrudFs store
    // pub estuary_api: String,
    /// The list of files in the manifest
    pub files: HashMap<String, CrudFile>,
}

/// Manifest - Represents the manifest file
impl Manifest {
    /// Create a new Manifest instance
    /// # Arguments
    /// * `contract_address` - The address of the contract that serves as our CrudFs backend
    pub fn new(
        contract_address: String,
        // estuary_api: String,
    ) -> Self {
        Self {
            contract_address,
            // estuary_api,
            files: HashMap::new(),
        }
    }

    /// Read a manifest from a file
    /// # Arguments
    /// * `path` - The path to the manifest file
    /// # Returns
    /// * `Result<Manifest, Error>` - The result
    pub fn read(path: &PathBuf) -> Result<Manifest, Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let manifest: Manifest = serde_json::from_str(&contents)?;
        Ok(manifest)
    }

    /// Write a manifest to a file
    /// # Arguments
    /// * `path` - The path to the manifest file
    /// # Returns
    /// * `Result<(), Error>` - The result
    pub fn write(&self, path: &PathBuf) -> Result<(), Error> {
        let mut file = File::create(path)?;
        let contents = serde_json::to_string_pretty(&self)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    /// Check if a path is in the manifest
    /// # Arguments
    /// * `path` - The path to the file
    /// # Returns
    /// * `Result<bool, Error>` - The result
    pub fn contains(&self, path: &PathBuf) -> Result<bool, Error> {
        let key = hash_path(path)?;
        let key_str = hex::encode(key);
        Ok(self.files.contains_key(&key_str))
    }

    /// Add a file to the manifest
    /// # Arguments
    /// * `crud_file` - The file to add
    /// # Returns
    /// * `Result<(), Error>` - The result
    pub fn add(&mut self, crud_file: CrudFile) -> Result<(), Error> {
        let key_str = hex::encode(crud_file.key);
        self.files.insert(key_str, crud_file);
        Ok(())
    }

    /// Remove a file from the manifest
    /// # Arguments
    /// * `path` - The path to the file
    /// # Returns
    /// * `Result<(), Error>` - The result
    pub fn rm(&mut self, path: &PathBuf) -> Result<(), Error> {
        let key = hash_path(path)?;
        let key_str = hex::encode(key);
        self.files.remove(&key_str);
        Ok(())
    }
}
