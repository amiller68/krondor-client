#[allow(unused_imports)]

use anyhow::{Error, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::{Write, Read}, path::PathBuf};
use sha3::{Digest, Keccak256};

use crate::types::{
    cid::Cid,
    crud_file::CrudFile as FileObject,
};

/// Get the key from the path of a file
/// # Arguments
/// * `path` - The path of the file
/// # Returns
/// The keccak256 hash of the path
fn key_from_path(path: &PathBuf) -> Result<String, Error> {
    let mut hasher = Keccak256::new();
    hasher.update(path.to_str().unwrap().as_bytes());
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

/// Our manifest data structure
/// Version 1.0.0 Specification
/// # Fields
/// * `version_number` - The major version number of the manifest
/// * `contract_address` - The address of the smart contract
/// * `ipfs_gateway` - The IPFS gateway or service to use for pinning
/// * `files` - The list of files in the manifest
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    /// The version of the manifest
    pub version: String,
    /// The address of the backend contract
    pub backend_address: String,
    /// The address of the IPFS node
    pub files: HashMap<String, FileObject>,
}

/// Manifest - Represents the manifest file
impl Manifest {
    /// Create a new Manifest instance
    /// # Arguments
    /// * `backend_address` - The address of the backend contract
    pub fn new(backend_address: String) -> Self {
        // Verify the backend address is valid. TODO: Do this properly
        if !backend_address.starts_with("0x") {
            panic!("Invalid backend address");
        }
        Self {
            version: String::from("1.0.0"),
            backend_address,
            files: HashMap::new(),
        }
    }

    // Save the manifest to a file
    // # Arguments
    // * `manifest_path` - The path to the manifest
    // pub fn save(&self, manifest_path: &PathBuf) -> Result<(), Error> {
    //     let manifest_json = serde_json::to_string_pretty(&self)?;
    //     let mut manifest_file = File::create(manifest_path)?;
    //     manifest_file.write_all(manifest_json.as_bytes())?;
    //     Ok(())
    // }
    //
    // /// Load the manifest from a file
    // /// # Arguments
    // /// * `manifest_path` - The path to the manifest
    // /// # Returns
    // /// * `Manifest` - The manifest
    // /// * `Error` - The error
    // pub fn load(manifest_path: &PathBuf) -> Result<Self, Error> {
    //     let manifest_file = File::open(manifest_path)?;
    //     let manifest: Manifest = serde_json::from_reader(manifest_file)?;
    //     Ok(manifest)
    // }
    //
    // /// Manifest CRUD
    //
    // /// Create a new file in the manifest
    // /// # Arguments
    // /// * `file` - The file to add to the manifest
    // /// * `metadata` - The metadata to attach to the file
    // /// # Returns
    // /// * `Result<FileObject, Error>` - The result
    // /// # Errors
    // /// * `std::io::Error` - If there is an error reading the file
    // /// * `cid::Error` - If there is an error creating the CID
    // pub fn create_file(
    //     &mut self,
    //     file: File,
    //     metadata: HashMap<String, String>,
    // ) -> Result<FileObject, Error> {
    //     let key = key_from_path(file.clone());
    //     if self.files.contains_key(key) {
    //         return anyhow!("File already exists in manifest");
    //     }
    //     let file_object = FileObject::create(file, metadata)?;
    //     self.files.insert(key, file_object.clone());
    //     Ok(file_object)
    // }
    //
    // /// Read a file's entry from the manifest
    // /// # Arguments
    // /// * file - The file to read from the manifest
    // /// # Returns
    // /// * `Result<FileObject, Error>` - The result
    // pub fn read_file(&self, file: File) -> Result<FileObject, Error> {
    //     let key = key_from_path(file);
    //     if !self.files.contains_key(key) {
    //         return anyhow!("File does not exist in manifest");
    //     }
    //     Ok(self.files[key].clone())
    // }
    //
    // /// Update a file's entry in the manifest
    // /// # Arguments
    // /// * `file` - The file to update in the manifest
    // /// * `metadata` - The metadata to attach to the file
    // /// # Returns
    // /// * `Result<FileObject, Error>` - The result
    // pub fn update_file(
    //     &mut self,
    //     file: File,
    //     metadata: HashMap<String, String>,
    // ) -> Result<FileObject, Error> {
    //     let key = key_from_path(file);
    //     if !self.files.contains_key(key) {
    //         return anyhow!("File does not exist in manifest");
    //     }
    //     let file_object = FileObject::create(file, metadata)?;
    //     self.files.insert(key, file_object.clone());
    //     Ok(file_object)
    // }
    //
    // /// Delete a file's entry from the manifest
    // /// # Arguments
    // /// * `file` - The file to delete from the manifest
    // /// # Returns
    // /// * `Result<FileObject, Error>` - The result
    // pub fn delete_file(&mut self, file: File) -> Result<FileObject, Error> {
    //     let key = key_from_path(file);
    //     if !self.files.contains_key(key) {
    //         return anyhow!("File does not exist in manifest");
    //     }
    //     let file_object = self.files.remove(key);
    //     Ok(file_object.unwrap())
    // }
}
