use anyhow::{Error, Result};
use ethers::abi::{InvalidOutputType, Token, Tokenizable};
use serde::{Deserialize, Serialize};
use std::io::ErrorKind::InvalidInput;
use std::{collections::HashMap, fs::File, path::PathBuf};

use crate::utils::hash::hash_path;
// Use our own Cid struct
use crate::types::{cid::Cid, metadata::Metadata};

// File Object - Represents a post entry in the manifest
/// # Fields
/// * `path` - The path to the file in the filesystem
/// * `key` - The key of the post. This is a 32 byte hash of the path (Keccak256)
/// * `cid` - The IPFS CID of the post
/// * `timestamp` - The timestamp of the post
/// * `metadata` - The metadata of the post. This is a JSON object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CrudFile {
    /// The path to the file in the filesystem
    pub path: PathBuf,
    /// The filename of the file in the filesystem
    pub filename: String,
    /// The key of the post. This is a 32 byte hash of the path (Keccak256)
    pub key: [u8; 32],
    /// The IPFS CID of the post
    pub cid: Cid,
    /// The timestamp of the post
    pub timestamp: u64,
    /// The metadata of the post. This is a JSON object
    pub metadata: Metadata,
}

impl CrudFile {
    /// New FileObject
    /// # Arguments
    /// * `path` - The path to the file
    /// # Returns
    /// Result<FileObject, Error>
    /// # Errors
    /// * `Error` - If the path is invalid
    /// * `Error` - If the file cannot be read
    /// * `Error` - If the CID cannot be created
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        let file = File::open(&path)?;
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let key = hash_path(&path)?;
        let cid = Cid::try_from(file)?;
        let timestamp = 0 as u64;
        let metadata = Metadata::new();
        Ok(Self {
            path,
            filename,
            key,
            cid,
            timestamp,
            metadata,
        })
    }

    /// Set Metadata
    /// # Arguments
    /// * `metadata` - The metadata to set
    /// # Returns
    /// Result<(), Error>
    pub fn set_metadata(&mut self, metadata: Metadata) -> Result<(), Error> {
        self.metadata = metadata;
        Ok(())
    }

    /// Set Timestamp
    /// # Arguments
    /// * `timestamp` - The timestamp to set
    /// # Returns
    /// Result<(), Error>
    pub fn set_timestamp(&mut self, timestamp: u64) -> Result<(), Error> {
        self.timestamp = timestamp;
        Ok(())
    }
}

impl Tokenizable for CrudFile {
    fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
        // Reject non-tuple tokens
        let mut tokens = match token {
            Token::Tuple(tokens) => tokens.into_iter(),
            other => {
                return Err(InvalidOutputType(format!(
                    "Expected `Tuple`, got {:?}",
                    other
                )))
            }
        };
        // Continue with the iterator
        let path = match tokens.next() {
            Some(Token::String(path)) => PathBuf::from(path),
            Some(other) => {
                return Err(InvalidOutputType(format!(
                    "Expected `String`, got {:?}",
                    other
                )))
            }
            None => return Err(InvalidOutputType("Expected `String`".to_string())),
        };
        // Get the filename from the path
        let filename = match path.file_name() {
            Some(filename) => filename.to_str().unwrap().to_string(),
            None => return Err(InvalidOutputType("Expected `String`".to_string())),
        };
        // Determine the Key from the path
        let key = hash_path(&path)
            .map_err(|e| InvalidOutputType(format!("Could not hash path: {:?}", e)))?;
        // Get the CID
        let cid = match tokens.next() {
            Some(Token::String(cid)) => Cid::from_str(cid)
                .map_err(|e| InvalidOutputType(format!("Expected `String`, got {:?}", e)))?,
            Some(other) => {
                return Err(InvalidOutputType(format!(
                    "Expected `String`, got {:?}",
                    other
                )))
            }
            None => return Err(InvalidOutputType("Expected `String`".to_string())),
        };
        // Get the timestamp
        let timestamp = match tokens.next() {
            Some(Token::Uint(timestamp)) => timestamp.as_u64(),
            Some(other) => {
                return Err(InvalidOutputType(format!(
                    "Expected `Uint`, got {:?}",
                    other
                )))
            }
            None => return Err(InvalidOutputType("Expected `Uint`".to_string())),
        };
        // Get the metadata
        let metadata = match tokens.next() {
            Some(Token::String(metadata)) => serde_json::from_str(&metadata)
                .map_err(|e| InvalidOutputType(format!("Expected JSON `String`, got {:?}", e)))?,
            Some(other) => {
                return Err(InvalidOutputType(format!(
                    "Expected `String`, got {:?}",
                    other
                )))
            }
            None => return Err(InvalidOutputType("Expected `String`".to_string())),
        };
        Ok(Self {
            path,
            filename,
            key,
            cid,
            timestamp,
            metadata,
        })
    }

    /// Convert a FileObject into a Token (for the purpose of C and U functions)
    fn into_token(self) -> Token {
        let path = self.path.to_str().unwrap();
        let cid = self.cid.to_string();
        let metadata = serde_json::to_string(&self.metadata).unwrap();
        Token::Tuple(vec![
            Token::String(path.to_string()),
            Token::String(cid),
            Token::String(metadata),
        ])
    }
}
