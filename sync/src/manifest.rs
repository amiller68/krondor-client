use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Post Entry - Represents a post entry in the manifest
/// # Fields
/// * `path` - The path to the post
/// * `post_id` - The post ID
/// * `title` - The title of the post
/// * `date` - The date the post was created
/// * 'cid' - The IPFS CID of the post
#[derive(Debug, Serialize, Deserialize)]
pub struct PostEntry {
    pub path: PathBuf,
    pub post_id: String,
    pub title: String,
    pub date: String,
    pub cid: String,
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
    pub post_entries: Vec<PostEntry>,
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
            post_entries: Vec::new(),
        }
    }

    /// Save the manifest to a file
    /// # Arguments
    /// * `manifest_path` - The path to the manifest
    pub fn save(&self, manifest_path: &PathBuf) -> Result<(), Error> {
        let manifest_json = serde_json::to_string_pretty(&self)?;
        let mut manifest_file = File::create(manifest_path)?;
        manifest_file.write_all(manifest_json.as_bytes())?;
        Ok(())
    }

    /// Load the manifest from a file
    /// # Arguments
    /// * `manifest_path` - The path to the manifest
    /// # Returns
    /// * `Manifest` - The manifest
    /// * `Error` - The error
    pub fn load(manifest_path: &PathBuf) -> Result<Self, Error> {
        let manifest_file = File::open(manifest_path)?;
        let manifest: Manifest = serde_json::from_reader(manifest_file)?;
        Ok(manifest)
    }

    /// Add a post entry to the manifest
    pub fn add_post_entry(&mut self, post_entry: PostEntry) {
        self.post_entries.push(post_entry);
    }

    /// Remove a post entry from the by its post ID
    pub fn remove_post_entry(&mut self, post_id: &str) {
        self.post_entries
            .retain(|post_entry| post_entry.post_id != post_id);
    }

    /// Update a post entry in the manifest  by its post ID
    pub fn update_post_entry(&mut self, post_entry: PostEntry) {
        for entry in self.post_entries.iter_mut() {
            if entry.post_id == post_entry.post_id {
                entry.path = post_entry.path.clone();
                entry.title = post_entry.title.clone();
                entry.date = post_entry.date.clone();
                entry.cid = post_entry.cid.clone();
            }
        }
    }
}
