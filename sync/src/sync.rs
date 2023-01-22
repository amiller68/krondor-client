use anyhow::{Error, Result};
use std::path::PathBuf;

use crate::manifest::Manifest;

/// Struct for managing the syncing process
pub struct Sync {
    /// The manifest
    manifest: Manifest,
    /// The path to the content directory
    content_path: PathBuf,
}

impl Sync {
    /* Constructors and Interfaces */

    /// Create a new Sync instance
    pub fn new(manifest: Manifest, content_path: PathBuf) -> Self {
        Self {
            manifest,
            content_path,
        }
    }

    /// Sync the local filesystem with the manifest, backend, and IPFS
    pub fn sync(&self) -> Result<(), Error> {
        panic!("Not implemented");
    }

    /* Private Methods */

    /// Add a file to the manifest, push to backend, and pin to IPFS
    fn add_content(&self) -> Result<(), Error> {
        panic!("Not implemented");
    }
    /// Update a file in the manifest, push changes to backend, and pin nw content to IPFS
    fn update_content(&self) -> Result<(), Error> {
        panic!("Not implemented");
    }

    /// Remove a file from the manifest, remove from backend, and unpin from IPFS
    fn delete_content(&self) -> Result<(), Error> {
        panic!("Not implemented");
    }

    /// Verify the manifest, backend, and IPFS against each other
    /// # Arguments
    fn verify(&self) -> Result<(), Error> {
        panic!("Not implemented");
    }
}
