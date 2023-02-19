use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    path::PathBuf,
};
use ethers::{
    abi::{Token, Tokenizable},

};
use crate::utils::hash::hash_path;

// Use our own Cid struct
use crate::types::cid::Cid;

// File Object - Represents a post entry in the manifest
/// # Fields
/// * `path` - The path to the file in the filesystem
/// * `key` - The key of the post. This is a 32 byte hash of the path (Keccak256)
/// * `cid` - The IPFS CID of the post
/// * `timestamp` - The timestamp of the post
/// * `metadata` - The metadata of the post. This is a JSON object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileObject {
    /// The path to the file in the filesystem
    pub path: PathBuf,
    /// The key of the post. This is a 32 byte hash of the path (Keccak256)
    pub key: [u8; 32],
    /// The IPFS CID of the post
    pub cid: Cid,
    /// The timestamp of the post
    pub timestamp: u64,
    /// The metadata of the post. This is a JSON object
    pub metadata: HashMap<String, String>,
}

impl FileObject {
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
        let key = hash_path(&path)?;
        let cid = Cid::try_from(file)?;
        let timestamp = 0 as u64;
        let metadata = HashMap::new();
        Ok(Self {
            path,
            key,
            cid,
            timestamp,
            metadata,
        })
    }
}

impl Tokenizable for FileObject {
    fn from_token(token: Token) -> Result<Self, ethers::abi::InvalidOutputType> {
        todo!("Implement from_token for FileObject");
        // anyhow!(EthError::InvalidData);
        // let tokens = match token {
        //     Token::Tuple(tokens) => tokens,
        //     _ => return Err(ethers::abi::Error::InvalidData),
        // };
        // let path = tokens.get(0).ok_or(ethers::abi::Error::InvalidData)?;
        // let path = path.to_string();
        // let path = PathBuf::from(path);
        // let cid = tokens.get(1).ok_or(ethers::abi::Error::InvalidData)?;
        // let cid = cid.to_string();
        // let cid = Cid::try_from(cid)?;
        // let timestamp = tokens.get(2).ok_or(ethers::abi::Error::InvalidData)?;
        // let timestamp = timestamp.to_string();
        // let timestamp = timestamp.parse::<u64>()?;
        // let metadata = tokens.get(3).ok_or(ethers::abi::Error::InvalidData)?;
        // let metadata = metadata.to_string();
        // let metadata = serde_json::from_str(&metadata)?;
        // Ok(Self {
        //     path,
        //     cid,
        //     timestamp,
        //     metadata,
        // })
    }

    fn into_token(self) -> Token {
        let path = self.path.to_str().unwrap();
        let cid = self.cid.to_string();
        let metadata = serde_json::to_string(&self.metadata).unwrap();
        Token::Tuple(vec![
            Token::String(path.to_string()),
            Token::String(cid.to_string()),
            // Token::String(self.timestamp.to_string()),
            Token::String(metadata.to_string()),
        ])
    }
}

/// I'm lazy, this is what we're doin
pub struct FileObjects {
    pub file_objects: Vec<FileObject>,
}

impl Into<FileObjects> for FileObject {
    fn into(self) -> FileObjects {
        FileObjects {
            file_objects: vec![self],
        }
    }
}

impl From<FileObjects> for FileObject {
    fn from(file_objects: FileObjects) -> Self {
        // Raise an error if there are multiple file objects
        if file_objects.file_objects.len() > 1 {
            panic!("Multiple file objects found");
        }
        file_objects.file_objects[0].clone()
    }
}

/// Implement Necessary methods for FileObjects
/// # Methods
/// * `from_token` - Create a FileObject[] from a Token (Contract always returns tuple of arrays)
/// * `to_tokens` - Create a Token[] from a FileObject[] (Contract can only add a single file object at a time)
impl FileObjects {
    fn from_token(&self, token: Token) -> Result<Self, ethers::abi::Error> {
        todo!()
        // // The Token is either a tuple of values or a tuple of arrays
        // // We need to handle both cases
        // // If we can read a FileObject from read the token as a tuple of values, wrap it in a FileObjects and return
        // // If we can't read a FileObject from read the token as a tuple of values, read it as a tuple of arrays and try again
        // // This time read a value from each array and create a FileObject from it. Return a FileObjects with all the FileObjects
        // let tokens = token.to_tuple().unwrap();
        // // Try and read the fileobject from the vec of tokens
        // let file_object = FileObject::from_tokens(tokens.clone());
        // let file_objects = match file_object {
        //     Ok(file_object) => FileObjects {
        //         file_objects: vec![file_object],
        //     },
        //     Err(_) => {
        //         // If we can't read a file object from the tokens, try and read a bunch of file objects from the tokens
        //         // The lists must be read together, so we need to zip them together
        //
        //         // Read the path list
        //         let path_list = tokens.get(0).ok_or(ethers::abi::Error::InvalidData)?;
        //         let path_list = path_list.to_array().unwrap();
        //         // Read the cid list
        //         let cid_list = tokens.get(1).ok_or(ethers::abi::Error::InvalidData)?;
        //         let cid_list = cid_list.to_array().unwrap();
        //         // Read the timestamp list
        //         let timestamp_list = tokens.get(2).ok_or(ethers::abi::Error::InvalidData)?;
        //         let timestamp_list = timestamp_list.to_array().unwrap();
        //         // Read the metadata list
        //         let metadata_list = tokens.get(3).ok_or(ethers::abi::Error::InvalidData)?;
        //         let metadata_list = metadata_list.to_array().unwrap();
        //
        //         // Zip the lists together
        //         let zipped = path_list
        //             .iter()
        //             .zip(cid_list.iter())
        //             .zip(timestamp_list.iter())
        //             .zip(metadata_list.iter())
        //             .map(|(((path, cid), timestamp), metadata)| {
        //                 (
        //                     path.to_string(),
        //                     cid.to_string(),
        //                     timestamp.to_string(),
        //                     metadata.to_string(),
        //                 )
        //             })
        //             .collect::<Vec<(String, String, String, String)>>();
        //         // Create a vec of FileObject from the zipped lists
        //         let mut file_objects = vec![];
        //         // Read each tuple from the zipped list as a vec of tokens and create a FileObject from it
        //         for (path, cid, timestamp, metadata) in zipped {
        //             let tokens = vec![
        //                 Token::String(path),
        //                 Token::String(cid),
        //                 Token::String(timestamp),
        //                 Token::String(metadata),
        //             ];
        //             let file_object = FileObject::from_tokens(tokens)?;
        //             file_objects.push(file_object);
        //         }
        //         FileObjects { file_objects }
        //     }
        // };
    }

    fn to_tokens(&self) -> Vec<Token> {
        todo!("Implement this")
    }
}
