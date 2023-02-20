use anyhow::{Error, Result, anyhow};
use sha3::{Digest, Keccak256};
use std::path::PathBuf;

pub fn hash_path(path: &PathBuf) -> Result<[u8; 32], Error> {
    let mut hasher = Keccak256::new();
    // Read the path as a string and hash it
    let path_string = path.to_str().unwrap().to_string();
    hasher.update(path_string.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    Ok(key)
}