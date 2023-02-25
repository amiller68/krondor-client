use anyhow::{Error, Result, anyhow};
use std::{
    convert::{From, TryFrom},
    path::PathBuf,
    env
};
use crate::{
    types::{
        crud_file::CrudFile,
        cid::Cid,
    },
    store::estuary::EstuaryClient,
};

// TODO: Refactor this into a trait so we can support multiple stores

pub struct StoreClient {
    estuary_client: EstuaryClient
}

impl Default for StoreClient {
    fn default() -> Self {
        let estuary_client = EstuaryClient::default();
        Self::new(estuary_client)
    }
}

impl StoreClient {
    pub fn new(estuary_client: EstuaryClient) -> Self {
        Self {
            estuary_client
        }
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
    use std::io::Write;
    use ethers::abi::Word;

    #[tokio::test]
    /// Put and Get a file from the store
    async fn test_put_get() {
        use super::*;
        use rand::Rng;
        use std::fs::File;
        use std::io::{Write, Read};

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
        let new_crud_file = client.get(crud_file.cid.clone(), new_path.clone()).await.unwrap();
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
