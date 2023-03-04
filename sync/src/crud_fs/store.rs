use crate::types::{cid::Cid, crud_file::CrudFile};
use anyhow::{Error, Result};
use ethers::providers::StreamExt;
use reqwest::{multipart, Body, Client};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use std::convert::{From, TryFrom};
use std::env;
use std::{
    borrow::Borrow,
    fmt,
    path::{Path, PathBuf},
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct StoreClient {
    estuary_client: EstuaryClient,
}

impl Default for StoreClient {
    fn default() -> Self {
        let estuary_client = EstuaryClient::default();
        Self::new(estuary_client)
    }
}

impl StoreClient {
    pub fn new(estuary_client: EstuaryClient) -> Self {
        Self { estuary_client }
    }

    /// Put a CrudFile into the store - placed from the current directory
    /// # Arguments
    /// - `crud_file` - The CrudFile to put into the store
    /// # Returns
    /// - `Result<(), Error>` - The result of the operation
    pub async fn put(&self, crud_file: CrudFile) -> Result<(), Error> {
        self.estuary_client.put(crud_file).await?;
        Ok(())
    }

    /// Get a CrudFile from the store - placed from the current directory
    /// # Arguments
    /// - `crud_file` - The CrudFile to get from the store
    /// # Returns
    /// - `Result<(), Error>` - The result of the operation
    pub async fn get(&self, cid: Cid, path: PathBuf) -> Result<CrudFile, Error> {
        let crud_file = self.estuary_client.get(cid, &path).await?;
        Ok(crud_file)
    }
}

#[cfg(test)]
mod tests {
    use ethers::abi::Word;
    use std::io::Write;

    #[tokio::test]
    /// Put and Get a file from the store
    async fn test_put_get() {
        use super::*;
        use rand::Rng;
        use std::fs::File;
        use std::io::{Read, Write};

        // Initialize the Client from the .env file
        dotenv::from_path("./../../env/.env").ok();
        let mut client = StoreClient::default();

        // Create test file
        // Create a test directory
        let test_dir = PathBuf::from(format!("test"));
        std::fs::create_dir_all(&test_dir).unwrap();
        let path = PathBuf::from(format!("test/file.txt"));
        let mut file = File::create(&path).unwrap();
        let message = "helll wooord";
        let data = message.as_bytes();
        file.write_all(data).unwrap();

        file.write_all(data).unwrap();

        // Get a Crud file from the path
        let crud_file = CrudFile::new(path).unwrap();
        // Put the file into the store
        println!("Putting {:?}", crud_file);
        client.put(crud_file.clone()).await.unwrap();
        // Get the file from the store. Put into a new path
        let new_path = PathBuf::from(format!("test/file2.txt"));
        let new_crud_file = client
            .get(crud_file.cid.clone(), new_path.clone())
            .await
            .unwrap();
        // Assert that the new file is the same as the old file
        println!("Got {:?}", new_crud_file);
        // Assert the Cid is the same
        assert_eq!(crud_file.cid, new_crud_file.cid);
        assert_eq!(new_path, new_crud_file.path);
        // Assert the content of their paths are the same
        // Read the content of the new file
        let mut new_file = File::open(new_crud_file.path).unwrap();
        let mut new_data: [u8; 12] = [0; 12];
        new_file.read_exact(&mut new_data).unwrap();

        // Read the content of the old file
        assert_eq!(data, new_data);
        // Remove the test directory
        std::fs::remove_dir_all(test_dir).unwrap();
    }
}

/* Estuary Client */

/// EstuaryClient - A struct for managing Requests to an Estuary API
pub struct EstuaryClient {
    /// The Estuary API Hostname
    pub estuary_api_hostname: String,
    /// The Estuary API Key
    pub estuary_api_key: String,
    /// The Reqwest Client
    reqwest_client: Client,
}

impl Default for EstuaryClient {
    fn default() -> Self {
        Self {
            estuary_api_hostname: String::from("https://api.estuary.tech"),
            estuary_api_key: env::var("ESTUARY_API_KEY").expect("ESTUARY_API_KEY must be set"),
            reqwest_client: Client::new(),
        }
    }
}

impl EstuaryClient {
    /// Create a new EstuaryClient using custom values
    /// # Arguments
    /// * `estuary_api_hostname` - The Hostname of the Estuary API to use.
    /// * `estuary_api_key` - The (optional) API Key to use for the Estuary API.
    pub fn new(estuary_api_hostname: String, estuary_api_key: String) -> Self {
        let reqwest_client = Client::new();
        Self {
            estuary_api_hostname,
            estuary_api_key,
            reqwest_client,
        }
    }

    /// Stage a File on Estuary
    /// # Arguments
    /// * `crud_file` - The CrudFile to stage on Estuary
    /// # Returns
    /// * `Result<(), Error>` - Errors if there is an error staging the file
    // pub async fn put(&self, path: PathBuf) -> Result<(), Error> {
    pub async fn put(&self, crud_file: CrudFile) -> Result<(), Error> {
        // Get the filename from the of the crud_file, and then open the file
        let mut file = File::open(crud_file.path).await?;
        // Read the file into a body stream
        let file_body = Body::wrap_stream(FramedRead::new(file, BytesCodec::new()));
        // Define a Form Part for the File
        let some_file = multipart::Part::stream(file_body)
            .file_name(crud_file.filename)
            .mime_str("text/plain")?;

        // Create the multipart form
        let form = multipart::Form::new().part("data", some_file); //add the file part
                                                                   // Add the Deal ID to the form, if it exists
        let res = self
            .reqwest_client
            // POST to the /content/add endpoint
            .post(format!("{}/content/add", self.estuary_api_hostname))
            // Set the Authorization Header
            .header("Authorization", format!("Bearer {}", self.estuary_api_key))
            // Add the Form
            .multipart(form)
            // Send the Request
            .send()
            // Await the Response
            .await?;
        // Check the Status Code
        if res.status().is_success() {
            // No Need to listen to the Response - We're good!
            res.text().await?;
            Ok(())
        } else {
            Err(Error::msg(format!(
                "Error putting file: {}",
                res.status().as_str()
            )))
        }
    }

    /// Download a file from Estuary by CID
    /// # Arguments
    /// * `cid` - The CID of the file to download
    /// * `path` - The path to save the file to - relative to the current working directory
    /// # Returns
    /// * `Result<(), Error>` - Errors if there is an error downloading the file
    pub async fn get(&self, cid: Cid, path: &PathBuf) -> Result<CrudFile, Error> {
        // Reqwest GET Request
        let res = self
            .reqwest_client
            .get(format!(
                "{}/get/{}",
                self.estuary_api_hostname,
                cid.to_string()
            ))
            .header("Application", "application/json")
            .send()
            .await?;
        // Check the Status Code
        if res.status().is_success() {
            // Get the response body as a stream
            let res = res.text().await?;
            // Read the response body into the file. Overwrite the file if it exists.
            let mut file = File::create(path).await?;
            file.write_all(res.as_bytes()).await?;
            // Return the crud file from the path
            let c = CrudFile::new(path.clone()).unwrap();
            Ok(c)
        } else {
            Err(Error::msg(format!(
                "Error getting file: {}",
                res.status().as_str()
            )))
        }
    }
}
