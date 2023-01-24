use anyhow::{Error, Result};
use std::path::PathBuf;

use crate::manifest::Manifest;

pub enum UpdateType {
    Create,
    Update,
    Delete,
}

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

    // /// Scan the local filesystem for new files using the manifest
    // fn scan(&self) -> Result<(), Error> {
    //     // Walk the content directory
    //     for entry in walkdir::WalkDir::new(&self.content_path) {
    //         let entry = entry?;
    //         let path = entry.path();
    //         // Skip directories
    //         if path.is_dir() {
    //             continue;
    //         }
    //         // Check if the file is in the manifest
    //         let path_str = path.to_str().unwrap();
    //         if self.manifest.post_entries.contains_key(path_str) {
    //             // The file is in the manifest. Check if it has been updated
    //             let post_entry = self.manifest.post_entries.get(path_str).unwrap();
    //             let file = File::open(path)?;
    //             let cid = Cid::try_from(file)?;
    //             if cid != post_entry.cid {
    //                 // The file has been updated. Update the manifest
    //                 self.update_manifest(path_str, UpdateType::Update)?;
    //             }
    //         } else {
    //             // The file is not in the manifest. Add it
    //             self.update_manifest(path_str, UpdateType::Create)?;
    //         }
    //     }
    // }

    /// Add a file to the manifest, push to backend, and pin to IPFS
    /// # Arguments
    /// * `file` - The file to add
    /// * `title` - The title of the post
    fn add_content(&self, file: File, title: String) -> Result<(), Error> {
        // Add the file to the manifest
        let post_entry = PostEntry::create(file, title);
        self.manifest.add_post_entry(post_entry);
        Ok(())
    }

    /// Update a file in the manifest, push changes to backend, and pin nw content to IPFS
    ///
    fn update_content(&self, file: File) -> Result<(), Error> {}

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
