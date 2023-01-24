use anyhow::{Error, Result};
use cid::multihash::{Code, MultihashDigest};
use cid::Cid;
use serde::{Deserialize, Serialize};
use std::fmt::Error;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    std::convert::TryFrom,
};

/// Impl TryFrom for Cid for Files
impl TryFrom<File> for Cid {
    type Error = anyhow::Error;
    fn try_from(file: File) -> Result<Self, Self::Error> {
        let mut hasher = Code::Sha2_256.digest();
        std::io::copy(&mut file, &mut hasher)?;
        let _cid = Cid::try_from(hasher.finish())?;
        Ok(_cid)
    }
}

/// Post Entry - Represents a post entry in the manifest
/// # Fields
/// * `path` - The path to the post
/// * `post_id` - The post ID. This is an integer that is unique to the post
/// * `title` - The title of the post
/// * `date` - The date the post was created
/// * 'cid' - The IPFS CID of the post
#[derive(Debug, Serialize, Deserialize)]
pub struct PostEntry {
    pub path: PathBuf,
    pub post_id: u32,
    pub title: String,
    pub cid: Cid,
}

impl PostEntry {
    /// Create a new PostEntry
    pub fn new(path: PathBuf, post_id: String, title: String, cid: String) -> Self {
        Self {
            path,
            post_id,
            title,
            cid,
        }
    }

    /// Create a new PostEntry from a File. Use an empty post id new posts.
    /// These will be filled in later after syncing with the backend.
    /// # Arguments
    /// * `file` - The file to create the PostEntry from
    /// * `title` - The title of the post
    pub fn create(file: File, title: String) -> Result<Self, Error> {
        let path = file.path();
        let post_id = 0;
        let cid = Cid::try_from(file)?;
        Ok(Self {
            path,
            post_id,
            title,
            cid,
        })
    }
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
    pub post_entries: HashMap<String, PostEntry>,
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
            post_entries: HashMap::new(),
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

    /// Get a post entry from the manifest by its File handle
    /// # Arguments
    /// * `file` - The file to get the post entry for
    /// # Returns
    /// * `Option<PostEntry>` - The post entry
    /// * `Error` - The error
    pub fn get_post_entry(&self, file: &File) -> Option<&PostEntry> {
        let path = file.path();
        self.post_entries.get(path.to_str().unwrap())
    }

    /// Add a post entry to the manifest
    /// # Arguments
    /// * 'post_entry' - The post entry to add
    /// # Returns
    /// * `Result<(), Error>` - The result
    pub fn add_post_entry(&mut self, post_entry: PostEntry) -> Result<(), Error> {
        self.post_entries.insert(post_entry.path, post_entry);
        Ok(())
    }

    /// Remove a post entry from the by its File handle
    /// # Arguments
    /// * `file` - The file to remove the post entry for
    /// # Returns
    /// * Result<(), Error> - The result
    pub fn remove_post_entry(&mut self, file: &File) -> Result<(), Error> {
        let path = file.path();
        self.post_entries.remove(path.to_str().unwrap());
        Ok(())
    }
}
