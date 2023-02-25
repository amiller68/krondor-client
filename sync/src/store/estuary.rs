use anyhow::{Error, Result};
use reqwest::{multipart, Body, Client};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};
use std::{
    fmt,
    path::{Path, PathBuf},
    borrow::Borrow,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use ethers::providers::StreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};
use std::env;

use crate::types::cid::Cid;
use crate::types::crud_file::CrudFile;

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

// TODO: Should I be initializing a ReqWest Client here, or is ok to do it in each function?
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
        let res = self.reqwest_client
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
        let res = self.reqwest_client
            .get(format!("{}/get/{}", self.estuary_api_hostname, cid.to_string()))
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
            // // Read the response body into the file. Overwrite the file if it exists.
            // let mut file = File::create(path).await?;
            // let mut body = res.bytes_stream();
            // let buffer = [0; 1024];
            // while let Some(chunk) = body.next().await {
            //     file.write_all(&buffer).await?;
            // }
            // // Return the crud file from the path
            // let c = CrudFile::new(path.clone()).unwrap();
            Ok(c)
        }
        else {
            Err(Error::msg(format!(
                "Error getting file: {}",
                res.status().as_str()
            )))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::cid::Cid;
    use crate::types::crud_file::CrudFile;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_get() {
        use std::io::Read;
        let cid = Cid::from_str("bafkreigy3be46tpdriepl3ep3o5oraqzlvo3efp4qcyh7qqxlu6l26oure".to_string()).unwrap();
        let path = PathBuf::from("test.txt");
        let estuary_client = EstuaryClient::default();
        let cf = estuary_client.get(cid, &path).await.unwrap();
        let expected = "helll wooordhelll wooord";
        println!("Got cf: {:?}", cf);
        println!("expected: {:?}", expected);
        let mut file = std::fs::File::open(cf.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("contents: {:?}", contents);
        assert_eq!(contents, expected);
        // Delete the file
        std::fs::remove_file("test.txt").unwrap();
    }
}



