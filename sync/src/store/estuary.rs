use anyhow::{Error, Result};
use reqwest::{multipart, Body, Client};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use std::fmt;
use tokio_util::codec::{BytesCodec, FramedRead};

/// EstuaryClient - A struct for managing Requests to an Estuary API
pub struct IPFSClient {
    /// The Estuary API Hostname
    pub estuary_api_hostname: String,
    /// The Estuary API Key
    pub estuary_api_key: Option<String>,
}

// TODO: Should I be initializing a ReqWest Client here, or is ok to do it in each function?
impl IPFSClient {
    /// Create a new EstuaryClient using custom values
    /// # Arguments
    /// * `estuary_api_hostname` - The Hostname of the Estuary API to use.
    /// * `estuary_api_key` - The (optional) API Key to use for the Estuary API.
    pub fn new(estuary_api_hostname: String, estuary_api_key: Option<String>) -> Self {
        Self {
            estuary_api_hostname,
            estuary_api_key,
        }
    }

    /// Stage a File on Estuary
    /// # Arguments
    /// * `path` - The path to the file to stage
    /// # Returns
    /// * `Result<(), Error>` - Errors if there is an error staging the file
    pub async fn put(&self, path: &PathBuf) -> Result<(), Error> {
        if self.estuary_api_key.is_none() {
            panic!("No Estuary API Key is set");
        }
        let estuary_api_key = self.estuary_api_key.clone().unwrap();
        // Initialize an HTTP Client
        let client = Client::new();
        // Read the File as a Tokio File
        let file = tokio::fs::File::open(&file_path).await?;
        // Read file body stream
        let file_body = Body::wrap_stream(FramedRead::new(file, BytesCodec::new()));
        // Define a Form Part for the File
        let some_file = multipart::Part::stream(file_body)
            .file_name(file_path)
            .mime_str("text/plain")?;
        // Create the multipart form
        let form = multipart::Form::new().part("data", some_file); //add the file part
                                                                   // Add the Deal ID to the form, if it exists
        let res = client
            // POST to the /content/add endpoint
            .post(format!("{}/content/add", self.estuary_api_hostname))
            // Set the Authorization Header
            .header("Authorization", format!("Bearer {}", estuary_api_key))
            // Add the Form
            .multipart(form)
            // Send the Request
            .send()
            // Await the Response
            .await?;
        // Check the Status Code
        if res.status().is_success() {
            // No Need to listen to the Response - We're good!
            Ok(())
        } else {
            Err(Error::msg(format!(
                "Error staging file: {}",
                res.status().as_str()
            )))
        }
    }

    /// Download a file from Estuary by CID
    /// # Arguments
    /// * `cid` - The CID of the file to download
    /// * `path` - The path to save the file to
    /// # Returns
    /// * `Result<(), Error>` - Errors if there is an error downloading the file
    pub async fn get
}
